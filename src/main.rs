use std::env;
use stroki::{run, Config};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();
    let path = if args.get(3).is_none() {
        std::fs::File::create("new-".to_string() + config.old_json.split("/").last().unwrap())
            .unwrap()
    } else {
        std::fs::File::create(args[3].to_string()).unwrap()
    };
    run(config, path);
}
