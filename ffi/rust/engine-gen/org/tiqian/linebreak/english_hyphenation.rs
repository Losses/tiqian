use crate::org::tiqian::linebreak::english_hyphenation_patterns::EnglishHyphenationPatterns;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::linebreak::liang_hyphenator::LiangHyphenator;
use crate::org::tiqian::linebreak::parse_tex_hyphenation_patterns::ParseTexHyphenationPatterns;
use crate::std::u_string_exception::UStringFault;
use std::cell::RefCell;


thread_local! {
    static ENGLISH_HYPHENATION_EN_US_CACHE: RefCell<Option<Box<dyn Hyphenator>>> = RefCell::new(None);
}

#[derive(Clone, Copy)]
pub struct EnglishHyphenation;

impl EnglishHyphenation {

    pub fn english_hyphenation_en_us() -> Result<Box<dyn Hyphenator>, UStringFault> {
        if ENGLISH_HYPHENATION_EN_US_CACHE.with(|c| c.borrow().clone()).is_none() {
            let parsed = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_parse(EnglishHyphenationPatterns::english_hyphenation_patterns_load()?.as_ustr());
            ENGLISH_HYPHENATION_EN_US_CACHE.with(|c| *c.borrow_mut() = Some(Box::new(LiangHyphenator::new((parsed.patterns).clone(), Some((parsed.exceptions).clone()), Some(2), Some(3)))));
        }
        return Ok((ENGLISH_HYPHENATION_EN_US_CACHE.with(|c| c.borrow().clone())).as_ref().unwrap().clone());
    }
}
