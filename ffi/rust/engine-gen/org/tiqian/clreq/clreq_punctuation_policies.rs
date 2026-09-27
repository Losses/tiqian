use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_policy::PunctuationPolicy;
use crate::org::tiqian::clreq::punctuation_width_policy::PunctuationWidthPolicy;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;


static CLREQ_PUNCTUATION_POLICIES_OPENING_UNITS: [u32; 12] = [8220, 8216, 65288, 12298, 12296, 12300, 12302, 12304, 12308, 12310, 12312, 12314];
static CLREQ_PUNCTUATION_POLICIES_CLOSING_UNITS: [u32; 12] = [8221, 8217, 65289, 12299, 12297, 12301, 12303, 12305, 12309, 12311, 12313, 12315];
static CLREQ_PUNCTUATION_POLICIES_PAUSE_OR_STOP_UNITS: [u32; 7] = [65292, 12289, 12290, 65307, 65306, 65281, 65311];
static CLREQ_PUNCTUATION_POLICIES_INTERPUNCT_UNITS: [u32; 3] = [12539, 8231, 8226];
static CLREQ_PUNCTUATION_POLICIES_CONNECTOR_UNITS: [u32; 4] = [65374, 126, 45, 8211];
static CLREQ_PUNCTUATION_POLICIES_SOLIDUS_UNITS: [u32; 2] = [47, 65295];
static CLREQ_PUNCTUATION_POLICIES_ELLIPSIS_UNITS: [u32; 2] = [8230, 8943];
static CLREQ_PUNCTUATION_POLICIES_DASH_UNITS: [u32; 2] = [8212, 11834];
static CLREQ_PUNCTUATION_POLICIES_SHORT_HYPHEN_CONNECTORS: [u32; 2] = [45, 8211];
static CLREQ_PUNCTUATION_POLICIES_SENTENCE_END_STOPS: [u32; 4] = [12290, 65281, 65311, 65294];
static CLREQ_PUNCTUATION_POLICIES_ASCII_POINT_MARKS: [u32; 6] = [44, 46, 58, 59, 33, 63];

#[derive(Clone, Copy)]
pub struct ClreqPunctuationPolicies;

impl ClreqPunctuationPolicies {

    pub fn clreq_punctuation_policies_is_ascii_point_mark(char: &UStr) -> bool {
        return ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_ASCII_POINT_MARKS, u_string::unit_at(&char, 0u32));
    }

    pub fn clreq_punctuation_policies_classify(char: &UStr) -> PunctuationClass {
        let unit = u_string::unit_at(&char, 0u32).unwrap_or(0);
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_OPENING_UNITS, Some(unit)) {
            return PunctuationClass::Opening;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_CLOSING_UNITS, Some(unit)) {
            return PunctuationClass::Closing;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_PAUSE_OR_STOP_UNITS, Some(unit)) {
            return PunctuationClass::PauseOrStop;
        }
        if unit == 183 {
            return PunctuationClass::MiddleDot;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_INTERPUNCT_UNITS, Some(unit)) {
            return PunctuationClass::Interpunct;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_CONNECTOR_UNITS, Some(unit)) {
            return PunctuationClass::Connector;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_SOLIDUS_UNITS, Some(unit)) {
            return PunctuationClass::Solidus;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_ELLIPSIS_UNITS, Some(unit)) {
            return PunctuationClass::Ellipsis;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_DASH_UNITS, Some(unit)) {
            return PunctuationClass::Dash;
        }
        return PunctuationClass::Other;
    }

    pub fn clreq_punctuation_policies_forced_half_width(char: &UStr, policy: PunctuationWidthPolicy) -> bool {
        let unit = u_string::unit_at(&char, 0u32).unwrap_or(0);
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_SHORT_HYPHEN_CONNECTORS, Some(unit)) {
            return true;
        }
        let cls = ClreqPunctuationPolicies::clreq_punctuation_policies_classify(char);
        if policy.gb_fixed_separators && (cls == PunctuationClass::Connector || cls == PunctuationClass::MiddleDot || cls == PunctuationClass::Interpunct || cls == PunctuationClass::Solidus) {
            return true;
        }
        if policy.interior == InteriorPunctuationStyle::Kaiming {
            if cls == PunctuationClass::Opening || cls == PunctuationClass::Closing {
                return true;
            }
            if cls == PunctuationClass::PauseOrStop && !ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_SENTENCE_END_STOPS, Some(unit)) {
                return true;
            }
        }
        return false;
    }

    pub fn clreq_punctuation_policies_policy_for(char: &UStr) -> PunctuationPolicy {
    let __units = u_string::units(&char);
    let __count = u_string::unit_count(&char);
        let punctuation_class = ClreqPunctuationPolicies::clreq_punctuation_policies_classify(char);
        return PunctuationPolicy::new(punctuation_class, !ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(char, KinsokuLevel::Basic), !ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(char, KinsokuLevel::Basic), ClreqPunctuationPolicies::clreq_punctuation_policies_default_punctuation_body_em(u_string::unit_at_from(&__units, 0u32), punctuation_class), Some(ClreqPunctuationPolicies::clreq_punctuation_policies_default_punctuation_advance_em(u_string::unit_at_from(&__units, 0u32), punctuation_class)));
    }

    pub fn clreq_punctuation_policies_forbidden_at_line_start(char: &UStr, level: KinsokuLevel) -> bool {
        if level == KinsokuLevel::None {
            return false;
        }
        let cls = ClreqPunctuationPolicies::clreq_punctuation_policies_classify(char);
        if cls == PunctuationClass::PauseOrStop || cls == PunctuationClass::Closing || cls == PunctuationClass::Connector || cls == PunctuationClass::MiddleDot || cls == PunctuationClass::Interpunct || cls == PunctuationClass::Solidus {
            return true;
        }
        if cls == PunctuationClass::Dash || cls == PunctuationClass::Ellipsis {
            return level == KinsokuLevel::Strict;
        }
        return false;
    }

    pub fn clreq_punctuation_policies_forbidden_at_line_end(char: &UStr, level: KinsokuLevel) -> bool {
        if level == KinsokuLevel::None {
            return false;
        }
        let cls = ClreqPunctuationPolicies::clreq_punctuation_policies_classify(char);
        if cls == PunctuationClass::Opening {
            return true;
        }
        if cls == PunctuationClass::Solidus {
            return level != KinsokuLevel::Basic;
        }
        return false;
    }

    pub(crate) fn clreq_punctuation_policies_default_punctuation_body_em(unit: Option<u32>, punctuation_class: PunctuationClass) -> f64 {
        if unit.as_ref().map_or(false, |v| v == &(11834)) {
            return 2.0f64;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_SHORT_HYPHEN_CONNECTORS, unit) {
            return 0.5f64;
        }
        if punctuation_class == PunctuationClass::PauseOrStop {
            return 0.5f64;
        }
        if punctuation_class == PunctuationClass::Closing {
            return 0.5f64;
        }
        if punctuation_class == PunctuationClass::Opening {
            return 0.5f64;
        }
        return 1.0f64;
    }

    pub(crate) fn clreq_punctuation_policies_default_punctuation_advance_em(unit: Option<u32>, _punctuation_class: PunctuationClass) -> f64 {
        if unit.as_ref().map_or(false, |v| v == &(11834)) {
            return 2.0f64;
        }
        if ClreqPunctuationPolicies::clreq_punctuation_policies_contains_unit(&CLREQ_PUNCTUATION_POLICIES_SHORT_HYPHEN_CONNECTORS, unit) {
            return 0.5f64;
        }
        return 1.0f64;
    }

    pub(crate) fn clreq_punctuation_policies_contains_unit(units: &[u32], unit: Option<u32>) -> bool {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((units.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if unit.as_ref().map_or(false, |v| v == &(units[usize::try_from(index).unwrap_or(0)])) {
                return true;
            }
            index = u32::wrapping_add(index, 1);
        }
        return false;
    }
}
