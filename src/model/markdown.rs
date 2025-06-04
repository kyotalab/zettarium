use crate::Zettel;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct Markdown {
    pub front_matter: FrontMatter,
    pub body: Body,
}

impl Markdown {
    pub fn get_front_matter(&self) -> &FrontMatter {
        &self.front_matter
    }

    pub fn get_body(&self) -> &Body {
        &self.body
    }
}

#[derive(Debug, Serialize)]
pub struct FrontMatter {
    #[serde(flatten)]
    pub zettel: Zettel,
    pub tags: Vec<String>,
}

impl FrontMatter {
    pub fn get_zettel(&self) -> &Zettel {
        &self.zettel
    }

    pub fn get_tags(&self) -> &[String] {
        &self.tags
    }
}

#[derive(Debug, Serialize)]
pub struct Body(pub String);

impl ToString for Body {
    fn to_string(&self) -> String {
        return self.0.clone();
    }
}

impl std::fmt::Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use serde_yaml;
        let serialized = serde_yaml::to_string(&self.front_matter).map_err(|_| fmt::Error)?;
        writeln!(f, "---\n{}---", serialized)?;

        writeln!(f, "\n{}", self.body.to_string())?;

        Ok(())
    }
}
