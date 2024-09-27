use crate::converters::builtin_interfaces;
use crate::converters::traits::Converter;
use anyhow::{Error, Result};
use cdr;
use rerun;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct CDRHeader {
    stamp: builtin_interfaces::CDRTime,
    frame_id: String,
}

// Converter for std_msgs/msg/Int8.msg
pub struct Int8Converter {}

impl Converter for Int8Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, i8, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/Int16.msg
pub struct Int16Converter {}

impl Converter for Int16Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, i16, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/Int32.msg
pub struct Int32Converter {}

impl Converter for Int32Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, i32, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/Int64.msg
pub struct Int64Converter {}

impl Converter for Int64Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, i64, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/Float32.msg
pub struct Float32Converter {}

impl Converter for Float32Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, f32, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/Float64.msg
pub struct Float64Converter {}

impl Converter for Float64Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, f64, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/UInt8.msg
pub struct UInt8Converter {}

impl Converter for UInt8Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, u8, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/UInt16.msg
pub struct UInt16Converter {}

impl Converter for UInt16Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, u16, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/UInt32.msg
pub struct UInt32Converter {}

impl Converter for UInt32Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, u32, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}

// Converter for std_msgs/msg/UInt64.msg
pub struct UInt64Converter {}

impl Converter for UInt64Converter {
    fn convert(
        &self,
        rec: &Arc<rerun::RecordingStream>,
        _topic: &str,
        _frame_id: &Option<String>,
        entity_path: &str,
        cdr_buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        let value = cdr::deserialize_from::<_, u64, _>(cdr_buffer, cdr::Infinite)?;
        rec.log(entity_path, &rerun::Scalar::new(value as f64))?;
        Ok(())
    }
}
