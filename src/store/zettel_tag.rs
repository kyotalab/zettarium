use anyhow::{Error, Result};

use crate::{
    Tag, ZettelTag,
    schema::{tags, zettel_tags, zettel_tags::dsl::*},
};
use diesel::{SqliteConnection, prelude::*};

#[derive(Insertable)]
#[diesel(table_name = zettel_tags)]
pub struct NewZettelTag {
    pub zettel_id: String,
    pub tag_id: String,
}

pub fn create_zettel_tag(
    conn: &mut SqliteConnection,
    z_id: &str,
    t_id: &str,
) -> Result<ZettelTag, Error> {
    // ZettelTag構造体にマッピング
    let new_zettel_tag = NewZettelTag {
        zettel_id: z_id.to_string(),
        tag_id: t_id.to_string(),
    };

    // SQLiteに保存する処理
    let zettel_tag = diesel::insert_into(zettel_tags::table)
        .values(&new_zettel_tag)
        .returning(ZettelTag::as_select())
        .get_result(conn)?;

    Ok(zettel_tag)
}

pub fn get_tag_by_zettel_id(conn: &mut SqliteConnection, id: &str) -> Result<Vec<Tag>, Error> {
    let tags = zettel_tags::table
        .inner_join(tags::table.on(zettel_tags::tag_id.eq(tags::id)))
        .filter(zettel_tags::zettel_id.eq(id))
        .select(tags::all_columns)
        .load::<Tag>(conn)?;

    Ok(tags)
}

pub fn exists_zettel_tag(
    conn: &mut SqliteConnection,
    zettel_id_: &str,
    tag_id_: &str,
) -> Result<bool, Error> {
    let count = zettel_tags
        .filter(zettel_id.eq(zettel_id_))
        .filter(tag_id.eq(tag_id_))
        .count()
        .get_result::<i64>(conn)?;

    Ok(count > 0)
}
