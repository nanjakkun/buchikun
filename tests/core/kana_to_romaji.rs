use buchikun::core::kana_to_romaji::{kana_to_romaji_hepburn, kana_to_romaji_kunrei};

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
    assert_eq!(kana_to_romaji_hepburn("キャンパス"), "kyampasu");
    assert_eq!(kana_to_romaji_hepburn("トウキョウ"), "toukyou");

    assert_eq!(kana_to_romaji_kunrei("キャンパス"), "kyanpasu");
    assert_eq!(kana_to_romaji_kunrei("トウキョウ"), "toukyou");

    // Difference
    assert_eq!(kana_to_romaji_hepburn("シャシン"), "shashin");
    assert_eq!(kana_to_romaji_kunrei("シャシン"), "syasin");
}

#[test]
fn test_long_vowel() {
    assert_eq!(kana_to_romaji_hepburn("パーティー"), "pa-ti-");
}

#[test]
fn test_hepburn_n_to_m() {
    assert_eq!(kana_to_romaji_hepburn("ナンバ"), "namba");
    assert_eq!(kana_to_romaji_hepburn("サンマ"), "samma");
    assert_eq!(kana_to_romaji_hepburn("カンパイ"), "kampai");
    assert_eq!(kana_to_romaji_hepburn("アンナイ"), "annai");
    assert_eq!(kana_to_romaji_hepburn("カンイ"), "kani");

    // Combo case
    assert_eq!(kana_to_romaji_hepburn("コンピュ"), "kompyu");
}

#[test]
fn test_kunrei_n_keeps_n() {
    assert_eq!(kana_to_romaji_kunrei("ナンバ"), "nanba");
    assert_eq!(kana_to_romaji_kunrei("サンマ"), "sanma");
    assert_eq!(kana_to_romaji_kunrei("カンパイ"), "kanpai");
    assert_eq!(kana_to_romaji_kunrei("アンナイ"), "annai");
    assert_eq!(kana_to_romaji_kunrei("カンイ"), "kani");

    // Combo case
    assert_eq!(kana_to_romaji_kunrei("コンピュ"), "konpyu");
}
