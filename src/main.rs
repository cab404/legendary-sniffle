use clap::Parser;
use diffhorror::{run, Config};
use simplelog::*;

fn main() {
    let config = Config::parse();
    let ll = if config.logging_level.is_some() {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    TermLogger::init(
        ll,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    run(config);
}
