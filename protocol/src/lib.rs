use std::time::SystemTime;

use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum CodeType {
    Pin,
    Fob,
}

#[derive(Debug, Encode, Decode)]
pub struct Audit {
    pub timestamp: SystemTime,
    pub code: String,
    pub code_type: CodeType,
    pub success: bool,
}

#[derive(Debug, Encode, Decode)]
pub enum UserAction {
    Add(String),
    Del(String),
}
