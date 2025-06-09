use crate::{
    schema::zettels,
    {NoteType, Zettel},
};
use anyhow::{Error, Result};
use chrono::{Local, NaiveDateTime};
use diesel::{SqliteConnection, prelude::*};
use serde::Serialize;

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = zettels)]
pub struct NewZettel {
    pub id: String,
    pub title: String,
    pub type_: NoteType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
}

pub fn create_zettel(
    conn: &mut SqliteConnection,
    title_: &str,
    type_: &str,
    tags: &[String],
) -> Result<Zettel, Error> {
    // Zettel構造体にマッピング
    let new_zettel = NewZettel {
        id: generate_zettel_id(),
        title: title_.to_string(),
        type_: type_.parse::<NoteType>()?,
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
        archived: false,
    };

    // SQLiteに保存する処理
    let zettel = diesel::insert_into(zettels::table)
        .values(&new_zettel)
        .returning(Zettel::as_select())
        .get_result(conn)?;

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
