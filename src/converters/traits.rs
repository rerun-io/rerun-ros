use anyhow::{Error, Result};
use rerun;
use std::io::Cursor;
use std::sync::Arc;

/// A trait for converting ROS messages (as CDR binary blobs) into the rerun system.
///
/// This trait requires implementors to be thread-safe (`Send` and `Sync`).
///
/// # Methods
///
/// ## `convert`
///
/// Converts a message into the desired format.
///
/// ### Parameters
/// - `rec`: A reference to a `RecordingStream` from the `rerun` crate.
/// - `topic`: The topic associated with the message.
/// - `frame_id`: An optional frame identifier.
/// - `entity_path`: The path to the entity.
/// - `message`: A mutable cursor pointing to a vector of bytes representing the message.
///
/// ### Returns
/// - `Result<(), Error>`: Returns `Ok(())` if the conversion is successful, otherwise returns an `Error`.
pub trait Converter: Send + Sync {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        topic: &str,
        frame_id: &Option<String>,
        entity_path: &str,
        message: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error>;
}
