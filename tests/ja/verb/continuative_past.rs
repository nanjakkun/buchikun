use buchikun::continuative_past;
use buchikun::ja::verb::{ConjugationType, continuative_past};

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
