use std::env;

use anyhow::{Error, Result};

fn main() -> Result<(), Error> {
    let context = rclrs::Context::new(env::args())?;

    let node = rclrs::create_node(&context, "minimal_subscriber")?;

    rclrs::spin(node).map_err(|err| err.into())
}
