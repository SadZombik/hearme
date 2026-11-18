use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ru,
    En,
}

impl Language {
    pub fn detect(text: &str) -> Self {
        let cyrillic = text
            .chars()
            .filter(|c| matches!(c, '\u{0400}'..='\u{04FF}'))
            .count();
        let latin = text.chars().filter(char::is_ascii_alphabetic).count();
        match cyrillic.cmp(&latin) {
            std::cmp::Ordering::Less => Self::En,
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => Self::Ru,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ru => "ru",
            Self::En => "en",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn detects_cyrillic_as_russian() {
        assert_eq!(Language::detect("Мне очень тяжело сегодня"), Language::Ru);
    }

    #[test]
    fn detects_latin_as_english() {
        assert_eq!(Language::detect("I feel anxious today"), Language::En);
    }
}
