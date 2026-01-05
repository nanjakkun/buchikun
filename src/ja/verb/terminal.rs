use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Terminal form (Shushikei).
///
/// In modern Japanese, this is the same as the dictionary form.
/// Returns the terminal form (sentence-ending form).
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
/// use buchikun::ja::verb::terminal::terminal;
///
/// assert_eq!(terminal("書く", ConjugationType::Godan), Ok("書く".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::terminal; // Macro export at crate root
///
/// assert_eq!(terminal!("書く"), Ok("書く".to_string()));
/// assert_eq!(terminal!("食べる"), Ok("食べる".to_string()));
/// ```
pub fn terminal(verb: &str, _conjugation: ConjugationType) -> Result<String, VerbError> {
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    return Ok(verb.to_string());
}

/// Macro to get terminal form, optionally inferring conjugation type.
#[macro_export]
macro_rules! terminal {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::terminal::terminal($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::terminal::terminal($verb, $conj)
    };
}
