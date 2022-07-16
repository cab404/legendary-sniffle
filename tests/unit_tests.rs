use std::{
    collections::{BTreeSet, HashMap},
    fs,
};
use stroki::{get_unique_key, pipeline, run, Config};
use strsim::jaro;
#[test]
fn get_key_simple() {
    let mut old_answer = vec![
        ("x:1".to_string(), "a".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("".to_string(), "".to_string()),
        ("x:2".to_string(), "b".to_string()),
        ("x:3".to_string(), "c".to_string()),
    ];
    for i in 1..11 {
        assert_eq!(
            format!("x:1{}", i-1 ),
            get_unique_key(&mut old_answer, i, &BTreeSet::new())
        );
        old_answer[i].0 = format!("x:1{}", i-1);
    }
    assert_eq!(
        format!("x:190"),
        get_unique_key(&mut old_answer, 11, &BTreeSet::new())
    );
    old_answer[11].0 = format!("x:190");
    assert_eq!(
        format!("x:191"),
        get_unique_key(&mut old_answer, 12, &BTreeSet::new())
    );
    old_answer[12].0 = format!("x:191");
    assert_eq!(old_answer, vec![]);
}
