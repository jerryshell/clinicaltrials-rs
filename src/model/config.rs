use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub query: String,
    pub keywords: Vec<String>,
    pub keywords_in_inclusion: bool,
    pub keywords_in_exclusion: bool,
    pub keywords_in_conditions: bool,
    pub tokio_task_sleep: Option<u64>,
}
