use anyhow::{Error, Result};
use rerun;
use std::io::Cursor;
use std::sync::Arc;

pub trait Converter: Send + Sync {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        message: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error>;
}
