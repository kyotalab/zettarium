use crate::{NoteType, Zettel, create_tag, create_zettel_tag, get_tag_name, schema::zettels};
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
    tags_name: &[String],
) -> Result<Zettel, Error> {
    conn.transaction::<Zettel, Error, _>(|conn| {
        // このクロージャ内で複数のDB操作を行う
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

        if !tags_name.is_empty() {
            for name in tags_name {
                // 1. tag_name が tags テーブルに存在するか確認（SELECT）
                let tag = match get_tag_name(conn, name) {
                    Ok(Some(existing)) => existing,
                    Ok(None) => create_tag(conn, name)?,
                    Err(e) => return Err(e),
                };

                create_zettel_tag(conn, &zettel.id, &tag.id)?;
            }
        }

        Ok(zettel)
    })
}

fn generate_zettel_id() -> String {
    Local::now().format("%Y%m%dT%H%M%S").to_string()
}
