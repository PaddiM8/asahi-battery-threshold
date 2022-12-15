#[macro_use]
extern crate log;

mod battery_handling;
mod config;

use std::env;

use color_eyre::eyre::{eyre, Result};
use config::Config;
use env_logger::{Builder, Env, Target};

fn main() -> Result<()> {
    color_eyre::install()?;

    setup_logger();
    info!("Initializing");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(eyre!("expected exactly 1 argument (config file path)"));
    }

    let config = Config::read_from_file(&args[1])?;
    info!("Loaded config file");
    battery_handling::start_handler(config)?;

    Ok(())
}

fn setup_logger() {
    let env = Env::default().filter_or("CHARGE_LOG_LEVEL", "info");
    let mut builder = Builder::from_env(env);
    builder.target(Target::Stdout);
    builder.init();
}
