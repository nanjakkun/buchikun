use buchikun::ja::verb::{ConjugationType, terminal};
use buchikun::terminal;

#[test]
fn test_terminal_godan() {
    assert_eq!(
        terminal("書く", ConjugationType::Godan),
        Ok("書く".to_string())
    );
    assert_eq!(
        terminal("走る", ConjugationType::Godan),
        Ok("走る".to_string())
    );
    assert_eq!(
        terminal("泳ぐ", ConjugationType::Godan),
        Ok("泳ぐ".to_string())
    );
    assert_eq!(
        terminal("話す", ConjugationType::Godan),
        Ok("話す".to_string())
    );
    assert_eq!(
        terminal("待つ", ConjugationType::Godan),
        Ok("待つ".to_string())
    );
    assert_eq!(
        terminal("死ぬ", ConjugationType::Godan),
        Ok("死ぬ".to_string())
    );
    assert_eq!(
        terminal("呼ぶ", ConjugationType::Godan),
        Ok("呼ぶ".to_string())
    );
    assert_eq!(
        terminal("飲む", ConjugationType::Godan),
        Ok("飲む".to_string())
    );
    assert_eq!(
        terminal("買う", ConjugationType::Godan),
        Ok("買う".to_string())
    );
}

#[test]
fn test_terminal_ichidan() {
    assert_eq!(
        terminal("見る", ConjugationType::KamiIchidan),
        Ok("見る".to_string())
    );
    assert_eq!(
        terminal("食べる", ConjugationType::ShimoIchidan),
        Ok("食べる".to_string())
    );
}

#[test]
fn test_terminal_sahen() {
    assert_eq!(
        terminal("する", ConjugationType::Sahen),
        Ok("する".to_string())
    );
    assert_eq!(
        terminal("勉強する", ConjugationType::Sahen),
        Ok("勉強する".to_string())
    );
}

#[test]
fn test_terminal_kahen() {
    assert_eq!(
        terminal("くる", ConjugationType::Kahen),
        Ok("くる".to_string())
    );
    assert_eq!(
        terminal("来る", ConjugationType::Kahen),
        Ok("来る".to_string())
    );
}

#[test]
fn test_terminal_macro() {
    assert_eq!(terminal!("書く"), Ok("書く".to_string()));
    assert_eq!(terminal!("食べる"), Ok("食べる".to_string()));
}
