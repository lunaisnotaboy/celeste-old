use crate::structures::{ImageCache, ImageType, Keyspace};
use log::{debug, error, warn, info};

// Open the base database (will be stored in a tauri state)
// Path will eventually be set differently on Windows/Linux/MacOS
pub fn open_database() -> Option<sled::Db> {
    let _config = sled::Config::default().path("database");
    //.use_compression(true)
    //.compression_factor(10);

    let db: sled::Db = match _config.open() {
        Ok(database) => database,
        Err(e) => {
            error!("Error opening database! {}", e);
            return None;
        }
    };

    return Some(db);
}

// Function to insert a blurhash struct into the database
fn insert_blurhash(entry: ImageCache, keyspace: Keyspace) -> Result<(), String>{
    let db = open_database().unwrap();
    let tree = db.open_tree(keyspace.to_str()).unwrap(); // Open the keyspace

    // Serialize the blurhash struct
    let bytes;
    match bincode::encode_to_vec(&entry, bincode::config::standard()) {
        Ok(result) => bytes = result,
        Err(error) => {
			return Err(error.to_string());
		}
    }

    // Insert it into the database
    match tree.insert(&entry.mxc, bytes) {
        Ok(_) => debug!("inserted blurhash: {}", &entry.mxc),
        Err(error) => error!("couldn't insert blurhash: {}", error),
    }

	return Ok(());
}

// Insert a test blurhash into the database
#[tauri::command]
pub async fn database_test(mxc: String) -> Result<String, String> {
    let test_blurhash = ImageCache {
        mxc,
        blurhash: "8034jgw9aqihg".to_string(),
        alt_text: None,
        spoiler: false,
        format: ImageType::Png,
    };
    match insert_blurhash(test_blurhash, Keyspace::Blurhash) {
		Ok(_) => {},
		Err(e) => return Err(e),
	}
    return Ok("Inserted test blurhash!".to_string());
}
