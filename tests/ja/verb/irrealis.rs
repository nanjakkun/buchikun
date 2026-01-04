use buchikun::irrealis;
use buchikun::ja::verb::{ConjugationType, irrealis};

#[test]
fn test_irrealis() {
    assert_eq!(
        irrealis("書く", ConjugationType::Godan),
        Ok("書こ".to_string())
    );
    assert_eq!(
        irrealis("泳ぐ", ConjugationType::Godan),
        Ok("泳ご".to_string())
    );
    assert_eq!(
        irrealis("買う", ConjugationType::Godan),
        Ok("買お".to_string())
    );
    assert_eq!(
        irrealis("見る", ConjugationType::KamiIchidan),
        Ok("見".to_string())
    );
    assert_eq!(
        irrealis("食べる", ConjugationType::ShimoIchidan),
        Ok("食べ".to_string())
    );
    assert_eq!(
        irrealis("する", ConjugationType::Sahen),
        Ok("し".to_string())
    );
    assert_eq!(
        irrealis("来る", ConjugationType::Kahen),
        Ok("こ".to_string())
    );
}

#[test]
fn test_irrealis_macro() {
    assert_eq!(irrealis!("書く"), Ok("書こ".to_string()));
    assert_eq!(irrealis!("食べる"), Ok("食べ".to_string()));
    assert_eq!(irrealis!("する"), Ok("し".to_string()));
}
