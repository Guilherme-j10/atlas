use serde_json::Value;
use std::time::Instant;

#[derive(Debug)]
pub struct MessageProtocol {
    pub operation: u8,
    pub bytes: u32,
    pub payload: Value,
}

#[derive(Debug)]
pub enum ClientRole {
    Producer,
    Consumer,
}

#[derive(Debug)]
pub struct ConnectedClient {
    client_id: String,
    ip_address: String, //ip and port
    role: ClientRole,
    topic: String,
    partition: Option<usize>,
    connected_at: Instant,
    last_heartbeat: Instant,
    ack_enabled: bool,
    retries: u8,
}

