use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum NoSuchElementError {
    Message { text: UString },
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
