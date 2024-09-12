use anyhow::{Error, Result};
use clap::Parser;
use rerun_ros::config::ConfigParser;

/// A bridge between rerun and ROS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct BridgeArgs {
    /// Path to the configuration file in TOML format
    #[arg(short, long)]
    config_file: String,
}

fn main() -> Result<(), Error> {
    let bridge_args = BridgeArgs::parse();

    if bridge_args.config_file.is_empty() {
        return Ok(());
    }

    println!("Starting bridge");
    let config_parser = ConfigParser::new(&bridge_args.config_file)?;
    println!("{:?}", config_parser.conversions());
    Ok(())
}
