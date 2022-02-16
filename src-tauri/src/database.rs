use crate::structures::{ImageCache, Keyspace};

pub fn open_database(path: &str) -> Option<sled::Db> {
    let db: sled::Db = match sled::open(path) {
        Ok(database) => database,
        Err(e) => {
            println!("Error opening database! {}", e);
            return None;
        }
    };

    return Some(db);
}

pub fn insert_blurhash(db: sled::Db, entry: ImageCache, keyspace: Keyspace) {
    let tree = db.open_tree(keyspace.to_str());

    let bytes;
}
