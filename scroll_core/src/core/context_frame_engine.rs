//! The ContextFrameEngine assembles relevant scrolls and history before an invocation runs.
//! It queries the archive and access logs to construct a rich ConstructContext.
//! See [ContextFrameEngine](../../AGENTS.md#contextframeengine) in the construct directory.
//=========================================
//
//         src/core/context_frame_engine.rs
//
//=========================================

use crate::archive::archive_memory::InMemoryArchive;
use crate::archive::scroll_access_log::ScrollAccessLog;
use crate::construct_ai::ConstructContext;
use crate::models::model_registry::ContextThresholds;
use crate::scroll::Scroll;
use uuid::Uuid;

use super::context_decisions::{ContextBuildReport, ContextDecision, ContextFrameSummary};

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
    pub thresholds: ContextThresholds,
    pub explain_construct: String,
}

impl<'a> ContextFrameEngine<'a> {
    pub fn new(archive: &'a InMemoryArchive, mode: ContextMode) -> Self {
        Self {
            archive,
            access_log: None,
            mode,
            max_scrolls: 5,
            thresholds: ContextThresholds::default(),
            explain_construct: "<unknown>".into(),
        }
    }

    pub fn with_access_log(mut self, log: &'a ScrollAccessLog) -> Self {
        self.access_log = Some(log);
        self
    }

    pub fn with_thresholds(mut self, thresholds: ContextThresholds) -> Self {
        self.max_scrolls = thresholds.max_items;
        self.thresholds = thresholds;
        self
    }

    pub fn with_construct_label(mut self, construct: &str) -> Self {
        self.explain_construct = construct.to_string();
        self
    }

    pub fn build_context(
        &self,
        triggering_scroll: &Scroll,
    ) -> (ConstructContext, ContextBuildReport) {
        let start = std::time::Instant::now();
        let frame_id = Uuid::new_v4();
        let mut running_tokens: usize = token_estimate(triggering_scroll);
        let mut scrolls = vec![triggering_scroll.clone()];

        let related = match self.mode {
            ContextMode::Narrow => self
                .archive
                .query_by_tags(&triggering_scroll.yaml_metadata.tags),
            ContextMode::Broad => {
                let query = compose_query(triggering_scroll);
                let semantic: Vec<_> = self
                    .archive
                    .query_semantic(&query, self.max_scrolls * 2)
                    .into_iter()
                    .filter(|(_, score)| *score >= self.thresholds.min_relevance_score)
                    .map(|(s, _)| s)
                    .collect();
                if semantic.is_empty() {
                    self.archive
                        .query_by_emotion(triggering_scroll.emotion_signature.clone())
                } else {
                    semantic
                }
            }
            ContextMode::Echo => {
                let query = compose_query(triggering_scroll);
                let semantic: Vec<_> = self
                    .archive
                    .query_semantic(&query, self.max_scrolls * 2)
                    .into_iter()
                    .filter(|(_, score)| *score >= self.thresholds.min_relevance_score)
                    .map(|(s, _)| s)
                    .collect();
                if semantic.is_empty() {
                    self.archive.query_by_links(&triggering_scroll.id)
                } else {
                    semantic
                }
            }
        };

        // Compute raw scores and recency-decayed scores for ordering
        let now = chrono::Utc::now();
        // Pull semantic scores again for related items to capture raw scores
        let query = compose_query(triggering_scroll);
        let mut candidates: Vec<(Scroll, f32, f32)> = self
            .archive
            .query_semantic(&query, related.len().max(self.max_scrolls * 2))
            .into_iter()
            .filter_map(|(s, score)| {
                if s.id == triggering_scroll.id {
                    return None;
                }
                let age_hours = (now
                    .signed_duration_since(s.origin.last_modified)
                    .num_seconds()
                    .max(1) as f32)
                    / 3600.0;
                Some((s, score, age_hours))
            })
            .collect();
        // If semantic results empty (e.g., Narrow mode), synthesize candidates from related list with neutral score
        if candidates.is_empty() {
            for s in related {
                if s.id == triggering_scroll.id {
                    continue;
                }
                let age_hours = (now
                    .signed_duration_since(s.origin.last_modified)
                    .num_seconds()
                    .max(1) as f32)
                    / 3600.0;
                candidates.push((s, 1.0, age_hours));
            }
        }

        // Order by recency-decayed score
        let half_life = self.thresholds.recency_half_life_hours.max(1.0);
        candidates.sort_by(|a, b| {
            let da = decay_score(a.1, a.2, half_life);
            let db = decay_score(b.1, b.2, half_life);
            db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut decisions: Vec<ContextDecision> = Vec::new();
        let mut included = 0usize;
        let mut excluded = 0usize;
        for (idx, (s, score, age_h)) in candidates.into_iter().enumerate() {
            let mut reason = String::new();
            let mut include = true;
            if score < self.thresholds.min_relevance_score {
                include = false;
                reason = "below_score_threshold".into();
            }
            if include && scrolls.len() >= self.max_scrolls {
                include = false;
                reason = "max_items_limit".into();
            }
            let item_tokens = token_estimate(&s);
            if include && running_tokens + item_tokens > self.thresholds.max_context_tokens {
                include = false;
                reason = "budget_exceeded".into();
            }
            if include {
                running_tokens += item_tokens;
                scrolls.push(s.clone());
                included += 1;
                decisions.push(ContextDecision {
                    construct: self.explain_construct.clone(),
                    frame_id,
                    candidate_path: s.yaml_metadata.file_path.clone(),
                    included: true,
                    reason: if reason.is_empty() {
                        "included".into()
                    } else {
                        reason
                    },
                    score: score as f32,
                    recency_hours: age_h,
                    running_tokens,
                    max_tokens: self.thresholds.max_context_tokens,
                });
            } else {
                excluded += 1;
                decisions.push(ContextDecision {
                    construct: self.explain_construct.clone(),
                    frame_id,
                    candidate_path: s.yaml_metadata.file_path.clone(),
                    included: false,
                    reason: if reason.is_empty() {
                        if idx >= self.max_scrolls {
                            "max_items_limit".into()
                        } else {
                            "excluded".into()
                        }
                    } else {
                        reason
                    },
                    score: score as f32,
                    recency_hours: age_h,
                    running_tokens,
                    max_tokens: self.thresholds.max_context_tokens,
                });
            }
        }

        let elapsed = start.elapsed().as_millis();
        #[cfg(feature = "metrics")]
        {
            use metrics::{counter, histogram};
            histogram!("context.build_ms", elapsed as f64);
            counter!("context.included", included as u64);
            counter!("context.excluded", excluded as u64);
        }

        let summary = ContextFrameSummary {
            construct: self.explain_construct.clone(),
            frame_id,
            max_tokens: self.thresholds.max_context_tokens,
            max_items: self.thresholds.max_items,
            min_relevance: self.thresholds.min_relevance_score,
            half_life_hours: self.thresholds.recency_half_life_hours,
            total_candidates: included + excluded,
            included_count: included,
            excluded_count: excluded,
            build_ms: elapsed,
        };

        (
            ConstructContext {
                scrolls,
                emotion_signature: triggering_scroll.emotion_signature.clone(),
                tags: triggering_scroll.yaml_metadata.tags.clone(),
                user_input: None,
            },
            ContextBuildReport { summary, decisions },
        )
    }
}

fn compose_query(scroll: &Scroll) -> String {
    let first_lines = scroll
        .markdown_body
        .lines()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "{} {} {}",
        scroll.title,
        scroll.yaml_metadata.tags.join(" "),
        first_lines
    )
}

fn token_estimate(s: &Scroll) -> usize {
    // crude estimate; consistent with CostManager approach
    (s.markdown_body.len() / 4).max(1)
}

fn decay_score(score: f32, age_hours: f32, half_life_hours: f32) -> f32 {
    let lambda = std::f32::consts::LN_2 / half_life_hours.max(1.0);
    let w = (-lambda * age_hours.max(0.0)).exp();
    score * w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archive::archive_memory::InMemoryArchive;
    use crate::schema::{ScrollType, YamlMetadata};

    fn mk_scroll(title: &str, body_len: usize, age_hours: i64) -> Scroll {
        let now = chrono::Utc::now() - chrono::Duration::hours(age_hours);
        Scroll {
            id: uuid::Uuid::new_v4(),
            title: title.into(),
            scroll_type: ScrollType::Canon,
            yaml_metadata: YamlMetadata {
                title: title.into(),
                scroll_type: ScrollType::Canon,
                emotion_signature: crate::schema::EmotionSignature::neutral(),
                tags: vec!["t".into()],
                archetype: None,
                quorum_required: false,
                last_modified: Some(now),
                file_path: Some(format!("/tmp/{}.md", title)),
            },
            tags: vec!["t".into()],
            archetype: None,
            quorum_required: false,
            markdown_body: "x".repeat(body_len),
            invocation_phrase: String::new(),
            sigil: String::new(),
            status: crate::schema::ScrollStatus::Draft,
            emotion_signature: crate::schema::EmotionSignature::neutral(),
            linked_scrolls: vec![],
            origin: crate::scroll::ScrollOrigin {
                created: now,
                authored_by: None,
                last_modified: now,
            },
        }
    }

    #[test]
    fn respects_token_budget_truncation() {
        let s0 = mk_scroll("root", 200, 0);
        let s1 = mk_scroll("a", 1000, 1);
        let s2 = mk_scroll("b", 1000, 2);
        let s3 = mk_scroll("c", 1000, 3);
        let archive = InMemoryArchive::new(vec![s0.clone(), s1.clone(), s2.clone(), s3.clone()]);
        let thresholds = ContextThresholds {
            max_context_tokens: 500, // root ~50 + one 250 => include only one candidate
            min_relevance_score: 0.0,
            recency_half_life_hours: 48.0,
            max_items: 5,
        };
        let engine = ContextFrameEngine::new(&archive, ContextMode::Broad)
            .with_thresholds(thresholds)
            .with_construct_label("Test");
        let (ctx, report) = engine.build_context(&s0);
        // 1 root + 1 candidate
        assert_eq!(ctx.scrolls.len(), 2);
        assert!(report.summary.included_count >= 1);
        assert!(report.summary.excluded_count >= 1);
    }
}
