use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Attributive form (Rentaikei).
///
/// In modern Japanese, this is the same as the dictionary form.
/// Returns the attributive form (noun-modifying form).
/// e.g.
/// Godan: "書く" -> "書く"
/// KamiIchidan: "見る" -> "見る"
/// ShimoIchidan: "食べる" -> "食べる"
/// Sahen: "する" -> "する"
/// Kahen: "くる" | "来る" -> "くる" | "来る"
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::attributive::attributive;
///
/// assert_eq!(attributive("書く", ConjugationType::Godan), Ok("書く".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::attributive; // Macro export at crate root
///
/// assert_eq!(attributive!("書く"), Ok("書く".to_string()));
/// assert_eq!(attributive!("食べる"), Ok("食べる".to_string()));
/// ```
pub fn attributive(verb: &str, _conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    Ok(verb.to_string())
}

/// Macro to get attributive form, optionally inferring conjugation type.
#[macro_export]
macro_rules! attributive {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::attributive::attributive($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::attributive::attributive($verb, $conj)
    };
}
