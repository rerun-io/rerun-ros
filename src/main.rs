use std::env;

use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "minimal_subscriber")?;

    let generic_subscription = node.create_generic_subscription(
        "topic",
        "std_msgs/msg/String",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg: rclrs::SerializedMessage| {
            // Process message and pass it to rerun
            println!("Serialized message: {:?}", msg);
        },
    )?;

    rclrs::spin(node).map_err(|err| err.into())
}
