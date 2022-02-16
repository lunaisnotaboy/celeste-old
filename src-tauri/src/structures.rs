#[derive(serde::Serialize)]
pub struct CustomResponse {
    pub message: String,
    pub other_val: usize,
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
enum ImageType {
    Png,
    Jpeg,
    WebP,
}

// Struct for cached image blurhashes. Key for db storage should be mxc:// uri
pub struct ImageCache {
    mxc: String,
    blurhash: String,
    alt_text: Option<String>,
    spoiler: bool,
    format: ImageType,
}
