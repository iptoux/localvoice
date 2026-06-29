/// Maps an app language code to the value passed to `whisper-cli --language`.
///
/// whisper.cpp accepts ISO 639-1 codes ("de", "en", …) directly and also "auto".
/// This function normalises caller-provided values and returns the correct CLI arg.
pub fn to_whisper_lang(code: &str) -> String {
    match code.trim().to_lowercase().as_str() {
        "de" | "german" => "de",
        "en" | "english" => "en",
        "fr" | "french" => "fr",
        "es" | "spanish" => "es",
        "it" | "italian" => "it",
        "pt" | "portuguese" => "pt",
        "nl" | "dutch" => "nl",
        "pl" | "polish" => "pl",
        "ru" | "russian" => "ru",
        "ja" | "japanese" => "ja",
        "zh" | "chinese" => "zh",
        "auto" | "" => "auto",
        other => return other.to_string(), // pass through unknown codes
    }
    .to_string()
}

/// Maps an app language code to the locale format used by Parakeet/Nemotron.
///
/// Nemotron accepts language prompts such as "de-DE" and also "auto". The
/// parakeet.cpp CLI also accepts "auto"; unsupported custom values are passed
/// through so advanced users can try newly supported locales without an app
/// update.
pub fn to_parakeet_locale(code: &str) -> String {
    match code.trim().to_lowercase().as_str() {
        "de" | "de-de" | "german" => "de-DE",
        "en" | "en-us" | "english" => "en-US",
        "en-gb" => "en-GB",
        "fr" | "fr-fr" | "french" => "fr-FR",
        "fr-ca" => "fr-CA",
        "es" | "es-es" | "spanish" => "es-ES",
        "es-us" => "es-US",
        "it" | "it-it" | "italian" => "it-IT",
        "pt" | "pt-pt" | "portuguese" => "pt-PT",
        "pt-br" => "pt-BR",
        "nl" | "nl-nl" | "dutch" => "nl-NL",
        "pl" | "pl-pl" | "polish" => "pl-PL",
        "ru" | "ru-ru" | "russian" => "ru-RU",
        "ja" | "ja-jp" | "japanese" => "ja-JP",
        "zh" | "zh-cn" | "chinese" => "zh-CN",
        "ko" | "ko-kr" | "korean" => "ko-KR",
        "ar" | "ar-ar" | "arabic" => "ar-AR",
        "hi" | "hi-in" | "hindi" => "hi-IN",
        "tr" | "tr-tr" | "turkish" => "tr-TR",
        "uk" | "uk-ua" | "ukrainian" => "uk-UA",
        "auto" | "" => "auto",
        other => return other.to_string(),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parakeet_locale_maps_core_languages() {
        assert_eq!(to_parakeet_locale("de"), "de-DE");
        assert_eq!(to_parakeet_locale("en"), "en-US");
        assert_eq!(to_parakeet_locale("auto"), "auto");
    }
}
