use super::infer_conjugation_type::{ConjugationType, VerbError};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Conjugate a Japanese verb to its Imperative form (Meireikei).
///
/// Returns the imperative form.
/// e.g.
/// Godan: "書く" -> "書け" (kake)
/// KamiIchidan: "見る" -> "見ろ" (miro)
/// ShimoIchidan: "食べる" -> "食べろ" (tabero)
/// Sahen: "する" -> "しろ" (shiro)
/// Kahen: "くる" | "来る" -> "こい" (koi)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::imperative::imperative;
///
/// assert_eq!(imperative("書く", ConjugationType::Godan), Ok("書け".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::imperative; // Macro export at crate root
///
/// assert_eq!(imperative!("書く"), Ok("書け".to_string()));
/// assert_eq!(imperative!("食べる"), Ok("食べろ".to_string()));
/// ```
pub fn imperative(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
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
                'う' => "え",
                'く' => "け",
                'ぐ' => "げ",
                'す' => "せ",
                'つ' => "て",
                'ぬ' => "ね",
                'ふ' => "へ",
                'ぶ' => "べ",
                'む' => "め",
                'る' => "れ",
                _ => return Err(VerbError::UnknownConjugation),
            };
            format!("{}{}", base, new_ending)
        }
        ConjugationType::KamiIchidan | ConjugationType::ShimoIchidan => {
            if !verb.ends_with('る') {
                return Err(VerbError::UnknownConjugation);
            }
            let base = &verb[..verb.len() - 'る'.len_utf8()];
            format!("{}ろ", base)
        }
        ConjugationType::Sahen => {
            if verb == "する" {
                "しろ".to_string()
            } else if verb.ends_with("する") {
                let base = &verb[..verb.len() - "する".len()];
                format!("{}しろ", base)
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
        ConjugationType::Kahen => {
            if verb == "くる" || verb == "来る" {
                "こい".to_string()
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
    };

    Ok(stem)
}

/// Macro to get imperative form, optionally inferring conjugation type.
#[macro_export]
macro_rules! imperative {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::imperative::imperative($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::imperative::imperative($verb, $conj)
    };
}
