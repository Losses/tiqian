use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectLineHeightDecisionInfo {
    pub base_line_height: f64,
    pub base_face_ascent: f64,
    pub base_face_descent: f64,
    pub available_interline_space: f64,
    pub minimum_clearance: f64,
    pub line_ascents: Vec<f64>,
    pub line_descents: Vec<f64>,
    pub line_extras: Vec<f64>,
    pub boundary_shifts_after: Vec<f64>,
    pub trailing_extra: f64,
    pub expanded_line_indices: Vec<u32>,
    pub reason: UString,
}

impl InlineObjectLineHeightDecisionInfo {
    pub fn new(base_line_height: f64, base_face_ascent: f64, base_face_descent: f64, available_interline_space: f64, minimum_clearance: f64, line_ascents: Vec<f64>, line_descents: Vec<f64>, line_extras: Vec<f64>, boundary_shifts_after: Vec<f64>, trailing_extra: f64, expanded_line_indices: Vec<u32>, reason: &UStr) -> Self {
        Self {
            base_line_height,
            base_face_ascent,
            base_face_descent,
            available_interline_space,
            minimum_clearance,
            line_ascents,
            line_descents,
            line_extras,
            boundary_shifts_after,
            trailing_extra,
            expanded_line_indices,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineObjectLineHeightDecisionInfo(")); __s += &(UString::from("baseLineHeight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_line_height)); __s += &(UString::from(", ")); __s += &(UString::from("baseFaceAscent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_face_ascent)); __s += &(UString::from(", ")); __s += &(UString::from("baseFaceDescent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.base_face_descent)); __s += &(UString::from(", ")); __s += &(UString::from("availableInterlineSpace=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.available_interline_space)); __s += &(UString::from(", ")); __s += &(UString::from("minimumClearance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.minimum_clearance)); __s += &(UString::from(", ")); __s += &(UString::from("lineAscents=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_ascents).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineDescents=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_descents).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineExtras=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("boundaryShiftsAfter=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.boundary_shifts_after).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trailingExtra=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_extra)); __s += &(UString::from(", ")); __s += &(UString::from("expandedLineIndices=")); __s += UString::from(format!("{}", {
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
