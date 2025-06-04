use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zettel {
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub archived: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoteType {
    Fleeting,
    Permanent,
    Literature,
    Structure,
    Index,
}
