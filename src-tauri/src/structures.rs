//* Code for all the various struct's and enums *//

// TODO: Make proper response types
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

// TODO: Replace with enum from image crate at some point
#[derive(bincode::Encode)]
pub enum ImageType {
    Png,
    Jpeg,
    WebP,
}

// Struct for cached image blurhashes. Key for db storage should be mxc:// uri
// TODO: Replace String in mxc field with proper type from matrix_sdk
#[derive(bincode::Encode)]
pub struct BlurhashCache {
    pub mxc: String,
    pub blurhash: String,
    pub alt_text: Option<String>,
    pub spoiler: bool,
    pub format: ImageType,
}
