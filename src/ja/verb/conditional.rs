use super::infer_conjugation_type::{ConjugationType, VerbError};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Conjugate a Japanese verb to its Conditional form (Kateikei).
///
/// Returns the stem for the conditional form (ba).
/// e.g.
/// Godan: "書く" -> "書け" (kake-ba)
/// KamiIchidan: "見る" -> "見れ" (mire-ba)
/// ShimoIchidan: "食べる" -> "食べれ" (tabere-ba)
/// Sahen: "する" -> "すれ" (sure-ba)
/// Kahen: "くる" | "来る" -> "くれ" (kure-ba)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::conditional::conditional;
///
/// assert_eq!(conditional("書く", ConjugationType::Godan), Ok("書け".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::conditional; // Macro export at crate root
///
/// assert_eq!(conditional!("書く"), Ok("書け".to_string()));
/// assert_eq!(conditional!("食べる"), Ok("食べれ".to_string()));
/// ```
pub fn conditional(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
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
            format!("{}れ", base)
        }
        ConjugationType::Sahen => {
            if verb == "する" {
                "すれ".to_string()
            } else if verb.ends_with("する") {
                let base = &verb[..verb.len() - "する".len()];
                format!("{}すれ", base)
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
        ConjugationType::Kahen => {
            if verb == "くる" || verb == "来る" {
                "くれ".to_string()
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
    };

    Ok(stem)
}

/// Macro to get conditional form, optionally inferring conjugation type.
#[macro_export]
macro_rules! conditional {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::conditional::conditional($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::conditional::conditional($verb, $conj)
    };
}
