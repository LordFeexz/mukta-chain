use crate::utils::generate_id::generate_uuid;
use bcrypt::{hash, DEFAULT_COST};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Block {
    pub id: String,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

impl Block {
    #[allow(dead_code)]
    pub fn new(previous_hash: String, data: String) -> Self {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let id = generate_uuid();
        Block {
            id: id.clone(),
            timestamp: time_stamp,
            previous_hash: previous_hash.clone(),
            hash: Block::calculate_hash(&id, time_stamp, &previous_hash, &data),
            data,
        }
    }

    fn calculate_hash(id: &str, timestamp: u64, previous_hash: &str, data: &str) -> String {
        hash(
            format!("{}{}{}{}", id, timestamp, previous_hash, data),
            DEFAULT_COST,
        )
        .unwrap()
    }
}
