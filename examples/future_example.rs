use std::path::PathBuf;
use simplelog::*;
use diffhorror::{run, Config};

fn main() {
    let config = Config {
        old_json: PathBuf::from("examples/future-generations.json.old"),
        new_string: PathBuf::from("examples/future-generations.md"),
        new_json_name: PathBuf::from("examples/x.json"),
        new_used_keys_name: PathBuf::from("examples/y.json"),
        used_keys: Some(PathBuf::from("examples/future-generations.usedKeys.json")),
        logging_level: None,
    };
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    run(config);
}
