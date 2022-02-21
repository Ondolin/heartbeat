use std::time::{Duration, Instant};

pub struct ServiceInfo {
    pub last_heartbeat: Instant,
    pub is_offline: bool,
    pub timeout: Duration,
}