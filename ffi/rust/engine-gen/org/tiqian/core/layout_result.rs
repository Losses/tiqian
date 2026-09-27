use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::layout_debug_info::LayoutDebugInfo;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::size::Size;
use std::fmt::Write;


#[derive(Clone, PartialEq)]
pub struct LayoutResult {
    pub input: LayoutInput,
    pub size: Size,
    pub clusters: Vec<Cluster>,
    pub glyph_runs: Vec<GlyphRun>,
    pub lines: Vec<LineBox>,
    pub debug: LayoutDebugInfo,
}

impl LayoutResult {
    pub fn new(input: LayoutInput, size: Size, clusters: Vec<Cluster>, glyph_runs: Vec<GlyphRun>, lines: Vec<LineBox>, debug: LayoutDebugInfo) -> Self {
        Self {
            input,
            size,
            clusters,
            glyph_runs,
            lines,
            debug,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LayoutResult(",
            "input=",
            (self.input).clone().to_string(),
            ", ",
            "size=",
            (self.size).clone().to_string(),
            ", ",
            "clusters=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.clusters).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "glyphRuns=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.glyph_runs).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "lines=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.lines).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "debug=",
            (self.debug).clone().to_string(),
            ")"
        );
    }
}
