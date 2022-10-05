use clap::Parser;
use diffhorror::{run, Config};

fn main() {
    let config = Config::parse();
    run(config);
}
