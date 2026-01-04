use buchikun::ja::verb::{ConjugationType, negative};
use buchikun::negative;

#[test]
fn test_negative() {
    assert_eq!(
        negative("書く", ConjugationType::Godan),
        Ok("書かない".to_string())
    );
    assert_eq!(
        negative("見る", ConjugationType::KamiIchidan),
        Ok("見ない".to_string())
    );
    assert_eq!(
        negative("食べる", ConjugationType::ShimoIchidan),
        Ok("食べない".to_string())
    );
    assert_eq!(
        negative("する", ConjugationType::Sahen),
        Ok("しない".to_string())
    );
    assert_eq!(
        negative("来る", ConjugationType::Kahen),
        Ok("こない".to_string())
    );
}

#[test]
fn test_negative_macro() {
    assert_eq!(negative!("書く"), Ok("書かない".to_string()));
    assert_eq!(negative!("食べる"), Ok("食べない".to_string()));
    assert_eq!(negative!("する"), Ok("しない".to_string()));
}
