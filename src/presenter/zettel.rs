use crate::{Zettel, get_tag_by_zettel_id};
use anyhow::Result;
use diesel::SqliteConnection;
use prettytable::{Table, row};

pub fn print_zettels_as_table(conn: &mut SqliteConnection, zettels: &Vec<Zettel>) -> Result<()> {
    if zettels.is_empty() {
        println!("No Zettels found matching your criteria.");
        return Ok(());
    }

    let mut table = Table::new();

    table.add_row(row![
        "ID", "Title", "Type", "Tags", "Created", "Updated", "Archived"
    ]);

    for zettel in zettels {
        let type_ = format!("{:?}", zettel.type_);
        let format_created = zettel.created_at.format("%Y/%m/%d %H:%M:%S").to_string();
        let format_updated = zettel.updated_at.format("%Y/%m/%d %H:%M:%S").to_string();

        let tags = get_tag_by_zettel_id(conn, &zettel.id)?;
        let tags: Vec<String> = tags.into_iter().map(|t| t.tag_name).collect();
        let tags = tags.join(",");

        table.add_row(row![
            zettel.id,
            zettel.title,
            type_,
            tags,
            format_created,
            format_updated,
            zettel.archived
        ]);
    }

    table.printstd();

    Ok(())
}
