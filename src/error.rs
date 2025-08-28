#[derive(Debug)]
#[derive(Clone)]
pub struct ClientError {
    pub message: String,
    pub code: i16,
}
