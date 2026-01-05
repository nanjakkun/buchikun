use buchikun::conditional;
use buchikun::ja::verb::{ConjugationType, conditional};

#[test]
fn test_conditional() {
    assert_eq!(
        conditional("書く", ConjugationType::Godan),
        Ok("書け".to_string())
    );
    assert_eq!(
        conditional("見る", ConjugationType::KamiIchidan),
        Ok("見れ".to_string())
    );
    assert_eq!(
        conditional("食べる", ConjugationType::ShimoIchidan),
        Ok("食べれ".to_string())
    );
    assert_eq!(
        conditional("する", ConjugationType::Sahen),
        Ok("すれ".to_string())
    );
    assert_eq!(
        conditional("来る", ConjugationType::Kahen),
        Ok("くれ".to_string())
    );
}

#[test]
fn test_conditional_macro() {
    assert_eq!(conditional!("書く"), Ok("書け".to_string()));
    assert_eq!(conditional!("食べる"), Ok("食べれ".to_string()));
    assert_eq!(conditional!("する"), Ok("すれ".to_string()));
}
