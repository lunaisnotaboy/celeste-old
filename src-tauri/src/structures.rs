pub struct Database;

#[derive(serde::Serialize)]
pub struct CustomResponse {
    pub message: String,
    pub other_val: usize,
}

// Replace with enum from image crate at some point
enum ImageType {
    Png,
    Jpeg,
    WebP,
}

// Struct for cached image blurhashes. Key for db storage should be mxc:// uri
struct ImageCache {
    mxc: String,
    blurhash: String,
    alt_text: Option<String>,
    spoiler: bool,
    format: ImageType,
}
