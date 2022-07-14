use std::{collections::HashMap, fs};
use stroki::{pipeline, Config, run};
use strsim::jaro;
#[test]
fn future_cut() {
    let config = Config::new(&[ "".to_string(),
        "tsts/future-generations-tail.json".to_string(),
        "tsts/future-generations-tail.md".to_string(),
    ]).unwrap();
    run(config);
}

#[test]
#[ignore]
fn telegram_simple() {
    let old_json = r#"{"a":"Text Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua","c":"More Text for the God of Text"}"#;
    let old_array: HashMap<String, String> = serde_json::from_str(old_json).unwrap();
    let old_array: Vec<(String, String)> = old_array.into_iter().collect();
    let new_string = "Text 123 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua

    Additional string
    With a single \n
    
    A lot more text to the god of text";
    let new_array = r#"{"a":"Text 123 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua","b":"Additional string\nWith a single \\n","c":"A lot more text to the god of text"}"#;
    println!("{:#?}", &pipeline(old_array, new_string));
}
#[test]
#[ignore]
fn simple() {
    let old_string = "Мама\n\nМыла\n\nРаму\n\n";
    let new_string = "Мыла\n\nРаму\n\nМама";
    let old_array: Vec<(String, String)> = vec![
        ("a".to_string(), "Мама".to_string()),
        ("b".to_string(), "Мыла".to_string()),
        ("c".to_string(), "Раму".to_string()),
    ];
    let mut answer = old_array.clone();
    answer.swap(0, 1);
    answer.swap(1, 2);
    assert_eq!(&answer, &pipeline(old_array, new_string));
}
#[test]
#[ignore]
fn simple_with_mistakes() {
    let old_string = "Мама\n\nМыла\n\nРаму\n\n";
    let new_string = "мыла\n\nРау\n\nМам";
    let old_array: Vec<(String, String)> = vec![
        ("a".to_string(), "Мама".to_string()),
        ("b".to_string(), "Мыла".to_string()),
        ("c".to_string(), "Раму".to_string()),
    ];
    let answer: Vec<(String, String)> = vec![
        ("b".to_string(), "мыла".to_string()),
        ("c".to_string(), "Рау".to_string()),
        ("a".to_string(), "Мам".to_string()),
    ];
    assert_eq!(&answer, &pipeline(old_array, new_string));
}
#[test]
#[ignore]
fn simple_with_mistakes_and_new_strings() {
    let old_string = "Мама\n\nМыла\n\nРаму\n\n";
    let new_string = "мыла\n\nКорыто\n\nРау\n\nМам\n\nРепа";
    let old_array: Vec<(String, String)> = vec![
        ("a".to_string(), "Мама".to_string()),
        ("b".to_string(), "Мыла".to_string()),
        ("c".to_string(), "Раму".to_string()),
    ];
    let answer: Vec<(String, String)> = vec![
        ("b".to_string(), "мыла".to_string()),
        ("".to_string(), "".to_string()),
        ("c".to_string(), "Рау".to_string()),
        ("a".to_string(), "Мам".to_string()),
        ("".to_string(), "".to_string()),
    ];
    assert_eq!(&answer, &pipeline(old_array, new_string));
}
#[test]
#[ignore]
fn alphabet_simple() {
    let old_string = "Мама\n\nРаму\n\nМыла\n\n";
    let new_string = "мыла\n\nКорыто\n\nРау\n\nМам\n\nРепа";
    let old_array: Vec<(String, String)> = vec![
        ("a".to_string(), "Мама".to_string()),
        ("b".to_string(), "Мыла".to_string()),
        ("c".to_string(), "Раму".to_string()),
    ];
    println!("{:?}", &pipeline(old_array, new_string));
}
#[test]
#[ignore]
fn telegram_future() {
    let old_array = std::fs::read_to_string("tsts/future-generations.json").unwrap();
    let new_string = std::fs::read_to_string("tsts/future-generations.md").unwrap();
    let old_array: HashMap<String, String> = serde_json::from_str(&old_array).unwrap();
    let old_array: Vec<(String, String)> = old_array.into_iter().collect();
    serde_json::to_writer_pretty(
        std::fs::File::create("new-future-generations.json").unwrap(),
        &pipeline(old_array, &new_string),
    );
}
#[test]
#[ignore]
fn telegram_career() {
    let old_array = fs::read_to_string("tsts/make-a-difference-with-your-career.json").unwrap();
    let new_string = fs::read_to_string("tsts/make-a-difference-with-your-career.md").unwrap();
    let old_array: HashMap<String, String> = serde_json::from_str(&old_array).unwrap();
    let old_array: Vec<(String, String)> = old_array.into_iter().collect();
    serde_json::to_writer_pretty(
        std::fs::File::create("new-make-a-difference-with-your-career.json").unwrap(),
        &pipeline(old_array, &new_string),
    );
}
#[test]
#[ignore]
fn sim() {
    let old = "A lot more text to the god of text";
    let new = "More Text for the God of Text";
    assert!(jaro(old, new) > 0.7);
}
