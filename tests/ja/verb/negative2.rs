use buchikun::ja::verb::{ConjugationType, negative2};
use buchikun::negative2;

#[test]
fn test_negative2() {
    assert_eq!(
        negative2("書く", ConjugationType::Godan),
        Ok("書か".to_string())
    );
    assert_eq!(
        negative2("見る", ConjugationType::KamiIchidan),
        Ok("見".to_string())
    );
    assert_eq!(
        negative2("食べる", ConjugationType::ShimoIchidan),
        Ok("食べ".to_string())
    );
    assert_eq!(
        negative2("する", ConjugationType::Sahen),
        Ok("せ".to_string())
    );
    assert_eq!(
        negative2("来る", ConjugationType::Kahen),
        Ok("こ".to_string())
    );
}

#[test]
fn test_negative2_macro() {
    assert_eq!(negative2!("書く"), Ok("書か".to_string()));
    assert_eq!(negative2!("食べる"), Ok("食べ".to_string()));
    assert_eq!(negative2!("する"), Ok("せ".to_string()));
}
