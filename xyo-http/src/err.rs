#[derive(Debug)]
pub struct HttpClientError {
    pub code: i16,
    pub message: String,
}
