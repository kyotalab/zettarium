use anyhow::{Error, Result};
use chrono::Local;
use crate::{NoteType, Zettel, Tag};


pub fn create_zettel(title: &str, r#type: &str, tags: &Vec<String>) -> Result<Zettel, Error> {
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

    // println!("ここでtagsテーブルに追加");
    // for (i, t) in tags.iter().enumerate() {
    //     let tag = Tag {
    //         id: format!("t-{:03}", i + 1),
    //         tag_name: t.to_string(),
    //     };
    //     println!("{tag}");
    // }
    // println!("ここまで");

    Ok(zettel)
}

fn generate_zettel_id() -> String {
    Local::now().format("%Y%m%dT%H%M%S").to_string()
}