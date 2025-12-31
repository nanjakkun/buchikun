/// Katakana to Romaji (Hepburn system).
///
/// ヘボン式でカタカナをローマ字に変換します。
///
/// # Examples
///
/// ```
/// use buchikun::core::kana_to_romaji::kana_to_romaji_hepburn;
/// assert_eq!(kana_to_romaji_hepburn("カタカナ"), "katakana");
/// ```
pub fn kana_to_romaji_hepburn(input: &str) -> String {
    convert_kana_to_romaji(input, System::Hepburn)
}

/// Katakana to Romaji (Kunrei system).
///
/// 訓令式でカタカナをローマ字に変換します。
///
/// # Examples
///
/// ```
/// use buchikun::core::kana_to_romaji::kana_to_romaji_kunrei;
/// assert_eq!(kana_to_romaji_kunrei("カタカナ"), "katakana");
/// ```
pub fn kana_to_romaji_kunrei(input: &str) -> String {
    convert_kana_to_romaji(input, System::Kunrei)
}

#[derive(Clone, Copy)]
enum System {
    Hepburn,
    Kunrei,
}

fn convert_kana_to_romaji(input: &str, system: System) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        // Check for combination (current + next)
        if i + 1 < chars.len() {
            if let Some(romaji) = get_combo_romaji(chars[i], chars[i + 1], system) {
                result.push_str(romaji);
                i += 2;
                continue;
            }
        }

        // Check for small tsu (ッ)
        if chars[i] == 'ッ' {
            if i + 1 < chars.len() {
                // Resolve the next romaji to find its first consonant
                let (next_romaji, _) = resolve_next_romaji(&chars[i + 1..], system);
                if let Some(first_char) = next_romaji.chars().next() {
                    // Only double if it's a consonant.
                    match (system, next_romaji.as_str()) {
                        (System::Hepburn, s) if s.starts_with("ch") => result.push('t'),
                        (_, _) if is_consonant(first_char) => result.push(first_char),
                        _ => {} // atomic small tsu? or ignore
                    }

                    i += 1;
                    continue;
                }
            }
        }

        // Single char
        let romaji = get_single_romaji(chars[i], system);
        result.push_str(romaji);
        i += 1;
    }
    result
}

fn resolve_next_romaji(chars: &[char], system: System) -> (String, usize) {
    if chars.is_empty() {
        return (String::new(), 0);
    }
    if chars.len() >= 2 {
        if let Some(romaji) = get_combo_romaji(chars[0], chars[1], system) {
            return (romaji.to_string(), 2);
        }
    }
    (get_single_romaji(chars[0], system).to_string(), 1)
}

fn is_consonant(c: char) -> bool {
    matches!(
        c,
        'b' | 'c'
            | 'd'
            | 'f'
            | 'g'
            | 'h'
            | 'j'
            | 'k'
            | 'l'
            | 'm'
            | 'n'
            | 'p'
            | 'q'
            | 'r'
            | 's'
            | 't'
            | 'v'
            | 'w'
            | 'x'
            | 'y'
            | 'z'
    )
}

fn get_single_romaji(c: char, system: System) -> &'static str {
    match system {
        System::Hepburn => match c {
            'ア' => "a",
            'イ' => "i",
            'ウ' => "u",
            'エ' => "e",
            'オ' => "o",
            'カ' => "ka",
            'キ' => "ki",
            'ク' => "ku",
            'ケ' => "ke",
            'コ' => "ko",
            'サ' => "sa",
            'シ' => "shi",
            'ス' => "su",
            'セ' => "se",
            'ソ' => "so",
            'タ' => "ta",
            'チ' => "chi",
            'ツ' => "tsu",
            'テ' => "te",
            'ト' => "to",
            'ナ' => "na",
            'ニ' => "ni",
            'ヌ' => "nu",
            'ネ' => "ne",
            'ノ' => "no",
            'ハ' => "ha",
            'ヒ' => "hi",
            'フ' => "fu",
            'ヘ' => "he",
            'ホ' => "ho",
            'マ' => "ma",
            'ミ' => "mi",
            'ム' => "mu",
            'メ' => "me",
            'モ' => "mo",
            'ヤ' => "ya",
            'ユ' => "yu",
            'ヨ' => "yo",
            'ラ' => "ra",
            'リ' => "ri",
            'ル' => "ru",
            'レ' => "re",
            'ロ' => "ro",
            'ワ' => "wa",
            'ヲ' => "wo",
            'ン' => "n",
            'ガ' => "ga",
            'ギ' => "gi",
            'グ' => "gu",
            'ゲ' => "ge",
            'ゴ' => "go",
            'ザ' => "za",
            'ジ' => "ji",
            'ズ' => "zu",
            'ゼ' => "ze",
            'ゾ' => "zo",
            'ダ' => "da",
            'ヂ' => "ji",
            'ヅ' => "zu",
            'デ' => "de",
            'ド' => "do",
            'バ' => "ba",
            'ビ' => "bi",
            'ブ' => "bu",
            'ベ' => "be",
            'ボ' => "bo",
            'パ' => "pa",
            'ピ' => "pi",
            'プ' => "pu",
            'ペ' => "pe",
            'ポ' => "po",
            'ァ' => "a",
            'ィ' => "i",
            'ゥ' => "u",
            'ェ' => "e",
            'ォ' => "o",
            'ャ' => "ya",
            'ュ' => "yu",
            'ョ' => "yo",
            'ヮ' => "wa",
            'ー' => "-",
            _ => "", // Fallback
        },
        System::Kunrei => match c {
            'ア' => "a",
            'イ' => "i",
            'ウ' => "u",
            'エ' => "e",
            'オ' => "o",
            'カ' => "ka",
            'キ' => "ki",
            'ク' => "ku",
            'ケ' => "ke",
            'コ' => "ko",
            'サ' => "sa",
            'シ' => "si",
            'ス' => "su",
            'セ' => "se",
            'ソ' => "so",
            'タ' => "ta",
            'チ' => "ti",
            'ツ' => "tu",
            'テ' => "te",
            'ト' => "to",
            'ナ' => "na",
            'ニ' => "ni",
            'ヌ' => "nu",
            'ネ' => "ne",
            'ノ' => "no",
            'ハ' => "ha",
            'ヒ' => "hi",
            'フ' => "hu",
            'ヘ' => "he",
            'ホ' => "ho",
            'マ' => "ma",
            'ミ' => "mi",
            'ム' => "mu",
            'メ' => "me",
            'モ' => "mo",
            'ヤ' => "ya",
            'ユ' => "yu",
            'ヨ' => "yo",
            'ラ' => "ra",
            'リ' => "ri",
            'ル' => "ru",
            'レ' => "re",
            'ロ' => "ro",
            'ワ' => "wa",
            'ヲ' => "wo",
            'ン' => "n",
            'ガ' => "ga",
            'ギ' => "gi",
            'グ' => "gu",
            'ゲ' => "ge",
            'ゴ' => "go",
            'ザ' => "za",
            'ジ' => "zi",
            'ズ' => "zu",
            'ゼ' => "ze",
            'ゾ' => "zo",
            'ダ' => "da",
            'ヂ' => "zi",
            'ヅ' => "zu",
            'デ' => "de",
            'ド' => "do",
            'バ' => "ba",
            'ビ' => "bi",
            'ブ' => "bu",
            'ベ' => "be",
            'ボ' => "bo",
            'パ' => "pa",
            'ピ' => "pi",
            'プ' => "pu",
            'ペ' => "pe",
            'ポ' => "po",
            'ァ' => "a",
            'ィ' => "i",
            'ゥ' => "u",
            'ェ' => "e",
            'ォ' => "o",
            'ャ' => "ya",
            'ュ' => "yu",
            'ョ' => "yo",
            'ヮ' => "wa",
            'ー' => "-",
            _ => "",
        },
    }
}

fn get_combo_romaji(c1: char, c2: char, system: System) -> Option<&'static str> {
    match system {
        System::Hepburn => match (c1, c2) {
            ('キ', 'ャ') => Some("kya"),
            ('キ', 'ュ') => Some("kyu"),
            ('キ', 'ョ') => Some("kyo"),
            ('シ', 'ャ') => Some("sha"),
            ('シ', 'ュ') => Some("shu"),
            ('シ', 'ョ') => Some("sho"),
            ('チ', 'ャ') => Some("cha"),
            ('チ', 'ュ') => Some("chu"),
            ('チ', 'ョ') => Some("cho"),
            ('ニ', 'ャ') => Some("nya"),
            ('ニ', 'ュ') => Some("nyu"),
            ('ニ', 'ョ') => Some("nyo"),
            ('ヒ', 'ャ') => Some("hya"),
            ('ヒ', 'ュ') => Some("hyu"),
            ('ヒ', 'ョ') => Some("hyo"),
            ('ミ', 'ャ') => Some("mya"),
            ('ミ', 'ュ') => Some("myu"),
            ('ミ', 'ョ') => Some("myo"),
            ('リ', 'ャ') => Some("rya"),
            ('リ', 'ュ') => Some("ryu"),
            ('リ', 'ョ') => Some("ryo"),
            ('ギ', 'ャ') => Some("gya"),
            ('ギ', 'ュ') => Some("gyu"),
            ('ギ', 'ョ') => Some("gyo"),
            ('ジ', 'ャ') => Some("ja"),
            ('ジ', 'ュ') => Some("ju"),
            ('ジ', 'ョ') => Some("jo"),
            ('ビ', 'ャ') => Some("bya"),
            ('ビ', 'ュ') => Some("byu"),
            ('ビ', 'ョ') => Some("byo"),
            ('ピ', 'ャ') => Some("pya"),
            ('ピ', 'ュ') => Some("pyu"),
            ('ピ', 'ョ') => Some("pyo"),
            ('ヂ', 'ャ') => Some("ja"),
            ('ヂ', 'ュ') => Some("ju"),
            ('ヂ', 'ョ') => Some("jo"),
            ('テ', 'ィ') => Some("ti"),
            ('デ', 'ィ') => Some("di"),
            ('ト', 'ゥ') => Some("tu"),
            ('ド', 'ゥ') => Some("du"),
            ('フ', 'ァ') => Some("fa"),
            ('フ', 'ィ') => Some("fi"),
            ('フ', 'ェ') => Some("fe"),
            ('フ', 'ォ') => Some("fo"),
            ('ウ', 'ィ') => Some("wi"),
            ('ウ', 'ェ') => Some("we"),
            ('ウ', 'ォ') => Some("wo"),
            ('シ', 'ェ') => Some("she"),
            ('ジ', 'ェ') => Some("je"),
            ('チ', 'ェ') => Some("che"),
            _ => None,
        },
        System::Kunrei => match (c1, c2) {
            ('キ', 'ャ') => Some("kya"),
            ('キ', 'ュ') => Some("kyu"),
            ('キ', 'ョ') => Some("kyo"),
            ('シ', 'ャ') => Some("sya"),
            ('シ', 'ュ') => Some("syu"),
            ('シ', 'ョ') => Some("syo"),
            ('チ', 'ャ') => Some("tya"),
            ('チ', 'ュ') => Some("tyu"),
            ('チ', 'ョ') => Some("tyo"),
            ('ニ', 'ャ') => Some("nya"),
            ('ニ', 'ュ') => Some("nyu"),
            ('ニ', 'ョ') => Some("nyo"),
            ('ヒ', 'ャ') => Some("hya"),
            ('ヒ', 'ュ') => Some("hyu"),
            ('ヒ', 'ョ') => Some("hyo"),
            ('ミ', 'ャ') => Some("mya"),
            ('ミ', 'ュ') => Some("myu"),
            ('ミ', 'ョ') => Some("myo"),
            ('リ', 'ャ') => Some("rya"),
            ('リ', 'ュ') => Some("ryu"),
            ('リ', 'ョ') => Some("ryo"),
            ('ギ', 'ャ') => Some("gya"),
            ('ギ', 'ュ') => Some("gyu"),
            ('ギ', 'ョ') => Some("gyo"),
            ('ジ', 'ャ') => Some("zya"),
            ('ジ', 'ュ') => Some("zyu"),
            ('ジ', 'ョ') => Some("zyo"),
            ('ビ', 'ャ') => Some("bya"),
            ('ビ', 'ュ') => Some("byu"),
            ('ビ', 'ョ') => Some("byo"),
            ('ピ', 'ャ') => Some("pya"),
            ('ピ', 'ュ') => Some("pyu"),
            ('ピ', 'ョ') => Some("pyo"),
            ('ヂ', 'ャ') => Some("zya"),
            ('ヂ', 'ュ') => Some("zyu"),
            ('ヂ', 'ョ') => Some("zyo"),
            ('テ', 'ィ') => Some("ti"),
            ('デ', 'ィ') => Some("di"),
            ('ト', 'ゥ') => Some("tu"),
            ('ド', 'ゥ') => Some("du"),
            ('フ', 'ァ') => Some("fa"),
            ('フ', 'ィ') => Some("fi"),
            ('フ', 'ェ') => Some("fe"),
            ('フ', 'ォ') => Some("fo"),
            ('ウ', 'ィ') => Some("wi"),
            ('ウ', 'ェ') => Some("we"),
            ('ウ', 'ォ') => Some("wo"),
            ('シ', 'ェ') => Some("sye"),
            ('ジ', 'ェ') => Some("zye"),
            ('チ', 'ェ') => Some("tye"),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hepburn_basic() {
        assert_eq!(kana_to_romaji_hepburn("カタカナ"), "katakana");
        assert_eq!(kana_to_romaji_hepburn("シブヤ"), "shibuya");
    }

    #[test]
    fn test_kunrei_basic() {
        assert_eq!(kana_to_romaji_kunrei("カタカナ"), "katakana");
        assert_eq!(kana_to_romaji_kunrei("シブヤ"), "sibuya");
    }

    #[test]
    fn test_small_tsu() {
        assert_eq!(kana_to_romaji_hepburn("カッパ"), "kappa");
        assert_eq!(kana_to_romaji_hepburn("チケット"), "chiketto");
        // Kunrei
        assert_eq!(kana_to_romaji_kunrei("カッパ"), "kappa");
        assert_eq!(kana_to_romaji_kunrei("チケット"), "tiketto");
    }

    #[test]
    fn test_combo() {
        assert_eq!(kana_to_romaji_hepburn("キャンパス"), "kyanpasu");
        assert_eq!(kana_to_romaji_hepburn("トウキョウ"), "toukyou");

        assert_eq!(kana_to_romaji_kunrei("キャンパス"), "kyanpasu");
        assert_eq!(kana_to_romaji_kunrei("トウキョウ"), "toukyou"); // toukyou (tokyo) matches standard strict transliteration?

        // Difference
        assert_eq!(kana_to_romaji_hepburn("シャシン"), "shashin");
        assert_eq!(kana_to_romaji_kunrei("シャシン"), "syasin");
    }

    #[test]
    fn test_long_vowel() {
        assert_eq!(kana_to_romaji_hepburn("パーティー"), "pa-ti-");
        // My implementation maps 'ー' to '-' currently.
    }
}
