#[derive(Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub tag_name: String,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tag ID: {}", self.id)?;
        writeln!(f, "Tag Name: {}", self.tag_name)?;
        Ok(())
    }
}

// Test
#[cfg(test)]
mod tests {
    use crate::dedup_and_warn;

    use super::*;

    #[test]
    fn test_tag_creation_and_display() {
        let tags = vec!["rust", "test"];
        for (i, t) in tags.iter().enumerate() {
            let tag = Tag {
                id: format!("t-{:03}", i + 1),
                tag_name: t.to_string(),
            };
            let output = format!("{tag}");
            assert!(output.contains(t));
        }
    }

    #[test]
    fn test_dedup_and_warn_basic() {
        let tags = vec![
            "Rust".to_string(),
            "rust".to_string(),
            "cli".to_string(),
            "CLI".to_string(),
            "tool".to_string(),
        ];
        let deduped = dedup_and_warn(tags);
        assert_eq!(deduped.len(), 3);
        assert!(deduped.contains(&"Rust".to_string()));
        assert!(deduped.contains(&"cli".to_string()));
        assert!(deduped.contains(&"tool".to_string()));
    }
}
