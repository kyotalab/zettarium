use anyhow::{Error, Result};

use crate::ZettelTag;

pub fn create_zettel_tag(zettel_id: &str, tag_id: &str) -> Result<ZettelTag, Error> {
    // ZettelTag構造体にマッピング
    let zettel_tag = ZettelTag {
        zettel_id: zettel_id.to_string,
        tag_id: tag_id.to_string,
    };

    // SQLiteに保存する処理

    Ok(zettel_tag);
}
