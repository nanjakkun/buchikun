use super::infer_conjugation_type::{ConjugationType, VerbError};

/// Conjugate a Japanese verb to its Continuative form before past tense (Onbin Ren'youkei / Ren'youkei Kako).
///
/// Returns the stem used before "た" (ta) or "て" (te).
/// e.g.
/// Godan: "書く" -> "書い" (kaita)
/// Godan: "行く" -> "行っ" (itta) - Exception
/// Godan: "泳ぐ" -> "泳い" (oyoida)
/// Godan: "洗う" -> "洗っ" (aratta)
/// Godan: "死ぬ" -> "死ん" (shinda)
/// Godan: "話す" -> "話し" (hanashita)
/// KamiIchidan: "見る" -> "見" (mita)
/// ShimoIchidan: "食べる" -> "食べ" (tabeta)
/// Sahen: "する" -> "し" (shita)
/// Kahen: "くる" | "来る" -> "き" (kita)
///
/// Note: For "ぐ", "ぬ", "ぶ", "む", the following "た/て" must be voiced ("だ/で").
///
/// # Examples
///
/// Use as a function:
/// ```
/// use buchikun::ja::verb::infer_conjugation_type::ConjugationType;
/// use buchikun::ja::verb::continuative_past::continuative_past;
///
/// assert_eq!(continuative_past("書く", ConjugationType::Godan), Ok("書い".to_string()));
/// ```
///
/// Use as a macro (supports omitting conjugation type):
/// ```
/// use buchikun::continuative_past; // Macro export at crate root
///
/// assert_eq!(continuative_past!("書く"), Ok("書い".to_string()));
/// assert_eq!(continuative_past!("食べる"), Ok("食べ".to_string()));
/// ```
pub fn continuative_past(verb: &str, conjugation: ConjugationType) -> Result<String, VerbError> {
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
            // Handle Exception: 行く (iku) -> 行っ (itta)
            if verb == "いく" {
                return Ok("いっ".to_string());
            }
            if verb == "行く" {
                return Ok("行っ".to_string());
            }

            let last_char = chars[len - 1];
            let stem = &verb[..verb.len() - last_char.len_utf8()];

            let new_ending = match last_char {
                'う' | 'つ' | 'る' => "っ",
                'ぬ' | 'ぶ' | 'む' => "ん",
                'く' | 'ぐ' => "い",
                'す' => "し",
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
                Ok("き".to_string())
            } else {
                Err(VerbError::UnknownConjugation)
            }
        }
    }
}

/// Macro to get continuative past form, optionally inferring conjugation type.
#[macro_export]
macro_rules! continuative_past {
    ($verb:expr) => {
        $crate::ja::verb::infer_conjugation_type($verb)
            .and_then(|c| $crate::ja::verb::continuative_past::continuative_past($verb, c))
    };
    ($verb:expr, $conj:expr) => {
        $crate::ja::verb::continuative_past::continuative_past($verb, $conj)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuative_past() {
        assert_eq!(
            continuative_past("書く", ConjugationType::Godan),
            Ok("書い".to_string())
        );
        assert_eq!(
            continuative_past("いく", ConjugationType::Godan),
            Ok("いっ".to_string())
        );
        assert_eq!(
            continuative_past("行く", ConjugationType::Godan),
            Ok("行っ".to_string())
        );
        assert_eq!(
            continuative_past("泳ぐ", ConjugationType::Godan),
            Ok("泳い".to_string())
        );
        assert_eq!(
            continuative_past("死ぬ", ConjugationType::Godan),
            Ok("死ん".to_string())
        );
        assert_eq!(
            continuative_past("遊ぶ", ConjugationType::Godan),
            Ok("遊ん".to_string())
        );
        assert_eq!(
            continuative_past("読む", ConjugationType::Godan),
            Ok("読ん".to_string())
        );
        assert_eq!(
            continuative_past("買う", ConjugationType::Godan),
            Ok("買っ".to_string())
        );
        assert_eq!(
            continuative_past("待つ", ConjugationType::Godan),
            Ok("待っ".to_string())
        );
        assert_eq!(
            continuative_past("取る", ConjugationType::Godan),
            Ok("取っ".to_string())
        );
        assert_eq!(
            continuative_past("話す", ConjugationType::Godan),
            Ok("話し".to_string())
        );
        assert_eq!(
            continuative_past("見る", ConjugationType::KamiIchidan),
            Ok("見".to_string())
        );
        assert_eq!(
            continuative_past("食べる", ConjugationType::ShimoIchidan),
            Ok("食べ".to_string())
        );
        assert_eq!(
            continuative_past("する", ConjugationType::Sahen),
            Ok("し".to_string())
        );
        assert_eq!(
            continuative_past("来る", ConjugationType::Kahen),
            Ok("き".to_string())
        );
    }

    #[test]
    fn test_continuative_past_macro() {
        assert_eq!(continuative_past!("書く"), Ok("書い".to_string()));
        assert_eq!(continuative_past!("行く"), Ok("行っ".to_string()));
        assert_eq!(continuative_past!("食べる"), Ok("食べ".to_string()));
        assert_eq!(continuative_past!("する"), Ok("し".to_string()));
    }
}
