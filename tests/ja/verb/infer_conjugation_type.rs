use buchikun::ja::verb::{ConjugationType, VerbError, infer_conjugation_type};

#[test]
fn test_godan_basic() {
    assert_eq!(infer_conjugation_type("書く"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("泳ぐ"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("話す"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("待つ"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("死ぬ"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("遊ぶ"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("読む"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("買う"), Ok(ConjugationType::Godan));

    // Godan ending in ru (a/u/o sound)
    assert_eq!(infer_conjugation_type("終わる"), Ok(ConjugationType::Godan)); // wa-ru
    assert_eq!(infer_conjugation_type("作る"), Ok(ConjugationType::Godan)); // ku-ru
    assert_eq!(infer_conjugation_type("登る"), Ok(ConjugationType::Godan)); // bo-ru
}

#[test]
fn test_godan_exceptions() {
    assert_eq!(infer_conjugation_type("走る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("帰る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("入る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("切る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("知る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("要る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("喋る"), Ok(ConjugationType::Godan));
    assert_eq!(infer_conjugation_type("減る"), Ok(ConjugationType::Godan));
}

#[test]
fn test_kamiichidan() {
    // KamiIchidan examples
    let kami_ichidan_verbs = [
        "見る",
        "起きる",
        "落ちる",
        "降りる",
        "借りる",
        "浴びる",
        "閉じる",
        "生きる",
        "尽きる",
        "過ぎる",
        "伸びる",
        "老いる",
        "用いる",
        "朽ちる",
        "満ちる",
    ];
    for v in kami_ichidan_verbs {
        assert_eq!(
            infer_conjugation_type(v),
            Ok(ConjugationType::KamiIchidan),
            "Failed for {}",
            v
        );
    }
}

#[test]
fn test_shimoichidan() {
    let shimo_ichidan_verbs = [
        "出る",
        "寝る",
        "食べる",
        "開ける",
        "閉める",
        "入れる",
        "出かける",
        "上げる",
        "下げる",
        "つける",
        "消える",
        "見せる",
        "教える",
        "覚える",
        "忘れる",
        "考える",
        "伝える",
        "迎える",
        "与える",
        "受ける",
        "避ける",
        "助ける",
        "調べる",
        "比べる",
        "変える",
        "替える",
    ];
    for v in shimo_ichidan_verbs {
        assert_eq!(
            infer_conjugation_type(v),
            Ok(ConjugationType::ShimoIchidan),
            "Failed for {}",
            v
        );
    }
}

#[test]
fn test_irregulars() {
    assert_eq!(infer_conjugation_type("する"), Ok(ConjugationType::Sahen));
    assert_eq!(
        infer_conjugation_type("勉強する"),
        Ok(ConjugationType::Sahen)
    );
    assert_eq!(infer_conjugation_type("くる"), Ok(ConjugationType::Kahen));
    assert_eq!(infer_conjugation_type("来る"), Ok(ConjugationType::Kahen));
}

#[test]
fn test_errors() {
    assert_eq!(infer_conjugation_type(""), Err(VerbError::NotAVerb));
    assert_eq!(infer_conjugation_type("あ"), Err(VerbError::NotAVerb));
    assert_eq!(infer_conjugation_type("リンゴ"), Err(VerbError::NotAVerb));
}
