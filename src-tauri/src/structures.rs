pub struct Database;

#[derive(serde::Serialize)]
pub struct CustomResponse{
    pub message: String,
    pub other_val: usize,
}
