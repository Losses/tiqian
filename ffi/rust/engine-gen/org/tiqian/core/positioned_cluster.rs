use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PositionedCluster(",
            "lineIndex=",
            crate::runtime::int_text::IntText::int_text(self.line_index),
            ", ",
            "clusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.cluster_index),
            ", ",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "left=",
            self.left,
            ", ",
            "top=",
            self.top,
            ", ",
            "right=",
            self.right,
            ", ",
            "bottom=",
            self.bottom,
            ", ",
            "baseline=",
            self.baseline,
            ", ",
            "drawX=",
            self.draw_x,
            ", ",
            "sourceStops=",
            (match &(self.source_stops) { None => "null".to_string(), Some(__option) => {
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
    }.to_string() }),
            ")"
        );
    }
}
