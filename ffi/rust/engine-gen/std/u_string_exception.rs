#[derive(Debug, Clone, PartialEq)]
pub enum UStringFault {
    InvalidCodePoint { code: u32 },
    UnpairedSurrogate { unit: u32 },
    TracedAssertionsFailFault(Box<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault>),
    PreparedParagraphToPreparedParagraphJsonFault(Box<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault>),
}

impl std::fmt::Display for UStringFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UStringFault::InvalidCodePoint { code } => {
                write!(formatter, "invalid code point: {}", code)
            }
            UStringFault::UnpairedSurrogate { unit } => {
                write!(formatter, "unpaired surrogate: {}", unit)
            }
            UStringFault::TracedAssertionsFailFault(inner) => write!(formatter, "{:?}", inner),
            UStringFault::PreparedParagraphToPreparedParagraphJsonFault(inner) => write!(formatter, "{:?}", inner),
        }
    }
}

impl std::error::Error for UStringFault {}
