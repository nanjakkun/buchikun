use super::guess_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Irrealis form (Mizenkei).
///
/// Returns the stem for the negative form (nai-form).
/// e.g.
/// Godan: "書く" -> "書か" (kaka-nai)
/// KamiIchidan: "見る" -> "見" (mi-nai)
/// ShimoIchidan: "食べる" -> "食べ" (tabe-nai)
/// Sahen: "する" -> "し" (shi-nai)
/// Kahen: "くる" | "来る" -> "こ" (ko-nai)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::guess_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::conjugate::get_irrealis_form;
///
/// assert_eq!(get_irrealis_form("書く", ConjugationType::Godan), Ok("書か".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::get_irrealis_form; // Macro export at crate root
///
/// assert_eq!(get_irrealis_form!("書く"), Ok("書か".to_string()));
/// assert_eq!(get_irrealis_form!("食べる"), Ok("食べ".to_string()));
/// ```
pub fn get_irrealis_form(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    let chars: Vec<char> = verb.chars().collect();
    let len = chars.len();

    // Basic validity check
    if len < 1 {
        // Should be caught by is_empty, but just in case
        return Err(VerbError::NotAVerb);
    }

    match conjugation {
        ConjugationType::Godan => {
            // Change final u-sound to a-sound
            // Exception: 'u' (う) becomes 'wa' (わ), not 'a' (あ)
            let last_char = chars[len - 1];
            let stem = &verb[..verb.len() - last_char.len_utf8()];

            let new_ending = match last_char {
                'う' => "わ",
                'く' => "か",
                'ぐ' => "が",
                'す' => "さ",
                'つ' => "た",
                'ぬ' => "な",
                'ふ' => "は",
                'ぶ' => "ば",
                'む' => "ま",
                'る' => "ら",
                _ => return Err(VerbError::UnknownConjugation),
            };
            Ok(format!("{}{}", stem, new_ending))
        }
        ConjugationType::KamiIchidan | ConjugationType::ShimoIchidan => {
            if !verb.ends_with('る') {
                return Err(VerbError::UnknownConjugation);
            }
            Ok(verb[..verb.len() - 'る'.len_utf8()].to_string())
        }
        ConjugationType::Sahen => {
            if verb == "する" {
                Ok("し".to_string())
            } else if verb.ends_with("する") {
                let stem = &verb[..verb.len() - "する".len()];
                Ok(format!("{}し", stem))
            } else {
                Err(VerbError::UnknownConjugation)
            }
        }
        ConjugationType::Kahen => {
            if verb == "くる" || verb == "来る" {
                Ok("こ".to_string())
            } else {
                Err(VerbError::UnknownConjugation)
            }
        }
    }
}

/// Macro to get irrealis form, optionally inferring conjugation type.
#[macro_export]
macro_rules! get_irrealis_form {
    ($verb:expr) => {
        $crate::ja::verb::guess_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::get_irrealis_form($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::get_irrealis_form($verb, $conj)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irrealis() {
        assert_eq!(
            get_irrealis_form("書く", ConjugationType::Godan),
            Ok("書か".to_string())
        );
        assert_eq!(
            get_irrealis_form("泳ぐ", ConjugationType::Godan),
            Ok("泳が".to_string())
        );
        assert_eq!(
            get_irrealis_form("死ぬ", ConjugationType::Godan),
            Ok("死な".to_string())
        );
        assert_eq!(
            get_irrealis_form("遊ぶ", ConjugationType::Godan),
            Ok("遊ば".to_string())
        );
        assert_eq!(
            get_irrealis_form("買う", ConjugationType::Godan),
            Ok("買わ".to_string())
        );
        assert_eq!(
            get_irrealis_form("見る", ConjugationType::KamiIchidan),
            Ok("見".to_string())
        );
        assert_eq!(
            get_irrealis_form("起きる", ConjugationType::KamiIchidan),
            Ok("起き".to_string())
        );
        assert_eq!(
            get_irrealis_form("食べる", ConjugationType::ShimoIchidan),
            Ok("食べ".to_string())
        );
        assert_eq!(
            get_irrealis_form("する", ConjugationType::Sahen),
            Ok("し".to_string())
        );
        assert_eq!(
            get_irrealis_form("勉強する", ConjugationType::Sahen),
            Ok("勉強し".to_string())
        );
        assert_eq!(
            get_irrealis_form("くる", ConjugationType::Kahen),
            Ok("こ".to_string())
        );
        assert_eq!(
            get_irrealis_form("来る", ConjugationType::Kahen),
            Ok("こ".to_string())
        );
    }

    #[test]
    fn test_irrealis_macro() {
        // Test macro usage with inferred type
        assert_eq!(get_irrealis_form!("書く"), Ok("書か".to_string()));
        assert_eq!(get_irrealis_form!("食べる"), Ok("食べ".to_string()));
        assert_eq!(get_irrealis_form!("する"), Ok("し".to_string()));

        // Explicit type
        assert_eq!(
            get_irrealis_form!("書く", ConjugationType::Godan),
            Ok("書か".to_string())
        );
    }

    #[test]
    fn test_irrealis_errors() {
        assert_eq!(
            get_irrealis_form("", ConjugationType::Godan),
            Err(VerbError::NotAVerb)
        );
        // Mismatch ending
        assert_eq!(
            get_irrealis_form("書く", ConjugationType::KamiIchidan),
            Err(VerbError::UnknownConjugation)
        );
    }
}
