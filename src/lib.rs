use clap::Parser;
use colored::Colorize;
use log::*;
use serde_json::Map;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{read_to_string, File};
use std::num::IntErrorKind;
use std::path::PathBuf;
use strsim::jaro;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    pub old_json: PathBuf,
    #[arg(long)]
    pub new_string: PathBuf,

    #[arg(long = "new-json", value_name = "NEW_JSON_NAME")]
    pub new_json_name: PathBuf,
    #[arg(long = "new-keys", value_name = "NEW_USED_KEYS_NAME")]
    pub new_used_keys_name: PathBuf,

    #[arg(short, long, value_name = "USED_KEYS")]
    pub used_keys: Option<PathBuf>,
    #[arg(short, long)]
    pub logging_level: Option<bool>,
}

pub fn run(config: Config) {
    let new_string = read_to_string(&config.new_string).unwrap();

    let used_keys = match config.used_keys {
        Some(x) => read_to_string(x).unwrap(),
        None => String::from(""),
    };
    let used_keys: Vec<String> = serde_json::from_str(&used_keys).unwrap_or_default();

    let old_array = read_to_string(&config.old_json).unwrap();
    let old_array: BTreeMap<String, String> = serde_json::from_str(&old_array).unwrap();
    let old_array: Vec<(String, String)> = old_array.into_iter().collect();

    println!("ran with {:?} {:?} {:?}", old_array, used_keys, &new_string);
    let (final_json, unused_keys) = pipeline(old_array, used_keys, &new_string);
    println!("got back {:?} {:?}", final_json, unused_keys);

    let mut result = Map::new();
    for (k, v) in final_json.into_iter() {
        result.insert(k, v.into());
    }

    serde_json::to_writer_pretty(File::create(config.new_json_name).unwrap(), &result).unwrap();
    serde_json::to_writer(
        File::create(config.new_used_keys_name).unwrap(),
        &unused_keys,
    )
    .unwrap();
}

#[cfg(test)]
use rand::{thread_rng, Rng};

#[cfg(test)]
const TEST_STRINGS: [&str; 9] = [
    "Blala", "Bol bol", "Balaba", "Kalaba", "Ka", "Raba", "Dabara", "Taba", "Mutaba",
];

#[cfg(test)]
pub fn gen_line(length: usize) -> String {
    fn gen_word() -> String {
        let mut rng = thread_rng();
        let f = rng.gen_range(0..(TEST_STRINGS.len()));
        TEST_STRINGS[f].to_string()
    }
    (0..length)
        .map(|_| gen_word())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
pub fn test_random_inserts() {
    let mut state: Vec<(String, String)> = vec![("mow:0".to_string(), "initial".to_string())];
    let mut keys: Vec<String> = vec![];
    let mut text_parts: Vec<String> = vec!["initial".to_string()];

    let mut rng = thread_rng();

    for _ in 0..100000 {
        match rng.gen_range(0..=4) {
            0 | 1 => {
                let index = rng.gen_range(0..=text_parts.len());
                let text = gen_line(12).to_string();
                println!("insert: line {index}, text {text}");
                text_parts.insert(index, text);
            }
            2 | 3 => {
                if text_parts.len() > 2 {
                    let index = rng.gen_range(0..text_parts.len());
                    text_parts.remove(index);
                    println!("delete: line {index}");
                } else {
                    println!("noop: tried deleting, but there's nothing left");
                }

            }
            _ => {
                println!("noop");
                // Doing literally nothing is a viable option, but won't give you dignity points.
            }
        }

        (state, keys) = pipeline(state, keys, text_parts.join("\n\n").as_str());
        println!("{} {}", state.len(), keys.len());
        println!("{keys:?}");
    }
}

#[test]
pub fn test_trivial_inserts() {
    let mut state: Vec<(String, String)> = vec![("mow:0".to_string(), "initial".to_string())];
    let mut keys: Vec<String> = vec![];
    let mut text_parts: Vec<String> = vec!["initial".to_string()];

    for _ in 0..100000 {
        let text = gen_line(12).to_string();
        text_parts.insert(1, text);
        (state, keys) = pipeline(state, keys, text_parts.join("\n\n").as_str());
        text_parts.remove(1);
        (state, keys) = pipeline(state, keys, text_parts.join("\n\n").as_str());
        println!("{} {}", state.len(), keys.len());
    }
}

pub fn pipeline(
    old_array: Vec<(String, String)>,
    used_keys: Vec<String>,
    new_string: &str,
) -> (Vec<(String, String)>, Vec<String>) {
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

    // dbg!(&unsorted_partial_answer);
    let old_keys = BTreeSet::from_iter(
        old_array
            .iter()
            .map(|(x, _)| get_key_number(x).unwrap())
            .chain(used_keys.iter().map(|x| get_key_number(x).unwrap())),
    );

    fill_all_strings(
        &old_keys,
        &new_string_array,
        &mut new_string_hashset,
        &mut unsorted_partial_answer,
    );
    // dbg!(&unsorted_partial_answer);
    // dbg!(&old_keys[]);
    alphabetical_sort(&mut unsorted_partial_answer, &old_keys);

    let _new_unused_keys = old_array_hashset
        .into_iter()
        .map(|(x, _)| x)
        .collect::<Vec<String>>();

    trace!("new unused keys are {:?}", &_new_unused_keys);
    let old_array_hashset: Vec<String> = _new_unused_keys
        .into_iter()
        .chain(used_keys.into_iter())
        .collect::<Vec<String>>();
    (unsorted_partial_answer, old_array_hashset)
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
    let mut log_count_inserted_strings = 0usize;
    let log_len = old_array_hashset.len();
    let mut answer: Vec<(String, String)> = Vec::with_capacity(new_array.len());
    answer.resize(new_array.len(), (String::from(""), String::from("")));

    for (i, text) in new_array.iter().enumerate() {
        if let Some(str) = old_array_hashset.iter().find(|&x| similar(&x.1, &text)) {
            log_count_inserted_strings += 1;
            let str = str.clone();
            trace!(
                "old string {} and new string {} with key {} are similar",
                &text.yellow(),
                &str.1.yellow(),
                &str.0.green()
            );
            if *new_string_hashset.get(text).unwrap() > 0 {
                answer[i] = (old_array_hashset.take(&str).unwrap().0, text.to_string());
                new_string_hashset.get_mut(text).map(|x| *x -= 1);
            }
        } else {
            info!("new string {} has no new similar strings", &text.red());
        }
    }
    info!(
        "{} old strings out of {} inserted",
        log_count_inserted_strings, log_len
    );
    answer
}
fn fill_all_strings(
    old_keys: &BTreeSet<usize>,
    new_array: &Vec<String>,
    new_string_hashset: &mut BTreeMap<String, usize>,
    old_answer: &mut Vec<(String, String)>,
) {
    for (i, text) in new_array.iter().enumerate() {
        if old_answer[i].1.is_empty() && old_answer[i].0.is_empty() {
            old_answer[i] = (get_unique_key(old_answer, i, old_keys), text.to_string());
            info!(
                "inserting new string {} with new unique key {}",
                old_answer[i].1.yellow(),
                old_answer[i].0.green()
            );
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
    let left = x.iter().rev().find(|(k, _)| !k.is_empty()).map(|x| &x.0);
    let right = y.iter().skip(1).find(|(k, _)| !k.is_empty()).map(|x| &x.0);
    // dbg!(&left, &right);
    match (left, right) {
        (Some(x), Some(y)) => {
            let left_key = get_key_number(&x).unwrap();
            let mut right_key = get_key_number(&y).unwrap();
            while left_key > right_key {
                right_key *= 10;
            }

            // dbg!(left_key, right_key);
            for i in left_key + 1..right_key {
                if !old_keys.contains(&(i as usize)) {
                    let diff = (i - left_key).try_into().unwrap();
                    return change_key_number(x.clone(), diff, 0).unwrap();
                }
            }
            let mut base = (left_key + right_key) * 5 - left_key;
            loop {
                if !old_keys.contains(&(base + left_key)) {
                    return match change_key_number(x.clone(), base as i32, 1) {
                        Ok(string) => string,
                        Err(_) => y.clone(),
                    };
                } else {
                    base += 1;
                }
            }
        }
        (Some(x), None) => change_key_number(x.clone(), 1, 0).unwrap(),
        (None, Some(y)) => change_key_number(y.clone(), 0, 0).unwrap(),
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

fn change_key_number(key: String, df: i32, len: usize) -> Result<String, IntErrorKind> {
    let len = key.split(":").last().unwrap().len() + len;
    let new_number =
        add_usize_i32(get_key_number(&key).unwrap(), df).ok_or(IntErrorKind::InvalidDigit)?;
    let key = key.trim_end_matches(|x: char| x.is_digit(10)).to_string();
    Ok(key + &format!("{:0len$}", new_number))
}

pub fn alphabetical_sort(array: &mut Vec<(String, String)>, old_keys: &BTreeSet<usize>) {
    for i in 0..array.len() {
        match (
            array.get(i.checked_sub(1).unwrap_or(usize::MAX)),
            array.get(i + 1),
        ) {
            (Some((x, _)), Some((y, _))) => {
                if !(x < &array[i].0 && &array[i].0 < y) {
                    let tmp = get_unique_key(array, i, old_keys);
                    info!(
                        "changing old_key {} to new key {}",
                        &array[i].0.yellow(),
                        &tmp.green()
                    );
                    array[i].0 = tmp;
                }
            }
            (None, Some((y, _))) => {
                if !(&array[i].0 < y) {
                    let tmp = get_unique_key(array, i, old_keys);
                    info!(
                        "changing old_key {} to new key {}",
                        &array[i].0.yellow(),
                        &tmp.green()
                    );
                    array[i].0 = tmp;
                }
            }
            (Some((x, _)), None) => {
                if !(x < &array[i].0) {
                    let tmp = get_unique_key(array, i, old_keys);
                    info!(
                        "changing old_key {} to new key {}",
                        &array[i].0.yellow(),
                        &tmp.green()
                    );
                    array[i].0 = tmp;
                }
            }
            (None, None) => {}
        }
    }
}
