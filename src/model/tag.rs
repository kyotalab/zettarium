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
}
