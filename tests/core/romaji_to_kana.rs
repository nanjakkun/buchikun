use buchikun::core::romaji_to_kana::romaji_to_kana;

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
    assert_eq!(romaji_to_kana("tokyo"), "ときょ");
}

#[test]
fn test_mixed() {
    assert_eq!(romaji_to_kana("romaji"), "ろまじ");
}
