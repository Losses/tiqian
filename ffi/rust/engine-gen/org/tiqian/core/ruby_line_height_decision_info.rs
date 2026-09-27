use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct RubyLineHeightDecisionInfo {
    pub mode: String,
    pub base_line_height: f64,
    pub base_face_height: f64,
    pub ruby_extent: f64,
    pub available_interline_space: f64,
    pub max_extra: f64,
    pub line_extras: Vec<f64>,
    pub expanded_line_indices: Vec<u32>,
    pub reason: String,
}

impl RubyLineHeightDecisionInfo {
    pub fn new(mode: &str, base_line_height: f64, base_face_height: f64, ruby_extent: f64, available_interline_space: f64, max_extra: f64, line_extras: Vec<f64>, expanded_line_indices: Vec<u32>, reason: &str) -> Self {
        Self {
            mode: mode.to_string(),
            base_line_height,
            base_face_height,
            ruby_extent,
            available_interline_space,
            max_extra,
            line_extras,
            expanded_line_indices,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RubyLineHeightDecisionInfo(",
            "mode=",
            (self.mode).to_string(),
            ", ",
            "baseLineHeight=",
            self.base_line_height,
            ", ",
            "baseFaceHeight=",
            self.base_face_height,
            ", ",
            "rubyExtent=",
            self.ruby_extent,
            ", ",
            "availableInterlineSpace=",
            self.available_interline_space,
            ", ",
            "maxExtra=",
            self.max_extra,
            ", ",
            "lineExtras=",
            {
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
    },
            ", ",
            "expandedLineIndices=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.expanded_line_indices).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(arr[i]));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
