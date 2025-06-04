use std::collections::HashSet;

pub fn dedup_and_warn(tags: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for tag in tags {
        let tag_lower = tag.to_lowercase();
        if seen.contains(&tag_lower) {
            println!("Waring: Duplicate tag \"{tag}\" detected and skipped.");
        } else {
            seen.insert(tag_lower.clone());
            unique.push(tag);
        }
    }

    unique
}
