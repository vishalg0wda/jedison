use eyre::{eyre, WrapErr};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct MistralAuth {
    api_key: String,
}
impl MistralAuth {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Default)]
pub struct MistralSDK {
    auth: MistralAuth,
    client: Client,
    agent: Option<String>,
}

#[derive(Debug, Serialize)]
struct MistralAgentCompletionRequest<'a> {
    agent_id: &'a str,
    messages: Vec<MistralAgentMessage>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct MistralAgentMessage {
    content: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct MistralAgentCompletionResponse {
    choices: Vec<MistralChatCompletionChoice>,
}

#[derive(Debug, Deserialize)]
struct MistralChatCompletionChoice {
    message: MistralAgentMessage,
}

impl MistralSDK {
    pub fn new(auth: MistralAuth) -> Self {
        let client = Client::default();
        Self {
            auth,
            client,
            agent: None,
        }
    }

    pub fn with_default_agent(mut self, agent_id: String) -> Self {
        self.agent = Some(agent_id);
        self
    }

    pub fn chat(&self, message: &str) -> eyre::Result<String> {
        let Some(agent_id) = &self.agent else {
            return Err(eyre!(
                "no default agent configured. please specify one when constructing the SKD"
            ));
        };

        self.chat_with(message, agent_id)
    }

    pub fn chat_with(&self, message: &str, agent_id: &str) -> eyre::Result<String> {
        let url = "https://api.mistral.ai/v1/agents/completions";
        let message_to_agent = MistralAgentMessage {
            content: message.into(),
            role: "user".into(),
        };
        let request = MistralAgentCompletionRequest {
            agent_id,
            messages: vec![message_to_agent],
        };
        let response = self
            .client
            .request(Method::POST, url)
            .bearer_auth(&self.auth.api_key)
            .header(CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .wrap_err("failed to serialize/send request")?;
        if !response.status().is_success() {
            return Err(eyre!("received status code {}", response.status().as_u16()));
        }
        let response: MistralAgentCompletionResponse =
            response.json().wrap_err("failed to parse response json")?;
        let response = response.choices[0].message.content.clone();

        Ok(response)
    }
}
