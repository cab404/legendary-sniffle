use stroki::{run, Config};

fn main() {
    let config = Config{ old_json: "./examples/future-generations.json".to_string(), new_string: "./examples/future-generations.md".to_string()};
    let path = std::fs::File::create("./examples/new-".to_string() + config.old_json.split("/").last().unwrap())
            .unwrap();
    run(config, path);
}