use alphabet::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::read_to_string;
use strsim::jaro;

pub struct Config {
    pub old_json: String,
    pub new_string: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let old_json = args[1].clone();
        let new_string = args[2].clone();

        Ok(Config {
            old_json,
            new_string,
        })
    }
}

pub fn run(config: Config) {
    let old_array = read_to_string(&config.old_json).unwrap();
    let new_string = read_to_string(&config.new_string).unwrap();

    let old_array: BTreeMap<String, String> = serde_json::from_str(&old_array).unwrap();
    let old_array: Vec<(String, String)> = old_array.into_iter().collect();

    let final_json = pipeline(old_array, &new_string);

    serde_json::to_writer_pretty(
        std::fs::File::create("new-".to_string() + config.old_json.split("/").last().unwrap())
            .unwrap(),
        &final_json,
    )
    .unwrap();
}

pub fn pipeline(old_array: Vec<(String, String)>, new_string: &str) -> Vec<(String, String)> {
    let new_string_array = string_to_array(new_string);

    let mut old_array_hashset: BTreeSet<(String, String)> =
        BTreeSet::from_iter(old_array.iter().cloned());
    let mut new_string_hashset: BTreeSet<String> =
        BTreeSet::from_iter(new_string_array.iter().cloned());

    let mut unsorted_partial_answer = fill_simmilary_strings(
        &new_string_array,
        &mut old_array_hashset,
        &mut new_string_hashset,
    );
    //dbg!(&old_array);
    //dbg!(&new_string_array);
    dbg!(&old_array_hashset);
    dbg!(&new_string_hashset);
    dbg!(&unsorted_partial_answer);
    fill_all_strings(
        &new_string_array,
        &mut new_string_hashset,
        &mut unsorted_partial_answer,
    );
    //dbg!(&unsorted_partial_answer);
    //alphabeticall_sort(&mut unsorted_partial_answer, Box::new(stupid_alphabet()));
    unsorted_partial_answer
}
fn string_to_array(str: &str) -> Vec<String> {
    str.split("\n\n").map(|x| x.to_string()).collect()
}

fn similar(a: &str, b: &str) -> bool {
    jaro(a, b) > 0.9
}

fn fill_simmilary_strings(
    new_array: &Vec<String>,
    old_array_hashset: &mut BTreeSet<(String, String)>,
    new_string_hashset: &mut BTreeSet<String>,
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
fn fill_all_strings(
    new_array: &Vec<String>,
    new_string_hashset: &mut BTreeSet<String>,
    old_answer: &mut Vec<(String, String)>,
) {
    for (i, text) in new_array.iter().enumerate() {
        if old_answer[i].1.is_empty() {
            old_answer[i] = (get_unique_key(old_answer, i), text.to_string());
            new_string_hashset.remove(text);
        }
    }
}

fn get_unique_key(old_answer: &mut Vec<(String, String)>, pos: usize) -> String {
    let (x, y) = old_answer.split_at(pos);
    let left = x
        .iter()
        .rev()
        .find(|(k, _)| !k.is_empty())
        .map(|x| (x.0).clone());
    let right = y.iter().find(|(k, _)| !k.is_empty()).map(|x| (x.0).clone());
    match (left, right) {
        (Some(x), Some(y)) => {
            if get_key_number(&x).unwrap() != get_key_number(&y).unwrap() - 1 {
                change_key_number(x, 1)
            } else {
                panic!()
            }
        }
        (Some(x), None) => change_key_number(x, 1),
        (None, Some(y)) => change_key_number(y, -1),
        (None, None) => "a".to_string(),
    }
}
fn get_key_number(key: &str) -> Result<usize, std::num::ParseIntError> {
    key.split(":").last().unwrap().parse::<usize>()
}

pub fn add_usize_i32(x: usize, y: i32) -> Option<usize> {
    if y.is_negative() {
        x.checked_sub(y.wrapping_abs() as usize)
    } else {
        x.checked_add(y as usize)
    }
}

fn change_key_number(key: String, df: i32) -> String {
    dbg!(&key);
    let new_number = add_usize_i32(get_key_number(&key).unwrap(), df).unwrap();
    let key = key.trim_end_matches(|x: char| x.is_digit(10)).to_string();
    key + &format!("{:03}", new_number)
}

fn stupid_alphabet() -> impl Iterator<Item = String> {
    alphabet!(SCREAM = "abcdefghijklmnopqrstuvwxyz");
    SCREAM.iter_words().skip(1 + 26 + 26 * 26)
}

fn alphabeticall_sort(
    array: &mut Vec<(String, String)>,
    mut iter: Box<dyn Iterator<Item = String>>,
) {
    for x in array {
        if !x.0.is_empty() {
            x.0 = iter.next().unwrap() + &x.0;
        }
    }
}
