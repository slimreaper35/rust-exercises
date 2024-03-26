use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientCmd {
    Get(String),
    Set(String, String),
    Exit,
}
