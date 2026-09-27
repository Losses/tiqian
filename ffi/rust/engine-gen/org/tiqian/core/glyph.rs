use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct Glyph {
    pub id: u32,
    pub cluster_range: TextRange,
    pub advance: f64,
    pub x: f64,
    pub y: f64,
    pub render_font_key: Option<String>,
    pub bounds: Option<Rect>,
    pub halt_advance: Option<f64>,
    pub halt_placement_x: Option<f64>,
}

impl Glyph {
    pub fn new(id: u32, cluster_range: TextRange, advance: f64, x: Option<f64>, y: Option<f64>, render_font_key: Option<String>, bounds: Option<Rect>, halt_advance: Option<f64>, halt_placement_x: Option<f64>) -> Self {
        let x = x.unwrap_or_else(|| 0.0);
        let y = y.unwrap_or_else(|| 0.0);
        let render_font_key = render_font_key.or_else(|| None);
        let bounds = bounds.or_else(|| None);
        let halt_advance = halt_advance.or_else(|| None);
        let halt_placement_x = halt_placement_x.or_else(|| None);
        Self {
            id,
            cluster_range,
            advance,
            x: x,
            y: y,
            render_font_key: render_font_key,
            bounds: bounds,
            halt_advance: halt_advance,
            halt_placement_x: halt_placement_x,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "Glyph(",
            "id=",
            crate::runtime::int_text::IntText::int_text(self.id),
            ", ",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "x=",
            self.x,
            ", ",
            "y=",
            self.y,
            ", ",
            "renderFontKey=",
            match (self.render_font_key).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "bounds=",
            (match &(self.bounds) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ", ",
            "haltAdvance=",
            match self.halt_advance { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "haltPlacementX=",
            match self.halt_placement_x { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ")"
        );
    }
}
