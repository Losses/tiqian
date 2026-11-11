use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct PositionedCluster {
    pub line_index: u32,
    pub cluster_index: u32,
    pub range: TextRange,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub baseline: f64,
    pub draw_x: f64,
    pub source_stops: Option<Vec<f64>>,
}

impl PositionedCluster {
    pub fn new(line_index: u32, cluster_index: u32, range: TextRange, left: f64, top: f64, right: f64, bottom: f64, baseline: f64, draw_x: f64, source_stops: Option<Vec<f64>>) -> Self {
        Self {
            line_index,
            cluster_index,
            range,
            left,
            top,
            right,
            bottom,
            baseline,
            draw_x,
            source_stops,
        }
    }

    pub fn get_width(&self) -> f64 {
        return self.right - self.left;
    }

    pub fn get_height(&self) -> f64 {
        return self.bottom - self.top;
    }

    pub fn get_rect(&self) -> Rect {
        return Rect::new(self.left, self.top, self.right, self.bottom);
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PositionedCluster(")); __s += &(UString::from("lineIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.line_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("clusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("left=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.left)); __s += &(UString::from(", ")); __s += &(UString::from("top=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top)); __s += &(UString::from(", ")); __s += &(UString::from("right=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.right)); __s += &(UString::from(", ")); __s += &(UString::from("bottom=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.bottom)); __s += &(UString::from(", ")); __s += &(UString::from("baseline=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline)); __s += &(UString::from(", ")); __s += &(UString::from("drawX=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.draw_x)); __s += &(UString::from(", ")); __s += &(UString::from("sourceStops=")); __s += UString::from(format!("{}", (match &(self.source_stops) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = &(*__option).clone();
        let n = arr.len();
        let mut i1 = 0usize;
        while i1 < n {
            if i1 > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i1]);
            i1 += 1;
        }
        out.push(']');
        out
    }).as_str()) })).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
