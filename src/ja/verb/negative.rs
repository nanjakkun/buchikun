use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Negative form (Uchikeshikei
///
/// Returns the full negative form by appending "ない" (nai) to the irrealis stem.
/// e.g.
/// Godan: "書く" -> "書かない" (kaka-nai)
/// KamiIchidan: "見る" -> "見ない" (mi-nai)
/// ShimoIchidan: "食べる" -> "食べない" (tabe-nai)
/// Sahen: "する" -> "しない" (shi-nai)
/// Kahen: "くる" | "来る" -> "こない" (ko-nai)
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::negative::negative;
///
/// assert_eq!(negative("書く", ConjugationType::Godan), Ok("書かない".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::negative; // Macro export at crate root
///
/// assert_eq!(negative!("書く"), Ok("書かない".to_string()));
/// assert_eq!(negative!("食べる"), Ok("食べない".to_string()));
/// ```
pub fn negative(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    // Reuse the irrealis_form logic to get the stem
    let stem = super::irrealis_form::irrealis_form(verb, conjugation)?;
    Ok(format!("{}ない", stem))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative() {
        assert_eq!(
            negative("書く", ConjugationType::Godan),
            Ok("書かない".to_string())
        );
        assert_eq!(
            negative("見る", ConjugationType::KamiIchidan),
            Ok("見ない".to_string())
        );
        assert_eq!(
            negative("食べる", ConjugationType::ShimoIchidan),
            Ok("食べない".to_string())
        );
        assert_eq!(
            negative("する", ConjugationType::Sahen),
            Ok("しない".to_string())
        );
        assert_eq!(
            negative("来る", ConjugationType::Kahen),
            Ok("こない".to_string())
        );
    }

    #[test]
    fn test_negative_macro() {
        assert_eq!(negative!("書く"), Ok("書かない".to_string()));
        assert_eq!(negative!("食べる"), Ok("食べない".to_string()));
        assert_eq!(negative!("する"), Ok("しない".to_string()));
    }
}
