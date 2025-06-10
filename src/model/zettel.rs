use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::*,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

use crate::ZettariumError;
use crate::schema::zettels;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = zettels)]
pub struct Zettel {
    pub id: String,
    pub title: String,
    pub type_: NoteType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
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

// --- ToSql<Text, Sqlite> 実装 ---
impl ToSql<Text, Sqlite> for NoteType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let value = match self {
            NoteType::Fleeting => "fleeting",
            NoteType::Permanent => "permanent",
            NoteType::Literature => "literature",
            NoteType::Structure => "structure",
            NoteType::Index => "index",
        };
        <str as ToSql<Text, Sqlite>>::to_sql(value, out)
    }
}

// --- FromSql<Text, Sqlite> 実装 ---
impl FromSql<Text, Sqlite> for NoteType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match unsafe { &*s } {
            "fleeting" => Ok(NoteType::Fleeting),
            "permanent" => Ok(NoteType::Permanent),
            "literature" => Ok(NoteType::Literature),
            "structure" => Ok(NoteType::Structure),
            "index" => Ok(NoteType::Index),
            other => Err(format!("Unrecognized NoteType variant: {}", other).into()),
        }
    }
}

impl std::fmt::Display for Zettel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Type: {:?}", self.type_)?;
        writeln!(
            f,
            "Created: {}",
            self.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
        )?;
        writeln!(
            f,
            "Updated: {}",
            self.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()
        )?;
        writeln!(f, "Archived: {}", self.archived)?;
        Ok(())
    }
}

// Test
#[cfg(test)]
mod tests {
    use crate::config::load_config;
    use crate::create_zettel;
    use crate::dedup_and_warn;
    use crate::establish_connection;

    use super::*;

    #[test]
    fn test_zettel_creation_and_display() {
        let config = load_config().unwrap_or_else(|e| {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        });
        let conn = &mut establish_connection(&config);
        let title = "this is a test";
        let type_ = "fleeting";
        // let tags = Some(vec!["rust", "test"]);
        let tags: Option<Vec<String>> = None;

        let mut tags_str: Vec<String> = vec![];
        if let Some(tags) = tags {
            tags_str = tags.into_iter().map(String::from).collect();
        }
        let cleaned_tags = dedup_and_warn(tags_str);

        let zettel = create_zettel(conn, title, type_, &cleaned_tags).unwrap();

        let output = format!("{:?}", zettel);
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
