use crate::models::ollama::{OllamaModel, OllamaStatus};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::process::Command;
use std::time::Duration;

const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";
const RECOMMENDED_MODEL: &str = "qwen3:8b";

pub struct OllamaService {
    api_url: String,
    http: Client,
}

impl OllamaService {
    pub fn new(api_url: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            http: Client::builder()
                .timeout(Duration::from_secs(2))
                .build()
                .expect("ollama client should build"),
        }
    }

    pub fn default() -> Self {
        let api_url = env::var("OLLAMA_HOST").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_string());
        Self::new(api_url)
    }

    pub fn pull_model(&self, model: &str) -> Result<(), String> {
        let output = Command::new("ollama")
            .arg("pull")
            .arg(model)
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            return Err(if stderr.is_empty() {
                format!("ollama pull {model} exited with {}", output.status)
            } else {
                stderr
            });
        }

        Ok(())
    }

    pub fn discover(&self) -> OllamaStatus {
        let version_result = self.detect_installation();
        let installed = version_result.is_ok();
        let version = version_result.ok();

        match self.fetch_models() {
            Ok(models) => OllamaStatus {
                installed,
                version,
                api_reachable: true,
                api_url: self.api_url.clone(),
                recommended_model_present: models.iter().any(|model| model.name == RECOMMENDED_MODEL),
                models,
                error: None,
            },
            Err(error) => OllamaStatus {
                installed,
                version,
                api_reachable: false,
                api_url: self.api_url.clone(),
                recommended_model_present: false,
                models: Vec::new(),
                error: Some(error),
            },
        }
    }

    fn detect_installation(&self) -> Result<String, String> {
        let output = Command::new("ollama")
            .arg("--version")
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            return Err(if stderr.is_empty() {
                format!("ollama --version exited with {}", output.status)
            } else {
                stderr
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    fn fetch_models(&self) -> Result<Vec<OllamaModel>, String> {
        let response = self
            .http
            .get(format!("{}/api/tags", self.api_url))
            .send()
            .map_err(|error| error.to_string())?;

        if !response.status().is_success() {
            return Err(format!("{} returned HTTP {}", self.api_url, response.status()));
        }

        let payload: OllamaTagsResponse = response.json().map_err(|error| error.to_string())?;
        Ok(payload
            .models
            .into_iter()
            .map(|model| OllamaModel {
                name: model.name,
                size: model.size,
                modified_at: model.modified_at,
            })
            .collect())
    }
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    #[serde(default)]
    models: Vec<OllamaTagModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagModel {
    name: String,
    size: Option<u64>,
    modified_at: Option<String>,
}
