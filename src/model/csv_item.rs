use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsvItem {
    pub id: String,
    pub sponsor: String,
    pub start_date: String,
    pub completion_date: String,
    pub drug: String,
}
