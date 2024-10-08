use anyhow::{Error, Result};
use clap::Parser;
use rerun_ros::config::ConfigParser;
use std::env;
use std::sync::Arc;

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

    let context = rclrs::Context::new(env::args())?;
    let node = rclrs::create_node(&context, "rerun_ros_bridge")?;
    // Clippy does not like iterating over the keys of a HashMap, so we collect it into a Vec
    let config_entries: Vec<_> = config_parser.conversions().iter().collect();

    // Prevent the subscriptions from being dropped
    let mut _subscriptions = Vec::new();
    for ((topic_name, _frame_id), (ros_type, _entity_path)) in config_entries {
        let msg_spec = rerun_ros::ros_introspection::MsgSpec::new(ros_type)?;

        println!("Subscribing to topic: {topic_name} with type: {ros_type}");
        let _generic_subscription = node.create_generic_subscription(
            topic_name,
            ros_type,
            rclrs::QOS_PROFILE_DEFAULT,
            move |_msg: rclrs::SerializedMessage| {
                let _msg_spec = Arc::new(&msg_spec);
                // Process message and pass it to rerun
            },
        )?;
        _subscriptions.push(_generic_subscription);
    }
    Ok(())
}
