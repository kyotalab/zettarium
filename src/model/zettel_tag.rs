use crate::schema::zettel_tags;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable, Selectable)]
#[diesel(table_name = zettel_tags)]
pub struct ZettelTag {
    pub zettel_id: String,
    pub tag_id: String,
}

impl std::fmt::Display for ZettelTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Zettel ID: {}", self.zettel_id)?;
        writeln!(f, "Tag ID: {}", self.tag_id)?;
        Ok(())
    }
}

//Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zettel_tag_creation_and_display() {
        let zettel_tag = ZettelTag {
            zettel_id: "20250604170100".into(),
            tag_id: "t-001".into(),
        };

        let output = format!("{zettel_tag}");
        assert!(output.contains("20250604170100"));
        assert!(output.contains("t-001"));
    }
}
