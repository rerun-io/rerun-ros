use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct CDRTime {
    sec: i32,
    nanosec: u32,
}
