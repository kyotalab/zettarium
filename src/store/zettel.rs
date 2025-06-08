use crate::{NoteType, Tag, Zettel};
use anyhow::{Error, Result};
use chrono::Local;

pub fn create_zettel(title: &str, r#type: &str, tags: &[String]) -> Result<Zettel, Error> {
    // Zettel構造体にマッピング
    let zettel = Zettel {
        id: generate_zettel_id(),
        title: title.to_string(),
        r#type: r#type.parse::<NoteType>()?,
        created: Local::now().naive_local(),
        updated: Local::now().naive_local(),
        archived: false,
    };

    // SQLiteに保存する処理

    //

    if !tags.is_empty() {
        for tag_name in tags {
            // 1. tag_name が tags テーブルに存在するか確認（SELECT）
            // 2. 存在しなければ INSERT
            // 3. zettel_tags にリレーションを INSERT
        }
    }

    Ok(zettel)
}

fn generate_zettel_id() -> String {
    Local::now().format("%Y%m%dT%H%M%S").to_string()
}
