use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    Ollama,
    Cloud,
    Mock,
}

impl ProviderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::Cloud => "cloud",
            Self::Mock => "mock",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub default_provider: ProviderKind,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub cloud_base_url: String,
    pub cloud_model: String,
    pub cloud_api_key: Option<String>,
    pub max_history: usize,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bind_addr: env_or("BIND_ADDR", "0.0.0.0:3000"),
            default_provider: parse_provider(&env_or("DEFAULT_PROVIDER", "ollama")),
            ollama_base_url: env_or("OLLAMA_BASE_URL", "http://localhost:11434/v1"),
            ollama_model: env_or("OLLAMA_MODEL", "qwen3:4b"),
            cloud_base_url: env_or(
                "CLOUD_BASE_URL",
                "https://generativelanguage.googleapis.com/v1beta/openai",
            ),
            cloud_model: env_or("CLOUD_MODEL", "gemini-2.5-flash"),
            cloud_api_key: non_empty_env("CLOUD_API_KEY"),
            max_history: env_or("MAX_HISTORY", "20").parse().unwrap_or(20),
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default.to_owned())
}

fn non_empty_env(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.trim().is_empty())
}

fn parse_provider(value: &str) -> ProviderKind {
    match value.trim().to_ascii_lowercase().as_str() {
        "cloud" | "gemini" | "groq" | "openrouter" => ProviderKind::Cloud,
        "mock" => ProviderKind::Mock,
        _ => ProviderKind::Ollama,
    }
}
