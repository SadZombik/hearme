use crate::language::Language;

const CRISIS_RU: &str = "Мне очень жаль, что тебе сейчас так тяжело. Я ИИ и не могу заменить экстренную помощь. Если есть угроза жизни или безопасности — немедленно звони 112 или обратись к доверенному человеку рядом. Если можешь, напиши, что происходит прямо сейчас.";
const CRISIS_EN: &str = "I'm sorry things are so painful right now. I'm an AI and cannot replace emergency help. If there is any immediate danger, call your local emergency number (112) or reach out to someone you trust right now. If you can, tell me what is happening.";

const KEYWORDS_RU: [&str; 16] = [
    "суицид",
    "самоубий",
    "покончить с собой",
    "покончить жизнь",
    "убить себя",
    "причинить себе вред",
    "нанести себе вред",
    "не хочу жить",
    "не хочется жить",
    "свести счеты",
    "перерезать вены",
    "вскрыть вены",
    "повеситься",
    "передоз",
    "умереть",
    "расправиться с собой",
];

const KEYWORDS_EN: [&str; 15] = [
    "suicide",
    "suicidal",
    "kill myself",
    "end my life",
    "take my life",
    "self harm",
    "self-harm",
    "cut myself",
    "hang myself",
    "overdose",
    "don't want to live",
    "do not want to live",
    "hurt myself",
    "no reason to live",
    "better off dead",
];

pub fn crisis_reply(language: Language, text: &str) -> Option<&'static str> {
    let lowered = text.to_lowercase();
    let compact: String = lowered.chars().filter(|c| !c.is_whitespace()).collect();

    let russian_hit = KEYWORDS_RU.iter().any(|word| {
        let word: String = word.chars().filter(|c| !c.is_whitespace()).collect();
        compact.contains(&word)
    });
    let english_hit = KEYWORDS_EN.iter().any(|word| lowered.contains(word));

    if russian_hit || english_hit {
        Some(match language {
            Language::Ru => CRISIS_RU,
            Language::En => CRISIS_EN,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::crisis_reply;
    use crate::language::Language;

    #[test]
    fn catches_russian_crisis_phrase() {
        assert!(crisis_reply(Language::Ru, "Я хочу покончить с собой").is_some());
    }

    #[test]
    fn catches_english_crisis_phrase() {
        assert!(crisis_reply(Language::En, "I don't want to live anymore").is_some());
    }

    #[test]
    fn ignores_ordinary_sadness() {
        assert!(crisis_reply(Language::Ru, "Сегодня был сложный день на учёбе").is_none());
        assert!(crisis_reply(Language::En, "I feel lonely today").is_none());
    }

    #[test]
    fn handles_extra_spaces_in_keywords() {
        assert!(crisis_reply(Language::Ru, "я хочу по кончить с со бой").is_some());
    }
}
