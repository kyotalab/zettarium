use crate::{
    NoteType, Zettel, create_tag, create_zettel_tag, get_tag_name,
    schema::{zettels, zettels::dsl::*},
    tags, zettel_tags,
};
use anyhow::{Error, Result, anyhow};
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

#[derive(AsChangeset)]
#[diesel(table_name = zettels)]
pub struct ArchivedZettel {
    pub archived: bool,
}

pub fn create_zettel(
    conn: &mut SqliteConnection,
    title_: &str,
    note_type: &str,
    tags_name: &[String],
) -> Result<Zettel, Error> {
    conn.transaction::<Zettel, Error, _>(|conn| {
        // このクロージャ内で複数のDB操作を行う
        // Zettel構造体にマッピング
        let new_zettel = NewZettel {
            id: generate_zettel_id(),
            title: title_.to_string(),
            type_: note_type.parse::<NoteType>()?,
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

pub fn list_zettels(
    conn: &mut SqliteConnection,
    zettel_id: Option<&str>,
    note_type: Option<&str>,
    tags_name: &[String],
    all: bool,
    archived_only: bool,
) -> Result<Vec<Zettel>, Error> {
    if all {
        return Ok(zettels.load::<Zettel>(conn)?);
    }

    let mut query = zettels.into_boxed();

    if let Some(id_filter) = zettel_id {
        query = query.filter(id.eq(id_filter));
    }

    if let Some(type_filter) = note_type {
        query = query.filter(type_.eq(type_filter));
    }

    if archived_only {
        query = query.filter(archived.eq(true));
    } else {
        query = query.filter(archived.eq(false));
    }

    if !tags_name.is_empty() {
        let query_with_tags = zettels
            .inner_join(zettel_tags::table.on(zettels::id.eq(zettel_tags::zettel_id)))
            .inner_join(tags::table.on(tags::id.eq(zettel_tags::tag_id)))
            .filter(tags::tag_name.eq_any(tags_name))
            .filter(archived.eq(archived_only))
            .select(Zettel::as_select())
            .distinct();

        return Ok(query_with_tags.load::<Zettel>(conn)?);
    }

    Ok(query.load::<Zettel>(conn)?)
}

pub fn archive_zettel(conn: &mut SqliteConnection, zettel_id: &str) -> Result<Zettel, Error> {
    let exist_zettel = ensure_zettel_exists(conn, zettel_id)?;

    if exist_zettel.archived {
        if exist_zettel.archived {
            return Err(anyhow!("Note is already archived"));
        }
    }

    let archived_zettel = diesel::update(zettels.find(zettel_id))
        .set(ArchivedZettel { archived: true })
        .returning(Zettel::as_select())
        .get_result(conn)?;

    Ok(archived_zettel)
}

fn generate_zettel_id() -> String {
    Local::now().format("%Y%m%dT%H%M%S").to_string()
}

pub fn ensure_zettel_exists(conn: &mut SqliteConnection, zettel_id: &str) -> Result<Zettel, Error> {
    let zettel = zettels
        .find(zettel_id)
        .select(Zettel::as_select())
        .first(conn)
        .optional()?;

    match zettel {
        Some(existing) => Ok(existing),
        None => Err(diesel::result::Error::NotFound.into()),
    }
}
