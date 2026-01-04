use buchikun::continuative_present;
use buchikun::ja::verb::{ConjugationType, continuative_present};

#[test]
fn test_continuative_present() {
    assert_eq!(
        continuative_present("書く", ConjugationType::Godan),
        Ok("書き".to_string())
    );
    assert_eq!(
        continuative_present("泳ぐ", ConjugationType::Godan),
        Ok("泳ぎ".to_string())
    );
    assert_eq!(
        continuative_present("死ぬ", ConjugationType::Godan),
        Ok("死に".to_string())
    );
    assert_eq!(
        continuative_present("遊ぶ", ConjugationType::Godan),
        Ok("遊び".to_string())
    );
    assert_eq!(
        continuative_present("買う", ConjugationType::Godan),
        Ok("買い".to_string())
    );
    assert_eq!(
        continuative_present("見る", ConjugationType::KamiIchidan),
        Ok("見".to_string())
    );
    assert_eq!(
        continuative_present("起きる", ConjugationType::KamiIchidan),
        Ok("起き".to_string())
    );
    assert_eq!(
        continuative_present("食べる", ConjugationType::ShimoIchidan),
        Ok("食べ".to_string())
    );
    assert_eq!(
        continuative_present("する", ConjugationType::Sahen),
        Ok("し".to_string())
    );
    assert_eq!(
        continuative_present("勉強する", ConjugationType::Sahen),
        Ok("勉強し".to_string())
    );
    assert_eq!(
        continuative_present("くる", ConjugationType::Kahen),
        Ok("き".to_string())
    );
    assert_eq!(
        continuative_present("来る", ConjugationType::Kahen),
        Ok("き".to_string())
    );
}

#[test]
fn test_continuative_present_macro() {
    assert_eq!(continuative_present!("書く"), Ok("書き".to_string()));
    assert_eq!(continuative_present!("食べる"), Ok("食べ".to_string()));
    assert_eq!(continuative_present!("する"), Ok("し".to_string()));
}
