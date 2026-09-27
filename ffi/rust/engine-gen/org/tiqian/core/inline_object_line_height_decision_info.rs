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
    pub reason: String,
}

impl InlineObjectLineHeightDecisionInfo {
    pub fn new(base_line_height: f64, base_face_ascent: f64, base_face_descent: f64, available_interline_space: f64, minimum_clearance: f64, line_ascents: Vec<f64>, line_descents: Vec<f64>, line_extras: Vec<f64>, boundary_shifts_after: Vec<f64>, trailing_extra: f64,
expanded_line_indices: Vec<u32>, reason: &str) -> Self {
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
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "InlineObjectLineHeightDecisionInfo(",
            "baseLineHeight=",
            self.base_line_height,
            ", ",
            "baseFaceAscent=",
            self.base_face_ascent,
            ", ",
            "baseFaceDescent=",
            self.base_face_descent,
            ", ",
            "availableInterlineSpace=",
            self.available_interline_space,
            ", ",
            "minimumClearance=",
            self.minimum_clearance,
            ", ",
            "lineAscents=",
            {
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
    },
            ", ",
            "lineDescents=",
            {
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
    },
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
            "boundaryShiftsAfter=",
            {
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
    },
            ", ",
            "trailingExtra=",
            self.trailing_extra,
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
