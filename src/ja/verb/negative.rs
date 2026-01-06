use super::infer_conjugation_type::{ConjugationType, VerbError};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Conjugate a Japanese verb to its Negative form (Uchikeshikei).
///
/// Returns the stem for the negative form (nai).
/// e.g.
/// Godan: "書く" -> "書か" (kaka)
/// KamiIchidan: "見る" -> "見" (mi)
/// ShimoIchidan: "食べる" -> "食べ" (tabe)
/// Sahen: "する" -> "し" (shi)
/// Kahen: "くる" | "来る" -> "こ" (ko)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::negative::negative;
///
/// assert_eq!(negative("書く", ConjugationType::Godan), Ok("書か".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::negative; // Macro export at crate root
///
/// assert_eq!(negative!("書く"), Ok("書か".to_string()));
/// assert_eq!(negative!("食べる"), Ok("食べ".to_string()));
/// ```
pub fn negative(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    let chars: Vec<char> = verb.chars().collect();
    let len = chars.len();

    if len < 1 {
        return Err(VerbError::NotAVerb);
    }

    let stem = match conjugation {
        ConjugationType::Godan => {
            let last_char = chars[len - 1];
            let base = &verb[..verb.len() - last_char.len_utf8()];

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
            format!("{}{}", base, new_ending)
        }
        ConjugationType::KamiIchidan | ConjugationType::ShimoIchidan => {
            if !verb.ends_with('る') {
                return Err(VerbError::UnknownConjugation);
            }
            verb[..verb.len() - 'る'.len_utf8()].to_string()
        }
        ConjugationType::Sahen => {
            if verb == "する" {
                "し".to_string()
            } else if verb.ends_with("する") {
                let base = &verb[..verb.len() - "する".len()];
                format!("{}し", base)
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
        ConjugationType::Kahen => {
            if verb == "くる" || verb == "来る" {
                "こ".to_string()
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
    };

    Ok(stem)
}

/// Macro to get negative form, optionally inferring conjugation type.
#[macro_export]
macro_rules! negative {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::negative::negative($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::negative::negative($verb, $conj)
    };
}
