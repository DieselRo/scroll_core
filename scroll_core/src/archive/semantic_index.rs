use std::collections::HashSet;
use uuid::Uuid;
use log::info;

use crate::scroll::Scroll;

pub struct SemanticIndex {
    vectors: Vec<(Uuid, HashSet<String>)>,
}

impl SemanticIndex {
    pub fn build(scrolls: &[Scroll]) -> Self {
        info!("Generating semantic vectors for {} scrolls", scrolls.len());
        let vectors = scrolls
            .iter()
            .map(|s| {
                let first_lines = s
                    .markdown_body
                    .lines()
                    .take(3)
                    .collect::<Vec<_>>()
                    .join(" ");
                let text = format!("{} {} {}", s.title, s.yaml_metadata.tags.join(" "), first_lines);
                (s.id, tokenize(&text))
            })
            .collect();
        info!("Vector generation complete");
        Self { vectors }
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
    if union == 0.0 { 0.0 } else { intersection / union }
}
