use anyhow::{Error, Result};

use crate::Tag;

pub fn create_tag(tag_name: &str) -> Result<Tag, Error> {
    // Tag構造体にマッピング
    let tag = Tag {
        id: generate_tag_id(),
        tag_name: tag_name.to_string(),
    };

    // SQLiteに保存する処理

    Ok(tag)
}

fn generate_tag_id() -> String {
    "t-001".into()
}
