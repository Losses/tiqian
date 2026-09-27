#[derive(Debug, Clone, PartialEq)]
pub enum TextRangeError {
    StartGreaterThanEnd,
    NegativeStart,
    Message { text: String },
}

impl std::fmt::Display for TextRangeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextRangeError::StartGreaterThanEnd => write!(formatter, "{}", "TextRange start must not be greater than end."),
            TextRangeError::NegativeStart => write!(formatter, "{}", "TextRange start must be non-negative."),
            TextRangeError::Message { text } => {
                write!(formatter, "{}", text)
            }
        }
    }
}

impl std::error::Error for TextRangeError {}
