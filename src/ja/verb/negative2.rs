use super::infer_conjugation_type::{ConjugationType, VerbError};
use super::negative;
use alloc::format;
use alloc::string::{String, ToString};

/// Conjugate a Japanese verb to its Negative form (Uchikeshikei).
///
/// Returns the stem for the negative form (zu).
/// e.g.
/// Godan: "書く" -> "書か" (kaka-zu)
/// KamiIchidan: "見る" -> "見" (mi-zu)
/// ShimoIchidan: "食べる" -> "食べ" (tabe-zu)
/// Sahen: "する" -> "せ" (se-zu)
/// Kahen: "くる" | "来る" -> "こ" (ko-zu)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::negative2::negative2;
///
/// assert_eq!(negative2("書く", ConjugationType::Godan), Ok("書か".to_string()));
/// assert_eq!(negative2("する", ConjugationType::Sahen), Ok("せ".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::negative2; // Macro export at crate root
///
/// assert_eq!(negative2!("書く"), Ok("書か".to_string()));
/// assert_eq!(negative2!("食べる"), Ok("食べ".to_string()));
/// assert_eq!(negative2!("する"), Ok("せ".to_string()));
/// ```
pub fn negative2(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if conjugation != ConjugationType::Sahen {
        return negative(verb, conjugation);
    }

    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    let stem = if verb == "する" {
        "せ".to_string()
    } else if verb.ends_with("する") {
        let base = &verb[..verb.len() - "する".len()];
        format!("{}せ", base)
    } else {
        return Err(VerbError::UnknownConjugation);
    };

    Ok(stem)
}

#[macro_export]
macro_rules! negative2 {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::negative2::negative2($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::negative2::negative2($verb, $conj)
    };
}
