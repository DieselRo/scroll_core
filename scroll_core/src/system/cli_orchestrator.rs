// ===============================
// src/system/cli_orchestrator.rs
// ===============================

use crate::construct_ai::ConstructAI;
use crate::construct_ai::ConstructContext;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};
use crate::scroll::Scroll;
use chrono::Utc;
use std::io::{self, Write};
use uuid::Uuid;

pub enum Command {
    Snapshot,
    Invoke(String),
    Symbolic,
    ListScrolls,
    Test(String),
    Exit,
    Help,
    Unknown,
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["snapshot"] => Command::Snapshot,
        ["invoke", name] => Command::Invoke(name.to_string()),
        ["symbolic"] => Command::Symbolic,
        ["list", "scrolls"] => Command::ListScrolls,
        ["test", rest @ ..] => Command::Test(rest.join(" ")),
        ["exit"] => Command::Exit,
        ["help"] => Command::Help,
        _ => Command::Unknown,
    }
}

pub fn run_cli(manager: &InvocationManager, aelren: &AelrenHerald, scrolls: &[Scroll]) {
    println!("\n📜 Welcome to the Scroll Core CLI. Type `help` for a list of commands.\n");

    loop {
        print!("⚙️ > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }

        match parse_command(&input) {
            Command::Snapshot => {
                use crate::system::snapshot;

                match snapshot::generate_source_index(
                    "C:/Users/Riede/scroll_core",
                    "C:/Users/Riede/scroll_core/scrolls/docs",
                ) {
                    Ok(_) => println!("✅ Source snapshot created."),
                    Err(e) => println!("❌ Snapshot failed: {}", e),
                }
            }
            Command::Invoke(name) => {
                if let Some(scroll) = scrolls.last() {
                    let context = manager.registry.build_context_from_scroll(
                        scroll,
                        &format!("Invoked via CLI command: {}", name),
                    );
                    let result = manager.invoke_by_name(&name, &context, 0);
                    println!("\nResult: {:?}\n", result);
                } else {
                    println!("No scrolls available to invoke with.");
                }
            }
            Command::Symbolic => {
                if let Some(scroll) = scrolls.last() {
                    let result = manager.invoke_symbolically_with_aelren(scroll, aelren);
                    println!("\nResult: {:?}\n", result);
                } else {
                    println!("No scrolls available for symbolic invocation.");
                }
            }
            Command::ListScrolls => {
                for (i, scroll) in scrolls.iter().enumerate() {
                    println!(
                        "{}: {} [tags: {:?}]",
                        i, scroll.title, scroll.yaml_metadata.tags
                    );
                }
            }
            Command::Test(prompt) => {
                use crate::invocation::constructs::openai_construct::{Mythscribe, OpenAIClient};
                let client = OpenAIClient::new_from_env();

                let mythscribe = Mythscribe::new(
                    client,
                    "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
                );

                let now = Utc::now();
                let pseudo_scroll = Scroll {
                    id: Uuid::new_v4(),
                    title: "Ephemeral Scroll".into(),
                    scroll_type: ScrollType::Echo,
                    yaml_metadata: YamlMetadata {
                        title: "Ephemeral Scroll".into(),
                        scroll_type: ScrollType::Echo,
                        emotion_signature: EmotionSignature::curious(),
                        tags: vec!["ephemeral".into(), "prompt".into()],
                        last_modified: Some(now),
                    },
                    markdown_body: prompt.clone(),
                    invocation_phrase: prompt.clone(),
                    sigil: "🪶".into(),
                    status: ScrollStatus::Draft,
                    emotion_signature: EmotionSignature::curious(),
                    linked_scrolls: vec![],
                    origin: crate::scroll::ScrollOrigin {
                        created: now,
                        authored_by: Some("CLI".into()),
                        last_modified: now,
                    },
                };

                let context = ConstructContext {
                    scrolls: vec![pseudo_scroll],
                    emotion_signature: EmotionSignature::curious(),
                    tags: vec!["ephemeral".into()],
                    user_input: Some(prompt.clone()),
                };

                println!("🔍 Sending prompt to Mythscribe...\n");
                let result = mythscribe.reflect_on_scroll(&context);

                match result {
                    crate::construct_ai::ConstructResult::Insight { text } => {
                        println!("\n🪶 Mythscribe:\n{}", text)
                    }
                    crate::construct_ai::ConstructResult::Refusal { reason, echo } => {
                        println!("Mythscribe refused: {}", reason);
                        if let Some(e) = echo {
                            println!("Echo: {}", e);
                        }
                    }
                    crate::construct_ai::ConstructResult::ModifiedScroll(scroll) => {
                        println!("(Modified scroll returned):\n{}", scroll.markdown_body)
                    }
                    crate::construct_ai::ConstructResult::ScrollDraft { content, .. } => {
                        println!("Draft:\n{}", content)
                    }
                }
            }
            Command::Exit => {
                println!("Exiting. May your scrolls be true.");
                break;
            }
            Command::Help => {
                println!(
                    "
Available commands:"
                );
                println!("  snapshot             - Generate source_index.md for GPT reference");
                println!("  invoke <name>       - Call a named Construct");
                println!("  symbolic            - Let Aelren choose the Construct");
                println!("  list scrolls        - View current scroll memory");
                println!("  exit                - Quit the program");
                println!("  help                - Show this help message\n");
            }
            Command::Unknown => {
                println!("Unknown command. Type `help` for options.");
            }
        }
    }
}
