use anyhow::{Error, Result};

use crate::{ZettelTag, schema::zettel_tags};
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
