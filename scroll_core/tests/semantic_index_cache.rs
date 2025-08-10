use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::Lazy;

use scroll_core::archive::archive_loader::load_scrolls_from_directory;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::error::ArchiveError;
use scroll_core::archive::semantic_index::{Embedder, TokenEmbedder};

struct CountingEmbedder<'a> {
    inner: &'a dyn Embedder,
    counter: &'a AtomicUsize,
}

static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

fn guard_env() -> MutexGuard<'static, ()> {
    TEST_MUTEX.lock().unwrap()
}

impl<'a> Embedder for CountingEmbedder<'a> {
    fn embed(&self, text: &str) -> Result<std::collections::HashSet<String>, ArchiveError> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        eprintln!("embed for: {}", text.replace('\n', "\\n"));
        self.inner.embed(text)
    }
}

fn write_scroll(
    dir: &tempfile::TempDir,
    name: &str,
    title: &str,
    tags: &[&str],
    body: &str,
) -> PathBuf {
    let path = dir.path().join(name);
    let tags_yaml = if tags.is_empty() {
        "[]".to_string()
    } else {
        format!(
            "[{}]",
            tags.iter()
                .map(|t| format!("\"{}\"", t))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let content = format!(
        "---\n\ntitle: {}\n\nscroll_type: Canon\n\nemotion_signature: neutral\n\ntags: {}\n\n---\n\n{}\n",
        title, tags_yaml, body
    );
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn cold_vs_warm_load_len_equal() {
    let dir = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let _g = guard_env();
    std::env::set_var("SC_INDEX_CACHE_DIR", cache.path());
    let _ = std::fs::create_dir_all(cache.path());

    write_scroll(&dir, "a.md", "One", &["alpha"], "hello world");
    write_scroll(&dir, "b.md", "Two", &["beta"], "lorem ipsum");

    let scrolls1 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive1 = InMemoryArchive::new(scrolls1);
    archive1.build_semantic_index(&TokenEmbedder).unwrap();
    let len1 = archive1.semantic_index_len();

    // Ensure cache files exist
    let meta = cache.path().join("semantic_index.v1.meta.json");
    let vecb = cache.path().join("semantic_index.v1.vec.bin");
    let fps = cache.path().join("fingerprints.v1.json");
    let entries: Vec<_> = std::fs::read_dir(cache.path()).unwrap().collect();
    eprintln!("cache dir entries: {}", entries.len());
    for e in entries {
        if let Ok(ent) = e {
            eprintln!("cache file: {}", ent.path().display());
        }
    }
    assert!(meta.exists() && vecb.exists() && fps.exists());

    // Warm load
    let scrolls2 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive2 = InMemoryArchive::new(scrolls2);
    archive2.build_semantic_index(&TokenEmbedder).unwrap();
    let len2 = archive2.semantic_index_len();

    assert_eq!(len1, len2);
}

#[test]
fn incremental_rebuild_only_changed() {
    let dir = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let _g = guard_env();
    std::env::set_var("SC_INDEX_CACHE_DIR", cache.path());

    let a = write_scroll(&dir, "a.md", "One", &["alpha"], "hello world");
    write_scroll(&dir, "b.md", "Two", &["beta"], "lorem ipsum");

    // Cold build
    let scrolls1 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive1 = InMemoryArchive::new(scrolls1);
    archive1.build_semantic_index(&TokenEmbedder).unwrap();

    let meta = cache.path().join("semantic_index.v1.meta.json");
    let vecb = cache.path().join("semantic_index.v1.vec.bin");
    let fps = cache.path().join("fingerprints.v1.json");
    let entries2: Vec<_> = std::fs::read_dir(cache.path()).unwrap().collect();
    eprintln!("cache dir entries: {}", entries2.len());
    for e in entries2 {
        if let Ok(ent) = e {
            eprintln!("cache file: {}", ent.path().display());
        }
    }
    assert!(meta.exists() && vecb.exists() && fps.exists());

    // Modify one file
    fs::write(&a, "---\n\ntitle: One\n\nscroll_type: Canon\n\nemotion_signature: neutral\n\ntags: [alpha]\n\n---\n\nchanged body\n").unwrap();

    // Warm build with counting embedder
    let calls = AtomicUsize::new(0);
    let counter = CountingEmbedder {
        inner: &TokenEmbedder,
        counter: &calls,
    };
    let scrolls2 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive2 = InMemoryArchive::new(scrolls2);
    archive2.build_semantic_index(&counter).unwrap();

    assert_eq!(archive2.semantic_index_len(), 2);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn full_invalidation_on_model_change() {
    let dir = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let _g = guard_env();
    std::env::set_var("SC_INDEX_CACHE_DIR", cache.path());

    write_scroll(&dir, "a.md", "One", &["alpha"], "hello");
    write_scroll(&dir, "b.md", "Two", &["beta"], "world");

    let scrolls1 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive1 = InMemoryArchive::new(scrolls1);
    archive1.build_semantic_index(&TokenEmbedder).unwrap();

    // Change model meta
    std::env::set_var("SC_EMBEDDER_MODEL", "different-model");

    let calls = AtomicUsize::new(0);
    let counter = CountingEmbedder {
        inner: &TokenEmbedder,
        counter: &calls,
    };
    let scrolls2 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive2 = InMemoryArchive::new(scrolls2);
    archive2.build_semantic_index(&counter).unwrap();

    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn corrupt_vec_bin_recovers() {
    let dir = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let _g = guard_env();
    std::env::set_var("SC_INDEX_CACHE_DIR", cache.path());

    write_scroll(&dir, "a.md", "One", &["alpha"], "hello");
    write_scroll(&dir, "b.md", "Two", &["beta"], "world");

    let scrolls1 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive1 = InMemoryArchive::new(scrolls1);
    archive1.build_semantic_index(&TokenEmbedder).unwrap();

    // Corrupt the binary vectors
    fs::write(
        cache.path().join("semantic_index.v1.vec.bin"),
        b"not a real bin file",
    )
    .unwrap();

    let calls = AtomicUsize::new(0);
    let counter = CountingEmbedder {
        inner: &TokenEmbedder,
        counter: &calls,
    };
    let scrolls2 = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive2 = InMemoryArchive::new(scrolls2);
    archive2.build_semantic_index(&counter).unwrap();

    assert_eq!(archive2.semantic_index_len(), 2);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn lock_file_skips_write() {
    let dir = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let _g = guard_env();
    std::env::set_var("SC_INDEX_CACHE_DIR", cache.path());

    write_scroll(&dir, "a.md", "One", &["alpha"], "hello");

    // Pre-create a lock file to simulate another writer
    fs::write(cache.path().join(".rebuild.lock"), b"").unwrap();

    let scrolls = load_scrolls_from_directory(dir.path()).unwrap();
    let mut archive = InMemoryArchive::new(scrolls);
    // Should not crash; may skip cache write
    archive.build_semantic_index(&TokenEmbedder).unwrap();
}
