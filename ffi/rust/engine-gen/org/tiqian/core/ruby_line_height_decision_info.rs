use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct RubyLineHeightDecisionInfo {
    pub mode: UString,
    pub base_line_height: f64,
    pub base_face_height: f64,
    pub ruby_extent: f64,
    pub available_interline_space: f64,
    pub max_extra: f64,
    pub line_extras: Vec<f64>,
    pub expanded_line_indices: Vec<u32>,
    pub reason: UString,
}

impl RubyLineHeightDecisionInfo {
    pub fn new(mode: &UStr, base_line_height: f64, base_face_height: f64, ruby_extent: f64, available_interline_space: f64, max_extra: f64, line_extras: Vec<f64>, expanded_line_indices: Vec<u32>, reason: &UStr) -> Self {
        Self {
            mode: mode.to_ustring(),
            base_line_height,
            base_face_height,
            ruby_extent,
            available_interline_space,
            max_extra,
            line_extras,
            expanded_line_indices,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RubyLineHeightDecisionInfo(")); __s += &(UString::from("mode=")); __s += (self.mode).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("baseLineHeight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_line_height)); __s += &(UString::from(", ")); __s += &(UString::from("baseFaceHeight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_face_height)); __s += &(UString::from(", ")); __s += &(UString::from("rubyExtent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ruby_extent)); __s += &(UString::from(", ")); __s += &(UString::from("availableInterlineSpace=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.available_interline_space)); __s += &(UString::from(", ")); __s += &(UString::from("maxExtra=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.max_extra)); __s += &(UString::from(", ")); __s += &(UString::from("lineExtras=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_extras).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("expandedLineIndices=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.expanded_line_indices).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", i32::from_ne_bytes(((arr[i]) as i32).to_ne_bytes()));
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
