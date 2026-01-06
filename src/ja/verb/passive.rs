use super::infer_conjugation_type::{ConjugationType, VerbError};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Conjugate a Japanese verb to its Passive form (Ukemikei).
///
/// Returns the passive verb (reru/rareru form).
/// e.g.
/// Godan: "書く" -> "書かれる" (kakareru)
/// KamiIchidan: "見る" -> "見られる" (mirareru)
/// ShimoIchidan: "食べる" -> "食べられる" (taberareru)
/// Sahen: "する" -> "される" (sareru)
/// Kahen: "くる" | "来る" -> "こられる" (korareru)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::ConjugationType;
/// use buchikun::ja::verb::passive::passive;
///
/// assert_eq!(passive("書く", ConjugationType::Godan), Ok("書かれる".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::passive; // Macro export at crate root
///
/// assert_eq!(passive!("書く"), Ok("書かれる".to_string()));
/// ```
pub fn passive(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    let chars: Vec<char> = verb.chars().collect();
    let len = chars.len();

    if len < 1 {
        return Err(VerbError::NotAVerb);
    }

    let passive_verb = match conjugation {
        ConjugationType::Godan => {
            let last_char = chars[len - 1];
            let base = &verb[..verb.len() - last_char.len_utf8()];

            let a_stem = match last_char {
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
            format!("{}{}れる", base, a_stem)
        }
        ConjugationType::KamiIchidan | ConjugationType::ShimoIchidan => {
            if !verb.ends_with('る') {
                return Err(VerbError::UnknownConjugation);
            }
            let stem = &verb[..verb.len() - 'る'.len_utf8()];
            format!("{}られる", stem)
        }
        ConjugationType::Sahen => {
            if verb == "する" {
                "される".to_string()
            } else if verb.ends_with("する") {
                let base = &verb[..verb.len() - "する".len()];
                format!("{}される", base)
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
        ConjugationType::Kahen => {
            if verb == "くる" || verb == "来る" {
                "こられる".to_string()
            } else {
                return Err(VerbError::UnknownConjugation);
            }
        }
    };

    Ok(passive_verb)
}

/// Macro to get passive form, optionally inferring conjugation type.
#[macro_export]
macro_rules! passive {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::passive::passive($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::passive::passive($verb, $conj)
    };
}
