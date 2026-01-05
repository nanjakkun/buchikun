use buchikun::imperative2;
use buchikun::ja::verb::{ConjugationType, imperative2};

#[test]
fn test_imperative2() {
    // Other types should behave same as imperative
    assert_eq!(
        imperative2("書く", ConjugationType::Godan),
        Ok("書け".to_string())
    );
    assert_eq!(
        imperative2("見る", ConjugationType::KamiIchidan),
        Ok("見ろ".to_string())
    );

    // Sahen should return "せよ"
    assert_eq!(
        imperative2("する", ConjugationType::Sahen),
        Ok("せよ".to_string())
    );
    assert_eq!(
        imperative2("勉強する", ConjugationType::Sahen),
        Ok("勉強せよ".to_string())
    );

    // Kahen should still be "こい"
    assert_eq!(
        imperative2("来る", ConjugationType::Kahen),
        Ok("こい".to_string())
    );
}

#[test]
fn test_imperative2_macro() {
    assert_eq!(imperative2!("する"), Ok("せよ".to_string()));
    assert_eq!(imperative2!("書く"), Ok("書け".to_string()));
}
