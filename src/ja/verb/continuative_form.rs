use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Continuative form (Ren'youkei).
///
/// Returns the stem for the polite form (masu-form).
/// e.g.
/// Godan: "書く" -> "書き" (kaki-masu)
/// KamiIchidan: "見る" -> "見" (mi-masu)
/// ShimoIchidan: "食べる" -> "食べ" (tabe-masu)
/// Sahen: "する" -> "し" (shi-masu)
/// Kahen: "くる" | "来る" -> "き" (ki-masu)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::continuative_form::continuative_form;
///
/// assert_eq!(continuative_form("書く", ConjugationType::Godan), Ok("書き".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::continuative_form; // Macro export at crate root
///
/// assert_eq!(continuative_form!("書く"), Ok("書き".to_string()));
/// assert_eq!(continuative_form!("食べる"), Ok("食べ".to_string()));
/// ```
pub fn continuative_form(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    let chars: Vec<char> = verb.chars().collect();
    let len = chars.len();

    if len < 1 {
        return Err(VerbError::NotAVerb);
    }

    match conjugation {
        ConjugationType::Godan => {
            // Change final u-sound to i-sound
            let last_char = chars[len - 1];
            let stem = &verb[..verb.len() - last_char.len_utf8()];

            let new_ending = match last_char {
                'う' => "い",
                'く' => "き",
                'ぐ' => "ぎ",
                'す' => "し",
                'つ' => "ち",
                'ぬ' => "に",
                'ふ' => "ひ",
                'ぶ' => "び",
                'む' => "み",
                'る' => "り",
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
                Ok("き".to_string())
            } else {
                Err(VerbError::UnknownConjugation)
            }
        }
    }
}

/// Macro to get continuative form, optionally inferring conjugation type.
#[macro_export]
macro_rules! continuative_form {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::continuative_form::continuative_form($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::continuative_form::continuative_form($verb, $conj)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuative() {
        assert_eq!(
            continuative_form("書く", ConjugationType::Godan),
            Ok("書き".to_string())
        );
        assert_eq!(
            continuative_form("泳ぐ", ConjugationType::Godan),
            Ok("泳ぎ".to_string())
        );
        assert_eq!(
            continuative_form("死ぬ", ConjugationType::Godan),
            Ok("死に".to_string())
        );
        assert_eq!(
            continuative_form("遊ぶ", ConjugationType::Godan),
            Ok("遊び".to_string())
        );
        assert_eq!(
            continuative_form("買う", ConjugationType::Godan),
            Ok("買い".to_string())
        );
        assert_eq!(
            continuative_form("見る", ConjugationType::KamiIchidan),
            Ok("見".to_string())
        );
        assert_eq!(
            continuative_form("起きる", ConjugationType::KamiIchidan),
            Ok("起き".to_string())
        );
        assert_eq!(
            continuative_form("食べる", ConjugationType::ShimoIchidan),
            Ok("食べ".to_string())
        );
        assert_eq!(
            continuative_form("する", ConjugationType::Sahen),
            Ok("し".to_string())
        );
        assert_eq!(
            continuative_form("勉強する", ConjugationType::Sahen),
            Ok("勉強し".to_string())
        );
        assert_eq!(
            continuative_form("くる", ConjugationType::Kahen),
            Ok("き".to_string())
        );
        assert_eq!(
            continuative_form("来る", ConjugationType::Kahen),
            Ok("き".to_string())
        );
    }

    #[test]
    fn test_continuative_macro() {
        assert_eq!(continuative_form!("書く"), Ok("書き".to_string()));
        assert_eq!(continuative_form!("食べる"), Ok("食べ".to_string()));
        assert_eq!(continuative_form!("する"), Ok("し".to_string()));
    }
}
