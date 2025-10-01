use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DATA_COLLECTOR_ADDR: &str = "127.0.0.1:8080";
const MAGIC_NUMBER: u64 = 0x1234567890ABCDEF;
const VERSION_NUMBER: u64 = 1;

fn unix_now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u64
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommand {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        avg_cpu_usage: f32,
    }
}

pub fn encode_v1(command: CollectorCommand) -> Vec<u8> {
    let json = serde_json::to_string(&command).unwrap();
    let json_bytes = json.as_bytes();
    let crc = crc32fast:
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
