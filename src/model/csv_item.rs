use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsvItem {
    pub id: String,
    pub sponsor: String,
    pub start_date: String,
    pub completion_date: String,
    pub status: String,
    pub phase: String,
    pub conditions: String,
    pub drug: String,
}
