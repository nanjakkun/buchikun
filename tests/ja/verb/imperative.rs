use buchikun::imperative;
use buchikun::ja::verb::{ConjugationType, imperative};

#[test]
fn test_imperative() {
    assert_eq!(
        imperative("書く", ConjugationType::Godan),
        Ok("書け".to_string())
    );
    assert_eq!(
        imperative("見る", ConjugationType::KamiIchidan),
        Ok("見ろ".to_string())
    );
    assert_eq!(
        imperative("食べる", ConjugationType::ShimoIchidan),
        Ok("食べろ".to_string())
    );
    assert_eq!(
        imperative("する", ConjugationType::Sahen),
        Ok("しろ".to_string())
    );
    assert_eq!(
        imperative("来る", ConjugationType::Kahen),
        Ok("こい".to_string())
    );
}

#[test]
fn test_imperative_macro() {
    assert_eq!(imperative!("書く"), Ok("書け".to_string()));
    assert_eq!(imperative!("食べる"), Ok("食べろ".to_string()));
    assert_eq!(imperative!("する"), Ok("しろ".to_string()));
}
