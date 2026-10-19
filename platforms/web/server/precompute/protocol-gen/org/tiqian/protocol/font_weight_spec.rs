#[derive(Debug, Clone, PartialEq)]
pub enum FontWeightSpec {
    Range { min: f64, max: f64 },
    Single { value: Option<f64> },
}

impl FontWeightSpec {
    pub fn to_string(&self) -> String {
        match self {
            FontWeightSpec::Range { min, max } => format!("Range(min={}, max={})", (min).to_string(), (max).to_string()),
            FontWeightSpec::Single { value } => format!("Single(value={})", match value { Some(__o) => format!("{}", (__o).to_string()), None => "null".to_string() }),
        }
    }
}
