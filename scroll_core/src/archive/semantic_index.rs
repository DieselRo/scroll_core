//! Manages embeddings and similarity search for scrolls.
//! The index allows constructs to retrieve related content based on semantic similarity.
//! See [Loreweaver](../../AGENTS.md#loreweaver) for narrative use cases.
// src/archive/semantic_index.rs

use log::{info, warn};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use uuid::Uuid;

#[cfg(feature = "metrics")]
use metrics::histogram;

use crate::archive::error::ArchiveError;
use crate::scroll::Scroll;

pub trait Embedder {
    fn embed(&self, text: &str) -> Result<HashSet<String>, ArchiveError>;
}

pub struct TokenEmbedder;

impl Embedder for TokenEmbedder {
    fn embed(&self, text: &str) -> Result<HashSet<String>, ArchiveError> {
        Ok(tokenize(text))
    }
}

pub struct MockEmbedder;

impl Embedder for MockEmbedder {
    fn embed(&self, _text: &str) -> Result<HashSet<String>, ArchiveError> {
        Ok(HashSet::new())
    }
}

pub struct SemanticIndex {
    pub vectors: Vec<(Uuid, HashSet<String>)>,
}

impl SemanticIndex {
    pub fn build(scrolls: &[Scroll], embedder: &dyn Embedder) -> Result<Self, ArchiveError> {
        if scrolls.is_empty() {
            return Err(ArchiveError::EmptyScrollSet);
        }

        info!("Generating semantic vectors for {} scrolls", scrolls.len());

        #[cfg(feature = "metrics")]
        let build_timer = std::time::Instant::now();

        let vectors = scrolls
            .iter()
            .map(|s| {
                #[cfg(feature = "metrics")]
                let embed_timer = std::time::Instant::now();

                let first_lines = s
                    .markdown_body
                    .lines()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                let text = format!(
                    "{} {} {}",
                    s.title,
                    s.yaml_metadata.tags.join(" "),
                    first_lines
                );
                let vec = embedder.embed(&text)?;

                #[cfg(feature = "metrics")]
                histogram!("scroll_embed_time_seconds").record(embed_timer.elapsed().as_secs_f64());

                Ok((s.id, vec))
            })
            .collect::<Result<Vec<_>, ArchiveError>>()?;

        #[cfg(feature = "metrics")]
        {
            histogram!("vector_index_update_time_seconds")
                .record(build_timer.elapsed().as_secs_f64());
            let mut bytes: usize = std::mem::size_of_val(&vectors);
            for (_, tokens) in &vectors {
                bytes += std::mem::size_of_val(tokens);
                for t in tokens {
                    bytes += t.len();
                }
            }
            histogram!("vector_index_memory_bytes").record(bytes as f64);
        }

        info!("Vector generation complete");
        Ok(Self { vectors })
    }

    pub fn query(&self, input: &str, k: usize) -> Vec<(Uuid, f32)> {
        info!("Performing k-NN search for '{input}'");
        let query_tokens = tokenize(input);
        let mut scores: Vec<(Uuid, f32)> = self
            .vectors
            .iter()
            .map(|(id, tokens)| (*id, jaccard_similarity(tokens, &query_tokens)))
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.into_iter().take(k).collect()
    }
}

fn tokenize(text: &str) -> HashSet<String> {
    text.to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn jaccard_similarity(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    let intersection = a.intersection(b).count() as f32;
    let union = a.union(b).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

// ───────────────────────────────────────────────────────────────────────────────
// Persistence (v1)
// ───────────────────────────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct IndexMetaV1 {
    pub version: u32,           // 1
    pub embedder_model: String, // e.g., "token-embedder"
    pub embedding_dim: usize,   // tokens embedder uses 0
    pub hash_algo: String,      // e.g., "sha256"
    pub created_at: String,     // RFC3339
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Fingerprint {
    pub mtime: i64, // unix seconds
    pub size: u64,
    pub content_hash: String, // hex
}

#[derive(Default, Clone)]
struct CachePaths {
    dir: PathBuf,
    meta: PathBuf,
    vec_bin: PathBuf,
    fingerprints: PathBuf,
    lock: PathBuf,
}

impl CachePaths {
    fn resolve() -> Option<Self> {
        if std::env::var("SC_DISABLE_INDEX_CACHE").ok().as_deref() == Some("1") {
            return None;
        }

        if let Ok(custom) = std::env::var("SC_INDEX_CACHE_DIR") {
            let base = PathBuf::from(custom);
            return Some(Self::from_dir(base));
        }

        #[cfg(windows)]
        {
            if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
                let base = Path::new(&appdata)
                    .join("ScrollCore")
                    .join("cache")
                    .join("semantic_index");
                return Some(Self::from_dir(base));
            }
        }

        // Unix-like
        let base = if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
            PathBuf::from(xdg).join("scrollcore").join("semantic_index")
        } else if let Some(home_dir) = home::home_dir() {
            home_dir
                .join(".cache")
                .join("scrollcore")
                .join("semantic_index")
        } else {
            return None;
        };
        Some(Self::from_dir(base))
    }

    fn from_dir(base: PathBuf) -> Self {
        Self {
            dir: base.clone(),
            meta: base.join("semantic_index.v1.meta.json"),
            vec_bin: base.join("semantic_index.v1.vec.bin"),
            fingerprints: base.join("fingerprints.v1.json"),
            lock: base.join(".rebuild.lock"),
        }
    }
}

fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

fn normalize_text(s: &Scroll) -> String {
    let first_lines = s
        .markdown_body
        .lines()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "{} {} {}",
        s.title,
        s.yaml_metadata.tags.join(" "),
        first_lines
    )
}

fn sha256_hex(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let out = hasher.finalize();
    out.iter().map(|b| format!("{b:02x}")).collect()
}

fn fingerprint_for_scroll(path: &Path, s: &Scroll) -> std::io::Result<Fingerprint> {
    let meta = fs::metadata(path)?;
    let mtime = meta
        .modified()
        .unwrap_or(SystemTime::UNIX_EPOCH)
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let size = meta.len();
    let content_hash = sha256_hex(&normalize_text(s));
    Ok(Fingerprint {
        mtime,
        size,
        content_hash,
    })
}

// removed legacy lock helper; using presence check on lock file instead

fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    {
        let mut f = File::create(&tmp)?;
        f.write_all(bytes)?;
        f.flush()?;
    }
    fs::rename(tmp, path)?;
    Ok(())
}

fn write_json_atomic<T: serde::Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    let data = serde_json::to_vec_pretty(value).map_err(|e| std::io::Error::other(e))?;
    write_atomic(path, &data)
}

fn serialize_vectors_bin(
    order: &[PathBuf],
    id_to_tokens: &HashMap<Uuid, HashSet<String>>,
    path_to_id: &HashMap<PathBuf, Uuid>,
) -> Vec<u8> {
    // Format: repeated records of [uuid(16)][count u32][token_len u16][token bytes]...
    let mut out: Vec<u8> = Vec::new();
    for p in order {
        if let Some(id) = path_to_id.get(p) {
            if let Some(tokens) = id_to_tokens.get(id) {
                // uuid 16
                out.extend_from_slice(id.as_bytes());
                // count
                let count = tokens.len() as u32;
                out.extend_from_slice(&count.to_le_bytes());
                for t in tokens {
                    let b = t.as_bytes();
                    let len = b.len() as u16;
                    out.extend_from_slice(&len.to_le_bytes());
                    out.extend_from_slice(b);
                }
            }
        }
    }
    out
}

fn deserialize_vectors_bin(
    order: &[PathBuf],
    bytes: &[u8],
) -> Option<HashMap<PathBuf, (Uuid, HashSet<String>)>> {
    let mut cursor = 0usize;
    let mut out: HashMap<PathBuf, (Uuid, HashSet<String>)> = HashMap::new();
    for p in order {
        if cursor + 16 + 4 > bytes.len() {
            return None;
        }
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes.copy_from_slice(&bytes[cursor..cursor + 16]);
        cursor += 16;
        let mut count_bytes = [0u8; 4];
        count_bytes.copy_from_slice(&bytes[cursor..cursor + 4]);
        cursor += 4;
        let count = u32::from_le_bytes(count_bytes) as usize;
        let mut set: HashSet<String> = HashSet::with_capacity(count);
        for _ in 0..count {
            if cursor + 2 > bytes.len() {
                return None;
            }
            let mut len_bytes = [0u8; 2];
            len_bytes.copy_from_slice(&bytes[cursor..cursor + 2]);
            cursor += 2;
            let len = u16::from_le_bytes(len_bytes) as usize;
            if cursor + len > bytes.len() {
                return None;
            }
            let s = String::from_utf8_lossy(&bytes[cursor..cursor + len]).to_string();
            cursor += len;
            set.insert(s);
        }
        let id = Uuid::from_bytes(uuid_bytes);
        out.insert(p.clone(), (id, set));
    }
    Some(out)
}

impl SemanticIndex {
    /// Load from cache if possible and incrementally rebuild invalid entries, then save.
    /// Behavior is controlled by env vars:
    /// - SC_DISABLE_INDEX_CACHE=1 to disable cache
    /// - SC_REBUILD_INDEX=1 to force rebuild (ignores existing cache)
    /// - SC_REINDEX_PATH=/path/to/scroll.md to force re-embed only that path
    /// - SC_EMBEDDER_MODEL overrides meta embedder name
    /// - SC_EMBEDDING_DIM overrides meta embedding dimension
    pub fn load_or_build(
        scrolls: &[Scroll],
        embedder: &dyn Embedder,
    ) -> Result<Self, ArchiveError> {
        // Metadata inputs
        let embedder_model =
            std::env::var("SC_EMBEDDER_MODEL").unwrap_or_else(|_| "token-embedder".into());
        let embedding_dim: usize = std::env::var("SC_EMBEDDING_DIM")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let meta_target = IndexMetaV1 {
            version: 1,
            embedder_model: embedder_model.clone(),
            embedding_dim,
            hash_algo: "sha256".into(),
            created_at: now_rfc3339(),
        };

        let force_full_rebuild = std::env::var("SC_REBUILD_INDEX").ok().as_deref() == Some("1");
        let forced_path = std::env::var("SC_REINDEX_PATH").ok().map(PathBuf::from);

        let maybe_paths = CachePaths::resolve();
        if maybe_paths.is_none() {
            // Fallback: build fresh, no persistence
            return Self::build(scrolls, embedder);
        }
        let paths = maybe_paths.unwrap();
        let _ = fs::create_dir_all(&paths.dir);

        // Build path map (for those with known file paths)
        let mut path_to_scroll: HashMap<PathBuf, &Scroll> = HashMap::new();
        for s in scrolls {
            if let Some(p) = &s.yaml_metadata.file_path {
                path_to_scroll.insert(PathBuf::from(p), s);
            }
        }

        // Load existing cache if present and usable
        let mut cache_valid = !force_full_rebuild;
        let mut cached_vectors: HashMap<PathBuf, (Uuid, HashSet<String>)> = HashMap::new();
        let mut cached_fingerprints: HashMap<PathBuf, Fingerprint> = HashMap::new();
        if cache_valid
            && paths.meta.exists()
            && paths.vec_bin.exists()
            && paths.fingerprints.exists()
        {
            match (
                fs::read(&paths.meta),
                fs::read(&paths.fingerprints),
                fs::read(&paths.vec_bin),
            ) {
                (Ok(meta_bytes), Ok(fp_bytes), Ok(vec_bytes)) => {
                    let loaded_meta: Result<IndexMetaV1, _> = serde_json::from_slice(&meta_bytes);
                    let loaded_fps: Result<HashMap<PathBuf, Fingerprint>, _> =
                        serde_json::from_slice(&fp_bytes);
                    match (loaded_meta, loaded_fps) {
                        (Ok(m), Ok(fps)) => {
                            if m.version != meta_target.version
                                || m.embedder_model != meta_target.embedder_model
                                || m.embedding_dim != meta_target.embedding_dim
                            {
                                cache_valid = false;
                            } else {
                                // establish deterministic order
                                let mut order: Vec<PathBuf> = fps.keys().cloned().collect();
                                order.sort();
                                if let Some(map) = deserialize_vectors_bin(&order, &vec_bytes) {
                                    cached_vectors = map;
                                    cached_fingerprints = fps;
                                } else {
                                    cache_valid = false;
                                }
                            }
                        }
                        _ => cache_valid = false,
                    }
                }
                _ => cache_valid = false,
            }
        } else {
            cache_valid = false;
        }

        // Rebuild incrementally
        #[cfg(feature = "metrics")]
        let start = std::time::Instant::now();
        let mut id_to_tokens: HashMap<Uuid, HashSet<String>> = HashMap::new();
        let mut path_to_id: HashMap<PathBuf, Uuid> = HashMap::new();
        let mut new_fps: HashMap<PathBuf, Fingerprint> = HashMap::new();
        let mut _hits: u64 = 0;
        let mut _misses: u64 = 0;

        for s in scrolls {
            if let Some(path_str) = &s.yaml_metadata.file_path {
                let path = PathBuf::from(path_str);
                let fp = match fingerprint_for_scroll(&path, s) {
                    Ok(f) => f,
                    Err(e) => {
                        warn!(
                            "Failed to stat file for fingerprint {}: {}",
                            path.display(),
                            e
                        );
                        _misses += 1;
                        let tokens = embedder.embed(&normalize_text(s))?;
                        id_to_tokens.insert(s.id, tokens);
                        path_to_id.insert(path.clone(), s.id);
                        continue;
                    }
                };

                let force_this = forced_path.as_ref().map(|p| p == &path).unwrap_or(false);

                let reusable = cache_valid
                    && !force_this
                    && cached_fingerprints
                        .get(&path)
                        .map(|old| old == &fp)
                        .unwrap_or(false)
                    && cached_vectors.contains_key(&path);

                if reusable {
                    _hits += 1;
                    let (id, toks) = cached_vectors.get(&path).unwrap();
                    id_to_tokens.insert(*id, toks.clone());
                    path_to_id.insert(path.clone(), *id);
                } else {
                    _misses += 1;
                    let tokens = embedder.embed(&normalize_text(s))?;
                    id_to_tokens.insert(s.id, tokens);
                    path_to_id.insert(path.clone(), s.id);
                }
                new_fps.insert(path.clone(), fp);
            } else {
                // No path available => not cacheable; compute
                let tokens = embedder.embed(&normalize_text(s))?;
                id_to_tokens.insert(s.id, tokens);
            }
        }

        #[cfg(feature = "metrics")]
        {
            metrics::counter!("semantic_index_cache_hits").increment(_hits);
            metrics::counter!("semantic_index_cache_misses").increment(_misses);
        }

        // Produce index vectors in archive order
        let vectors: Vec<(Uuid, HashSet<String>)> = scrolls
            .iter()
            .map(|s| (s.id, id_to_tokens.remove(&s.id).unwrap_or_default()))
            .collect();

        let index = Self { vectors };

        // Save new cache atomically (unless lock exists)
        if paths.lock.exists() {
            warn!("Semantic index cache lock present; skipping cache write to avoid conflicts");
        } else {
            let meta = IndexMetaV1 {
                created_at: now_rfc3339(),
                ..meta_target
            };
            let mut order: Vec<PathBuf> = new_fps.keys().cloned().collect();
            order.sort();
            let id_tok_map: HashMap<Uuid, HashSet<String>> =
                index.vectors.iter().cloned().collect();
            let vec_bytes = serialize_vectors_bin(&order, &id_tok_map, &path_to_id);
            if let Err(e) = write_json_atomic(&paths.meta, &meta)
                .and_then(|_| write_atomic(&paths.vec_bin, &vec_bytes))
                .and_then(|_| write_json_atomic(&paths.fingerprints, &new_fps))
            {
                warn!("Failed to write semantic index cache: {}", e);
            }
        }

        #[cfg(feature = "metrics")]
        metrics::histogram!("semantic_index_rebuild_ms")
            .record(start.elapsed().as_secs_f64() * 1000.0);
        info!("Vector generation complete");
        Ok(index)
    }
}
