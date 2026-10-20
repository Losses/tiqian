use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LineDebugInfo {
    pub repair: Option<UString>,
    pub notes: Vec<UString>,
}

impl LineDebugInfo {
    pub fn new(repair: Option<UString>, notes: Option<Vec<UString>>) -> Self {
        let notes = notes.unwrap_or_else(|| vec![]);
        Self {
            repair,
            notes: notes,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineDebugInfo(")); __s += &(UString::from("repair=")); __s += match &((self.repair).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("notes=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.notes).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
