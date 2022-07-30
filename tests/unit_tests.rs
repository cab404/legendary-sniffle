use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
};
use stroki::{alphabetical_sort, fill_similar_strings, get_unique_key, pipeline, run, Config};
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
            format!("x:1{}", i - 1),
            get_unique_key(&mut old_answer, i, &BTreeSet::new())
        );
        old_answer[i].0 = format!("x:1{}", i - 1);
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
}
#[test]
fn fill_simmilary_strings_simple() {
    let mut old_answer = BTreeSet::from_iter(
        vec![
            ("x:1".to_string(), "a".to_string()),
            ("x:2".to_string(), "b".to_string()),
            ("x:3".to_string(), "c".to_string()),
        ]
        .into_iter(),
    );
    let new_array = vec!["a".to_string(), "c".to_string(), "b".to_string()];
    let mut new_string_hashset = BTreeMap::from_iter(vec![
        ("a".to_string(), 1),
        ("b".to_string(), 1),
        ("c".to_string(), 1),
    ]);
    let t: Vec<(String, String)> = Vec::new();
    assert_eq!(
        &t,
        &fill_similar_strings(&new_array, &mut old_answer, &mut new_string_hashset)
    );
}
#[test]
fn alphabetical_sort_simple() {
    let mut old_json = BTreeSet::from_iter(
        vec![
            ("x:1".to_string(), "a".to_string()),
            ("x:2".to_string(), "b".to_string()),
            ("x:3".to_string(), "c".to_string()),
        ]
        .into_iter(),
    );
    let mut new_json = vec![
        ("x:1".to_string(), "a".to_string()),
        ("x:3".to_string(), "c".to_string()),
        ("x:2".to_string(), "b".to_string()),
    ];

    &alphabetical_sort(&mut new_json, &BTreeSet::from([1, 2, 3]));

    assert_eq!(
        &vec![
            ("x:1".to_string(), "a".to_string()),
            ("x:10".to_string(), "c".to_string()),
            ("x:2".to_string(), "b".to_string()),
        ],
        &new_json
    );
}
#[test]
fn alphabetical_sort_collision() {
    let mut old_json = BTreeSet::from_iter(
        vec![
            ("x:1".to_string(), "a".to_string()),
            ("x:10".to_string(), "d".to_string()),
            ("x:5".to_string(), "c".to_string()),
            ("x:8".to_string(), "b".to_string()),
        ]
        .into_iter(),
    );
    let mut new_json = vec![
        ("x:1".to_string(), "a".to_string()),
        ("x:5".to_string(), "c".to_string()),
        ("x:10".to_string(), "d".to_string()),
        ("x:8".to_string(), "b".to_string()),
    ];

    &alphabetical_sort(&mut new_json, &BTreeSet::from([1, 10, 5, 8]));

    assert_eq!(
        &vec![
            ("x:1".to_string(), "a".to_string()),
            ("x:2".to_string(), "c".to_string()),
            ("x:3".to_string(), "d".to_string()),
            ("x:8".to_string(), "b".to_string()),
        ],
        &new_json
    );
}
