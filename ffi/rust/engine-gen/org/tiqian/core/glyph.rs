use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct Glyph {
    pub id: u32,
    pub cluster_range: TextRange,
    pub advance: f64,
    pub x: f64,
    pub y: f64,
    pub render_font_key: Option<UString>,
    pub bounds: Option<Rect>,
    pub halt_advance: Option<f64>,
    pub halt_placement_x: Option<f64>,
}

impl Glyph {
    pub fn new(id: u32, cluster_range: TextRange, advance: f64, x: Option<f64>, y: Option<f64>, render_font_key: Option<UString>, bounds: Option<Rect>, halt_advance: Option<f64>, halt_placement_x: Option<f64>) -> Self {
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Glyph(")); __s += &(UString::from("id=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.id)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("clusterRange=")); __s += UString::from(format!("{}", (self.cluster_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("x=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.x)); __s += &(UString::from(", ")); __s += &(UString::from("y=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.y)); __s += &(UString::from(", ")); __s += &(UString::from("renderFontKey=")); __s += match &((self.render_font_key).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("bounds=")); __s += (match &(self.bounds) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("haltAdvance=")); __s += &(match self.halt_advance { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("haltPlacementX=")); __s += &(match self.halt_placement_x { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(")")); __s }).as_str());
    }
}
