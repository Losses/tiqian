#[derive(Debug, Clone, PartialEq)]
pub enum NoSuchElementError {
    Message { text: String },
}

impl std::fmt::Display for NoSuchElementError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoSuchElementError::Message { text } => {
                write!(formatter, "{}", text)
            }
        }
    }
}

impl std::error::Error for NoSuchElementError {}
