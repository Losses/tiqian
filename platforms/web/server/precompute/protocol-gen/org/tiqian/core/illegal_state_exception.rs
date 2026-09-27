#[derive(Debug, Clone, PartialEq)]
pub struct IllegalStateException {
    pub message: String,
}
impl IllegalStateException {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}
impl std::fmt::Display for IllegalStateException {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}
impl std::error::Error for IllegalStateException {}
