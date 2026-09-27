use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct DefaultHyphenator;

impl DefaultHyphenator {
    pub fn default_hyphenator_default_hyphenator() -> Result<Box<dyn Hyphenator>, UStringFault> {
        return Ok(EnglishHyphenation::english_hyphenation_en_us()?);
    }
}
