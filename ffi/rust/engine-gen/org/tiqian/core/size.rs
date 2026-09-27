#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "Size(",
            "width=",
            self.width,
            ", ",
            "height=",
            self.height,
            ")"
        );
    }
}
