use rand::{distributions::Alphanumeric, *};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Note {
    pub contents: String,
    pub uses: u8,
}

impl Note {
    pub fn rand(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}
