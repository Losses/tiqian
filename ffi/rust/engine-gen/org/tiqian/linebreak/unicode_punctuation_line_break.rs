use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::linebreak::unicode_punctuation_line_break_data::UnicodePunctuationLineBreakData;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct UnicodePunctuationLineBreak;

impl UnicodePunctuationLineBreak {
    pub fn unicode_punctuation_line_break_class_of(code_point: u32) -> Result<UnicodePunctuationLineBreakClass, TextRangeError> {
        if code_point > 2147483647 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) > (1114111) {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 57343 {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Surrogate is not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        let kind = UnicodePunctuationLineBreakData::unicode_punctuation_line_break_data_lookup(u32::from_ne_bytes(((code_point) as u32).to_ne_bytes()));
        if kind == 0 {
            return Ok(UnicodePunctuationLineBreakClass::BreakAfter);
        }
        if kind == 1 {
            return Ok(UnicodePunctuationLineBreakClass::BreakBoth);
        }
        if kind == 2 {
            return Ok(UnicodePunctuationLineBreakClass::ClosePunctuation);
        }
        if kind == 3 {
            return Ok(UnicodePunctuationLineBreakClass::CloseParenthesis);
        }
        if kind == 4 {
            return Ok(UnicodePunctuationLineBreakClass::Exclamation);
        }
        if kind == 5 {
            return Ok(UnicodePunctuationLineBreakClass::HyphenHh);
        }
        if kind == 6 {
            return Ok(UnicodePunctuationLineBreakClass::Hyphen);
        }
        if kind == 7 {
            return Ok(UnicodePunctuationLineBreakClass::Inseparable);
        }
        if kind == 8 {
            return Ok(UnicodePunctuationLineBreakClass::InfixNumericSeparator);
        }
        if kind == 9 {
            return Ok(UnicodePunctuationLineBreakClass::Nonstarter);
        }
        if kind == 10 {
            return Ok(UnicodePunctuationLineBreakClass::OpenPunctuation);
        }
        if kind == 11 {
            return Ok(UnicodePunctuationLineBreakClass::Quotation);
        }
        if kind == 12 {
            return Ok(UnicodePunctuationLineBreakClass::SymbolsAllowingBreakAfter);
        }
        return Ok(UnicodePunctuationLineBreakClass::Other);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnicodePunctuationLineBreakClass {
    BreakAfter,
    BreakBoth,
    ClosePunctuation,
    CloseParenthesis,
    Exclamation,
    HyphenHh,
    Hyphen,
    Inseparable,
    InfixNumericSeparator,
    Nonstarter,
    OpenPunctuation,
    Quotation,
    SymbolsAllowingBreakAfter,
    Other,
}

pub fn compare_unicode_punctuation_line_break_class(a: &UnicodePunctuationLineBreakClass, b: &UnicodePunctuationLineBreakClass) -> i32 {
    if a == b { return 0; }
    fn rank(v: &UnicodePunctuationLineBreakClass) -> i32 {
        match v {
            UnicodePunctuationLineBreakClass::BreakAfter => 0,
            UnicodePunctuationLineBreakClass::BreakBoth => 1,
            UnicodePunctuationLineBreakClass::ClosePunctuation => 2,
            UnicodePunctuationLineBreakClass::CloseParenthesis => 3,
            UnicodePunctuationLineBreakClass::Exclamation => 4,
            UnicodePunctuationLineBreakClass::HyphenHh => 5,
            UnicodePunctuationLineBreakClass::Hyphen => 6,
            UnicodePunctuationLineBreakClass::Inseparable => 7,
            UnicodePunctuationLineBreakClass::InfixNumericSeparator => 8,
            UnicodePunctuationLineBreakClass::Nonstarter => 9,
            UnicodePunctuationLineBreakClass::OpenPunctuation => 10,
            UnicodePunctuationLineBreakClass::Quotation => 11,
            UnicodePunctuationLineBreakClass::SymbolsAllowingBreakAfter => 12,
            UnicodePunctuationLineBreakClass::Other => 13,
        }
    }
    rank(a) - rank(b)
}

impl UnicodePunctuationLineBreakClass {
    pub fn to_string(&self) -> String {
        match self {
            UnicodePunctuationLineBreakClass::BreakAfter => "BreakAfter".to_string(),
            UnicodePunctuationLineBreakClass::BreakBoth => "BreakBoth".to_string(),
            UnicodePunctuationLineBreakClass::ClosePunctuation => "ClosePunctuation".to_string(),
            UnicodePunctuationLineBreakClass::CloseParenthesis => "CloseParenthesis".to_string(),
            UnicodePunctuationLineBreakClass::Exclamation => "Exclamation".to_string(),
            UnicodePunctuationLineBreakClass::HyphenHh => "HyphenHH".to_string(),
            UnicodePunctuationLineBreakClass::Hyphen => "Hyphen".to_string(),
            UnicodePunctuationLineBreakClass::Inseparable => "Inseparable".to_string(),
            UnicodePunctuationLineBreakClass::InfixNumericSeparator => "InfixNumericSeparator".to_string(),
            UnicodePunctuationLineBreakClass::Nonstarter => "Nonstarter".to_string(),
            UnicodePunctuationLineBreakClass::OpenPunctuation => "OpenPunctuation".to_string(),
            UnicodePunctuationLineBreakClass::Quotation => "Quotation".to_string(),
            UnicodePunctuationLineBreakClass::SymbolsAllowingBreakAfter => "SymbolsAllowingBreakAfter".to_string(),
            UnicodePunctuationLineBreakClass::Other => "Other".to_string(),
        }
    }
}

impl UnicodePunctuationLineBreakClass {
    pub fn name(&self) -> &'static str {
        match self {
            UnicodePunctuationLineBreakClass::BreakAfter => "BreakAfter",
            UnicodePunctuationLineBreakClass::BreakBoth => "BreakBoth",
            UnicodePunctuationLineBreakClass::ClosePunctuation => "ClosePunctuation",
            UnicodePunctuationLineBreakClass::CloseParenthesis => "CloseParenthesis",
            UnicodePunctuationLineBreakClass::Exclamation => "Exclamation",
            UnicodePunctuationLineBreakClass::HyphenHh => "HyphenHH",
            UnicodePunctuationLineBreakClass::Hyphen => "Hyphen",
            UnicodePunctuationLineBreakClass::Inseparable => "Inseparable",
            UnicodePunctuationLineBreakClass::InfixNumericSeparator => "InfixNumericSeparator",
            UnicodePunctuationLineBreakClass::Nonstarter => "Nonstarter",
            UnicodePunctuationLineBreakClass::OpenPunctuation => "OpenPunctuation",
            UnicodePunctuationLineBreakClass::Quotation => "Quotation",
            UnicodePunctuationLineBreakClass::SymbolsAllowingBreakAfter => "SymbolsAllowingBreakAfter",
            UnicodePunctuationLineBreakClass::Other => "Other",
        }
    }
}
