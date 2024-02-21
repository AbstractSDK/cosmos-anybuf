pub mod modules;
mod whitelist_parser;

use std::{fs, time::Duration};

use anyhow::Result as AResult;
use chatgpt::{
    client::ChatGPT,
    config::{ChatGPTEngine, ModelConfigurationBuilder},
    converse::Conversation,
    types::{ChatMessage, Role},
};

const GPT_TURBO: ChatGPTEngine = ChatGPTEngine::Custom("gpt-4-1106-preview");

fn remove_last_line(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().collect();
    lines.pop();
    lines.join("\n")
}

pub fn prompt_query_setup(
    (example_input, example_output): (String, String),
    whitelist_example: String,
) -> AResult<Conversation> {
    let config = ModelConfigurationBuilder::default()
        .engine(GPT_TURBO)
        .temperature(0.2)
        .top_p(0.1)
        .max_tokens(4096u32)
        .timeout(Duration::from_secs(300))
        .build()?;

    let api_key = std::env::var("API_KEY").unwrap();

    let client = ChatGPT::new_with_config(api_key, config)?;

    let system_prompt = fs::read_to_string("prompt_query.md")?;

    let example_input = [whitelist_example, example_input].concat();
    let convo = Conversation::new_with_history(
        client,
        vec![
            ChatMessage {
                role: Role::System,
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: example_input,
            },
            ChatMessage {
                role: Role::Assistant,
                content: example_output,
            },
        ],
    );
    Ok(convo)
}

pub fn prompt_tx_setup((example_input, example_output): (String, String)) -> AResult<Conversation> {
    let config = ModelConfigurationBuilder::default()
        .engine(GPT_TURBO)
        .temperature(0.2)
        .top_p(0.1)
        .max_tokens(4096u32)
        .timeout(Duration::from_secs(300))
        .build()?;

    let api_key = std::env::var("API_KEY").unwrap();

    let client = ChatGPT::new_with_config(api_key, config)?;

    let system_prompt = fs::read_to_string("prompt_tx.md")?;

    let convo = Conversation::new_with_history(
        client,
        vec![
            ChatMessage {
                role: Role::System,
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: Role::User,
                content: example_input,
            },
            ChatMessage {
                role: Role::Assistant,
                content: example_output,
            },
        ],
    );
    Ok(convo)
}
