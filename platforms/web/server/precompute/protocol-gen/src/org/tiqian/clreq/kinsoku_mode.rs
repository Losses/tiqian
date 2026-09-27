use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;


#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuMode {
    Fixed { level: KinsokuLevel, hanging: HangingPunctuationStyle },
    MeasureAdaptive { hang_below_em: f64, gb_above_em: f64, strict_above_em: f64 },
}

impl KinsokuMode {
    pub fn to_string(&self) -> String {
        match self {
            KinsokuMode::Fixed { level, hanging } => format!("Fixed(level={}, hanging={})", (level).to_string(), (hanging).to_string()),
            KinsokuMode::MeasureAdaptive { hang_below_em, gb_above_em, strict_above_em } => format!("MeasureAdaptive(hang_below_em={}, gb_above_em={}, strict_above_em={})", (hang_below_em).to_string(), (gb_above_em).to_string(), (strict_above_em).to_string()),
        }
    }
}
