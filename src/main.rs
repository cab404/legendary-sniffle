use std::env;
use stroki::{run, Config};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();
    run(config);
}
