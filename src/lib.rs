use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
    panic::PanicInfo,
    string,
};
use alphabet::*;
use strsim::jaro;

fn string_to_array(str: &str) -> Vec<String> {
    str.split("\n\n").map(|x| x.to_string()).collect()
}

fn similar(a: &str, b: &str) -> bool {
    jaro(a, b) > 0.7
}

fn fill_simmilary_strings(
    old_array: &Vec<(String, String)>,
    new_array: &Vec<String>,
    old_array_hashset: &mut HashSet<(String, String)>,
    new_string_hashset: &mut HashSet<String>,
) -> Vec<(String, String)> {
    let mut answer: Vec<(String, String)> = Vec::with_capacity(new_array.len());
    answer.resize(new_array.len(), (String::from(""), String::from("")));
    for (i, text) in new_array.iter().enumerate() {
        if let Some(str) = old_array_hashset.iter().find(|&x| similar(&x.1, &text)) {
            let str = str.clone();
            if new_string_hashset.contains(text) {
                answer[i] = (old_array_hashset.take(&str).unwrap().0, text.to_string());
                new_string_hashset.remove(text);
            }
        }
    }
    answer
}
fn get_unique_key(old_answer: &mut Vec<(String, String)>, pos: usize) -> String {
    let (x, y) = old_answer.split_at(pos);
    let left = x.iter().rev().find(|(k, _)| !k.is_empty()).map(|x| (x.0).clone());
    let right = y.iter().find(|(k, _)| !k.is_empty()).map(|x| (x.0).clone());
    match (left, right) {
        (Some(x), Some(y)) => x + &y,
        (Some(x), None) => x,
        (None, Some(y)) => y,
        (None, None) => "a".to_string(),
    }

}
fn fill_all_strings(
    old_array: &Vec<(String, String)>,
    new_array: &Vec<String>,
    old_array_hashset: &mut HashSet<(String, String)>,
    new_string_hashset: &mut HashSet<String>,
    old_answer: &mut Vec<(String, String)>,
) {
    for (i, text) in new_array.iter().enumerate() {
        if old_answer[i].1.is_empty() {
            old_answer[i] = (get_unique_key(old_answer, i), text.to_string());
            new_string_hashset.remove(text);
        }
    }
}


fn pipeline(old_array: Vec<(String, String)>, new_string: &str) -> Vec<(String, String)> {
    let new_string_array = string_to_array(new_string);
    let mut old_array_hashset: HashSet<(String, String)> =
        HashSet::from_iter(old_array.iter().cloned());
    let mut new_string_hashset: HashSet<String> =
        HashSet::from_iter(new_string_array.iter().cloned());
    let mut unsorted_partial_answer = fill_simmilary_strings(
        &old_array,
        &new_string_array,
        &mut old_array_hashset,
        &mut new_string_hashset,
    );
    fill_all_strings(&old_array, &new_string_array, &mut old_array_hashset, &mut new_string_hashset, &mut unsorted_partial_answer);
    alphabeticall_sort(&mut unsorted_partial_answer, Box::new(stupid_alphabet()));
    unsorted_partial_answer
}
fn stupid_alphabet() -> impl Iterator<Item = String> {
    alphabet!(SCREAM = "abcdefghijklmnopqrstuvwxyz");
    SCREAM.iter_words().skip(1 + 26 + 26 * 26)

}

fn alphabeticall_sort(array: &mut Vec<(String, String)>, mut iter: Box<dyn Iterator<Item=String>>) {
    for x in array {
        if !x.0.is_empty() {
            x.0 = iter.next().unwrap() + &x.0;
        }
    }
}

#[derive(Hash, Eq, Clone)]
struct SimString(String);
impl PartialEq for SimString {
    fn eq(&self, other: &Self) -> bool {
        jaro(&self.0, &other.0) > 0.7
    }
    fn ne(&self, other: &Self) -> bool {
        jaro(&self.0, &other.0) < 0.7
    }
}
#[cfg(test)]
mod tests {
    use std::fs::{self, read_to_string};

    use super::*;
    #[test]
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
    fn telegram_future() {
        let old_array = fs::read_to_string("tsts/future-generations.json").unwrap();
        let new_string = fs::read_to_string("tsts/future-generations.md").unwrap();
        let old_array: HashMap<String, String> = serde_json::from_str(&old_array).unwrap();
        let old_array: Vec<(String, String)> = old_array.into_iter().collect();
        println!("{:#?}", pipeline(old_array, &new_string));

    }
    #[test]
    fn telegram_career() {
        let old_array = fs::read_to_string("tsts/make-a-difference-with-your-career.json").unwrap();
        let new_string = fs::read_to_string("tsts/make-a-difference-with-your-career.md").unwrap();
        let old_array: HashMap<String, String> = serde_json::from_str(&old_array).unwrap();
        let old_array: Vec<(String, String)> = old_array.into_iter().collect();
        println!("{:#?}", pipeline(old_array, &new_string));

    }
    #[test]
    fn sim() {
        let old = "A lot more text to the god of text";
        let new = "More Text for the God of Text";
        assert!(jaro(old, new) > 0.7);
    }
}
