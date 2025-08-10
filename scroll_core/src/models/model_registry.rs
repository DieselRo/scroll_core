//! Model registry: resolves provider/model per construct from ENV and optional YAML.
//! Load order: ENV > models.yaml > built-in defaults.
//! Exposes resolved ModelSpec and CostProfile thresholds.

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    OpenAI,
    Local,
    Anthropic,
    Mock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpec {
    pub provider: Provider,
    pub model: String,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
    #[serde(default)]
    pub extra: serde_json::Value, // escape hatch for provider-specific knobs
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThresholdCostProfile {
    pub daily_usd_cap: Option<f32>,
    pub per_request_usd_limit: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextThresholds {
    pub max_context_tokens: usize,
    pub min_relevance_score: f32,
    pub recency_half_life_hours: f32,
    pub max_items: usize,
}

impl Default for ContextThresholds {
    fn default() -> Self {
        Self {
            max_context_tokens: 3000,
            min_relevance_score: 0.35,
            recency_half_life_hours: 48.0,
            max_items: 12,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextConfig {
    pub defaults: ContextThresholds,
    #[serde(default)]
    pub constructs: HashMap<String, ContextThresholds>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelsConfig {
    pub version: u32,
    pub default: Option<ModelSpec>,
    #[serde(default)]
    pub constructs: HashMap<String, ModelSpec>,
    #[serde(default)]
    pub cost_profiles: HashMap<String, ThresholdCostProfile>,
    #[serde(default)]
    pub context: Option<ContextConfig>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RedactedConfig {
    pub source_file: Option<PathBuf>,
    pub default: ModelSpec,
    pub constructs: HashMap<String, ModelSpec>,
    pub cost_profiles: HashMap<String, ThresholdCostProfile>,
    pub context: ContextConfig,
}

pub struct ModelRegistry {
    default_spec: ModelSpec,
    constructs: HashMap<String, ModelSpec>,
    cost_profiles: HashMap<String, ThresholdCostProfile>,
    source_file: Option<PathBuf>,
    context_defaults: ContextThresholds,
    context_constructs: HashMap<String, ContextThresholds>,
}

#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("config load error: {0}")]
    Load(String),
    #[error("unknown construct: {0}")]
    UnknownConstruct(String),
    #[error("invalid env override: {0}")]
    Env(String),
}

static GLOBAL_REGISTRY: OnceCell<std::sync::Arc<ModelRegistry>> = OnceCell::new();

impl ModelRegistry {
    pub fn set_global(
        reg: std::sync::Arc<ModelRegistry>,
    ) -> Result<(), std::sync::Arc<ModelRegistry>> {
        GLOBAL_REGISTRY.set(reg)
    }

    pub fn get_global() -> Option<&'static std::sync::Arc<ModelRegistry>> {
        GLOBAL_REGISTRY.get()
    }

    pub fn from_env_and_file(path: Option<&Path>) -> Result<Self, RegistryError> {
        // 1) built-in defaults
        let mut default_spec = builtin_default_spec();
        let mut constructs: HashMap<String, ModelSpec> = HashMap::new();
        let mut cost_profiles: HashMap<String, ThresholdCostProfile> = HashMap::new();
        let mut source_file: Option<PathBuf> = None;
        let mut context_defaults: ContextThresholds = ContextThresholds::default();
        let mut context_constructs: HashMap<String, ContextThresholds> = HashMap::new();

        // 2) load YAML if present
        let yaml_path = resolve_config_path(path);
        if let Some(p) = yaml_path {
            match std::fs::read_to_string(&p) {
                Ok(raw) => match serde_yaml::from_str::<ModelsConfig>(&raw) {
                    Ok(cfg) => {
                        source_file = Some(p.clone());
                        if let Some(def) = cfg.default {
                            default_spec = def;
                        }
                        constructs = cfg.constructs;
                        cost_profiles = cfg.cost_profiles;
                        if let Some(ctx) = cfg.context {
                            context_defaults = ctx.defaults;
                            context_constructs = ctx.constructs;
                        }
                    }
                    Err(e) => return Err(RegistryError::Load(e.to_string())),
                },
                Err(e) => return Err(RegistryError::Load(e.to_string())),
            }
        }

        // 3) apply ENV overrides
        apply_global_env_overrides(&mut default_spec)?;
        apply_construct_env_overrides(&mut constructs)?;
        apply_cost_env_overrides(&mut cost_profiles)?;
        apply_context_env_overrides(&mut context_defaults, &mut context_constructs)?;

        Ok(Self {
            default_spec,
            constructs,
            cost_profiles,
            source_file,
            context_defaults,
            context_constructs,
        })
    }

    pub fn by_construct(&self, name: &str) -> Result<ModelSpec, RegistryError> {
        // ENV construct-specific overrides take highest precedence and are already applied in the map
        // Next: YAML constructs map
        // Fallback: default spec
        if let Some(spec) = self.constructs.get(name) {
            return Ok(spec.clone());
        }
        // If not present, return default; this keeps env-only behavior
        Ok(self.default_spec.clone())
    }

    pub fn cost_profile(&self, name: &str) -> ThresholdCostProfile {
        self.cost_profiles
            .get(name)
            .cloned()
            .or_else(|| self.cost_profiles.get("default").cloned())
            .unwrap_or_default()
    }

    pub fn effective_config(&self) -> RedactedConfig {
        RedactedConfig {
            source_file: self.source_file.clone(),
            default: self.default_spec.clone(),
            constructs: self.constructs.clone(),
            cost_profiles: self.cost_profiles.clone(),
            context: ContextConfig {
                defaults: self.context_defaults.clone(),
                constructs: self.context_constructs.clone(),
            },
        }
    }

    pub fn context_for(&self, construct: &str) -> ContextThresholds {
        self.context_constructs
            .get(construct)
            .cloned()
            .unwrap_or_else(|| self.context_defaults.clone())
    }
}

fn builtin_default_spec() -> ModelSpec {
    // Keep current env-only behavior by reading the same env used by factory today
    let provider_str = std::env::var("SC_LLM_PROVIDER").ok().unwrap_or_else(|| {
        if cfg!(test) {
            "mock".into()
        } else {
            "openai".into()
        }
    });
    let provider = match provider_str.to_lowercase().as_str() {
        "mock" => Provider::Mock,
        "local" => Provider::Local,
        "anthropic" => Provider::Anthropic,
        _ => Provider::OpenAI,
    };
    let model = std::env::var("SC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o".to_string());
    let max_output_tokens = std::env::var("SC_LLM_MAX_OUTPUT_TOKENS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok());
    let temperature = std::env::var("SC_LLM_TEMPERATURE")
        .ok()
        .and_then(|s| s.parse::<f32>().ok());
    ModelSpec {
        provider,
        model,
        max_output_tokens,
        temperature,
        extra: serde_json::Value::Object(serde_json::Map::new()),
    }
}

fn resolve_config_path(path: Option<&Path>) -> Option<PathBuf> {
    if let Some(p) = path {
        return Some(p.to_path_buf());
    }
    if let Ok(env_path) = std::env::var("SC_MODELS_CONFIG") {
        return Some(PathBuf::from(env_path));
    }
    let default = PathBuf::from("config/models.yaml");
    if default.exists() {
        Some(default)
    } else {
        None
    }
}

fn apply_global_env_overrides(default: &mut ModelSpec) -> Result<(), RegistryError> {
    if let Ok(provider) = std::env::var("SC_LLM_PROVIDER") {
        default.provider = match provider.to_lowercase().as_str() {
            "mock" => Provider::Mock,
            "local" => Provider::Local,
            "anthropic" => Provider::Anthropic,
            _ => Provider::OpenAI,
        };
    }
    if let Ok(model) = std::env::var("SC_LLM_MODEL") {
        default.model = model;
    }
    if let Ok(m) = std::env::var("SC_LLM_MAX_OUTPUT_TOKENS") {
        default.max_output_tokens = m.parse().ok();
    }
    if let Ok(t) = std::env::var("SC_LLM_TEMPERATURE") {
        default.temperature = t.parse().ok();
    }
    Ok(())
}

// Per-construct env names: SC_MODEL_<NAME>_PROVIDER, SC_MODEL_<NAME>_MODEL, SC_MODEL_<NAME>_MAX_OUTPUT_TOKENS, SC_MODEL_<NAME>_TEMPERATURE
fn apply_construct_env_overrides(
    constructs: &mut HashMap<String, ModelSpec>,
) -> Result<(), RegistryError> {
    // scan all env vars starting with SC_MODEL_
    for (k, v) in std::env::vars() {
        if !k.starts_with("SC_MODEL_") {
            continue;
        }
        // format: SC_MODEL_{NAME}_KEY
        let rest = &k[9..];
        let (name, key) = match rest.rsplit_once('_') {
            Some((n, k)) => (n.to_string(), k.to_string()),
            None => continue,
        };
        let entry = constructs
            .entry(name.clone())
            .or_insert_with(builtin_default_spec);
        match key.as_str() {
            "PROVIDER" => {
                entry.provider = match v.to_lowercase().as_str() {
                    "mock" => Provider::Mock,
                    "local" => Provider::Local,
                    "anthropic" => Provider::Anthropic,
                    _ => Provider::OpenAI,
                };
            }
            "MODEL" => entry.model = v,
            "MAX_OUTPUT_TOKENS" => entry.max_output_tokens = v.parse::<u32>().ok(),
            "TEMPERATURE" => entry.temperature = v.parse::<f32>().ok(),
            _ => {}
        }
    }
    Ok(())
}

// Env overrides for cost thresholds: global SC_COST_DAILY_USD_CAP, SC_COST_PER_REQUEST_USD_LIMIT and per-construct SC_COST_<NAME>_DAILY_USD_CAP, SC_COST_<NAME>_PER_REQUEST_USD_LIMIT
fn apply_cost_env_overrides(
    costs: &mut HashMap<String, ThresholdCostProfile>,
) -> Result<(), RegistryError> {
    let default = costs.entry("default".into()).or_default();
    if let Ok(cap) = std::env::var("SC_COST_DAILY_USD_CAP") {
        default.daily_usd_cap = cap.parse::<f32>().ok();
    }
    if let Ok(limit) = std::env::var("SC_COST_PER_REQUEST_USD_LIMIT") {
        default.per_request_usd_limit = limit.parse::<f32>().ok();
    }
    for (k, v) in std::env::vars() {
        if !k.starts_with("SC_COST_") {
            continue;
        }
        // SC_COST_{NAME}_KEY
        let rest = &k[8..];
        let (name, key) = match rest.rsplit_once('_') {
            Some((n, k)) => (n.to_string(), k.to_string()),
            None => continue,
        };
        let entry = costs.entry(name.clone()).or_default();
        match key.as_str() {
            "DAILY_USD_CAP" => entry.daily_usd_cap = v.parse::<f32>().ok(),
            "PER_REQUEST_USD_LIMIT" => entry.per_request_usd_limit = v.parse::<f32>().ok(),
            _ => {}
        }
    }
    Ok(())
}

// Env overrides for context thresholds
// Global vars: SC_CONTEXT_MAX_TOKENS, SC_CONTEXT_MIN_RELEVANCE, SC_CONTEXT_RECENCY_HALF_LIFE_HOURS, SC_CONTEXT_MAX_ITEMS
// Per-construct: SC_CONTEXT_<NAME>_MAX_TOKENS, ... similarly suffixed
fn apply_context_env_overrides(
    defaults: &mut ContextThresholds,
    constructs: &mut HashMap<String, ContextThresholds>,
) -> Result<(), RegistryError> {
    if let Ok(v) = std::env::var("SC_CONTEXT_MAX_TOKENS") {
        if let Ok(n) = v.parse::<usize>() {
            defaults.max_context_tokens = n;
        }
    }
    if let Ok(v) = std::env::var("SC_CONTEXT_MIN_RELEVANCE") {
        if let Ok(f) = v.parse::<f32>() {
            defaults.min_relevance_score = f;
        }
    }
    if let Ok(v) = std::env::var("SC_CONTEXT_RECENCY_HALF_LIFE_HOURS") {
        if let Ok(f) = v.parse::<f32>() {
            defaults.recency_half_life_hours = f;
        }
    }
    if let Ok(v) = std::env::var("SC_CONTEXT_MAX_ITEMS") {
        if let Ok(n) = v.parse::<usize>() {
            defaults.max_items = n;
        }
    }
    // Per-construct
    for (k, v) in std::env::vars() {
        if !k.starts_with("SC_CONTEXT_") {
            continue;
        }
        // SC_CONTEXT_{NAME}_KEY
        let rest = &k[11..];
        let (name, key) = match rest.rsplit_once('_') {
            Some((n, k)) => (n.to_string(), k.to_string()),
            None => continue,
        };
        let entry = constructs
            .entry(name.clone())
            .or_insert_with(ContextThresholds::default);
        match key.as_str() {
            "MAX_TOKENS" => {
                if let Ok(n) = v.parse::<usize>() {
                    entry.max_context_tokens = n;
                }
            }
            "MIN_RELEVANCE" => {
                if let Ok(f) = v.parse::<f32>() {
                    entry.min_relevance_score = f;
                }
            }
            "RECENCY_HALF_LIFE_HOURS" => {
                if let Ok(f) = v.parse::<f32>() {
                    entry.recency_half_life_hours = f;
                }
            }
            "MAX_ITEMS" => {
                if let Ok(n) = v.parse::<usize>() {
                    entry.max_items = n;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

// ──────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────
impl Default for ModelSpec {
    fn default() -> Self {
        builtin_default_spec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn precedence_env_over_yaml_over_default() {
        let dir = tempdir().unwrap();
        let yaml = dir.path().join("models.yaml");
        fs::write(
            &yaml,
            r#"
version: 1
default:
  provider: openai
  model: gpt-4.1-mini
constructs:
  Mythscribe:
    provider: openai
    model: gpt-4.1
cost_profiles:
  default:
    daily_usd_cap: 5.0
  Mythscribe:
    per_request_usd_limit: 0.2
"#,
        )
        .unwrap();

        std::env::set_var("SC_LLM_PROVIDER", "mock");
        std::env::set_var("SC_MODEL_Mythscribe_MODEL", "override-model");
        let reg = ModelRegistry::from_env_and_file(Some(&yaml)).unwrap();
        let myth = reg.by_construct("Mythscribe").unwrap();
        assert_eq!(myth.model, "override-model");
        // default spec takes env provider mock
        let def = reg.by_construct("Unknown").unwrap();
        assert_eq!(matches!(def.provider, Provider::Mock), true);
        // cost profiles fall back to default
        let c = reg.cost_profile("Unknown");
        assert_eq!(c.daily_usd_cap, Some(5.0));

        // cleanup env
        std::env::remove_var("SC_LLM_PROVIDER");
        std::env::remove_var("SC_MODEL_Mythscribe_MODEL");
    }

    #[test]
    fn no_yaml_env_only_behaves() {
        std::env::set_var("SC_LLM_PROVIDER", "mock");
        std::env::set_var("SC_LLM_MODEL", "gpt-test");
        let reg = ModelRegistry::from_env_and_file(None).unwrap();
        let spec = reg.by_construct("Anything").unwrap();
        assert!(matches!(spec.provider, Provider::Mock));
        assert_eq!(spec.model, "gpt-test");
        std::env::remove_var("SC_LLM_PROVIDER");
        std::env::remove_var("SC_LLM_MODEL");
    }

    #[test]
    fn context_thresholds_precedence_env_over_yaml_over_default() {
        let dir = tempdir().unwrap();
        let yaml = dir.path().join("models.yaml");
        fs::write(
            &yaml,
            r#"
version: 1
context:
  defaults:
    max_context_tokens: 3000
    min_relevance_score: 0.4
    recency_half_life_hours: 24
    max_items: 10
  constructs:
    Mythscribe:
      max_context_tokens: 5000
      min_relevance_score: 0.5
      recency_half_life_hours: 12
      max_items: 16
"#,
        )
        .unwrap();

        std::env::set_var("SC_CONTEXT_MAX_TOKENS", "2000");
        std::env::set_var("SC_CONTEXT_Mythscribe_MAX_ITEMS", "8");

        let reg = ModelRegistry::from_env_and_file(Some(&yaml)).unwrap();
        // Global default max tokens overridden by env
        let def = reg.context_for("Unknown");
        assert_eq!(def.max_context_tokens, 2000);
        // Per-construct override merged with env
        let ms = reg.context_for("Mythscribe");
        assert_eq!(ms.max_items, 8);
        assert!((ms.min_relevance_score - 0.5).abs() < 1e-6);

        std::env::remove_var("SC_CONTEXT_MAX_TOKENS");
        std::env::remove_var("SC_CONTEXT_Mythscribe_MAX_ITEMS");
    }
}
