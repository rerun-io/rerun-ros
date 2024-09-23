use anyhow::{Error, Result};
use clap::Parser;
use rerun;
use rerun_ros::config::ConfigParser;
use rerun_ros::converters::ConverterRegistry;
use std::env;
use std::io::Cursor;
use std::slice;
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
    let context = rclrs::Context::new(env::args())?;
    let node = rclrs::create_node(&context, "rerun_ros_bridge")?;

    let converter_registry = Arc::new(ConverterRegistry::load_configuration());
    let config_parser = Arc::new(ConfigParser::new(&bridge_args.config_file)?);

    let rec = Arc::new(rerun::RecordingStreamBuilder::new("rerun_ros_bridge").connect()?);

    // Prevent the subscriptions from being dropped
    let mut _subscriptions = Vec::new();

    for ((topic_name, frame_id), (ros_type, entity_path)) in config_parser.conversions().clone() {
        let msg_spec = rerun_ros::ros_introspection::MsgSpec::new(&ros_type)?;
        println!(
            "Subscribing to topic: {topic_name} and frame_id {} with type: {ros_type}",
            frame_id.clone().unwrap_or("None".to_string()),
        );
        let rec = Arc::clone(&rec);
        let converter_registry = Arc::clone(&converter_registry);
        let generic_subscription = node.create_generic_subscription(
            &topic_name.clone(),
            &ros_type.clone(),
            rclrs::QOS_PROFILE_DEFAULT,
            move |msg: rclrs::SerializedMessage| {
                println!("Received message");
                let serialized_message = &msg.handle().get_rcl_serialized_message().lock().unwrap();
                let buffer = serialized_message.buffer;
                let buffer_length = serialized_message.buffer_length;
                // Wrap data in a CDR buffer
                let mut cdr_buffer =
                    Cursor::new(unsafe { slice::from_raw_parts(buffer, buffer_length) }.to_vec());
                if let Err(e) = converter_registry.process(
                    &rec,
                    &topic_name,
                    &frame_id,
                    &ros_type,
                    &entity_path,
                    &mut cdr_buffer,
                ) {
                    eprintln!("Error processing message: {e}");
                }
            },
        )?;
        _subscriptions.push(generic_subscription);
    }
    rclrs::spin(node).map_err(|err| err.into())
}
