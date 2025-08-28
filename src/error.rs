#[derive(Debug)]
pub struct ClientError {
    pub message: String,
    pub code: i16,
}
