use alphabet::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::read_to_string;
use strsim::{jaro, levenshtein};

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
    let mut new_string_hashset: BTreeMap<String, usize> = BTreeMap::new();
    for x in &new_string_array {
        new_string_hashset
            .entry(x.to_string())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    let mut unsorted_partial_answer = fill_similar_strings(
        &new_string_array,
        &mut old_array_hashset,
        &mut new_string_hashset,
    );
    dbg!(&unsorted_partial_answer);
    let old_keys = BTreeSet::from_iter(
        old_array_hashset
            .iter()
            .map(|(x, _)| get_key_number(x).unwrap()),
    );
    fill_all_strings(
        &old_keys,
        &new_string_array,
        &mut new_string_hashset,
        &mut unsorted_partial_answer,
    );
    alphabetical_sort(&mut unsorted_partial_answer, &old_keys);
    unsorted_partial_answer
}
fn string_to_array(str: &str) -> Vec<String> {
    str.split("\n\n").map(|x| x.to_string()).collect()
}

fn similar(a: &str, b: &str) -> bool {
    jaro(a, b) > 0.8
    //levenshtein(a, b) < std::cmp::max(a.len(), b.len()) * 3 / 2 - std::cmp::min(a.len(), b.len())
}

pub fn fill_similar_strings(
    new_array: &Vec<String>,
    old_array_hashset: &mut BTreeSet<(String, String)>,
    new_string_hashset: &mut BTreeMap<String, usize>,
) -> Vec<(String, String)> {
    let mut answer: Vec<(String, String)> = Vec::with_capacity(new_array.len());
    answer.resize(new_array.len(), (String::from(""), String::from("")));
    for (i, text) in new_array.iter().enumerate() {
        if let Some(str) = old_array_hashset.iter().find(|&x| similar(&x.1, &text)) {
            dbg!(&text);
            let str = str.clone();
            if *new_string_hashset.get(text).unwrap() > 0 {
                answer[i] = (old_array_hashset.take(&str).unwrap().0, text.to_string());
                new_string_hashset.get_mut(text).map(|x| *x -= 1);
            }
        }
    }
    answer
}
fn fill_all_strings(
    old_keys: &BTreeSet<usize>,
    new_array: &Vec<String>,
    new_string_hashset: &mut BTreeMap<String, usize>,
    old_answer: &mut Vec<(String, String)>,
) {
    for (i, text) in new_array.iter().enumerate() {
        if old_answer[i].1.is_empty() {
            old_answer[i] = (get_unique_key(old_answer, i, old_keys), text.to_string());
            new_string_hashset.get_mut(text).map(|x| *x -= 1);
        }
    }
}

pub fn get_unique_key(
    old_answer: &mut Vec<(String, String)>,
    pos: usize,
    old_keys: &BTreeSet<usize>,
) -> String {
    let (x, y) = old_answer.split_at(pos);
    let left = x
        .iter()
        .rev()
        .find(|(k, _)| !k.is_empty())
        .map(|x| (x.0).clone());
    let right = y.iter().skip(1).find(|(k, _)| !k.is_empty()).map(|x| (x.0).clone());
    dbg!(&left, &right);
    match (left, right) {
        (Some(x), Some(y)) => {
            let mut left_key = get_key_number(&x).unwrap();
            let right_key = get_key_number(&y).unwrap();
            for i in left_key + 1..right_key {
                if !old_keys.contains(&(i as usize)) {
                    let diff = (i - left_key).try_into().unwrap();
                    return change_key_number(x, diff);
                }
            }
            let mut i = if left_key < right_key {
                left_key * 10
            } else {
                //left_key = right_key;
                left_key
            };
            loop {
                for j in i..i + 10 {
                    dbg!(&j);
                    if left_key.to_string() < j.to_string()
                        && j.to_string() < right_key.to_string()
                        && !old_keys.contains(&j)
                    {
                        let diff = (j - left_key).try_into().unwrap();
                        return change_key_number(x, diff);
                    }
                }
                i *= 10;
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
    let len = key.split(":").last().unwrap().len();
    let new_number = add_usize_i32(get_key_number(&key).unwrap(), df).unwrap();
    let key = key.trim_end_matches(|x: char| x.is_digit(10)).to_string();
    key + &format!("{:0len$}", new_number)
}

fn stupid_alphabet() -> impl Iterator<Item = String> {
    alphabet!(SCREAM = "abcdefghijklmnopqrstuvwxyz");
    SCREAM.iter_words().skip(1 + 26 + 26 * 26)
}

pub fn alphabetical_sort(array: &mut Vec<(String, String)>, old_keys: &BTreeSet<usize>) {
    for i in 0..array.len() {
        dbg!(&array[i]);
        match (array.get(i.checked_sub(1).unwrap_or(usize::MAX)), array.get(i + 1)) {
            (Some((x, _)), Some((y, _))) => {
                if !(x < &array[i].0 && &array[i].0 < y) {
                    array[i].0 = get_unique_key(array, i, old_keys);
                }
            }
            (None, Some((y, _))) => {
                if !(&array[i].0 < y) {
                    array[i].0 = get_unique_key(array, i, old_keys);
                }
            }
            (Some((x, _)), None) => {
                if !(x < &array[i].0) {
                    array[i].0 = get_unique_key(array, i, old_keys);
                }
            }
            (None, None) => {}
        }
    }
}
