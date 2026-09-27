use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LineDebugInfo {
    pub repair: Option<String>,
    pub notes: Vec<String>,
}

impl LineDebugInfo {
    pub fn new(repair: Option<String>, notes: Option<Vec<String>>) -> Self {
        let notes = notes.unwrap_or_else(|| vec![]);
        Self {
            repair: match repair { Some(v) => Some(v.to_string()), None => None },
            notes: notes,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "LineDebugInfo(",
            "repair=",
            match (self.repair).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "notes=",
            {
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
    },
            ")"
        );
    }
}
