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
