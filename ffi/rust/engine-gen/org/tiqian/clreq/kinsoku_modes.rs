use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::clreq::resolved_kinsoku::ResolvedKinsoku;


#[derive(Clone, Copy)]
pub struct KinsokuModes;

impl KinsokuModes {
    pub fn kinsoku_modes_resolve(mode: KinsokuMode, measure_em: f64) -> ResolvedKinsoku {
        return match mode {
            KinsokuMode::Fixed { level: _p0, hanging: _p1 } => KinsokuModes::kinsoku_modes_resolve_fixed(_p0, _p1),
            KinsokuMode::MeasureAdaptive { hang_below_em: _p0, gb_above_em: _p1, strict_above_em: _p2 } => KinsokuModes::kinsoku_modes_resolve_measure_adaptive(measure_em, _p0, _p1, _p2),
        };
    }

    pub fn kinsoku_modes_render(mode: KinsokuMode) -> String {
        return match mode {
            KinsokuMode::Fixed { level: _p0, hanging: _p1 } => format!("{}{}{}{}{}",
                    "Fixed(level=",
                    _p0.name(),
                    ", hanging=",
                    _p1.name(),
                    ")"
                ),
            KinsokuMode::MeasureAdaptive { hang_below_em: _p0, gb_above_em: _p1, strict_above_em: _p2 } => format!("{}{}{}{}{}{}{}",
                    "MeasureAdaptive(hangBelowEm=",
                    _p0,
                    ", gbAboveEm=",
                    _p1,
                    ", strictAboveEm=",
                    _p2,
                    ")"
                ),
        };
    }

    pub fn kinsoku_modes_same_mode(a: KinsokuMode, b: KinsokuMode) -> bool {
        return match a {
            KinsokuMode::Fixed { level: _p0, hanging: _p1 } => KinsokuModes::kinsoku_modes_same_fixed(_p0, _p1, (b).clone()),
            KinsokuMode::MeasureAdaptive { hang_below_em: _p0, gb_above_em: _p1, strict_above_em: _p2 } => KinsokuModes::kinsoku_modes_same_measure_adaptive(_p0, _p1, _p2, (b).clone()),
        };
    }

    pub(crate) fn kinsoku_modes_resolve_fixed(level: KinsokuLevel, hanging: HangingPunctuationStyle) -> ResolvedKinsoku {
        return ResolvedKinsoku::new(level, hanging, format!("{}{}{}",
            "Fixed:",
            level.name(),
            (if hanging != HangingPunctuationStyle::Disabled { "+Hang".to_string() } else { "".to_string() })
        ).as_str());
    }

    pub(crate) fn kinsoku_modes_resolve_measure_adaptive(measure_em: f64, hang_below_em: f64, gb_above_em: f64, strict_above_em: f64) -> ResolvedKinsoku {
        let level = if measure_em > (strict_above_em) { KinsokuLevel::Strict } else { if measure_em > (gb_above_em) { KinsokuLevel::GbStyle } else { KinsokuLevel::Basic } };
        let hanging = if measure_em < (hang_below_em) { HangingPunctuationStyle::PauseStops } else { HangingPunctuationStyle::Disabled };
        let tag = format!("{}{}{}{}{}",
            "MeasureAdaptiveKinsoku:",
            crate::runtime::int_text::IntText::int_text(u32::from_ne_bytes((match f64::from(measure_em) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match
((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e
=> u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes())),
            "字→",
            level.name(),
            (if hanging != HangingPunctuationStyle::Disabled { "+Hang".to_string() } else { "".to_string() })
        );
        return ResolvedKinsoku::new(level, hanging, tag.as_str());
    }

    pub(crate) fn kinsoku_modes_same_fixed(level: KinsokuLevel, hanging: HangingPunctuationStyle, b: KinsokuMode) -> bool {
        return match b {
            KinsokuMode::Fixed { level: _p0, hanging: _p1 } => level == _p0 && hanging == _p1,
            KinsokuMode::MeasureAdaptive { .. } => false,
        };
    }

    pub(crate) fn kinsoku_modes_same_measure_adaptive(hang_below_em: f64, gb_above_em: f64, strict_above_em: f64, b: KinsokuMode) -> bool {
        return match b {
            KinsokuMode::Fixed { .. } => false,
            KinsokuMode::MeasureAdaptive { hang_below_em: _p0, gb_above_em: _p1, strict_above_em: _p2 } => hang_below_em == _p0 && gb_above_em == _p1 && strict_above_em == _p2,
        };
    }
}
