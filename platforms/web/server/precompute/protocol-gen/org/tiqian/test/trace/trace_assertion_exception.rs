#[derive(Debug, Clone, PartialEq)]
pub enum TraceAssertionError {
    AssertionFailed { message: String },
}

impl std::fmt::Display for TraceAssertionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceAssertionError::AssertionFailed { message } => {
                write!(formatter, "{}", message)
            }
        }
    }
}

impl std::error::Error for TraceAssertionError {}
