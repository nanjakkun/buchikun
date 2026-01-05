use buchikun::ja::verb::{ConjugationType, negative};
use buchikun::negative;

#[test]
fn test_negative() {
    assert_eq!(
        negative("書く", ConjugationType::Godan),
        Ok("書か".to_string())
    );
    assert_eq!(
        negative("見る", ConjugationType::KamiIchidan),
        Ok("見".to_string())
    );
    assert_eq!(
        negative("食べる", ConjugationType::ShimoIchidan),
        Ok("食べ".to_string())
    );
    assert_eq!(
        negative("する", ConjugationType::Sahen),
        Ok("し".to_string())
    );
    assert_eq!(
        negative("来る", ConjugationType::Kahen),
        Ok("こ".to_string())
    );
}

#[test]
fn test_negative_macro() {
    assert_eq!(negative!("書く"), Ok("書か".to_string()));
    assert_eq!(negative!("食べる"), Ok("食べ".to_string()));
    assert_eq!(negative!("する"), Ok("し".to_string()));
}
