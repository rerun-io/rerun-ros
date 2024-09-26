use crate::converters::traits::Converter;
use anyhow::{Error, Result};
use cdr;
use rerun;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct CDRTime {
    sec: i32,
    nanosec: u32,
}
