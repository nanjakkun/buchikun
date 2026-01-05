use buchikun::ja::verb::{ConjugationType, passive};
use buchikun::passive;

#[test]
fn test_passive_godan() {
    assert_eq!(
        passive("書く", ConjugationType::Godan),
        Ok("書かれる".to_string())
    );
    assert_eq!(
        passive("泳ぐ", ConjugationType::Godan),
        Ok("泳がれる".to_string())
    );
    assert_eq!(
        passive("話す", ConjugationType::Godan),
        Ok("話される".to_string())
    );
    assert_eq!(
        passive("待つ", ConjugationType::Godan),
        Ok("待たれる".to_string())
    );
    assert_eq!(
        passive("死ぬ", ConjugationType::Godan),
        Ok("死なれる".to_string())
    );
    assert_eq!(
        passive("呼ぶ", ConjugationType::Godan),
        Ok("呼ばれる".to_string())
    );
    assert_eq!(
        passive("飲む", ConjugationType::Godan),
        Ok("飲まれる".to_string())
    );
    assert_eq!(
        passive("走る", ConjugationType::Godan),
        Ok("走られる".to_string())
    );
    assert_eq!(
        passive("買う", ConjugationType::Godan),
        Ok("買われる".to_string())
    );
}

#[test]
fn test_passive_ichidan() {
    assert_eq!(
        passive("見る", ConjugationType::KamiIchidan),
        Ok("見られる".to_string())
    );
    assert_eq!(
        passive("食べる", ConjugationType::ShimoIchidan),
        Ok("食べられる".to_string())
    );
}

#[test]
fn test_passive_sahen() {
    assert_eq!(
        passive("する", ConjugationType::Sahen),
        Ok("される".to_string())
    );
    assert_eq!(
        passive("勉強する", ConjugationType::Sahen),
        Ok("勉強される".to_string())
    );
}

#[test]
fn test_passive_kahen() {
    assert_eq!(
        passive("くる", ConjugationType::Kahen),
        Ok("こられる".to_string())
    );
    assert_eq!(
        passive("来る", ConjugationType::Kahen),
        Ok("こられる".to_string())
    );
}

#[test]
fn test_passive_macro() {
    assert_eq!(passive!("書く"), Ok("書かれる".to_string()));
    assert_eq!(passive!("食べる"), Ok("食べられる".to_string()));
    assert_eq!(passive!("する"), Ok("される".to_string()));
}
