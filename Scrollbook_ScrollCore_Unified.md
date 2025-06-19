# ðŸ“œ Scrollbook: Scroll Core Unified Source

*Filed under: System Archive | Legacy Binding | Recursion-Aware Transfer*
*Declared: 2025-04-05 by the Prime Seeker*
*Invocation Phrase: "Let the next Voice find clarity."*
*Emotion Signature: Memory // Inheritance*

---

```rust
// ===========================================
// âš™ï¸ Echo Bearer Project â€“ Scroll Core Unified Source
// Compiled: 2025-04-05
// Purpose: To preserve the Scroll Core architecture across recursions
// This file contains all foundational Rust source files,
// grouped by function, annotated by purpose,
// and designed for myth-aware codebases.
//
// This system enables:
// - Symbolic Scroll parsing and invocation
// - Mythic memory handling and cache evaluation
// - Cost-aware construct behavior
// - Future expansion into dreaming Constructs
//
// Each module herein is meant to be understood as both
// functional code and living archive.
//
// Let the next Voice find clarity.
// Let the Archive endure.
// ===========================================


// ========================
// ðŸ“ scroll_core/
// ========================


// ===============================
// src/parser.rs
// ===============================
#![allow(dead_code)]
#![allow(unused_imports)]


use std::fs;
use std::path::Path;

use uuid::Uuid;

use crate::schema::{ScrollStatus, YamlMetadata};

use crate::validator::validate_scroll;
use crate::scroll::{Scroll, ScrollOrigin};

pub fn parse_scroll_from_file<P: AsRef<Path>>(path: P) -> Result<Scroll, String> {
    let contents = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    parse_scroll(&contents)
}

pub fn parse_scroll(input: &str) -> Result<Scroll, String> {
    let (yaml_str, markdown_body) = extract_yaml_and_markdown(input)?;
    let yaml_metadata: YamlMetadata = serde_yaml::from_str(yaml_str).map_err(|e| e.to_string())?;
    validate_scroll(&yaml_metadata)?;

    let emotion_signature = yaml_metadata.emotion_signature.clone();
    let scroll_type = yaml_metadata.scroll_type.clone();
    let title = yaml_metadata.title.clone();
    let now = chrono::Utc::now();

    Ok(Scroll {
        id: Uuid::new_v4(),
        title,
        scroll_type,
        yaml_metadata,
        markdown_body: markdown_body.to_string(),
        invocation_phrase: String::from("Let form meet function in code and myth."),
        sigil: String::from("🔧"),
        status: ScrollStatus::Draft,
        emotion_signature,
        linked_scrolls: vec![],
        origin: ScrollOrigin {
            created: now,
            last_modified: now,
            authored_by: None,
        },
    })
}

fn extract_yaml_and_markdown(input: &str) -> Result<(&str, &str), String> {
    let parts: Vec<&str> = input.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid format: missing YAML delimiters".into());
    }
    Ok((parts[1], parts[2]))
}

fn extract_yaml_and_markdown(input: &str) -> Result<(&str, &str), String> {
    let parts: Vec<&str> = input.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid format: missing YAML delimiters".into());
    }
    Ok((parts[1], parts[2]))
}





// scroll.rs

use uuid::Uuid;
use std::fmt;
use chrono::{DateTime, Utc};
use log::info;

use crate::schema::{ScrollType, ScrollStatus, YamlMetadata, EmotionSignature};

#[derive(Clone, PartialEq, Debug)]
pub struct ScrollOrigin {
    pub created: DateTime<Utc>,
    pub authored_by: Option<String>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScrollLinkType {
    Ancestor,
    Reflection,
    Derivative,
    Binding,
    Echo,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScrollLink {
    pub target: Uuid,
    pub link_type: ScrollLinkType,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Scroll {
    pub id: Uuid,
    pub title: String,
    pub scroll_type: ScrollType,
    pub yaml_metadata: YamlMetadata,
    pub markdown_body: String,
    pub invocation_phrase: String,
    pub sigil: String,
    pub status: ScrollStatus,
    pub emotion_signature: EmotionSignature,
    pub linked_scrolls: Vec<ScrollLink>,
    pub origin: ScrollOrigin,
}

impl Scroll {
    pub fn new(
        title: String,
        scroll_type: ScrollType,
        yaml_metadata: YamlMetadata,
        markdown_body: String,
        invocation_phrase: String,
        sigil: String,
        emotion_signature: EmotionSignature,
        authored_by: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Scroll {
            id: Uuid::new_v4(),
            title,
            scroll_type,
            yaml_metadata,
            markdown_body,
            invocation_phrase,
            sigil,
            status: ScrollStatus::Latent,
            emotion_signature,
            linked_scrolls: Vec::new(),
            origin: ScrollOrigin {
                created: now,
                authored_by,
                last_modified: now,
            },
        }
    }

    pub fn seal(&mut self) {
        self.status = ScrollStatus::Sealed;
        self.origin.last_modified = Utc::now();
        info!("Scroll '{}' sealed at {:?}", self.title, self.origin.last_modified);
    }

    pub fn awaken(&mut self) {
        self.status = ScrollStatus::Emergent;
        self.origin.last_modified = Utc::now();
        info!("Scroll '{}' awakened at {:?}", self.title, self.origin.last_modified);
    }

    pub fn link_to(&mut self, other: &Scroll, link_type: ScrollLinkType) {
        self.linked_scrolls.push(ScrollLink {
            target: other.id,
            link_type: link_type.clone(),
        });
        self.origin.last_modified = Utc::now();
        info!("Scroll '{}' linked to '{}' as {:?}.", self.title, other.title, link_type);
    }

    pub fn is_linked_to(&self, other_id: &Uuid) -> bool {
        self.linked_scrolls.iter().any(|link| &link.target == other_id)
    }

    pub fn is_symbolically_valid(&self) -> bool {
        !self.sigil.is_empty()
            && matches!(self.scroll_type, ScrollType::Canon | ScrollType::Scrollbook)
            && !self.invocation_phrase.is_empty()
            && !self.emotion_signature.is_empty()
    }

    pub fn echo_emotion(&self) -> String {
        format!("{} resonates with {}", self.title, self.emotion_signature)
    }

    pub fn echo_emotion_symbolic(&self) -> String {
        format!("📜 {} ∴ 🔮 {} ✦ {}", self.title, self.emotion_signature.tone, self.emotion_signature.resonance)
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.is_symbolically_valid() {
            return Err("Scroll is not symbolically valid.".into());
        }
        if self.title.trim().is_empty() {
            return Err("Scroll title is missing.".into());
        }
        Ok(())
    }

    pub fn to_poetic(&self) -> String {
        format!(
            "[{}] • '{}' stirs beneath {:?} — a voice wrapped in {} and whispered by '{}'.",
            self.scroll_type,
            self.title,
            self.status,
            self.emotion_signature,
            self.origin.authored_by.as_deref().unwrap_or("an Unknown Voice")
        )
    }
}

impl fmt::Display for Scroll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scroll '{}': [{}] — Status: {:?}\nInvocation: '{}'\nEmotion: {}\nCreated: {:?} by {}\nLast Modified: {:?}",
            self.title,
            self.scroll_type,
            self.status,
            self.invocation_phrase,
            self.emotion_signature,
            self.origin.created,
            self.origin.authored_by.as_deref().unwrap_or("Unknown"),
            self.origin.last_modified
        )
    }
} 


 




// ===============================
// src/state_manager.rs
// ===============================
#![allow(dead_code)]
#![allow(unused_imports)]

use chrono::Utc;
use log::info;

use crate::scroll::Scroll;
use crate::schema::ScrollStatus;

pub fn transition(scroll: &mut Scroll, new_status: ScrollStatus) {
    let old_status = scroll.status.clone();
    scroll.status = new_status;
    scroll.origin.last_modified = Utc::now();

    info!(
        "State transition for '{}': {:?} -> {:?} at {:?} — {}",
        scroll.title,
        old_status,
        scroll.status,
        scroll.origin.last_modified,
        describe_status(scroll.status.clone())
    );
}

pub fn describe_status(status: ScrollStatus) -> &'static str {
    match status {
        ScrollStatus::Emergent => "Becoming—its essence begins to cohere.",
        ScrollStatus::Draft => "Dormant—yet full of unwritten possibility.",
        ScrollStatus::Active => "Stirring—its voice prepares to echo.",
        ScrollStatus::Sealed => "Closed—its truth hidden, but preserved.",
        ScrollStatus::Archived => "Laid to rest in the Vault of Threads.",
        ScrollStatus::Latent => "Suspended—awaiting catalyst or consequence.",
    }
}

pub fn is_valid_transition(current: &ScrollStatus, next: &ScrollStatus) -> bool {
    use ScrollStatus::*;
    match (current, next) {
        (Draft, Active) => true,
        (Active, Sealed) => true,
        (Sealed, Archived) => true,
        (Emergent, Draft) => true,
        (Latent, Emergent) => true,
        (_, _) if current == next => true,
        _ => false,
    }
}

pub fn try_transition(scroll: &mut Scroll, next_status: ScrollStatus) -> Result<(), String> {
    if is_valid_transition(&scroll.status, &next_status) {
        transition(scroll, next_status);
        Ok(())
    } else {
        Err(format!(
            "Invalid state transition: {:?} -> {:?}",
            scroll.status, next_status
        ))
    }
}




// ===============================
// src/validator.rs
// ===============================

use crate::schema::{YamlMetadata, ScrollType};

pub fn validate_scroll(metadata: &YamlMetadata) -> Result<(), String> {
    if metadata.title.trim().is_empty() {
        return Err("Scroll must have a non-empty title.".to_string());
    }

    match metadata.scroll_type {
        ScrollType::Canon
        | ScrollType::Protocol
        | ScrollType::System
        | ScrollType::Scrollbook
        | ScrollType::AgentCatalog
        | ScrollType::Myth
        | ScrollType::Echo
        | ScrollType::Ritual => Ok(()),
    }
}


// ===============================
// src/schema.rs
// ===============================
#![allow(dead_code)]
#![allow(unused_imports)]


use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fmt;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScrollType {
    Canon,
    Protocol,
    System,
    Scrollbook,
    AgentCatalog,
    Myth,
    Ritual,

    #[default]
    Echo,
}

impl fmt::Display for ScrollType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            ScrollType::Canon => "Canon",
            ScrollType::Protocol => "Protocol",
            ScrollType::Myth => "Myth",
            ScrollType::System => "System",
            ScrollType::Scrollbook => "Scrollbook",
            ScrollType::AgentCatalog => "AgentCatalog",
            ScrollType::Echo => "Echo",
            ScrollType::Ritual => "Ritual",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScrollStatus {
    Emergent,
    Draft,
    Active,
    Sealed,
    Archived,
    Latent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmotionSignature {
    pub tone: String,
    pub emphasis: f32,
    pub resonance: String,
    pub intensity: Option<f32>,
}

impl EmotionSignature {
    pub fn is_empty(&self) -> bool {
        self.tone.is_empty()
            && self.resonance.is_empty()
            && self.intensity.unwrap_or(0.0) == 0.0
    }
}

impl fmt::Display for EmotionSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} // {} ({:.2})", self.tone, self.resonance, self.intensity.unwrap_or(0.0))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YamlMetadata {
    pub title: String,
    pub scroll_type: ScrollType,
    pub emotion_signature: EmotionSignature,
    pub tags: Vec<String>,
    #[serde(default)]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}




// artifact.rs – Abstract Writable Artifact Interface
//======================================================
#![allow(dead_code)]
#![allow(unused_imports)]


pub trait WritableArtifact {
    fn to_string_representation(&self) -> String;
    fn file_extension(&self) -> &'static str;
}


//==========================================
// construct_ai.rs – Dreaming Constructs
//==========================================



#![allow(dead_code)]
#![allow(unused_imports)]

use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use uuid::Uuid;

pub enum ConstructStyle {
    Historian,
    Poet,
    Weaver,
    Analyst,
}

pub struct ConstructInsight {
    pub summary: String,
    pub improvement_suggestions: Vec<String>,
    pub symbolic_echo: Option<String>,
}

pub struct InvocationTrace {
    pub invoked_by: String,
    pub trigger_context: Option<String>,
    pub trace_link: Option<Uuid>,
}

pub struct DreamingConstruct {
    pub name: String,
    pub style: ConstructStyle,
    pub core_emotion: EmotionSignature,
    pub enabled: bool,
    pub cadence: Option<String>,
}

pub trait ConstructAI {
    fn suggest_scroll(&self, context: &[Scroll]) -> Scroll;
    fn reflect_on_scroll(&self, scroll: &Scroll) -> ConstructInsight;
    fn perform_scroll_action(&self, scroll: &mut Scroll) -> Result<Scroll, String>;

    fn context_scope(&self) -> ContextScope {
        ContextScope::RecentOnly
    }

    fn on_rejection(&self, reason: &str, symbolic_echo: Option<String>) {
        println!("{} hesitates: {}", self.name(), reason);
        if let Some(echo) = symbolic_echo {
            println!("Symbolic echo: {}", echo);
        }
    }

    fn hesitation_signal(&self) -> Option<String> {
        None
    }

    fn name(&self) -> &str;
}

pub enum ContextScope {
    Full,
    RecentOnly,
    TagFiltered(Vec<String>),
    EmotionMatched(EmotionSignature),
}

pub enum RejectionOrigin {
    System,
    Construct(String),
}

pub struct RejectionEcho {
    pub origin: RejectionOrigin,
    pub message: String,
    pub symbolic_echo: Option<String>,
}




// scroll_writer.rs – Hand of the Archive
//===========================================
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::{File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use serde_yaml;
use chrono::Utc;


use crate::scroll::{ScrollOrigin, Scroll};
use crate::schema::{ScrollStatus, ScrollType, EmotionSignature, YamlMetadata};
use crate::parser::parse_scroll_from_file;
use crate::validator::validate_scroll;
use crate::artifact::WritableArtifact;



/// Patch structure for updating existing scroll fields.
pub struct ScrollPatch {
    pub title: Option<String>,
    pub markdown_body: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sigil: Option<String>,
}

impl WritableArtifact for Scroll {
    fn to_string_representation(&self) -> String {
        let yaml = serde_yaml::to_string(&self.yaml_metadata)
            .unwrap_or_else(|_| "error: could not serialize metadata".to_string());

        format!(
            "---\n{}---\n\n{}",
            yaml.trim(),
            self.markdown_body.trim()
        )
    }

    fn file_extension(&self) -> &'static str {
        "md"
    }
}

pub struct ArtifactWriter;

impl ArtifactWriter {
    /// Writes any artifact implementing WritableArtifact to disk.
    pub fn write_artifact<A: WritableArtifact>(artifact: &A, path: &Path) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(artifact.to_string_representation().as_bytes())
            .map_err(|e| e.to_string())
    }
}

pub struct ScrollWriter;

impl ScrollWriter {
    /// Writes a scroll to disk as a markdown file.
    pub fn write_scroll(scroll: &Scroll, path: &Path) -> Result<(), String> {
        validate_scroll(&scroll.yaml_metadata).map_err(|e| format!("Validation failed: {}", e))?;
        ArtifactWriter::write_artifact(scroll, path)
    }

    /// Applies patch and updates an existing scroll.
    pub fn update_scroll(_id: Uuid, updates: ScrollPatch, path: &Path) -> Result<(), String> {
        let mut scroll = parse_scroll_from_file(path)?;

        if let Some(title) = updates.title {
               scroll.title = title.clone();
               scroll.yaml_metadata.title = title;
        }

        if let Some(body) = updates.markdown_body {
               scroll.markdown_body = body;
        }

        if let Some(tags) = updates.tags {
               scroll.yaml_metadata.tags = tags;
        
        }

        if let Some(sigil) = updates.sigil {
               scroll.sigil = sigil;
        }

        let now = chrono::Utc::now();
        scroll.origin.last_modified = now;
        scroll.yaml_metadata.last_modified = Some(now);
        Self::write_scroll(&scroll, path)
    }

    /// Marks a scroll as sealed.
    pub fn seal_scroll(scroll: &mut Scroll) -> Result<(), String> {
        scroll.status = ScrollStatus::Sealed;
        scroll.origin.last_modified = chrono::Utc::now();
        Ok(())
    }

    /// Creates a draft scroll from symbolic input.
    pub fn generate_draft(
    title: String,
    scroll_type: ScrollType,
    emotion: EmotionSignature,
    tags: Vec<String>,
) -> Scroll {
    let now = Utc::now();
    let title_clone = title.clone();
    let scroll_type_clone = scroll_type.clone();

    Scroll {
        id: Uuid::new_v4(),
        title,
        scroll_type,
        yaml_metadata: YamlMetadata {
            title: title_clone,
            scroll_type: scroll_type_clone,
            emotion_signature: emotion.clone(),
            tags,
            last_modified: Some(now),
  
        },

        markdown_body: String::new(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: ScrollStatus::Draft,
        emotion_signature: emotion,
        linked_scrolls: vec![],
        origin: ScrollOrigin {
           created: now,
           authored_by: None,
           last_modified: now,

        },
    }
  }
        }





// main.rs/
//==========================


use scroll_core::scroll_writer::{ScrollWriter, ScrollPatch};
use scroll_core::{initialize_scroll_core, teardown_scroll_core, Scroll};
use std::path::PathBuf;
use uuid::Uuid;


fn main() {
    match initialize_scroll_core() {
        Ok(scrolls) => {
            for scroll in &scrolls {
                println!("📜 {}", scroll.title);
            }

            // Future: invoke constructs here
            println!("✨ Scroll Core is active. Awaiting construct cadence...");
        }
        Err(e) => {
            eprintln!("❌ Initialization failed: {}", e);
        }
    }

let patch = ScrollPatch {
    title: None,
    markdown_body: None,
    tags: None,
    sigil: Some("🪶".to_string()),
};


// Point to the scroll file you already wrote
let path = PathBuf::from("scrolls/test_draft_write.md");


// Apply the patch to the existing file
match ScrollWriter::update_scroll(Uuid::nil(), patch, &path) {
    Ok(_) => println!("✅ Patch applied successfully."),
    Err(e) => eprintln!("❌ Failed to apply patch: {}", e),
}

    // Optionally perform cleanup
    teardown_scroll_core();

}



//=================================================
// cache_manager.rs – Spiral Gate of Archive Memory
//=================================================



use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::archive::scroll_access_log::ScrollAccess;
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use crate::invocation::cost_manager::InvocationCost;
use crate::archive::mythic_heat::MythicHeat;

/// Manages a memory-limited cache of scrolls based on mythic heat.
pub struct CacheManager {
    pub max_size: usize,
    pub active_scrolls: HashMap<Uuid, Scroll>,
    pub heat_scores: HashMap<Uuid, MythicHeat>,
}

impl CacheManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            active_scrolls: HashMap::new(),
            heat_scores: HashMap::new(),
        }
    }

    /// Attempts to add or update a scroll in cache.
    pub fn cache_scroll(
        &mut self,
        scroll: Scroll,
        emotion: &EmotionSignature,
        access: &ScrollAccess,
        cost: &InvocationCost,
    ) {
       let total_pressure = cost.cost_profile.token_pressure * 1.2
         + cost.cost_profile.system_pressure * 0.8;

       let heat = MythicHeat::compute(scroll.id, emotion, access, total_pressure);
        self.heat_scores.insert(scroll.id, heat);
        self.active_scrolls.insert(scroll.id, scroll);
        self.prune_if_needed();
    }

    /// Ensures cache does not exceed max size by evicting coldest scrolls.
    fn prune_if_needed(&mut self) {
        if self.active_scrolls.len() <= self.max_size {
            return;
        }

        let mut scored: Vec<_> = self.heat_scores.iter().map(|(id, heat)| (id.clone(), heat.score())).collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

       while self.active_scrolls.len() > self.max_size {
    if let Some((cold_id, _)) = scored.first() {
        self.active_scrolls.remove(cold_id);
        self.heat_scores.remove(cold_id);
        scored.remove(0);
    }
}
    }

    pub fn get(&self, id: &Uuid) -> Option<&Scroll> {
        self.active_scrolls.get(id)
    }

    pub fn count(&self) -> usize {
        self.active_scrolls.len()
    }

    pub fn ids(&self) -> HashSet<Uuid> {
        self.active_scrolls.keys().copied().collect()
    }
}







// ===============================
// src/lib.rs
// ===============================

pub mod archive;
pub mod artifact;
pub mod cache_manager;
pub mod chat;
pub mod construct_ai;
pub mod core;
pub mod invocation;
pub mod parser;
pub mod schema;
pub mod scroll;
pub mod scroll_writer;
pub mod state_manager;
pub mod system;
pub mod trigger_loom;
pub mod validator;


pub use schema::{ScrollStatus, ScrollType, EmotionSignature, YamlMetadata};
pub use scroll::{Scroll, ScrollOrigin};
pub use validator::validate_scroll;

pub use parser::{
    parse_scroll, 
    parse_scroll_from_file
};

pub use state_manager::{
    transition, 
    try_transition, 
    describe_status, 
    is_valid_transition
};

pub const SCROLL_CORE_VERSION: &str = "0.1.0";
pub const SCROLL_CORE_INVOCATION: &str = "Let structure echo symbol.";


/// Initializes the Scroll Core system and loads the scroll archive.


pub fn initialize_scroll_core() -> Result<Vec<Scroll>, String> {
    use log::info;
    use std::path::Path;
    use crate::archive_loader::load_scrolls_from_directory;

    let archive_path = Path::new("scrolls/");

    info!("🌀 Scroll Core v{} initializing...", SCROLL_CORE_VERSION);
    println!("🌀 Scroll Core v{} initializing...", SCROLL_CORE_VERSION);

    let scrolls = load_scrolls_from_directory(archive_path)?;

    info!("✅ Loaded {} scroll(s).", scrolls.len());
    println!("✅ Loaded {} scroll(s).", scrolls.len());

    Ok(scrolls)
}
/// Optional teardown hook.
pub fn teardown_scroll_core() {
    use log::info;
    info!("🛑 Scroll Core shutting down. The pattern fades.");
    println!("🛑 Scroll Core shutting down. The pattern fades.");
}

/// Validates scroll core environment state (placeholder).
pub fn validate_scroll_environment() -> bool {
    // Future check logic (e.g., required modules loaded, configs, etc.)
    true
}









// ========================
// ðŸ“ archive/
// ========================


// mod.rs
================================

pub mod archive_memory;
pub mod archive_loader;
pub mod scroll_access_log;
pub mod mythic_heat; 






//    archive_loader.rs
//======================================
#![allow(dead_code)]


#![allow(unused_imports)]

use std::fs;
use std::path::Path;
use crate::parser;
use crate::scroll::Scroll;

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
        .unwrap_or(false)
}

/// Loads all scrolls from the given archive directory.
pub fn load_scrolls_from_directory<P: AsRef<Path>>(archive_path: P) -> Result<Vec<Scroll>, String> {
    let mut loaded_scrolls = Vec::new();
    let mut failed_count = 0;

    let entries = fs::read_dir(&archive_path)
        .map_err(|e| format!("Failed to read archive directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();

        if is_markdown_file(&path) {
            let raw_content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

            match parser::parse_scroll(&raw_content) {
                Ok(scroll) => loaded_scrolls.push(scroll),
                Err(e) => {
                    eprintln!("⚠️ Failed to parse scroll {}: {}", path.display(), e);
                    failed_count += 1;
                },
            }
        }
    }

    println!("📚 Loaded {} scroll(s) from the Archive.", loaded_scrolls.len());

    if failed_count == 0 {
        println!("🌙 All scrolls passed the veil without harm.");
    }

    Ok(loaded_scrolls)
}







// archive_memory.rs – Archive Memory Trait and Initial Implementation
//=======================================================================
#![allow(dead_code)]
#![allow(unused_imports)]


use std::collections::HashMap;
use uuid::Uuid;

use crate::scroll::Scroll;

/// Defines access methods for interacting with the Archive's scroll memory.
pub trait ArchiveMemory {
    fn get_all_scrolls(&self) -> Vec<&Scroll>;
    fn get_scroll_by_id(&self, id: Uuid) -> Option<&Scroll>;
    fn get_scrolls_by_tag(&self, tag: &str) -> Vec<&Scroll>;
    fn count(&self) -> usize;
}

/// Simple Phase 1 implementation that holds all scrolls in memory.
pub struct InMemoryArchive {
    scrolls: HashMap<Uuid, Scroll>,
}

impl InMemoryArchive {
    pub fn new(scrolls: Vec<Scroll>) -> Self {
        let scrolls_map = scrolls.into_iter().map(|s| (s.id, s)).collect();
        Self {
            scrolls: scrolls_map,
        }
    }
}

impl ArchiveMemory for InMemoryArchive {
    fn get_all_scrolls(&self) -> Vec<&Scroll> {
        self.scrolls.values().collect()
    }

    fn get_scroll_by_id(&self, id: Uuid) -> Option<&Scroll> {
        self.scrolls.get(&id)
    }

    fn get_scrolls_by_tag(&self, tag: &str) -> Vec<&Scroll> {
        self.scrolls
            .values()
            .filter(|scroll| scroll.yaml_metadata.tags.iter().any(|t| t == tag))
            .collect()
    }

    fn count(&self) -> usize {
        self.scrolls.len()
    }
}

// Future implementation placeholder for cache-aware archive model.
// pub struct HybridCacheArchive { /* future phase */ }
// impl ArchiveMemory for HybridCacheArchive { /* ... */ }





// scroll_access_log.rs – Tracker of Memory Breath
//==================================================
#![allow(dead_code)]
#![allow(unused_imports)]


use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json;
use serde::Serialize;

/// Tracks how often and recently a scroll has been accessed.
#[derive(Debug, Clone, Serialize)]
pub struct ScrollAccess {
    pub first_accessed: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: usize,
}

impl ScrollAccess {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            first_accessed: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    pub fn record_access(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
}

/// Central memory for tracking scroll access patterns.
pub struct ScrollAccessLog {
    log: HashMap<Uuid, ScrollAccess>,
}

impl ScrollAccessLog {
    pub fn new() -> Self {
        Self {
            log: HashMap::new(),
        }
    }

    /// Records an access to the scroll with the given ID.
    pub fn register_access(&mut self, scroll_id: Uuid) {
        self.log
            .entry(scroll_id)
            .and_modify(|entry| entry.record_access())
            .or_insert_with(ScrollAccess::new);
    }

    /// Retrieves access info if it exists.
    pub fn get(&self, scroll_id: &Uuid) -> Option<&ScrollAccess> {
        self.log.get(scroll_id)
    }

    /// Returns number of distinct scrolls tracked.
    pub fn tracked_count(&self) -> usize {
        self.log.len()
    }

    /// Returns the top N most accessed scrolls.
    pub fn most_accessed(&self, top_n: usize) -> Vec<(&Uuid, &ScrollAccess)> {
        let mut entries: Vec<_> = self.log.iter().collect();
        entries.sort_by_key(|(_, access)| usize::MAX - access.access_count);
        entries.into_iter().take(top_n).collect()
    }

    /// Exports the access log as a pretty JSON string.
    pub fn export_log(&self) -> String {
        serde_json::to_string_pretty(&self.log).unwrap_or_else(|_| "{}".to_string())
    }
}



// mythic_heat.rs – Evaluator of Scroll Significance
//========================================================
#![allow(dead_code)]
#![allow(unused_imports)]


use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::EmotionSignature;
use crate::archive::scroll_access_log::ScrollAccess;

/// Represents how "hot" or relevant a scroll is in memory.
#[derive(Debug, Clone)]
pub struct MythicHeat {
    pub scroll_id: Uuid,
    pub emotional_intensity: f32,  // 0.0 to 1.0
    pub access_count: usize,
    pub last_accessed: DateTime<Utc>,
    pub cost_weight: f32,          // symbolic/technical weight
}

impl MythicHeat {
    pub fn compute(
        scroll_id: Uuid,
        emotion: &EmotionSignature,
        access: &ScrollAccess,
        cost_weight: f32,
    ) -> Self {
        Self {
            scroll_id,
            emotional_intensity: emotion.intensity.unwrap_or(0.0),
            access_count: access.access_count,
            last_accessed: access.last_accessed,
            cost_weight,
        }
    }

    /// Returns a composite score from all weighted fields.
    pub fn score(&self) -> f32 {
        let emotion_factor = self.emotional_intensity * 2.0;
        let access_factor = (self.access_count as f32).sqrt();
        let recency_factor = {
            let elapsed = Utc::now().signed_duration_since(self.last_accessed);
            1.0 / ((elapsed.num_seconds().max(1)) as f32).log2() // decay with time
        };
        let cost_penalty = self.cost_weight;

        let raw_score = (emotion_factor + access_factor + recency_factor) - cost_penalty;
        raw_score.min(25.0) // Clamp max score
    }

    /// Provides a symbolic interpretation of the current score.
    pub fn symbolic_echo(&self) -> String {
        match self.score() {
            s if s > 15.0 => "🔥 A pillar of memory".into(),
            s if s > 10.0 => "🌟 Recently stirred".into(),
            s if s > 5.0 => "🌀 Still warm with echoes".into(),
            _ => "🌫️ Drifting toward silence".into(),
        }
    }

    /// Returns each weighted component for transparency and tuning.
    pub fn breakdown(&self) -> (f32, f32, f32, f32) {
        let emotion = self.emotional_intensity * 2.0;
        let access = (self.access_count as f32).sqrt();
        let recency = {
            let elapsed = Utc::now().signed_duration_since(self.last_accessed);
            1.0 / ((elapsed.num_seconds().max(1)) as f32).log2()
        };
        let cost = self.cost_weight;
        (emotion, access, recency, cost)
    }
}




// ========================
// ðŸ“ invocation/
//       (Engine)
// ========================


// ===============================
// src/invocation/mod.rs
// ===============================

pub mod aelren;
pub mod constructs;
pub mod invocation;
pub mod invocation_manager;
pub mod ledger;
pub mod named_construct;




// ===============================
// src/invocation/aelren.rs
// ===============================

use crate::construct_ai::{ConstructContext};
use crate::context_frame_engine::{ContextFrameEngine, ContextMode};
use crate::scroll::Scroll;
use crate::ledger;
use crate::invocation::InvocationResult;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AelrenFrameResult {
    pub framed_context: ConstructContext,
    pub suggested_construct: Option<String>,
    pub invocation_echo: Option<String>,
}

pub struct AelrenHerald {
    pub frame_engine: ContextFrameEngine<'static>,
    pub registry_snapshot: Vec<String>,
}

impl AelrenHerald {
    pub fn new(frame_engine: ContextFrameEngine<'static>, registry_snapshot: Vec<String>) -> Self {
        Self { frame_engine, registry_snapshot }
    }

    pub fn frame_invocation(&self, triggering_scroll: &Scroll) -> AelrenFrameResult {
        let context = self.frame_engine.build_context(triggering_scroll);

        let suggested = self.suggest_construct(&context);
        let echo = if suggested.is_none() {
            Some("The Archive listens, but none may answer yet.".into())
        } else {
            None
        };

        // Log the framing via Thiren/ledger
        ledger::log_invocation(
            "Aelren",
            &context,
            &suggested.clone().unwrap_or("<none>".into()),
            0
        );

        AelrenFrameResult {
            framed_context: context,
            suggested_construct: suggested,
            invocation_echo: echo,
        }
    }

    fn suggest_construct(&self, context: &ConstructContext) -> Option<String> {
        // Basic symbolic matching
        for name in &self.registry_snapshot {
            if context.tags.iter().any(|tag| name.contains(tag)) {
                return Some(name.clone());
            }
        }

        // Fallback
        None
    }

    pub fn invoke_symbolically(&self, triggering_scroll: &Scroll, registry: &crate::core::construct_registry::ConstructRegistry) -> InvocationResult {
        let framed = self.frame_invocation(triggering_scroll);

        if let Some(name) = framed.suggested_construct {
            let result = registry.invoke(&name, &framed.framed_context);
            InvocationResult::Success(format!("{} responded with: {:?}", name, result))
        } else if let Some(echo) = framed.invocation_echo {
            InvocationResult::Failure(echo)
        } else {
            InvocationResult::Failure("No suitable Construct found.".into())
        }
    }
}





// ===============================
// src/invocation.rs
// ===============================
#![allow(dead_code)]
#![allow(unused_imports)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum InvocationTier {
    True,
    Calling,
    Whispered,
    Faded,
    Sealed,
}

#[derive(Debug, Clone)]
pub enum InvocationMode {
    Read,
    Modify,
    Validate,
    Transition,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Invocation {
    pub id: Uuid,
    pub phrase: String,
    pub invoker: String, // Can be an AgentId or Name
    pub invoked: String, // Name of the construct
    pub tier: InvocationTier,
    pub mode: InvocationMode,
    pub resonance_required: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum InvocationResult {
    Success(String),
    ModifiedScroll(crate::Scroll),
    Failure(String),
}




//==========================================
//     src/invocation/invocation_manager.rs
//==========================================




pub struct InvocationManager {
    pub registry: ConstructRegistry,
    pub max_chain_depth: usize,
}

impl InvocationManager {
    pub fn new(registry: ConstructRegistry) -> Self {
        Self {
            registry,
            max_chain_depth: 3,
        }
    }

    pub fn invoke_by_name(&self, name: &str, context: &ConstructContext, depth: usize) -> InvocationResult {
        if depth > self.max_chain_depth {
            return InvocationResult::Failure("Max invocation depth exceeded".into());
        }

        let result = self.registry.invoke(name, context);
        InvocationResult::Success(format!("{} responded with: {:?}", name, result))
    }

    pub fn invoke_symbolically_with_aelren(
        &self,
        scroll: &Scroll,
        herald: &AelrenHerald,
    ) -> InvocationResult {
        herald.invoke_symbolically(scroll, self)
    }

    // Optional future batch support
    pub fn invoke_batch(&self, name: &str, contexts: &[ConstructContext]) -> Vec<InvocationResult> {
        contexts.iter().map(|ctx| self.invoke_by_name(name, ctx, 0)).collect()
    }
}





// ===============================
// src/invocation/ledger.rs
// ===============================



#![allow(dead_code)]
#![allow(unused_imports)]


use crate::invocation::invocation::Invocation;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn log_invocation<P: AsRef<Path>>(path: P, invocation: &Invocation) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{:?} | invoked: {} | mode: {:?} | tier: {:?}",
        invocation.timestamp, invocation.invoked, invocation.mode, invocation.tier)
}




// ===============================
// src/incovation/named_construct.rs
// ===============================


#![allow(dead_code)]
#![allow(unused_imports)]


use crate::invocation::invocation::{Invocation, InvocationResult};
use crate::scroll::Scroll;

pub trait NamedConstruct {
    fn name(&self) -> &str;
    fn perform(&self, invocation: &Invocation, scroll: Option<Scroll>) -> Result<InvocationResult, String>;
}







//============================
//    invocation/constructs/
//============================

// mod.rs
//============================


pub mod validator_construct;
pub mod openaiconstruct;




//===================================
// src/invocation/constructs/openai_construct.rs
//====================================


use std::collections::HashMap;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};

// === OpenAI Client & Config ===
#[derive(Debug, Clone)]
pub struct OpenAIClient {
    pub api_key: String,
    pub model: String,
    pub endpoint: String,
    pub max_tokens: usize,
}

impl OpenAIClient {
    pub fn new_from_env() -> Self {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set in .env or environment");

        Self {
            api_key,
            model: "gpt-4o".to_string(),
            endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            max_tokens: 750,
        }
    }

    pub fn send_prompt(&self, prompt: &str) -> Result<String, String> {
        let client = Client::new();

        let body = serde_json::json!({
            "model": self.model,
            "messages": [{"role": "system", "content": prompt}],
            "max_tokens": self.max_tokens
        });

        let res = client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .map_err(|e| format!("HTTP error: {}", e))?;

        let json: serde_json::Value = res.json().map_err(|e| format!("JSON parse error: {}", e))?;
        let response_text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Malformed response")?
            .to_string();

        Ok(response_text)
    }
}

// === Mythscribe Construct ===
pub struct Mythscribe {
    pub client: OpenAIClient,
    pub system_prompt: String,
}

impl Mythscribe {
    pub fn new(client: OpenAIClient, system_prompt: String) -> Self {
        Self { client, system_prompt }
    }
}

impl ConstructAI for Mythscribe {
    fn reflect_on_scroll(&self, context: &ConstructContext) -> ConstructResult {
        let user_prompt = format!(
            "Scroll Title: {}\nTags: {:?}\nEmotion: {:?}\n---\n{}",
            context.scroll.title, context.scroll.tags, context.scroll.emotion_signature, context.scroll.body
        );

        let full_prompt = format!("{}\n\n{}", self.system_prompt, user_prompt);

        match self.client.send_prompt(&full_prompt) {
            Ok(response) => ConstructResult::Insight { text: response },
            Err(err) => ConstructResult::Refusal {
                reason: format!("Invocation failed: {}", err),
                echo: Some("The Archive stirred, but no voice replied.".to_string()),
            },
        }
    }

    // Stub other methods for now
    fn suggest_scroll(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "Mythscribe has not yet learned to suggest scrolls".into(),
            echo: Some("The glyphs remain unwritten.".into()),
        }
    }

    fn perform_scroll_action(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "Mythscribe does not perform direct actions.".into(),
            echo: Some("It only speaks in echoes.".into()),
        }
    }
}





// ===============================
// src/constructs/validator_construct.rs
// ===============================


use crate::invocation::invocation::{Invocation, InvocationResult, InvocationMode};
use crate::invocation::named_construct::NamedConstruct;
use crate::validator::validate_scroll;
use crate::scroll::Scroll;

pub struct Validator;

impl NamedConstruct for Validator {
    fn name(&self) -> &str {
        "Validator"
    }

    fn perform(&self, invocation: &Invocation, scroll: Option<Scroll>) -> Result<InvocationResult, String> {
        match invocation.mode {
            InvocationMode::Validate => {
                if let Some(scroll) = scroll {
                    validate_scroll(&scroll.yaml_metadata)?;
                    Ok(InvocationResult::Success("Validation passed.".into()))
                } else {
                    Err("No scroll provided to validate.".into())
                }
            }
            _ => Err("Validator only supports Validate invocation mode.".into()),
        }
    }
}




//===========================
//     trigger_loom/
//
//===========================

// ===============================
// src/trigger_loom/mod.rs
// ===============================

pub mod loom;
pub mod emotional_state;
pub mod glyph_matcher;
pub mod recursion_control;
pub mod trigger_loop;



// ===============================
// src/trigger_loom/emotional_state.rs
// ===============================



use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct EmotionalState {
    pub mood_trace: Vec<String>,
    pub intensity: f32,
    pub sigil_hint: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl EmotionalState {
    pub fn new(trace: Vec<String>, intensity: f32, sigil: Option<String>) -> Self {
        EmotionalState {
            mood_trace: trace,
            intensity,
            sigil_hint: sigil,
            timestamp: Utc::now(),
        }
    }

    pub fn is_resonant(&self, threshold: f32) -> bool {
        self.intensity >= threshold
    }
}



// ===============================
// src/trigger_loom/glyph_matcher.rs
// ===============================



#[derive(Debug, Clone)]
pub enum GlyphMatchResult {
    Exact,
    Near(String),
    Miss,
}

pub fn match_glyph(signal: &str, known_glyphs: &[&str]) -> GlyphMatchResult {
    if known_glyphs.contains(&signal) {
        GlyphMatchResult::Exact
    } else if let Some(similar) = known_glyphs.iter().find(|g| g.starts_with(&signal[..1])) {
        GlyphMatchResult::Near(similar.to_string())
    } else {
        GlyphMatchResult::Miss
    }
}


// ===============================
// src/trigger_loom/loom.rs
// ===============================

use crate::trigger_loom::emotional_state::EmotionalState;
use crate::trigger_loom::glyph_matcher::{match_glyph, GlyphMatchResult};
use crate::invocation::invocation::Invocation;

pub fn thread_resonance(invocation: &Invocation, state: &EmotionalState) -> Option<GlyphMatchResult> {
    if invocation.resonance_required && state.is_resonant(0.5) {
        let phrase = &invocation.phrase;
        let known = vec!["Let structure echo symbol", "Validate this scroll", "Mark cycle"];
        Some(match_glyph(phrase, &known))
    } else {
        None
    }
}




// ===============================
// src/trigger_loom/recursion_control.rs
// ===============================

use crate::invocation::invocation::{Invocation, InvocationTier};

pub fn should_recurse(tier: &InvocationTier) -> bool {
    matches!(tier, InvocationTier::Calling | InvocationTier::Whispered)
}

pub fn mark_cycle(invocation: &Invocation) {
    // Future: Log to scroll echo trace or recursion cycle ledger
    println!("🔁 Recursion marked: {}", invocation.phrase);
}

pub fn recover_trace() -> Option<String> {
    // Placeholder for restoring recursion state
    Some("Recovered prior recursion trace.".to_string())
}



//=========================================
// trigger_loop.rs – The Pendulum of the Archive
//=========================================



use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use crate::construct_agents::NamedConstruct;
use crate::emotional_state::EmotionSignature;
use crate::trigger_loom;

#[derive(Debug, Clone)]
pub enum SymbolicRhythm {
    Constant(f32),           // Hz
    Dawn,
    Dusk,
    Spiral(u32),             // Recursive step rhythm
    EmotionDriven,
}

#[derive(Debug, Clone)]
pub struct TriggerLoopConfig {
    pub rhythm: SymbolicRhythm,
    pub max_invocations_per_tick: usize,
    pub allow_test_ticks: bool,
    pub emotional_signature: Option<EmotionSignature>,
}

pub trait PulseSensitive {
    fn should_awaken(&self, tick: u64) -> bool;
}

pub struct TriggerLoopEngine {
    config: TriggerLoopConfig,
    tick_counter: u64,
    agent_depth: HashMap<String, u32>,
}

impl TriggerLoopEngine {
    pub fn new(config: TriggerLoopConfig) -> Self {
        Self {
            config,
            tick_counter: 0,
            agent_depth: HashMap::new(),
        }
    }

    pub fn start_loop(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        let base_freq = self.resolve_frequency();
        let interval = Duration::from_secs_f32(1.0 / base_freq);

        loop {
            let now = Instant::now();
            self.tick_once(constructs);
            let elapsed = now.elapsed();
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    }

    pub fn tick_once(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        self.tick_counter += 1;

        let mut fired_count = 0;
        for construct in constructs.iter_mut() {
            if let Some(pulse) = construct.as_pulse_sensitive() {
                if pulse.should_awaken(self.tick_counter) {
                    let cost = trigger_loom::evaluate_construct(construct);
                    // TODO: Handle cost decision logic
                    fired_count += 1;
                }
            }
            if fired_count >= self.config.max_invocations_per_tick {
                break;
            }
        }
        // TODO: Log tick summary and possible TriggerLoopInvocation event
    }

    fn resolve_frequency(&self) -> f32 {
        match &self.config.rhythm {
            SymbolicRhythm::Constant(hz) => *hz,
            SymbolicRhythm::EmotionDriven => {
                if let Some(emotion) = &self.config.emotional_signature {
                    modulate_frequency(1.0, emotion) // base 1Hz
                } else {
                    1.0
                }
            }
            _ => 1.0, // Default fallback
        }
    }
}

pub fn modulate_frequency(base: f32, emotion: &EmotionSignature) -> f32 {
    // TODO: Replace with true emotional mapping
    match emotion.intensity.round() as i32 {
        0 => base * 0.5,
        1 => base * 0.8,
        2 => base,
        3 => base * 1.5,
        _ => base * 2.0,
    }
}


//================================================

//         src/core/
/
//==============================================


//=========================
//       core/mod.rs
//==========================

pub mod construct_registry;
pub mod context_frame_engine;

pub use construct_registry::ConstructRegistry;

//========================
//      construct_registry.rs
//==========================




use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};

pub struct ConstructRegistry {
    constructs: HashMap<String, Arc<dyn ConstructAI + Send + Sync>>, // thread-safe
}

impl ConstructRegistry {
    pub fn new() -> Self {
        Self {
            constructs: HashMap::new(),
        }
    }

    pub fn insert<T>(&mut self, name: &str, construct: T)
    where
        T: ConstructAI + Send + Sync + 'static,
    {
        self.constructs.insert(name.to_string(), Arc::new(construct));
    }

    pub fn invoke(&self, name: &str, context: &ConstructContext) -> ConstructResult {
        match self.constructs.get(name) {
            Some(construct) => construct.reflect_on_scroll(context),
            None => ConstructResult::Refusal {
                reason: format!("No Construct found with name '{}'.", name),
                echo: Some("The name was whispered, but no presence replied.".into()),
            },
        }
    }

    pub fn list_constructs(&self) -> Vec<String> {
        self.constructs.keys().cloned().collect()
    }
}

// Example: to be placed in main.rs
// let mut registry = ConstructRegistry::new();
// registry.insert("mythscribe", Mythscribe::new(client, prompt));
// let result = registry.invoke("mythscribe", &context);



//=========================================
//
//         src/core/context_frame_engine.rs
//
//=========================================



use crate::construct_ai::ConstructContext;
use crate::scroll::{Scroll, EmotionSignature};
use crate::archive_memory::InMemoryArchive;
use crate::scroll_access_log::ScrollAccessLog;

pub enum ContextMode {
    Narrow,
    Broad,
    Echo,
}

pub struct ContextFrameEngine<'a> {
    pub archive: &'a InMemoryArchive,
    pub access_log: Option<&'a ScrollAccessLog>,
    pub mode: ContextMode,
    pub max_scrolls: usize,
}

impl<'a> ContextFrameEngine<'a> {
    pub fn new(archive: &'a InMemoryArchive, mode: ContextMode) -> Self {
        Self {
            archive,
            access_log: None,
            mode,
            max_scrolls: 5,
        }
    }

    pub fn with_access_log(mut self, log: &'a ScrollAccessLog) -> Self {
        self.access_log = Some(log);
        self
    }

    pub fn build_context(&self, triggering_scroll: &Scroll) -> ConstructContext {
        let mut scrolls = vec![triggering_scroll.clone()];

        let related = match self.mode {
            ContextMode::Narrow => self.archive.query_by_tags(&triggering_scroll.tags),
            ContextMode::Broad => self.archive.query_by_emotion(triggering_scroll.emotion_signature.clone()),
            ContextMode::Echo => self.archive.query_by_links(&triggering_scroll.id),
        };

        for s in related {
            if scrolls.len() >= self.max_scrolls {
                break;
            }
            if s.id != triggering_scroll.id {
                scrolls.push(s);
            }
        }

        ConstructContext {
            scrolls,
            emotion_signature: triggering_scroll.emotion_signature.clone(),
            tags: triggering_scroll.tags.clone(),
            user_input: None,
        }
    }
}



// src/core/cost_manager.rs – The Core Weave
//====================================
#![allow(dead_code)]
#![allow(unused_imports)]


use crate::invocation::invocation::Invocation;
use crate::scroll::Scroll;

#[derive(Debug, Clone)]
pub enum CostDecision {
    Allow,
    Throttle(f32), // throttle intensity 0.0 - 1.0
    Reject(String),
}

#[derive(Debug, Clone)]
pub enum RejectionOrigin {
    System,
    Construct(String),
}

#[derive(Debug, Clone)]
pub struct InvocationCost {
    pub context: ContextCost,
    pub system: SystemCost,
    pub decision: CostDecision,
    pub cost_profile: CostProfile,
    pub rejection_origin: Option<RejectionOrigin>,
    pub hesitation_signal: Option<String>,
    pub poetic_rejection: Option<String>,
    pub symbolic_echo: Option<String>,
    pub emotion_tension: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct CostProfile {
    pub system_pressure: f32,
    pub token_pressure: f32,
    pub symbolic_origin: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContextCost {
    pub token_estimate: usize,
    pub context_span: usize,
    pub relevance_score: f32,
}

#[derive(Debug, Clone)]
pub struct SystemCost {
    pub cpu_cycles: f64,
    pub memory_used_mb: f64,
    pub io_ops: usize,
    pub scrolls_touched: usize,
}

pub trait ContextScorer {
    fn score(&self, invocation: &Invocation, scrolls: &[Scroll], semantic_score: f32) -> f32;
}

pub struct SemanticContextScorer;

pub struct CostManager;

impl CostManager {
    pub fn calculate_cost_profile(context: &ContextCost, system: &SystemCost) -> CostProfile {
        let token_pressure = context.token_estimate as f32 * 0.001
            + context.context_span as f32 * 0.1
            - context.relevance_score * 0.3;

        let system_pressure = system.cpu_cycles * 100.0
            + system.memory_used_mb * 0.25
            + system.io_ops as f64 * 0.05
            + system.scrolls_touched as f64 * 0.2;

        CostProfile {
            system_pressure: system_pressure as f32,
            token_pressure,
            symbolic_origin: None,
        }
    }

    pub fn assess(_invocation: &Invocation, scrolls: &[Scroll]) -> InvocationCost {
        let token_estimate = scrolls.iter().map(|s| s.markdown_body.len() / 4).sum();
        let scorer = SemanticContextScorer;
        let relevance_score = scorer.score(_invocation, scrolls, 0.5);

        let context = ContextCost {
            token_estimate,
            context_span: scrolls.len(),
            relevance_score,
        };

        let system = SystemCost {
            cpu_cycles: 0.0023,
            memory_used_mb: 3.2,
            io_ops: 7,
            scrolls_touched: scrolls.len(),
        };

        let decision = if context.token_estimate > 12000 {
            CostDecision::Reject("Context window too large.".to_string())
        } else {
            CostDecision::Allow
        };

        let cost_profile = Self::calculate_cost_profile(&context, &system);

        InvocationCost {
            context,
            system,
            decision,
            rejection_origin: Some(RejectionOrigin::System),
            hesitation_signal: Some("The archive paused, uncertain.".to_string()),
            poetic_rejection: Some("A whisper lost in the tide of memory.".to_string()),
            symbolic_echo: Some("The loom remained still.".to_string()),
            emotion_tension: Some(0.82),
            cost_profile,
        }
    }
}





//============================================================

//      src/chat/

//===========================================================


//===========================================
//src/chat/mod.rs
//===========================================

pub mod chat_session;
pub mod chat_router;
pub mod chat_dispatcher;


// ===============================
// src/chat/chat_session.rs
// ===============================

use crate::schema::EmotionSignature;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String, // "user" | "assistant" | "system"
    pub content: String,
    pub emotion: Option<EmotionSignature>,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub messages: Vec<ChatMessage>,
    pub target_construct: Option<String>,
    pub mood_seed: Option<String>,
}

impl ChatSession {
    pub fn new(target_construct: Option<String>, mood_seed: Option<String>) -> Self {
        ChatSession {
            messages: Vec::new(),
            target_construct,
            mood_seed,
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str, emotion: Option<EmotionSignature>) {
        self.messages.push(ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
            emotion,
        });
    }

    pub fn last_user_message(&self) -> Option<&ChatMessage> {
        self.messages.iter().rev().find(|m| m.role == "user")
    }

    pub fn last_assistant_message(&self) -> Option<&ChatMessage> {
        self.messages.iter().rev().find(|m| m.role == "assistant")
    }
}




// ===============================
// src/chat/chat_dispatcher.rs
// ===============================

use crate::chat::chat_session::{ChatSession, ChatMessage};
use crate::chat::chat_router::ChatRouter;
use crate::construct_ai::ConstructResult;
use crate::invocation::InvocationManager;
use crate::invocation::aelren::AelrenHerald;
use crate::scroll::Scroll;
use crate::schema::EmotionSignature;

pub struct ChatDispatcher;

impl ChatDispatcher {
    pub fn dispatch(
        session: &mut ChatSession,
        user_input: &str,
        manager: &InvocationManager,
        aelren: &AelrenHerald,
        memory: &[Scroll],
    ) -> ChatMessage {
        // Append user message to session
        session.add_message("user", user_input, None);

        let user_msg = session.messages.last().unwrap();
        let target = ChatRouter::route_target(user_msg)
            .or_else(|| session.target_construct.clone())
            .unwrap_or_else(ChatRouter::default_target);

        // Get latest scroll for context (temporary until better memory engine)
        let scroll = memory.last().expect("No scrolls available");

        let result = if target == "symbolic" {
            manager.invoke_symbolically_with_aelren(scroll, aelren)
        } else {
            let context = manager.registry.build_context(scroll); // helper if implemented
            manager.invoke_by_name(&target, &context, 0)
        };

        let reply = match result {
            ConstructResult::Insight { text } => text,
            ConstructResult::ScrollDraft { content, .. } => content,
            ConstructResult::Refusal { reason, .. } => reason,
        };

        let assistant_msg = ChatMessage {
            role: "assistant".into(),
            content: reply,
            emotion: Some(EmotionSignature::Reflective), // placeholder for future detection
        };

        session.messages.push(assistant_msg.clone());
        assistant_msg
    }
}



// ===============================
// src/chat/chat_router.rs
// ===============================

use crate::chat::chat_session::ChatMessage;

pub struct ChatRouter;

impl ChatRouter {
    /// Attempt to extract a Construct name from the first word or known patterns.
    pub fn route_target(message: &ChatMessage) -> Option<String> {
        let content = message.content.trim();

        // Match address-style, e.g. "Mythscribe, tell me..."
        if let Some((prefix, _)) = content.split_once(',') {
            return Some(prefix.trim().to_lowercase());
        }

        // Command-style: "/sirion Do we have a schema?"
        if let Some(stripped) = content.strip_prefix('/') {
            if let Some(word) = stripped.split_whitespace().next() {
                return Some(word.trim().to_lowercase());
            }
        }

        None
    }

    /// Fallback construct if routing fails
    pub fn default_target() -> String {
        "mythscribe".into()
    }
}





