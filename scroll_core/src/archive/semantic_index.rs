use log::info;
use std::collections::HashSet;
use uuid::Uuid;

#[cfg(feature = "metrics")]
use metrics::histogram;

use crate::scroll::Scroll;

pub struct SemanticIndex {
    vectors: Vec<(Uuid, HashSet<String>)>,
}

impl SemanticIndex {
    pub fn build(scrolls: &[Scroll]) -> Self {
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
                let result = (s.id, tokenize(&text));

                #[cfg(feature = "metrics")]
                histogram!("scroll_embed_time_seconds").record(embed_timer.elapsed().as_secs_f64());

                result
            })
            .collect::<Vec<_>>();

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
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}
