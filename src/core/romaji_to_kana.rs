pub fn romaji_to_kana(input: &str) -> String {
    let mut result = String::new();

    // We'll use a loop and advance manually
    // However, since we need lookahead for longest match, working with a string slice or char vec is easier.
    // Let's use the remaining string slice.

    let mut current_idx = 0;
    while current_idx < input.len() {
        let remaining = &input[current_idx..];

        // Try to find a match in the map
        // The map should be ordered by length descending effectively
        // Since we don't want to iterate a huge list every time, we can try to match based on known prefixes.
        // But a simple iteration over a static sorted list is fine for reasonable input length.

        if let Some((len, kana)) = find_match(remaining) {
            result.push_str(kana);
            current_idx += len;
            continue;
        }

        // Handle small tsu (double consonant)
        // If current char is a consonant and matches next char
        // Note: We need to be careful not to consume it if it's part of a valid mapping that we missed (unlikely if map is complete)
        // Rule: if s[0] == s[1] and is consonant -> insert small tsu, consume 1 char (the first consonant)
        // The second consonant will start the next match.
        // e.g. "tt" -> "っ" + "t" -> next loop "ta" -> "た" => "った"
        let first_char = remaining.chars().next().unwrap();
        let first_char_len = first_char.len_utf8();

        if remaining.len() > first_char_len {
            let next_char = remaining[first_char_len..].chars().next().unwrap();
            if first_char == next_char && is_consonant(first_char) {
                result.push('っ');
                current_idx += first_char_len;
                continue;
            }
        }

        // 'n' handling handling
        // 'n' alone should be 'ん' if it didn't match 'na', 'ni' etc (which are in the map).
        // If 'n' is mapped in the map as 'ん', it works (since longer keys like 'na' are checked first).

        // Default: copy char
        result.push(first_char);
        current_idx += first_char_len;
    }

    result
}

fn is_consonant(c: char) -> bool {
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => false,
        // 'n' is somewhat special, but for the purpose of 'tt', 'ss', 'pp', 'n' is not doubled usually as small tsu (nn -> ん)
        'n' => false,
        _ => c.is_ascii_alphabetic(),
    }
}

fn find_match(s: &str) -> Option<(usize, &str)> {
    // Mappings sorted by length descending
    // This is a subset of Hepburn, expand as needed
    const MAPPINGS: &[(&str, &str)] = &[
        ("kya", "きゃ"),
        ("kyu", "きゅ"),
        ("kyo", "きょ"),
        ("sha", "しゃ"),
        ("shu", "しゅ"),
        ("sho", "しょ"),
        ("cha", "ちゃ"),
        ("chu", "ちゅ"),
        ("cho", "ちょ"),
        ("nya", "にゃ"),
        ("nyu", "にゅ"),
        ("nyo", "にょ"),
        ("hya", "ひゃ"),
        ("hyu", "ひゅ"),
        ("hyo", "ひょ"),
        ("mya", "みゃ"),
        ("myu", "みゅ"),
        ("myo", "みょ"),
        ("rya", "りゃ"),
        ("ryu", "りゅ"),
        ("ryo", "りょ"),
        ("gya", "ぎゃ"),
        ("gyu", "ぎゅ"),
        ("gyo", "ぎょ"),
        ("ja", "じゃ"),
        ("ju", "じゅ"),
        ("jo", "じょ"),
        ("bya", "びゃ"),
        ("byu", "びゅ"),
        ("byo", "びょ"),
        ("pya", "ぴゃ"),
        ("pyu", "ぴゅ"),
        ("pyo", "ぴょ"),
        ("shi", "し"),
        ("chi", "ち"),
        ("tsu", "つ"),
        ("ka", "か"),
        ("ki", "き"),
        ("ku", "く"),
        ("ke", "け"),
        ("ko", "こ"),
        ("sa", "さ"),
        ("su", "す"),
        ("se", "せ"),
        ("so", "そ"),
        ("ta", "た"),
        ("te", "て"),
        ("to", "と"),
        ("na", "な"),
        ("ni", "に"),
        ("nu", "ぬ"),
        ("ne", "ね"),
        ("no", "の"),
        ("ha", "は"),
        ("hi", "ひ"),
        ("hu", "ふ"),
        ("fu", "ふ"),
        ("he", "へ"),
        ("ho", "ほ"),
        ("ma", "ま"),
        ("mi", "み"),
        ("mu", "む"),
        ("me", "め"),
        ("mo", "も"),
        ("ya", "や"),
        ("yu", "ゆ"),
        ("yo", "よ"),
        ("ra", "ら"),
        ("ri", "り"),
        ("ru", "る"),
        ("re", "れ"),
        ("ro", "ろ"),
        ("wa", "わ"),
        ("wo", "を"),
        ("ga", "が"),
        ("gi", "ぎ"),
        ("gu", "ぐ"),
        ("ge", "げ"),
        ("go", "ご"),
        ("za", "ざ"),
        ("ji", "じ"),
        ("zu", "ず"),
        ("ze", "ぜ"),
        ("zo", "ぞ"),
        ("da", "だ"),
        ("ji", "ぢ"),
        ("zu", "づ"),
        ("de", "で"),
        ("do", "ど"),
        ("ba", "ば"),
        ("bi", "び"),
        ("bu", "ぶ"),
        ("be", "べ"),
        ("bo", "ぼ"),
        ("pa", "ぱ"),
        ("pi", "ぴ"),
        ("pu", "ぷ"),
        ("pe", "ぺ"),
        ("po", "ぽ"),
        ("a", "あ"),
        ("i", "い"),
        ("u", "う"),
        ("e", "え"),
        ("o", "お"),
        ("n", "ん"),
        ("-", "ー"),
    ];

    for (romaji, kana) in MAPPINGS {
        if s.starts_with(romaji) {
            return Some((romaji.len(), kana));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(romaji_to_kana("konnichiha"), "こんにちは");
        assert_eq!(romaji_to_kana("arigatou"), "ありがとう");
    }

    #[test]
    fn test_small_tsu() {
        assert_eq!(romaji_to_kana("gakkou"), "がっこう");
        assert_eq!(romaji_to_kana("zettai"), "ぜったい");
    }

    #[test]
    fn test_contracted() {
        assert_eq!(romaji_to_kana("shumi"), "しゅみ");
        assert_eq!(romaji_to_kana("tokyo"), "ときょ"); // Wait, tokyo is toukyou usually or tokyo
        // In this map, 'kyo' -> 'きょ'. 'to' -> 'と'. So 'tokyo' -> 'ときょ'. 'toukyou' -> 'とうきょう'.
    }

    #[test]
    fn test_mixed() {
        assert_eq!(romaji_to_kana("romaji"), "ろまじ"); // ro ma ji
    }
}
