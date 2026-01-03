#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConjugationType {
    Godan,        // 五段
    KamiIchidan,  // 上一段
    ShimoIchidan, // 下一段
    Sahen,        // サ変(する)
    Kahen,        // カ変(来る)
}

#[derive(Debug, PartialEq, Eq)]
pub enum VerbError {
    NotAVerb,
    UnknownConjugation,
}

/// Suggest the conjugation type of a Japanese verb (heuristically).
///
/// Note: This function relies on surface form heuristics and cannot perfectly distinguish
/// between Ichidan and Godan verbs ending in 'iru'/'eru'
///
/// (e.g., 'kaeru' can be Godan (return) or Ichidan (change)).
/// 帰る and 変える have the same pronunciation, but different conjugation types.
/// 帰る conjugates as Godan, but 変える conjugates as Ichidan.
///
/// # Examples
///
/// ```
/// use buchikun::ja::verb::{infer_conjugation_type, ConjugationType};
///
/// assert_eq!(infer_conjugation_type("書く"), Ok(ConjugationType::Godan));
/// assert_eq!(infer_conjugation_type("食べる"), Ok(ConjugationType::ShimoIchidan));
/// assert_eq!(infer_conjugation_type("見る"), Ok(ConjugationType::KamiIchidan));
/// assert_eq!(infer_conjugation_type("する"), Ok(ConjugationType::Sahen));
/// assert_eq!(infer_conjugation_type("来る"), Ok(ConjugationType::Kahen));
/// ```
/// ```
pub fn infer_conjugation_type(verb: &str) -> Result<ConjugationType, VerbError> {
    const GODAN_EXCEPTIONS: &[&str] = &[
        "入る",
        "要る",
        "いる",
        "切る",
        "千切る",
        "限る",
        "かぎる",
        "握る",
        "にぎる",
        "知る",
        "しる",
        "走る",
        "はしる",
        "交じる",
        "混じる",
        "まじる",
        "散る",
        "ちる",
        "帰る",
        "蹴る",
        "ける",
        "焦る",
        "あせる",
        "減る",
        "へる",
        "滑る",
        "すべる",
        "喋る",
        "しゃべる",
    ];
    if verb.is_empty() {
        return Err(VerbError::NotAVerb);
    }

    let chars: Vec<char> = verb.chars().collect();
    let len = chars.len();
    let last_char = chars[len - 1];

    // Check for Sahen (Suru)
    if verb == "する" || verb.ends_with("する") {
        return Ok(ConjugationType::Sahen);
    }
    // Check for Kahen (Kuru) - Strictly just 'kuru' usually, but let's handle it.
    // Kanji '来る' is 'kuru'. Hiragana 'くる' is 'kuru'.
    if verb == "くる" || verb == "来る" {
        return Ok(ConjugationType::Kahen);
    }

    // Check generic verb endings
    match last_char {
        'う' | 'く' | 'ぐ' | 'す' | 'つ' | 'ぬ' | 'ぶ' | 'む' => Ok(ConjugationType::Godan),
        'る' => {
            // Check exception list for Godan verbs ending in 'ru' that look like Ichidan
            if GODAN_EXCEPTIONS.contains(&verb) {
                return Ok(ConjugationType::Godan);
            }

            let prev_char = chars[len - 2];
            if is_i_sound(prev_char) {
                // Preceding vowel 'i' -> KamiIchidan (likely)
                Ok(ConjugationType::KamiIchidan)
            } else if is_e_sound(prev_char) {
                // Preceding vowel 'e' -> ShimoIchidan (likely)
                Ok(ConjugationType::ShimoIchidan)
            } else {
                Ok(ConjugationType::Godan)
            }
        }
        _ => Err(VerbError::NotAVerb),
    }
}

fn is_i_sound(c: char) -> bool {
    // Hiragana 'i' column
    matches!(
        c,
        'い' | 'き'
            | 'ぎ'
            | 'し'
            | 'じ'
            | 'ち'
            | 'ぢ'
            | 'に'
            | 'ひ'
            | 'び'
            | 'ぴ'
            | 'み'
            | 'り'
            | '見'
    )
}

fn is_e_sound(c: char) -> bool {
    // Hiragana 'e' column
    matches!(
        c,
        'え' | 'け'
            | 'げ'
            | 'せ'
            | 'ぜ'
            | 'て'
            | 'で'
            | 'ね'
            | 'へ'
            | 'べ'
            | 'ぺ'
            | 'め'
            | 'れ'
            | '出'
            | '寝'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_godan_basic() {
        assert_eq!(infer_conjugation_type("書く"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("泳ぐ"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("話す"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("待つ"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("死ぬ"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("遊ぶ"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("読む"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("買う"), Ok(ConjugationType::Godan));

        // Godan ending in ru (a/u/o sound)
        assert_eq!(infer_conjugation_type("終わる"), Ok(ConjugationType::Godan)); // wa-ru
        assert_eq!(infer_conjugation_type("作る"), Ok(ConjugationType::Godan)); // ku-ru
        assert_eq!(infer_conjugation_type("登る"), Ok(ConjugationType::Godan)); // bo-ru
    }

    #[test]
    fn test_godan_exceptions() {
        assert_eq!(infer_conjugation_type("走る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("帰る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("入る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("切る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("知る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("要る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("喋る"), Ok(ConjugationType::Godan));
        assert_eq!(infer_conjugation_type("減る"), Ok(ConjugationType::Godan));
    }

    #[test]
    fn test_kamiichidan() {
        // KamiIchidan examples
        let kami_ichidan_verbs = [
            "見る",
            "起きる",
            "落ちる",
            "降りる",
            "借りる",
            "浴びる",
            "閉じる",
            "生きる",
            "尽きる",
            "過ぎる",
            "伸びる",
            "老いる",
            "用いる",
            "朽ちる",
            "満ちる",
        ];
        for v in kami_ichidan_verbs {
            assert_eq!(
                infer_conjugation_type(v),
                Ok(ConjugationType::KamiIchidan),
                "Failed for {}",
                v
            );
        }
    }

    #[test]
    fn test_shimoichidan() {
        let shimo_ichidan_verbs = [
            "出る",
            "寝る",
            "食べる",
            "開ける",
            "閉める",
            "入れる",
            "出かける",
            "上げる",
            "下げる",
            "つける",
            "消える",
            "見せる",
            "教える",
            "覚える",
            "忘れる",
            "考える",
            "伝える",
            "迎える",
            "与える",
            "受ける",
            "避ける",
            "助ける",
            "調べる",
            "比べる",
            "変える",
            "替える",
        ];
        for v in shimo_ichidan_verbs {
            assert_eq!(
                infer_conjugation_type(v),
                Ok(ConjugationType::ShimoIchidan),
                "Failed for {}",
                v
            );
        }
    }

    #[test]
    fn test_irregulars() {
        assert_eq!(infer_conjugation_type("する"), Ok(ConjugationType::Sahen));
        assert_eq!(
            infer_conjugation_type("勉強する"),
            Ok(ConjugationType::Sahen)
        );
        assert_eq!(infer_conjugation_type("くる"), Ok(ConjugationType::Kahen));
        assert_eq!(infer_conjugation_type("来る"), Ok(ConjugationType::Kahen));
    }

    #[test]
    fn test_errors() {
        assert_eq!(infer_conjugation_type(""), Err(VerbError::NotAVerb));
        assert_eq!(infer_conjugation_type("あ"), Err(VerbError::NotAVerb));
        assert_eq!(infer_conjugation_type("リンゴ"), Err(VerbError::NotAVerb));
    }
}
