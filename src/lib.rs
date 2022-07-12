use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
    panic::PanicInfo,
};
use strsim::jaro;

fn string_to_array(str: &str) -> Vec<String> {
    str.split("\n\n").map(|x| x.to_string()).collect()
}

fn similar(a: &str, b: &str) -> bool {
    jaro(a, b) > 0.7
}

fn fill_simmilary_strings(
    old_array: Vec<(String, String)>,
    new_array: Vec<String>,
) -> Vec<(String, String)> {
    let mut answer: Vec<(String, String)> = Vec::with_capacity(new_array.len());
    answer.resize(new_array.len(), (String::from(""), String::from("")));
    let mut hashset: HashSet<(String, String)> = HashSet::from_iter(old_array.iter().cloned());
    for (i, text) in new_array.iter().enumerate() {
        if let Some(str) = hashset.iter().find(|&x| similar(&x.1, &text)) {
            let str = str.clone();
            answer[i] = (hashset.take(&str).unwrap().0, text.clone());
        }
    }
    answer
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
    use super::*;
    #[test]
    fn telegram() {
        let old_json = r#"{"a":"Text Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua","c":"More Text for the God of Text"}"#;
        let old_array: HashMap<String, String> = serde_json::from_str(old_json).unwrap();
        let old_array: Vec<(String, String)> = old_array.into_iter().collect();
        let new_string = "Text 123 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua

    Additional string
    With a single \n
    
    A lot more text to the god of text";
        let new_array = r#"{"a":"Text 123 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua","b":"Additional string\nWith a single \\n","c":"A lot more text to the god of text"}"#;
        assert_eq!(1, 2);
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
        let mut answer = old_array.clone(); answer.swap(0, 1); answer.swap(1, 2);
        assert_eq!(&answer, &fill_simmilary_strings(
            old_array,
            string_to_array(&new_string)));
    }
    #[test]
    fn simple_with_mistakes() {
        let old_string = "Мама\n\nМыла\n\nРаму\n\n";
        let new_string = "Мыма\n\nРау\n\nама";
        let old_array: Vec<(String, String)> = vec![
            ("a".to_string(), "Мама".to_string()),
            ("b".to_string(), "Мыла".to_string()),
            ("c".to_string(), "Раму".to_string()),
        ];
        let answer: Vec<(String, String)> = vec![
            ("b".to_string(), "Мыма".to_string()),
            ("c".to_string(), "Рау".to_string()),
            ("a".to_string(), "ама".to_string()),
        ];
        assert_eq!(&answer, &fill_simmilary_strings(
            old_array,
            string_to_array(&new_string)));
    }
    #[test]
    fn sim() {
        let old = "A lot more text to the god of text";
        let new = "More Text for the God of Text";
        assert_eq!(jaro(old, new).to_string(), "2");
    }
    #[test]
    fn alpha() {
        let old = "A lot more text to the god of text";
        let new = "More Text for the God of Text";
        assert!("aab" < "ac");
    }
}
