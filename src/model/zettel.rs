use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::ZettariumError;

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

impl FromStr for NoteType {
    type Err = ZettariumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fleeting" | "f" => Ok(NoteType::Fleeting),
            "permanent" | "p" => Ok(NoteType::Permanent),
            "literature" | "l" => Ok(NoteType::Literature),
            "structure" | "s" => Ok(NoteType::Structure),
            "index" | "i" => Ok(NoteType::Index),
            _ => Err(ZettariumError::InvalidNoteType(s.into())),
        }
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

// Test
#[cfg(test)]
mod tests {
    use crate::create_zettel;
    use crate::dedup_and_warn;

    use super::*;

    #[test]
    fn test_zettel_creation_and_display() {
        let title = "this is a test";
        let r#type = "fleeting";
        // let tags = Some(vec!["rust", "test"]);
        let tags: Option<Vec<String>> = None;

        let mut tags_str: Vec<String> = vec![];
        if let Some(tags) = tags {
            tags_str = tags.into_iter().map(String::from).collect();
        }
        let cleaned_tags = dedup_and_warn(tags_str);

        let zettel = create_zettel(title, r#type, &cleaned_tags).unwrap();

        let output = format!("{zettel}");
        assert!(output.contains("this is a test"));
        assert!(output.contains("Fleeting"));
    }

    #[test]
    fn test_invalid_note_type_parse() {
        let result = "invalid_type".parse::<NoteType>();
        assert!(result.is_err());

        if let Err(e) = result {
            let msg = format!("{}", e);
            assert!(msg.contains("Invalid note type"));
        }
    }
}
