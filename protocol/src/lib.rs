use std::time::Duration;

use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct Audit {
    pub timestamp: Duration,
    pub code: u32,
    pub success: bool,
}

#[derive(Debug, Encode, Decode)]
pub enum UserAction {
    Add(u32),
    Del(u32),
}
