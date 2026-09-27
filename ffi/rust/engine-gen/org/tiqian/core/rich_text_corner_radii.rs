#[derive(Debug, Clone, PartialEq)]
pub struct RichTextCornerRadii {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl RichTextCornerRadii {
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub fn get_is_square(&self) -> bool {
        return self.top_left == 0.0f64 && self.top_right == 0.0f64 && self.bottom_right == 0.0f64 && self.bottom_left == 0.0f64;
    }

    pub fn get_is_uniform(&self) -> bool {
        return self.top_left == self.top_right && self.top_right == self.bottom_right && self.bottom_right == self.bottom_left;
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RichTextCornerRadii(",
            "topLeft=",
            self.top_left,
            ", ",
            "topRight=",
            self.top_right,
            ", ",
            "bottomRight=",
            self.bottom_right,
            ", ",
            "bottomLeft=",
            self.bottom_left,
            ")"
        );
    }
}
