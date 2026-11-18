use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::config::ProviderKind;
use crate::language::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Character {
    Girl,
    Boy,
}

impl Character {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Girl => "girl",
            Self::Boy => "boy",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatTurn {
    pub role: Role,
    pub content: String,
}

impl ChatTurn {
    pub fn user(content: String) -> Self {
        Self {
            role: Role::User,
            content,
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: Role::Assistant,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub language: Language,
    pub character: Character,
    pub provider: ProviderKind,
    pub history: Vec<ChatTurn>,
}

impl Session {
    pub fn new(
        id: String,
        language: Language,
        character: Character,
        provider: ProviderKind,
    ) -> Self {
        Self {
            id,
            language,
            character,
            provider,
            history: Vec::new(),
        }
    }
}

pub type Sessions = Mutex<HashMap<String, Session>>;
