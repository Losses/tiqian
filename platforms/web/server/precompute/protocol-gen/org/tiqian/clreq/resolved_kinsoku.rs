use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedKinsoku {
    pub level: KinsokuLevel,
    pub hanging: HangingPunctuationStyle,
    pub reason: UString,
}

impl ResolvedKinsoku {
    pub fn new(level: KinsokuLevel, hanging: HangingPunctuationStyle, reason: &UStr) -> Self {
        Self {
            level,
            hanging,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ResolvedKinsoku(")); __s += &(UString::from("level=")); __s += UString::from(self.level.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("hanging=")); __s += UString::from(self.hanging.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

fn resolved_kinsoku_hanging_order(v: &HangingPunctuationStyle) -> i32 {
    match v {
        HangingPunctuationStyle::PauseStops => 1,
        HangingPunctuationStyle::Disabled => 0,
    }
}
fn resolved_kinsoku_level_order(v: &KinsokuLevel) -> i32 {
    match v {
        KinsokuLevel::Strict => 3,
        KinsokuLevel::None => 0,
        KinsokuLevel::GbStyle => 2,
        KinsokuLevel::Basic => 1,
    }
}
pub fn compare_resolved_kinsoku(a: &ResolvedKinsoku, b: &ResolvedKinsoku) -> i32 {
    let cmp_level = match resolved_kinsoku_level_order(&a.level).cmp(&resolved_kinsoku_level_order(&b.level)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_level != 0 { return cmp_level; }
    let cmp_hanging = match resolved_kinsoku_hanging_order(&a.hanging).cmp(&resolved_kinsoku_hanging_order(&b.hanging)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_hanging != 0 { return cmp_hanging; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
