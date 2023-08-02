use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub cond_query: String,
    pub term_query: String,
    pub keywords: Vec<String>,
}
