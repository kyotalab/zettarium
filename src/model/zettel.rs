use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zettel {
    pub id: String,
    pub title: String,
    pub r#type: NoteType,
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

#[derive(Debug)]
pub enum NoteTypeParseError {
    InvalidNoteType(String),
}

impl std::fmt::Display for NoteTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteTypeParseError::InvalidNoteType(r#type) => {
                writeln!(f, "Unrecognized note type variant: {}", r#type)
            }
        }
    }
}

impl std::error::Error for NoteTypeParseError {}

pub fn parse_note_type(r#type: &str) -> Result<NoteType, NoteTypeParseError> {
    match r#type {
        "fleeting" | "f" => Ok(NoteType::Fleeting),
        "permanent" | "p" => Ok(NoteType::Permanent),
        "literature" | "l" => Ok(NoteType::Literature),
        "structure" | "s" => Ok(NoteType::Structure),
        "index" | "i" => Ok(NoteType::Index),
        other => Err(NoteTypeParseError::InvalidNoteType(other.to_string())),
    }
}

impl std::fmt::Display for Zettel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Type: {:?}", self.r#type)?;
        writeln!(
            f,
            "Created: {}",
            self.created.format("%Y-%m-%d %H:%M:%S").to_string()
        )?;
        writeln!(
            f,
            "Updated: {}",
            self.updated.format("%Y-%m-%d %H:%M:%S").to_string()
        )?;
        writeln!(f, "Archived: {}", self.archived)?;
        Ok(())
    }
}
