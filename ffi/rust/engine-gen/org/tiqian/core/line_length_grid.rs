use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;


#[derive(Debug, Clone, PartialEq)]
pub struct LineLengthGrid {
    pub enabled: bool,
    pub body_alignment: Option<LastLineAlignment>,
}

impl LineLengthGrid {
    pub fn new(enabled: Option<bool>, body_alignment: Option<LastLineAlignment>) -> Self {
        let enabled = enabled.unwrap_or_else(|| true);
        let body_alignment = body_alignment.or_else(|| None);
        Self {
            enabled: enabled,
            body_alignment: body_alignment,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "LineLengthGrid(",
            "enabled=",
            self.enabled,
            ", ",
            "bodyAlignment=",
            (match &(self.body_alignment) { None => "null".to_string(), Some(__option) => (*__option).name().to_string() }),
            ")"
        );
    }
}
