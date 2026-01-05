use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Imperative form (Meireikei), variant "seyo" for Sahen.
///
/// Returns the imperative form.
/// e.g.
/// Godan: "書く" -> "書け" (kake)
/// KamiIchidan: "見る" -> "見ろ" (miro)
/// ShimoIchidan: "食べる" -> "食べろ" (tabero)
/// Sahen: "する" -> "せよ" (seyo)
/// Kahen: "くる" | "来る" -> "こい" (koi)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::imperative2::imperative2;
///
/// assert_eq!(imperative2("する", ConjugationType::Sahen), Ok("せよ".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::imperative2; // Macro export at crate root
///
/// assert_eq!(imperative2!("する"), Ok("せよ".to_string()));
/// ```
pub fn imperative2(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    if let ConjugationType::Sahen = conjugation {
        if verb == "する" {
            return Ok("せよ".to_string());
        } else if verb.ends_with("する") {
            let base = &verb[..verb.len() - "する".len()];
            return Ok(format!("{}せよ", base));
        } else {
            return Err(VerbError::UnknownConjugation);
        }
    }

    super::imperative::imperative(verb, conjugation)
}

/// Macro to get imperative2 form, optionally inferring conjugation type.
#[macro_export]
macro_rules! imperative2 {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::imperative2::imperative2($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::imperative2::imperative2($verb, $conj)
    };
}
