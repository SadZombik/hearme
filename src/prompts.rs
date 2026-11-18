use crate::language::Language;
use crate::session::Character;

const SAFETY_RU: &str = include_str!("../prompts/safety.ru.txt");
const SAFETY_EN: &str = include_str!("../prompts/safety.en.txt");
const GIRL_RU: &str = include_str!("../prompts/girl.ru.txt");
const GIRL_EN: &str = include_str!("../prompts/girl.en.txt");
const BOY_RU: &str = include_str!("../prompts/boy.ru.txt");
const BOY_EN: &str = include_str!("../prompts/boy.en.txt");

pub fn system_prompt(language: Language, character: Character) -> String {
    let safety = match language {
        Language::Ru => SAFETY_RU,
        Language::En => SAFETY_EN,
    };
    let persona = match (language, character) {
        (Language::Ru, Character::Girl) => GIRL_RU,
        (Language::Ru, Character::Boy) => BOY_RU,
        (Language::En, Character::Girl) => GIRL_EN,
        (Language::En, Character::Boy) => BOY_EN,
    };
    format!("{safety}\n\n{persona}")
}
