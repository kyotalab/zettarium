use crate::{
    Tag,
    schema::{tags, tags::dsl::*},
};
use anyhow::{Error, Result};
use diesel::{SqliteConnection, prelude::*};

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub id: String,
    pub tag_name: String,
}

pub fn create_tag(conn: &mut SqliteConnection, tag_str: &str) -> Result<Tag, Error> {
    // Tag構造体にマッピング
    let new_tag = NewTag {
        id: generate_tag_id(conn)?,
        tag_name: tag_str.to_string(),
    };

    // SQLiteに保存する処理
    let tag = diesel::insert_into(tags::table)
        .values(&new_tag)
        .returning(Tag::as_select())
        .get_result(conn)?;

    Ok(tag)
}

pub fn get_tag_name(conn: &mut SqliteConnection, tag: &str) -> Result<Option<Tag>, Error> {
    let tag = tags
        .filter(tag_name.eq(tag))
        .select(Tag::as_select())
        .first(conn)
        .optional()?;
    Ok(tag)
}

fn generate_tag_id(conn: &mut SqliteConnection) -> Result<String, Error> {
    use regex::Regex;

    let all_ids: Vec<String> = tags.select(id).load::<String>(conn)?;

    let re = Regex::new(r"t-(\d{3})").unwrap();
    let max_num = all_ids
        .iter()
        .filter_map(|tag_id| {
            re.captures(tag_id)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);

    let next_id = format!("t-{:03}", max_num + 1);
    Ok(next_id)
}
