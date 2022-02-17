#[derive(serde::Serialize)]
pub struct StatusResponse {
    pub status: bool,
    pub message: String,
}

// Possible keyspaces for the database
#[derive(Debug)]
pub enum Keyspace {
    Blurhash,
    Message,
    EncryptionKeys,
}

impl Keyspace {
    pub fn to_str(&self) -> &str {
        match &self {
            Keyspace::Blurhash => "blurhash",
            Keyspace::Message => "message",
            Keyspace::EncryptionKeys => "encryption_keys",
        }
    }
}

// Replace with enum from image crate at some point
#[derive(bincode::Encode)]
pub enum ImageType {
    Png,
    Jpeg,
    WebP,
}

// Struct for cached image blurhashes. Key for db storage should be mxc:// uri
#[derive(bincode::Encode)]
pub struct ImageCache {
    pub mxc: String,
    pub blurhash: String,
    pub alt_text: Option<String>,
    pub spoiler: bool,
    pub format: ImageType,
}

// Struct for user login info
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub homeserver: matrix_sdk::reqwest::Url,
    pub password: String,
    pub device_id: Option<String>,
}
