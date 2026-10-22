use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct TiqianTextContent {
    pub text: UString,
    pub spans: Vec<TextSpan>,
    pub source_boundaries: Vec<u32>,
    pub line_break_spans: Vec<LineBreakSpan>,
    pub auto_space_suppressed_ranges: Vec<TextRange>,
}

impl TiqianTextContent {
    pub fn new(text: &UStr, spans: Option<Vec<TextSpan>>, source_boundaries: Option<Vec<u32>>, line_break_spans: Option<Vec<LineBreakSpan>>, auto_space_suppressed_ranges: Option<Vec<TextRange>>) -> Self {
        let spans = spans.unwrap_or_else(|| vec![]);
        let source_boundaries = source_boundaries.unwrap_or_else(|| vec![]);
        let line_break_spans = line_break_spans.unwrap_or_else(|| vec![]);
        let auto_space_suppressed_ranges = auto_space_suppressed_ranges.unwrap_or_else(|| vec![]);
        Self {
            text: text.to_ustring(),
            spans: spans,
            source_boundaries: source_boundaries,
            line_break_spans: line_break_spans,
            auto_space_suppressed_ranges: auto_space_suppressed_ranges,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TiqianTextContent(")); __s += &(UString::from("text=")); __s += (self.text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("spans=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceBoundaries=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.source_boundaries).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", i32::from_ne_bytes(((arr[i]) as i32).to_ne_bytes()));
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineBreakSpans=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_break_spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("autoSpaceSuppressedRanges=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.auto_space_suppressed_ranges).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
