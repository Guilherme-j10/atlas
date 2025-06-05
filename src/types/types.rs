use serde_json::Value;

#[derive(Debug)]
pub struct MessageProtocol {
    pub operation: u8,
    pub bytes: u32,
    pub payload: Value,
}