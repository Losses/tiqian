#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Rect {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn get_width(&self) -> f64 {
        return self.right - self.left;
    }

    pub fn get_height(&self) -> f64 {
        return self.bottom - self.top;
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}",
            "Rect(left=",
            self.left,
            ", top=",
            self.top,
            ", right=",
            self.right,
            ", bottom=",
            self.bottom,
            ")"
        );
    }
}
