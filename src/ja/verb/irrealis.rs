use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Irrealis form (Mizenkei) for "u/you" connection.
///
/// Returns the stem for the volitional form (u/you-form).
/// e.g.
/// Godan: "書く" -> "書こ" (kako-u)
/// KamiIchidan: "見る" -> "見" (mi-you)
/// ShimoIchidan: "食べる" -> "食べ" (tabe-you)
/// Sahen: "する" -> "し" (shi-you)
/// Kahen: "くる" | "来る" -> "こ" (ko-you)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::irrealis::irrealis;
///
/// assert_eq!(irrealis("書く", ConjugationType::Godan), Ok("書こ".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::irrealis; // Macro export at crate root
///
/// assert_eq!(irrealis!("書く"), Ok("書こ".to_string()));
/// assert_eq!(irrealis!("食べる"), Ok("食べ".to_string()));
/// ```
pub fn irrealis(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
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
            // Change final u-sound to o-sound
            let last_char = chars[len - 1];
            let stem = &verb[..verb.len() - last_char.len_utf8()];

            let new_ending = match last_char {
                'う' => "お",
                'く' => "こ",
                'ぐ' => "ご",
                'す' => "そ",
                'つ' => "と",
                'ぬ' => "の",
                'ふ' => "ほ",
                'ぶ' => "ぼ",
                'む' => "も",
                'る' => "ろ",
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
macro_rules! irrealis {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::irrealis::irrealis($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::irrealis::irrealis($verb, $conj)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irrealis() {
        assert_eq!(
            irrealis("書く", ConjugationType::Godan),
            Ok("書こ".to_string())
        );
        assert_eq!(
            irrealis("泳ぐ", ConjugationType::Godan),
            Ok("泳ご".to_string())
        );
        assert_eq!(
            irrealis("買う", ConjugationType::Godan),
            Ok("買お".to_string())
        );
        assert_eq!(
            irrealis("見る", ConjugationType::KamiIchidan),
            Ok("見".to_string())
        );
        assert_eq!(
            irrealis("食べる", ConjugationType::ShimoIchidan),
            Ok("食べ".to_string())
        );
        assert_eq!(
            irrealis("する", ConjugationType::Sahen),
            Ok("し".to_string())
        );
        assert_eq!(
            irrealis("来る", ConjugationType::Kahen),
            Ok("こ".to_string())
        );
    }

    #[test]
    fn test_irrealis_macro() {
        assert_eq!(irrealis!("書く"), Ok("書こ".to_string()));
        assert_eq!(irrealis!("食べる"), Ok("食べ".to_string()));
        assert_eq!(irrealis!("する"), Ok("し".to_string()));
    }
}
