use buchikun::attributive;
use buchikun::ja::verb::{ConjugationType, attributive};

#[test]
fn test_attributive() {
    assert_eq!(
        attributive("書く", ConjugationType::Godan),
        Ok("書く".to_string())
    );
    assert_eq!(
        attributive("見る", ConjugationType::KamiIchidan),
        Ok("見る".to_string())
    );
    assert_eq!(
        attributive("食べる", ConjugationType::ShimoIchidan),
        Ok("食べる".to_string())
    );
    assert_eq!(
        attributive("する", ConjugationType::Sahen),
        Ok("する".to_string())
    );
    assert_eq!(
        attributive("来る", ConjugationType::Kahen),
        Ok("来る".to_string())
    );
}

#[test]
fn test_attributive_macro() {
    assert_eq!(attributive!("書く"), Ok("書く".to_string()));
    assert_eq!(attributive!("食べる"), Ok("食べる".to_string()));
}
