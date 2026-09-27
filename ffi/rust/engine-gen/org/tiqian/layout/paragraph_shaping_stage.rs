use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::core::accurate_sum::AccurateSum;
use crate::org::tiqian::core::break_opportunity_decision_info::BreakOpportunityDecisionInfo;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::emergency_tracking_eligibility_decision_info::EmergencyTrackingEligibilityDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_word_character_data::UnicodeWordCharacterData;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::layout::cluster_role_resolution::ResolvedClusterRange;
use crate::org::tiqian::layout::contextual_punctuation_display_substitution::ContextualPunctuationDisplaySubstitutionFns;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineFns;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_tier_priority::ProgressiveBreakTierPriority;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShapingStageShapeParagraphFault {
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
}

impl From<ParagraphShapingStageShapeParagraphFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: ParagraphShapingStageShapeParagraphFault) -> Self {
        match value {
            ParagraphShapingStageShapeParagraphFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageShapeParagraphFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: ParagraphShapingStageShapeParagraphFault) -> Self {
        match value {
            ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageShapeParagraphFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: ParagraphShapingStageShapeParagraphFault) -> Self {
        match value {
            ParagraphShapingStageShapeParagraphFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<ParagraphShapingStageShapeParagraphFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: ParagraphShapingStageShapeParagraphFault) -> Self {
        match value {
            ParagraphShapingStageShapeParagraphFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for ParagraphShapingStageShapeParagraphFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        ParagraphShapingStageShapeParagraphFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for ParagraphShapingStageShapeParagraphFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for ParagraphShapingStageShapeParagraphFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        ParagraphShapingStageShapeParagraphFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for ParagraphShapingStageShapeParagraphFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        ParagraphShapingStageShapeParagraphFault::TextShaperShapeFaultFault(value)
    }
}

#[derive(Clone, PartialEq)]
pub struct ParagraphShapingStageResult {
    pub shaping_results: Vec<ShapingResult>,
    pub hyphen_offsets: SortedSetTable<u32>,
    pub hyphen_advance: f64,
    pub hyphen_glyphs: Vec<Glyph>,
    pub substitution_rollbacks: SortedMapTable<TextRange, String>,
    pub break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>,
    pub emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>,
    pub progressive_break_offsets: SortedMapTable<u32, ProgressiveBreakOpportunity>,
    pub segment_shaping_cache: SortedMapTable<TextRange, ShapingResult>,
}

impl ParagraphShapingStageResult {
    pub fn new(shaping_results: Vec<ShapingResult>, hyphen_offsets: SortedSetTable<u32>, hyphen_advance: f64, hyphen_glyphs: Vec<Glyph>, substitution_rollbacks: SortedMapTable<TextRange, String>, break_opportunity_decisions: Vec<BreakOpportunityDecisionInfo>,
emergency_tracking_eligibility_decisions: Vec<EmergencyTrackingEligibilityDecisionInfo>, progressive_break_offsets: SortedMapTable<u32, ProgressiveBreakOpportunity>, segment_shaping_cache: Option<SortedMapTable<TextRange, ShapingResult>>) -> Self {
        let segment_shaping_cache = segment_shaping_cache.unwrap_or_else(|| SortedTable::sorted_table_map_builder::<TextRange, ShapingResult>(Arc::new(compare_text_range)).build());
        Self {
            shaping_results,
            hyphen_offsets,
            hyphen_advance,
            hyphen_glyphs,
            substitution_rollbacks,
            break_opportunity_decisions,
            emergency_tracking_eligibility_decisions,
            progressive_break_offsets,
            segment_shaping_cache: segment_shaping_cache,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ParagraphShapingStageResult(",
            "shapingResults=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.shaping_results).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "hyphenOffsets=",
            {
        let mut out = String::new();
        out.push('[');
        let set = (self.hyphen_offsets).clone();
        let n = set.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", crate::runtime::int_text::IntText::int_text(set.at(i)));
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "hyphenAdvance=",
            self.hyphen_advance,
            ", ",
            "hyphenGlyphs=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.hyphen_glyphs).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "substitutionRollbacks=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.substitution_rollbacks).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", map.key_at(i).to_string(), map.value_at(i));
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "breakOpportunityDecisions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.break_opportunity_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "emergencyTrackingEligibilityDecisions=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.emergency_tracking_eligibility_decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "progressiveBreakOffsets=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.progressive_break_offsets).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", crate::runtime::int_text::IntText::int_text(map.key_at(i)), map.value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    },
            ", ",
            "segmentShapingCache=",
            {
        let mut out = String::new();
        out.push('{');
        let map = (self.segment_shaping_cache).clone();
        let n = map.size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", map.key_at(i).to_string(), map.value_at(i).to_string());
            i += 1;
        }
        out.push('}');
        out
    },
            ")"
        );
    }
}

#[derive(Clone, Copy)]
pub struct ParagraphShapingStage;

impl ParagraphShapingStage {
    pub fn paragraph_shaping_stage_is_digit(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 48 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 57;
    }

    pub fn paragraph_shaping_stage_is_letter(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 65 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 90 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 97 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 122 || UnicodeWordCharacterData::unicode_word_character_data_contains(c) &&
!ParagraphShapingStage::paragraph_shaping_stage_is_digit(c);
    }

    pub fn paragraph_shaping_stage_is_letter_or_digit(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 48 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 57 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 65 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 90 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 97 &&
(i32::from_ne_bytes((c).to_ne_bytes())) <= 122 || UnicodeWordCharacterData::unicode_word_character_data_contains(c);
    }

    pub fn paragraph_shaping_stage_is_upper_case(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 65 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 90;
    }

    pub fn paragraph_shaping_stage_is_lower_case(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 97 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 122;
    }

    pub fn paragraph_shaping_stage_is_whitespace(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 9 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 13 || c == 32 || c == 160 || c == 5760 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 8192 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 8202 || c == 8232 || c == 8233 || c ==
8239 || c == 8287 || c == 12288;
    }

    pub fn paragraph_shaping_stage_is_all_digits(s: &str) -> bool {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        if __count == 0 {
            return false;
        }
        for i in 0..match u32::try_from(u_string::unit_count(&(s))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units, i).unwrap_or(0);
            if i32::from_ne_bytes((c).to_ne_bytes()) < (48) || (i32::from_ne_bytes((c).to_ne_bytes())) > (57) {
                return false;
            }
        }
        return true;
    }

    pub fn paragraph_shaping_stage_is_shared_curly_quote(c: u32) -> bool {
        return c == 8216 || c == 8217 || c == 8220 || c == 8221;
    }

    pub fn paragraph_shaping_stage_is_progressive_technical_break_after_char(c: u32) -> bool {
        return c == 47 || c == 92 || c == 46 || c == 45 || c == 95 || c == 58 || c == 59 || c == 44 || c == 63 || c == 38 || c == 61 || c == 35 || c == 37 || c == 126 || c == 43 || c == 42 || c == 124 || c == 41 || c == 93 || c == 125;
    }

    pub fn paragraph_shaping_stage_sort_ints(arr: &mut Vec<u32>) {
        let mut i = 1u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((arr.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let current = arr[usize::try_from(i).unwrap_or(0)];
            let mut j = i;
            while (j) > (0) && ({ let v: u32 = arr[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = current; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                { while arr.len() <= usize::try_from(j).unwrap_or(0) { arr.push(0); } arr[usize::try_from(j).unwrap_or(0)] = arr[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; };
                j = u32::wrapping_sub(j, 1);
            }
            { while arr.len() <= usize::try_from(j).unwrap_or(0) { arr.push(0); } arr[usize::try_from(j).unwrap_or(0)] = current; };
            i = u32::wrapping_add(i, 1);
        }
    }

    pub fn paragraph_shaping_stage_tier_name(tier: ProgressiveBreakTier) -> String {
        if tier == ProgressiveBreakTier::Whitespace {
            return "Whitespace".to_string();
        }
        if tier == ProgressiveBreakTier::Structural {
            return "Structural".to_string();
        }
        if tier == ProgressiveBreakTier::Syllable {
            return "Syllable".to_string();
        }
        if tier == ProgressiveBreakTier::WholeToken {
            return "WholeToken".to_string();
        }
        if tier == ProgressiveBreakTier::Emergency {
            return "Emergency".to_string();
        }
        return "Unknown".to_string();
    }

    pub fn paragraph_shaping_stage_cjk_punctuation_full_width_features(role: FontRole, display_text: &str) -> Vec<String> {
    let __units1 = u_string::units(&display_text);
    let __count1 = u_string::unit_count(&display_text);
        if role == FontRole::CjkPunctuation {
            for i in 0..match u32::try_from(u_string::unit_count(&(display_text))) { Ok(value) => value, Err(_) => u32::MAX } {
                let c = u_string::unit_at_from(&__units1, i).unwrap_or(0);
                if ParagraphShapingStage::paragraph_shaping_stage_is_shared_curly_quote(c) {
                    return vec!["fwid=1".to_string()];
                }
            }
        }
        return vec![];
    }

    pub fn paragraph_shaping_stage_is_url_like_latin_token(token: &str) -> bool {
        let lower = token.to_lowercase();
        return (u32::from_ne_bytes((u_string::find_from(&token, "://", 0)).to_ne_bytes())) <= 2147483647 || (lower).starts_with(&"www.") || ParagraphShapingStage::paragraph_shaping_stage_has_domain_like_dot(token);
    }

    pub fn paragraph_shaping_stage_has_domain_like_dot(token: &str) -> bool {
    let __units2 = u_string::units(&token);
    let __count2 = u_string::unit_count(&token);
        for i in 0..match u32::try_from(u_string::unit_count(&(token))) { Ok(value) => value, Err(_) => u32::MAX } {
            if !(u_string::unit_at_from(&__units2, i).as_ref().map_or(false, |v| v == &(46))) || i == 0 || (i32::from_ne_bytes((u32::wrapping_add(i, 2)).to_ne_bytes())) >= i32::from_ne_bytes((__count2).to_ne_bytes()) {
                continue;
            }
            if !ParagraphShapingStage::paragraph_shaping_stage_is_letter_or_digit(*(u_string::unit_at_from(&__units2, u32::wrapping_sub(i, 1))).as_ref().unwrap()) || !ParagraphShapingStage::paragraph_shaping_stage_is_letter_or_digit(*(u_string::unit_at_from(&__units2,
u32::wrapping_add(i, 1))).as_ref().unwrap()) {
                continue;
            }
            let mut tld = 0u32;
            let mut j = u32::wrapping_add(i, 1);
            let __units3 = u_string::units(&token);
            let __count3 = u_string::unit_count(&token);
            while (i32::from_ne_bytes((j).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) && ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at_from(&__units2, j)).as_ref().unwrap()) {
                tld = u32::wrapping_add(tld, 1);
                j = u32::wrapping_add(j, 1);
            }
            if i32::from_ne_bytes((tld).to_ne_bytes()) >= 2 {
                return true;
            }
        }
        return false;
    }

    pub fn paragraph_shaping_stage_is_latin_token_break_after(token: &str, index: u32, keep_url_scheme: bool) -> bool {
    let __units4 = u_string::units(&token);
    let __count4 = u_string::unit_count(&token);
        if index > 2147483647 || (i32::from_ne_bytes((index).to_ne_bytes())) >= i32::from_ne_bytes((u32::wrapping_sub(__count4, 1)).to_ne_bytes()) {
            return false;
        }
        let c = u_string::unit_at_from(&__units4, index).unwrap_or(0);
        if c == 47 {
            return !keep_url_scheme || (index == 0 || !(u_string::unit_at_from(&__units4, u32::wrapping_sub(index, 1)).as_ref().map_or(false, |v| v == &(58))));
        }
        if c == 46 || c == 45 || c == 95 || c == 63 || c == 38 || c == 61 || c == 35 || c == 37 || c == 126 {
            return true;
        }
        return false;
    }

    pub fn paragraph_shaping_stage_bibliographic_numeric_locator_break_offsets(token: &str) -> Vec<u32> {
    let __units5 = u_string::units(&token);
    let __count5 = u_string::unit_count(&token);
        let open = u_string::find_from(&token, "(", 0);
        if open <= 0 || !ParagraphShapingStage::paragraph_shaping_stage_is_digit(*(u_string::unit_at_from(&__units5, 0u32)).as_ref().unwrap()) {
            return vec![];
        }
        let close = u_string::find_from(&token, ")", i32::from_ne_bytes((i32::wrapping_add(open, 1)).to_ne_bytes()));
        if close <= i32::wrapping_add(open, 1) {
            return vec![];
        }
        let colon = u_string::find_from(&token, ":", i32::from_ne_bytes((i32::wrapping_add(close, 1)).to_ne_bytes()));
        if colon != i32::wrapping_add(close, 1) || (colon) >= i32::wrapping_sub(i32::from_ne_bytes((__count5).to_ne_bytes()), 1) {
            return vec![];
        }
        let volume = u_string::substring(&token, 0i32, i32::from_ne_bytes((open).to_ne_bytes()));
        let issue = u_string::substring(&token, i32::from_ne_bytes((i32::wrapping_add(open, 1)).to_ne_bytes()), i32::from_ne_bytes((close).to_ne_bytes()));
        let mut pages = u_string::substring_from(&token, i32::from_ne_bytes((i32::wrapping_add(colon, 1)).to_ne_bytes()));
        if pages.ends_with(&".") {
            pages = u_string::substring(&pages, 0i32, i32::from_ne_bytes((u32::wrapping_sub(u_string::unit_count(&(pages)), 1)).to_ne_bytes()));
        }
        if u_string::unit_count(&(volume)) == 0 || u_string::unit_count(&(issue)) == 0 || u_string::unit_count(&(pages)) == 0 {
            return vec![];
        }
        if !ParagraphShapingStage::paragraph_shaping_stage_is_all_digits(volume.as_str()) || !ParagraphShapingStage::paragraph_shaping_stage_is_all_digits(issue.as_str()) {
            return vec![];
        }
        let mut range_separator = -1i32;
        for i in 0..match u32::try_from(u_string::unit_count(&(pages))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at(&pages, i).unwrap_or(0);
            if c == 45 || c == 8211 || c == 8212 {
                range_separator = i32::from_ne_bytes((i).to_ne_bytes());
                break;
            }
        }
        let pages_are_numeric = if range_separator < (0) { ParagraphShapingStage::paragraph_shaping_stage_is_all_digits(pages.as_str()) } else { range_separator > (0) && (range_separator) < (i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(pages))).to_ne_bytes()), 1))
&& ParagraphShapingStage::paragraph_shaping_stage_is_all_digits(u_string::substring(&pages, 0i32, i32::from_ne_bytes((range_separator).to_ne_bytes())).as_str()) && ParagraphShapingStage::paragraph_shaping_stage_is_all_digits(u_string::substring_from(&pages,
i32::from_ne_bytes((i32::wrapping_add(range_separator, 1)).to_ne_bytes())).as_str()) };
        if !pages_are_numeric {
            return vec![];
        }
        return vec![
    u32::from_ne_bytes((open).to_ne_bytes()),
    u32::from_ne_bytes((i32::wrapping_add(colon, 1)).to_ne_bytes()),
];
    }

    pub fn paragraph_shaping_stage_has_breakable_latin_solidus(token: &str) -> bool {
    let __units6 = u_string::units(&token);
    let __count6 = u_string::unit_count(&token);
        for i in 1..__count6.saturating_sub(1) {
            if u_string::unit_at_from(&__units6, i).as_ref().map_or(false, |v| v == &(47)) && ParagraphShapingStage::paragraph_shaping_stage_is_letter_or_digit(*(u_string::unit_at_from(&__units6, u32::wrapping_sub(i, 1))).as_ref().unwrap()) &&
ParagraphShapingStage::paragraph_shaping_stage_is_letter_or_digit(*(u_string::unit_at_from(&__units6, u32::wrapping_add(i, 1))).as_ref().unwrap()) {
                return true;
            }
        }
        return false;
    }

    pub fn paragraph_shaping_stage_existing_hyphen_cuts(text: &str, word_range: TextRange) -> Vec<u32> {
        let w = u_string::substring(&text, i32::from_ne_bytes((word_range.start).to_ne_bytes()), i32::from_ne_bytes((word_range.end).to_ne_bytes()));
        let mut cuts: Vec<u32> = Vec::new();
        for i in 0..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
            if !(u_string::unit_at(&w, i).as_ref().map_or(false, |v| v == &(45))) {
                continue;
            }
            let mut before = 0u32;
            let mut j = i;
            while (j) > (0) && ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at(&w, u32::wrapping_sub(j, 1))).as_ref().unwrap()) {
                before = u32::wrapping_add(before, 1);
                j = u32::wrapping_sub(j, 1);
            }
            let mut after = 0u32;
            let mut k = u32::wrapping_add(i, 1);
            let __units7 = u_string::units(&w);
            let __count7 = u_string::unit_count(&w);
            while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((__count7).to_ne_bytes())) && ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at_from(&__units7, k)).as_ref().unwrap()) {
                after = u32::wrapping_add(after, 1);
                k = u32::wrapping_add(k, 1);
            }
            if i32::from_ne_bytes((before).to_ne_bytes()) >= 2 && (i32::from_ne_bytes((after).to_ne_bytes())) >= 2 {
                cuts.push(u32::wrapping_add(u32::wrapping_add(word_range.start, i), 1));
            }
        }
        return cuts;
    }

    pub fn paragraph_shaping_stage_camel_case_cuts(text: &str, word_range: TextRange) -> Vec<u32> {
        let w = u_string::substring(&text, i32::from_ne_bytes((word_range.start).to_ne_bytes()), i32::from_ne_bytes((word_range.end).to_ne_bytes()));
        let mut humps: Vec<u32> = Vec::new();
        for i in 1..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at(&w, i).unwrap_or(0);
            let prev = u_string::unit_at(&w, u32::wrapping_sub(i, 1)).unwrap_or(0);
            if ParagraphShapingStage::paragraph_shaping_stage_is_upper_case(c) && (ParagraphShapingStage::paragraph_shaping_stage_is_lower_case(prev) || ParagraphShapingStage::paragraph_shaping_stage_is_upper_case(prev) && (i32::from_ne_bytes((u32::wrapping_add(i,
1)).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(w))).to_ne_bytes())) && ParagraphShapingStage::paragraph_shaping_stage_is_lower_case(*(u_string::unit_at(&w, u32::wrapping_add(i, 1))).as_ref().unwrap())) {
                humps.push(i);
            }
        }
        let mut bounds = vec![0];
        for h in 0..match u32::try_from(humps.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            bounds.push(humps[usize::try_from(h).unwrap_or(0)]);
        }
        bounds.push(u_string::unit_count(&(w)));
        let mut result: Vec<u32> = Vec::new();
        for &h in &humps {
            let mut last_before = 0u32;
            for b in 0..match u32::try_from(bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = bounds[usize::try_from(b).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = h; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                    last_before = bounds[usize::try_from(b).unwrap_or(0)];
                }
            }
            let mut first_after = u_string::unit_count(&(w));
            for b in 0..match u32::try_from(bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = bounds[usize::try_from(b).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = h; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                    first_after = bounds[usize::try_from(b).unwrap_or(0)];
                    break;
                }
            }
            if i32::from_ne_bytes((u32::wrapping_sub(h, last_before)).to_ne_bytes()) >= 2 && (i32::from_ne_bytes((u32::wrapping_sub(first_after, h)).to_ne_bytes())) >= 2 {
                result.push(u32::wrapping_add(word_range.start, h));
            }
        }
        return result;
    }

    pub fn paragraph_shaping_stage_alpha_numeric_transition_cuts(text: &str, word_range: TextRange) -> Vec<u32> {
        let w = u_string::substring(&text, i32::from_ne_bytes((word_range.start).to_ne_bytes()), i32::from_ne_bytes((word_range.end).to_ne_bytes()));
        let mut cuts: Vec<u32> = Vec::new();
        for index in 1..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
            let left = u_string::unit_at(&w, u32::wrapping_sub(index, 1)).unwrap_or(0);
            let right = u_string::unit_at(&w, index).unwrap_or(0);
            if ParagraphShapingStage::paragraph_shaping_stage_is_letter(left) && ParagraphShapingStage::paragraph_shaping_stage_is_digit(right) || ParagraphShapingStage::paragraph_shaping_stage_is_digit(left) && ParagraphShapingStage::paragraph_shaping_stage_is_letter(right) {
                cuts.push(u32::wrapping_add(word_range.start, index));
            }
        }
        return cuts;
    }

    pub fn paragraph_shaping_stage_strong_non_lexical_reason(w: &str) -> Option<String> {
    let __units8 = u_string::units(&w);
    let __count8 = u_string::unit_count(&w);
        if i32::from_ne_bytes((__count8).to_ne_bytes()) < (12) {
            return None;
        }
        let mut all_letters = true;
        let first_char_lower = u_string::char_at_from(&__units8, 0u32).to_lowercase();
        let mut all_same_letter = true;
        for i in 0..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units8, i).unwrap_or(0);
            if !ParagraphShapingStage::paragraph_shaping_stage_is_letter(c) {
                all_letters = false;
                all_same_letter = false;
            } else {
                if u_string::char_at_from(&__units8, i).to_lowercase() != first_char_lower {
                    all_same_letter = false;
                }
            }
        }
        if all_letters && all_same_letter {
            return Some("LongRepeatedLetterRun".to_string());
        }
        let mut any_letter = false;
        let mut all_hex_or_digit = true;
        for i in 0..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units8, i).unwrap_or(0);
            if ParagraphShapingStage::paragraph_shaping_stage_is_letter(c) {
                any_letter = true;
            }
            let is_hex_digit = ParagraphShapingStage::paragraph_shaping_stage_is_digit(c) || (i32::from_ne_bytes((c).to_ne_bytes())) >= 65 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 70 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 97 && (i32::from_ne_bytes((c).to_ne_bytes()))
<= 102;
            if !is_hex_digit {
                all_hex_or_digit = false;
            }
        }
        if any_letter && all_hex_or_digit {
            return Some("LongHexIdentityRun".to_string());
        }
        let mut any_digit = false;
        for i in 0..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
            if ParagraphShapingStage::paragraph_shaping_stage_is_digit(*(u_string::unit_at_from(&__units8, i)).as_ref().unwrap()) {
                any_digit = true;
                break;
            }
        }
        if any_letter && any_digit {
            let mut transitions = 0u32;
            for i in 0..__count8.saturating_sub(1) {
                let left = u_string::unit_at_from(&__units8, i).unwrap_or(0);
                let right = u_string::unit_at_from(&__units8, u32::wrapping_add(i, 1)).unwrap_or(0);
                if ParagraphShapingStage::paragraph_shaping_stage_is_letter(left) && ParagraphShapingStage::paragraph_shaping_stage_is_digit(right) || ParagraphShapingStage::paragraph_shaping_stage_is_digit(left) && ParagraphShapingStage::paragraph_shaping_stage_is_letter(right)
{
                    transitions = u32::wrapping_add(transitions, 1);
                }
            }
            if i32::from_ne_bytes((transitions).to_ne_bytes()) >= 2 {
                return Some("LongMixedAlphaNumericIdentifier".to_string());
            }
        }
        return None;
    }

    pub fn paragraph_shaping_stage_mandatory_break_shaping_result(text: &str, range: TextRange) -> ShapingResult {
        let source_text = u_string::substring(&text, i32::from_ne_bytes((range.start).to_ne_bytes()), i32::from_ne_bytes((range.end).to_ne_bytes()));
        let cluster = Cluster::new((range).clone(), source_text.as_str(), ParagraphLayoutEngineFns::PARAGRAPH_LAYOUT_ENGINE_FNS_MANDATORY_BREAK_FONT_KEY.to_string().as_str(), 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        return ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![].to_vec(), Some(vec![]));
    }

    pub fn paragraph_shaping_stage_zero_width_soft_break_shaping_result(text: &str, range: TextRange) -> ShapingResult {
        let source_text = u_string::substring(&text, i32::from_ne_bytes((range.start).to_ne_bytes()), i32::from_ne_bytes((range.end).to_ne_bytes()));
        let cluster = Cluster::new((range).clone(), source_text.as_str(), "zero-width-space", 0.0f64, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let decision = ShapingDecisionInfo::new((range).clone(), source_text.as_str(), "", "zero-width-space", 0u32, 0.0f64, "StructuralControl", "ZeroWidthSpaceSoftBreakNoShape", Some(0), Some(0), None, None, None, None, None, None);
        return ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![].to_vec(), Some(vec![(decision).clone()]));
    }

    pub fn paragraph_shaping_stage_inline_object_shaping_result(text: &str, inline_object: InlineObjectSpan) -> ShapingResult {
        let source_text = u_string::substring(&text, i32::from_ne_bytes(((inline_object.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((inline_object.range).clone().end).to_ne_bytes()));
        let cluster = Cluster::new((inline_object.range).clone(), source_text.as_str(), "inline-object", inline_object.advance, Some("".to_string()), Some(0.0), Some(0.0), Some(0.0));
        let decision = ShapingDecisionInfo::new((inline_object.range).clone(), source_text.as_str(), "", "inline-object", 0u32, inline_object.advance, "InlineObject", "MeasurableOpaqueInlineObject:no-font-shaping", Some(0), Some(0), None, None, None, None, None, None);
        return ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![].to_vec(), Some(vec![(decision).clone()]));
    }

    pub fn paragraph_shaping_stage_map_to_cluster_range(glyphs: &Vec<Glyph>, cluster: Cluster) -> Vec<Glyph> {
        let mut source_terms: Vec<f64> = vec![];
        for i in 0..match u32::try_from(glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            source_terms.push(glyphs[usize::try_from(i).unwrap_or(0)].advance);
        }
        let source_advance = AccurateSum::accurate_sum_of(&source_terms);
        if source_advance <= 0.0f64 {
            let count = if i32::from_ne_bytes((u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (1) { u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) } else { 1 };
            let fallback_advance = cluster.advance / format!("{}", { let v: u32 = count; i32::from_ne_bytes(v.to_ne_bytes()) }).parse::<f64>().unwrap_or(0.0);
            let capacity = glyphs.len();
            let mut result = Vec::with_capacity(capacity);
            for i in 0..match u32::try_from(glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let g = (glyphs[usize::try_from(i).unwrap_or(0)]).clone();
                result.push(Glyph::new(g.id, (cluster.range).clone(), fallback_advance, Some(g.x), Some(g.y), g.render_font_key.clone(), (g.bounds).clone(), g.halt_advance, g.halt_placement_x));
            }
            return result;
        }
        let capacity = glyphs.len();
        let mut result = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let g = (glyphs[usize::try_from(i).unwrap_or(0)]).clone();
            result.push(Glyph::new(g.id, (cluster.range).clone(), g.advance, Some(g.x), Some(g.y), g.render_font_key.clone(), (g.bounds).clone(), g.halt_advance, g.halt_placement_x));
        }
        return result;
    }

    pub fn paragraph_shaping_stage_is_mandatory_break_cluster(cluster: Cluster) -> bool {
        return (cluster.font_key).to_string() == ParagraphLayoutEngineFns::PARAGRAPH_LAYOUT_ENGINE_FNS_MANDATORY_BREAK_FONT_KEY.to_string() && u_string::unit_count(&((cluster.display_text).to_string())) == 0;
    }

    pub fn paragraph_shaping_stage_is_zero_width_soft_break_cluster(cluster: Cluster) -> bool {
        return (cluster.font_key).to_string() == "zero-width-space" && u_string::unit_count(&((cluster.display_text).to_string())) == 0;
    }

    pub fn paragraph_shaping_stage_is_inline_object_cluster(cluster: Cluster) -> bool {
        return (cluster.font_key).to_string() == "inline-object";
    }

    pub fn paragraph_shaping_stage_shaping_segments(decision: FontDecision, text: &str) -> Result<Vec<TextRange>, TextRangeError> {
    let __units9 = u_string::units(&text);
    let __count9 = u_string::unit_count(&text);
        if decision.role != FontRole::LatinText {
            return Ok(vec![(decision.range).clone()]);
        }
        let mut segments: Vec<TextRange> = Vec::new();
        let mut seg_start = (decision.range).clone().start;
        let mut in_space = u_string::unit_at_from(&__units9, (decision.range).clone().start).as_ref().map_or(false, |v| v == &(32));
        let mut i = u32::wrapping_add((decision.range).clone().start, 1);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes(((decision.range).clone().end).to_ne_bytes())) {
            let is_space = u_string::unit_at_from(&__units9, i).as_ref().map_or(false, |v| v == &(32));
            if is_space != in_space {
                segments.push(TextRange::new(seg_start, i)?);
                seg_start = i;
                in_space = is_space;
            }
            i = u32::wrapping_add(i, 1);
        }
        segments.push(TextRange::new(seg_start, (decision.range).clone().end)?);
        return Ok(segments);
    }

    pub(crate) fn paragraph_shaping_stage_copy_font_families(families: &[String]) -> Vec<String> {
        let capacity = families.len();
        let mut result = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(families.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            result.push((families[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return result;
    }

    pub(crate) fn paragraph_shaping_stage_copy_ints(arr: &Vec<u32>) -> Vec<u32> {
        let capacity = arr.len();
        let mut result = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(arr.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            result.push(arr[usize::try_from(i).unwrap_or(0)]);
        }
        return result;
    }

    pub fn paragraph_shaping_stage_shape_paragraph(engine: &mut ExplainableStubParagraphLayoutEngine, input: LayoutInput, text: &str, font_size: f64, measure: f64, cluster_ranges: &Vec<ResolvedClusterRange>, font_decision_by_range: SortedMapTable<TextRange, FontDecision>,
inline_object_by_range: SortedMapTable<TextRange, InlineObjectSpan>, punctuation_glyph_substitutor: ClreqPunctuationGlyphSubstitutor, style_at: Arc<dyn Fn(u32) -> TextStyle + Send + Sync>, emphasis_italic_at: Arc<dyn Fn(u32) -> bool + Send + Sync>,
rejected_technical_tiers_by_span: SortedMapTable<TextRange, SortedSetTable<u32>>, cached_segment_shaping: Option<SortedMapTable<TextRange, ShapingResult>>, cached_substitution_rollbacks: Option<SortedMapTable<TextRange, String>>) -> Result<ParagraphShapingStageResult,
ParagraphShapingStageShapeParagraphFault> {
        let segment_shaping_cache_keys: Arc<Mutex<Vec<TextRange>>> = Arc::new(Mutex::new(Vec::new()));
        let segment_shaping_cache_values: Arc<Mutex<Vec<ShapingResult>>> = Arc::new(Mutex::new(Vec::new()));
        match &(cached_segment_shaping) {
            Some(__option) => {
                let mut __loop_guard = segment_shaping_cache_keys.lock().unwrap();
                let mut __loop_guard1 = segment_shaping_cache_values.lock().unwrap();
                for i in 0..u32::from_ne_bytes((__option.size()).to_ne_bytes()) {
                    __loop_guard.push(__option.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
                    __loop_guard1.push(__option.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
                }
                drop(__loop_guard);
                drop(__loop_guard1);
            }
            None => {
            }
        }
        let substitution_rollback_keys: Arc<Mutex<Vec<TextRange>>> = Arc::new(Mutex::new(Vec::new()));
        let substitution_rollback_values: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        match &(cached_substitution_rollbacks) {
            Some(__option1) => {
                let mut __loop_guard2 = substitution_rollback_keys.lock().unwrap();
                let mut __loop_guard3 = substitution_rollback_values.lock().unwrap();
                for i in 0..u32::from_ne_bytes((__option1.size()).to_ne_bytes()) {
                    __loop_guard2.push(__option1.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
                    __loop_guard3.push(__option1.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
                }
                drop(__loop_guard2);
                drop(__loop_guard3);
            }
            None => {
            }
        }
        let get_cached_segment_shaping: Arc<dyn Fn(TextRange) -> Option<ShapingResult> + Send + Sync + 'static> = { let segment_shaping_cache_keys = Arc::clone(&segment_shaping_cache_keys); let segment_shaping_cache_values = Arc::clone(&segment_shaping_cache_values); Arc::new({
let segment_shaping_cache_keys = Arc::clone(&segment_shaping_cache_keys); let segment_shaping_cache_values = Arc::clone(&segment_shaping_cache_values); move |r| {
        let mut __loop_guard4 = segment_shaping_cache_keys.lock().unwrap();
        let mut __loop_guard5 = segment_shaping_cache_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard4.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let k = (__loop_guard4[usize::try_from(i).unwrap_or(0)]).clone();
            if k.start == r.start && k.end == r.end {
                return Some((__loop_guard5[usize::try_from(i).unwrap_or(0)]).clone());
            }
        }
        drop(__loop_guard4);
        drop(__loop_guard5);
        return None;
} }) };
        let put_cached_segment_shaping: Arc<dyn Fn(TextRange, ShapingResult) -> () + Send + Sync + 'static> = { let segment_shaping_cache_keys = Arc::clone(&segment_shaping_cache_keys); let segment_shaping_cache_values = Arc::clone(&segment_shaping_cache_values); Arc::new({ let
segment_shaping_cache_keys = Arc::clone(&segment_shaping_cache_keys); let segment_shaping_cache_values = Arc::clone(&segment_shaping_cache_values); move |r, v| {
        let mut __loop_guard6 = segment_shaping_cache_keys.lock().unwrap();
        let mut __loop_guard7 = segment_shaping_cache_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard6.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let k = (__loop_guard6[usize::try_from(i).unwrap_or(0)]).clone();
            if k.start == r.start && k.end == r.end {
                __loop_guard7[usize::try_from(i).unwrap_or(0)] = v;
                return;
            }
        }
        drop(__loop_guard6);
        drop(__loop_guard7);
        segment_shaping_cache_keys.lock().unwrap().push(r.clone());
        segment_shaping_cache_values.lock().unwrap().push(v.clone());
} }) };
        let put_substitution_rollback: Arc<dyn Fn(TextRange, &str) -> () + Send + Sync + '_> = { let substitution_rollback_keys = Arc::clone(&substitution_rollback_keys); let substitution_rollback_values = Arc::clone(&substitution_rollback_values); Arc::new({ let
substitution_rollback_keys = Arc::clone(&substitution_rollback_keys); let substitution_rollback_values = Arc::clone(&substitution_rollback_values); move |r, v| {
        let mut __loop_guard8 = substitution_rollback_keys.lock().unwrap();
        let mut __loop_guard9 = substitution_rollback_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard8.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let k = (__loop_guard8[usize::try_from(i).unwrap_or(0)]).clone();
            if k.start == r.start && k.end == r.end {
                { while __loop_guard9.len() <= usize::try_from(i).unwrap_or(0) { __loop_guard9.push(String::new()); } __loop_guard9[usize::try_from(i).unwrap_or(0)] = v.to_string(); };
                return;
            }
        }
        drop(__loop_guard8);
        drop(__loop_guard9);
        substitution_rollback_keys.lock().unwrap().push(r.clone());
        substitution_rollback_values.lock().unwrap().push(v.to_string());
} }) };
        let dash_ink_coverage_deficient: Arc<dyn Fn(ShapingResult, &str, f64) -> bool + Send + Sync + '_> = {  Arc::new(move |shaped, display_text, segment_font_size| {
        if u32::from_ne_bytes((u_string::find_from(&display_text, "⸺", 0)).to_ne_bytes()) > 2147483647 {
            return false;
        }
        let mut total_glyphs = 0u32;
        let mut single_glyph: Option<Glyph> = None;
        for i in 0..match u32::try_from(shaped.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let r = (shaped.glyph_runs[usize::try_from(i).unwrap_or(0)]).clone();
            for j in 0..match u32::try_from(r.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                total_glyphs = u32::wrapping_add(total_glyphs, 1);
                single_glyph = Some((r.glyphs[usize::try_from(j).unwrap_or(0)]).clone());
            }
        }
        if total_glyphs != 1 || single_glyph.is_none() {
            return false;
        }
        let ink = (single_glyph.as_ref().unwrap().bounds).clone().clone();
        if ink.is_none() {
            return false;
        }
        let target_advance = 2.0f64 * segment_font_size;
        return ((ink).as_ref().unwrap().right - (ink).as_ref().unwrap().left) < (target_advance * 0.85f64);
}) };
        let shape_segment: Arc<dyn Fn(FontDecision, TextRange) -> ShapingResult + Send + Sync + 'static> = { let get_cached_segment_shaping = (get_cached_segment_shaping).clone(); let text = (text).to_string(); let punctuation_glyph_substitutor =
(punctuation_glyph_substitutor).clone(); let style_at = (style_at).clone(); let emphasis_italic_at = (emphasis_italic_at).clone(); let engine = (engine).clone(); let dash_ink_coverage_deficient = (dash_ink_coverage_deficient).clone(); let put_substitution_rollback =
(put_substitution_rollback).clone(); let put_cached_segment_shaping = (put_cached_segment_shaping).clone(); Arc::new(move |decision, segment_range| {
        let mut engine = (engine).clone();
        let cached = get_cached_segment_shaping((segment_range).clone());
        match &(cached) {
            Some(__option2) => {
                return (*__option2).clone();
            }
            None => {
            }
        }
        let source_text = u_string::substring(&text.as_str(), i32::from_ne_bytes((segment_range.start).to_ne_bytes()), i32::from_ne_bytes((segment_range.end).to_ne_bytes()));
        let substitution = ContextualPunctuationDisplaySubstitutionFns::contextual_punctuation_display_substitution_fns_substitute_for_role((punctuation_glyph_substitutor).clone(), source_text.as_str(), decision.role).unwrap();
        let base_segment_style = style_at(segment_range.start);
        let mut segment_style = (base_segment_style).clone();
        if decision.role == FontRole::LatinText && emphasis_italic_at(segment_range.start) {
            segment_style = TextStyle::new(Some(ParagraphShapingStage::paragraph_shaping_stage_copy_font_families(&base_segment_style.font_families)), Some(base_segment_style.font_size), Some((base_segment_style.locale).to_string()), Some(base_segment_style.font_weight),
Some(true), Some(base_segment_style.baseline_shift), Some(base_segment_style.inline_attachment));
        }
        let shaped = engine.text_shaper.shape(ShapingInput::new(text.as_str(), (segment_range).clone(), (segment_style).clone(), (decision).clone(), Some((substitution.display_text).to_string()),
Some(ParagraphShapingStage::paragraph_shaping_stage_cjk_punctuation_full_width_features(decision.role, (substitution.display_text).to_string().as_str())))).unwrap();
        let mut rollback_cause: Option<String> = None;
        if substitution.display_text.to_string() != source_text {
            let mut has_unverified = false;
            let mut has_missing = false;
            for i in 0..match u32::try_from(shaped.decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let d = (shaped.decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if d.capability_issue.as_ref().map_or(false, |v| v == &(TextShaper::TEXT_SHAPER_UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE.to_string())) {
                    has_unverified = true;
                }
                if i32::from_ne_bytes((d.missing_glyphs).to_ne_bytes()) > (0) {
                    has_missing = true;
                }
            }
            if has_unverified {
                rollback_cause = Some("SubstitutionRollbackOnUnverifiedGlyphCoverage".to_string());
            } else {
                if has_missing {
                    rollback_cause = Some("SubstitutionRollbackOnMissingGlyph".to_string());
                } else {
                    if dash_ink_coverage_deficient((shaped).clone(), (substitution.display_text).to_string().as_str(), segment_style.font_size) {
                        rollback_cause = Some("DashSubstitutionInkCoverageRollback".to_string());
                    }
                }
            }
        }
        let mut result = (shaped).clone();
        match &(rollback_cause) {
            Some(__option3) => {
                put_substitution_rollback((segment_range).clone(), __option3.as_str());
                result = engine.text_shaper.shape(ShapingInput::new(text.as_str(), (segment_range).clone(), (segment_style).clone(), (decision).clone(), Some((source_text).to_string()),
Some(ParagraphShapingStage::paragraph_shaping_stage_cjk_punctuation_full_width_features(decision.role, source_text.as_str())))).unwrap();
            }
            None => {
            }
        }
        put_cached_segment_shaping((segment_range).clone(), (result).clone());
        return result;
}) };
        let shape_segment_with_point_mark_prefix: Arc<dyn Fn(FontDecision, TextRange) -> Vec<ShapingResult> + Send + Sync + 'static> = { let text = (text).to_string(); let shape_segment = (shape_segment).clone(); Arc::new(move |decision, segment_range| {
        let mut prefix_end = segment_range.start;
        while (i32::from_ne_bytes((prefix_end).to_ne_bytes())) < (i32::from_ne_bytes((segment_range.end).to_ne_bytes())) && ClreqPunctuationPolicies::clreq_punctuation_policies_is_ascii_point_mark(u_string::substring(&text.as_str(), i32::from_ne_bytes((prefix_end).to_ne_bytes()),
i32::wrapping_add(i32::from_ne_bytes((prefix_end).to_ne_bytes()), 1)).as_str()) {
            prefix_end = u32::wrapping_add(prefix_end, 1);
        }
        if i32::from_ne_bytes((prefix_end).to_ne_bytes()) > (i32::from_ne_bytes((segment_range.start).to_ne_bytes())) && (i32::from_ne_bytes((prefix_end).to_ne_bytes())) < (i32::from_ne_bytes((segment_range.end).to_ne_bytes())) {
            return vec![
    (shape_segment((decision).clone(), TextRange::new(segment_range.start, prefix_end).unwrap())).clone(),
    (shape_segment((decision).clone(), TextRange::new(prefix_end, segment_range.end).unwrap())).clone(),
];
        } else {
            return vec![(shape_segment((decision).clone(), (segment_range).clone())).clone()];
        }
}) };
        let mut hyphen_offsets: Vec<u32> = Vec::new();
        let mut hyphen_advance_or_null: Option<f64> = None;
        let mut hyphen_glyphs: Vec<Glyph> = Vec::new();
        let latin_word_cuts: Arc<dyn Fn(FontDecision, TextRange, &Vec<u32>) -> Vec<u32> + Send + Sync + '_> = { let shape_segment = (shape_segment).clone(); Arc::new(move |decision, word_range, syllable| {
        let capacity = syllable.len();
        let mut cuts = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(syllable.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            cuts.push(u32::wrapping_add(word_range.start, syllable[usize::try_from(i).unwrap_or(0)]));
        }
        let mut rel_bounds = vec![0];
        for &s in syllable {
            let mut exists = false;
            for j in 0..match u32::try_from(rel_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if rel_bounds[usize::try_from(j).unwrap_or(0)] == s {
                    exists = true;
                    break;
                }
            }
            if !exists {
                rel_bounds.push(s);
            }
        }
        let mut len_exists = false;
        for j in 0..match u32::try_from(rel_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if rel_bounds[usize::try_from(j).unwrap_or(0)] == word_range.get_length() {
                len_exists = true;
                break;
            }
        }
        if !len_exists {
            rel_bounds.push(word_range.get_length());
        }
        ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut rel_bounds);
        for i in 0..u32::try_from((rel_bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            let a = rel_bounds[usize::try_from(i).unwrap_or(0)];
            let b = rel_bounds[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
            let piece_shaped = shape_segment((decision).clone(), TextRange::new(u32::wrapping_add(word_range.start, a), u32::wrapping_add(word_range.start, b)).unwrap());
            let piece_advance = if u32::try_from((piece_shaped.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 { piece_shaped.clusters[0usize].advance } else { 0.0f64 };
            if piece_advance <= measure {
                continue;
            }
            let lo = u32::wrapping_add(a, 2);
            let hi = u32::wrapping_sub(b, 3);
            let mut off = if i32::from_ne_bytes((lo).to_ne_bytes()) <= i32::from_ne_bytes((hi).to_ne_bytes()) { lo } else { u32::wrapping_add(a, 1) };
            let end_off = if i32::from_ne_bytes((lo).to_ne_bytes()) <= i32::from_ne_bytes((hi).to_ne_bytes()) { u32::wrapping_add(hi, 1) } else { b };
            while ({ let v: u32 = off; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = end_off; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                let c = u32::wrapping_add(word_range.start, off);
                let mut exists = false;
                for k in 0..match u32::try_from(cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if cuts[usize::try_from(k).unwrap_or(0)] == c {
                        exists = true;
                        break;
                    }
                }
                if !exists {
                    cuts.push(c);
                }
                off = u32::wrapping_add(off, 1);
            }
        }
        ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut cuts);
        return cuts;
}) };
        let break_opportunity_decisions: Arc<Mutex<Vec<BreakOpportunityDecisionInfo>>> = Arc::new(Mutex::new(Vec::new()));
        let emergency_tracking_eligibility_decisions: Arc<Mutex<Vec<EmergencyTrackingEligibilityDecisionInfo>>> = Arc::new(Mutex::new(Vec::new()));
        let register_emergency_tracking_eligibility: Arc<dyn Fn(TextRange, &str) -> () + Send + Sync + '_> = { let emergency_tracking_eligibility_decisions = Arc::clone(&emergency_tracking_eligibility_decisions); let text = (text).to_string(); Arc::new({ let
emergency_tracking_eligibility_decisions = Arc::clone(&emergency_tracking_eligibility_decisions); move |range, reason| {
        let mut __loop_guard10 = emergency_tracking_eligibility_decisions.lock().unwrap();
        let mut __loop_i = 0u32;
        loop {
            let __loop_len = match u32::try_from(__loop_guard10.len()) { Ok(value) => value, Err(_) => u32::MAX };
            if __loop_i >= __loop_len { break; }
            let d = (__loop_guard10[usize::try_from(__loop_i).unwrap_or(0)]).clone();
            if d.range.clone().start == range.start && (d.range).clone().end == range.end && (d.reason).to_string() == reason {
                return;
            }
            __loop_i += 1;
        }
        drop(__loop_guard10);
        emergency_tracking_eligibility_decisions.lock().unwrap().push(EmergencyTrackingEligibilityDecisionInfo::new((range).clone(), u_string::substring(&text.as_str(), i32::from_ne_bytes((range.start).to_ne_bytes()), i32::from_ne_bytes((range.end).to_ne_bytes())).as_str(),
reason));
} }) };
        let prog_break_keys: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
        let prog_break_values: Arc<Mutex<Vec<ProgressiveBreakOpportunity>>> = Arc::new(Mutex::new(Vec::new()));
        let put_progressive_break: Arc<dyn Fn(u32, ProgressiveBreakOpportunity) -> () + Send + Sync + 'static> = { let prog_break_keys = Arc::clone(&prog_break_keys); let prog_break_values = Arc::clone(&prog_break_values); Arc::new({ let prog_break_keys =
Arc::clone(&prog_break_keys); let prog_break_values = Arc::clone(&prog_break_values); move |offset, opp| {
        let mut __loop_guard11 = prog_break_keys.lock().unwrap();
        let mut __loop_guard12 = prog_break_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard11.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if __loop_guard11[usize::try_from(i).unwrap_or(0)] == offset {
                let current = (__loop_guard12[usize::try_from(i).unwrap_or(0)]).clone();
                if i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(opp.tier)).to_ne_bytes()) < (i32::from_ne_bytes((ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(current.tier)).to_ne_bytes())) {
                    __loop_guard12[usize::try_from(i).unwrap_or(0)] = opp;
                }
                return;
            }
        }
        drop(__loop_guard11);
        drop(__loop_guard12);
        prog_break_keys.lock().unwrap().push(offset);
        prog_break_values.lock().unwrap().push(opp.clone());
} }) };
        let latin_separator_cuts: Arc<dyn Fn(TextRange, f64, bool) -> Vec<u32> + Send + Sync + 'static> = { let text = (text).to_string(); let break_opportunity_decisions = Arc::clone(&break_opportunity_decisions); Arc::new({ let break_opportunity_decisions =
Arc::clone(&break_opportunity_decisions); move |token_range, token_advance, force_opaque_breaks| {
        let token = u_string::substring(&text.as_str(), i32::from_ne_bytes((token_range.start).to_ne_bytes()), i32::from_ne_bytes((token_range.end).to_ne_bytes()));
        let url_like = ParagraphShapingStage::paragraph_shaping_stage_is_url_like_latin_token(token.as_str());
        let mut opaque = false;
        for i in 0..match u32::try_from(u_string::unit_count(&(token))) { Ok(value) => value, Err(_) => u32::MAX } {
            if !ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at(&token, i)).as_ref().unwrap()) {
                opaque = true;
                break;
            }
        }
        let structural_solidus = ParagraphShapingStage::paragraph_shaping_stage_has_breakable_latin_solidus(token.as_str());
        let bibliographic_locator_cuts = ParagraphShapingStage::paragraph_shaping_stage_bibliographic_numeric_locator_break_offsets(token.as_str());
        let opaque_separator_mode = url_like || opaque && ((token_advance) > (measure) || force_opaque_breaks);
        if !structural_solidus && !opaque_separator_mode && u32::try_from((bibliographic_locator_cuts.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let capacity = bibliographic_locator_cuts.len();
        let mut cuts = Vec::with_capacity(capacity);
        for i in 0..match u32::try_from(bibliographic_locator_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            cuts.push(u32::wrapping_add(token_range.start, bibliographic_locator_cuts[usize::try_from(i).unwrap_or(0)]));
        }
        if i32::from_ne_bytes((u32::try_from((bibliographic_locator_cuts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            break_opportunity_decisions.lock().unwrap().push(BreakOpportunityDecisionInfo::new((token_range).clone(), token.as_str(), ParagraphShapingStage::paragraph_shaping_stage_copy_ints(&cuts).to_vec(), "BibliographicNumericLocatorBreak", None));
        }
        for i in 0..u_string::unit_count(&(token)).saturating_sub(1) {
            let break_after = structural_solidus && !url_like && u_string::unit_at(&token, i).as_ref().map_or(false, |v| v == &(47)) || opaque_separator_mode && ParagraphShapingStage::paragraph_shaping_stage_is_latin_token_break_after(token.as_str(), i, (token_advance) <=
measure);
            if break_after {
                cuts.push(u32::wrapping_add(u32::wrapping_add(token_range.start, i), 1));
            }
        }
        return cuts;
} }) };
        let latin_opaque_hard_cuts: Arc<dyn Fn(FontDecision, TextRange, &Vec<u32>, bool) -> Vec<u32> + Send + Sync + '_> = { let shape_segment = (shape_segment).clone(); let text = (text).to_string(); Arc::new(move |decision, token_range, clean_cuts, force_opaque_breaks| {
        let mut rel_bounds = vec![0];
        for i in 0..match u32::try_from(clean_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let rel = u32::wrapping_sub(clean_cuts[usize::try_from(i).unwrap_or(0)], token_range.start);
            let mut exists = false;
            for j in 0..match u32::try_from(rel_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if rel_bounds[usize::try_from(j).unwrap_or(0)] == rel {
                    exists = true;
                    break;
                }
            }
            if !exists {
                rel_bounds.push(rel);
            }
        }
        let mut len_exists = false;
        for j in 0..match u32::try_from(rel_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if rel_bounds[usize::try_from(j).unwrap_or(0)] == token_range.get_length() {
                len_exists = true;
                break;
            }
        }
        if !len_exists {
            rel_bounds.push(token_range.get_length());
        }
        ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut rel_bounds);
        let mut cuts: Vec<u32> = Vec::new();
        for i in 0..u32::try_from((rel_bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
            let a = rel_bounds[usize::try_from(i).unwrap_or(0)];
            let b = rel_bounds[usize::try_from(u32::wrapping_add(i, 1)).unwrap_or(0)];
            if i32::from_ne_bytes((u32::wrapping_sub(b, a)).to_ne_bytes()) <= 1 {
                continue;
            }
            let piece_range = TextRange::new(u32::wrapping_add(token_range.start, a), u32::wrapping_add(token_range.start, b)).unwrap();
            let piece_shaped = shape_segment((decision).clone(), (piece_range).clone());
            let piece_advance = if u32::try_from((piece_shaped.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 { piece_shaped.clusters[0usize].advance } else { 0.0f64 };
            if piece_advance <= measure && !(force_opaque_breaks && (i32::from_ne_bytes((u32::wrapping_sub(b, a)).to_ne_bytes())) >= 24) {
                continue;
            }
            let graphemes = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text.as_str(), (piece_range).clone());
            for &pos in &graphemes {
                if ({ let v: u32 = pos; i32::from_ne_bytes(v.to_ne_bytes()) }) > (i32::from_ne_bytes((piece_range.start).to_ne_bytes())) && ({ let v: u32 = pos; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((piece_range.end).to_ne_bytes())) {
                    let mut exists = false;
                    for k in 0..match u32::try_from(cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if cuts[usize::try_from(k).unwrap_or(0)] == pos {
                            exists = true;
                            break;
                        }
                    }
                    if !exists {
                        cuts.push(pos);
                    }
                }
            }
        }
        ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut cuts);
        return cuts;
}) };
        let get_inline_object: Arc<dyn Fn(TextRange) -> Option<InlineObjectSpan> + Send + Sync + 'static> = { let inline_object_by_range = (inline_object_by_range).clone(); Arc::new(move |range| {
        if false {
            return None;
        }
        for i in 0..u32::from_ne_bytes((inline_object_by_range.size()).to_ne_bytes()) {
            let k = inline_object_by_range.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if k.start == range.start && k.end == range.end {
                return Some(inline_object_by_range.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
            }
        }
        return None;
}) };
        let get_font_decision: Arc<dyn Fn(TextRange) -> Option<FontDecision> + Send + Sync + 'static> = { let font_decision_by_range = (font_decision_by_range).clone(); Arc::new(move |range| {
        if false {
            return None;
        }
        for i in 0..u32::from_ne_bytes((font_decision_by_range.size()).to_ne_bytes()) {
            let k = font_decision_by_range.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if k.start == range.start && k.end == range.end {
                return Some(font_decision_by_range.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
            }
        }
        return None;
}) };
        let get_rejected_tiers: Arc<dyn Fn(TextRange) -> Option<SortedSetTable<u32>> + Send + Sync + 'static> = { let rejected_technical_tiers_by_span = (rejected_technical_tiers_by_span).clone(); Arc::new(move |range| {
        if false {
            return None;
        }
        for i in 0..u32::from_ne_bytes((rejected_technical_tiers_by_span.size()).to_ne_bytes()) {
            let k = rejected_technical_tiers_by_span.key_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) });
            if k.start == range.start && k.end == range.end {
                return Some(rejected_technical_tiers_by_span.value_at({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }));
            }
        }
        return None;
}) };
        let span_advance_cache_keys: Arc<Mutex<Vec<TextRange>>> = Arc::new(Mutex::new(Vec::new()));
        let span_advance_cache_values: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
        let progressive_span_advance: Arc<dyn Fn(TextRange) -> f64 + Send + Sync + 'static> = { let span_advance_cache_keys = Arc::clone(&span_advance_cache_keys); let span_advance_cache_values = Arc::clone(&span_advance_cache_values); let cluster_ranges =
(cluster_ranges).to_vec(); let get_inline_object = (get_inline_object).clone(); let get_font_decision = (get_font_decision).clone(); let text = (text).to_string(); let shape_segment = (shape_segment).clone(); Arc::new({ let span_advance_cache_keys =
Arc::clone(&span_advance_cache_keys); let span_advance_cache_values = Arc::clone(&span_advance_cache_values); move |span_range| {
        let mut __loop_guard13 = span_advance_cache_keys.lock().unwrap();
        let mut __loop_guard14 = span_advance_cache_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard13.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let k = (__loop_guard13[usize::try_from(i).unwrap_or(0)]).clone();
            if k.start == span_range.start && k.end == span_range.end {
                return __loop_guard14[usize::try_from(i).unwrap_or(0)];
            }
        }
        drop(__loop_guard13);
        drop(__loop_guard14);
        let mut advance_terms: Vec<f64> = vec![];
        for resolved_range in &cluster_ranges {
            if resolved_range.mandatory_break || resolved_range.zero_width_soft_break || get_inline_object((resolved_range.range).clone()).is_some() {
                continue;
            }
            let decision = get_font_decision((resolved_range.range).clone());
            if decision.is_none() {
                continue;
            }
            let candidates = ParagraphShapingStage::paragraph_shaping_stage_shaping_segments(((decision).as_ref().unwrap().clone()).clone(), text.as_str()).unwrap();
            for candidate in &candidates {
                let start = if i32::from_ne_bytes((candidate.start).to_ne_bytes()) > (i32::from_ne_bytes((span_range.start).to_ne_bytes())) { candidate.start } else { span_range.start };
                let end = if i32::from_ne_bytes((candidate.end).to_ne_bytes()) < (i32::from_ne_bytes((span_range.end).to_ne_bytes())) { candidate.end } else { span_range.end };
                if ({ let v: u32 = start; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = end; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                    let piece_shaped = shape_segment(((decision).as_ref().unwrap().clone()).clone(), TextRange::new(start, end).unwrap());
                    for cl_idx in 0..match u32::try_from(piece_shaped.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        advance_terms.push(piece_shaped.clusters[usize::try_from(cl_idx).unwrap_or(0)].advance);
                    }
                }
            }
        }
        let total_advance = AccurateSum::accurate_sum_of(&advance_terms);
        span_advance_cache_keys.lock().unwrap().push(span_range.clone());
        span_advance_cache_values.lock().unwrap().push(total_advance);
        return total_advance;
} }) };
        let mut shaping_results: Vec<ShapingResult> = Vec::new();
        for resolved_range in cluster_ranges {
            let inline_obj = get_inline_object((resolved_range.range).clone());
            match &(inline_obj) {
                Some(__option4) => {
                    shaping_results.push(ParagraphShapingStage::paragraph_shaping_stage_inline_object_shaping_result(text, ((*__option4).clone()).clone()));
                    continue;
                }
                None => {
                }
            }
            if resolved_range.mandatory_break {
                shaping_results.push(ParagraphShapingStage::paragraph_shaping_stage_mandatory_break_shaping_result(text, (resolved_range.range).clone()));
                continue;
            }
            if resolved_range.zero_width_soft_break {
                shaping_results.push(ParagraphShapingStage::paragraph_shaping_stage_zero_width_soft_break_shaping_result(text, (resolved_range.range).clone()));
                continue;
            }
            let mut decision = get_font_decision((resolved_range.range).clone());
            if decision.is_none() {
                decision = Some(engine.fallback_resolver.resolve(text, (resolved_range.range).clone(), FontRequest::new(((input.text_style).clone().font_families).clone(), ((input.text_style).clone().locale).to_string().as_str(), resolved_range.role)).clone());
            }
            let segments = ParagraphShapingStage::paragraph_shaping_stage_shaping_segments(((decision).as_ref().unwrap().clone()).clone(), text).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
            for segment_range in &segments {
                let shaped = shape_segment(((decision).as_ref().unwrap().clone()).clone(), (segment_range).clone());
                let is_latin = (decision).as_ref().unwrap().role == FontRole::LatinText && (i32::from_ne_bytes((segment_range.get_length()).to_ne_bytes())) > (0);
                let w = if is_latin { u_string::substring(&text, i32::from_ne_bytes((segment_range.start).to_ne_bytes()), i32::from_ne_bytes((segment_range.end).to_ne_bytes())).to_string() } else { "".to_string() };
                let mut progressive_span: Option<LineBreakSpan> = None;
                for s_idx in 0..match u32::try_from((input.content).clone().line_break_spans.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let s = ((input.content).clone().line_break_spans[usize::try_from(s_idx).unwrap_or(0)]).clone();
                    if s.policy == LineBreakPolicy::ProgressiveTechnical && (i32::from_ne_bytes((segment_range.start).to_ne_bytes())) >= i32::from_ne_bytes(((s.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((segment_range.end).to_ne_bytes())) <=
i32::from_ne_bytes(((s.range).clone().end).to_ne_bytes()) {
                        progressive_span = Some(s.clone());
                        break;
                    }
                }
                let mut all_letters = is_latin && (i32::from_ne_bytes((u_string::unit_count(&(w))).to_ne_bytes())) > (0);
                if is_latin {
                    for c_idx in 0..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
                        if !ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at(&w, c_idx)).as_ref().unwrap()) {
                            all_letters = false;
                            break;
                        }
                    }
                }
                let mut is_all_caps = all_letters && (i32::from_ne_bytes((u_string::unit_count(&(w))).to_ne_bytes())) >= 2;
                if is_all_caps {
                    for c_idx in 0..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
                        if ParagraphShapingStage::paragraph_shaping_stage_is_lower_case(*(u_string::unit_at(&w, c_idx)).as_ref().unwrap()) {
                            is_all_caps = false;
                            break;
                        }
                    }
                }
                let is_abbreviation = is_all_caps && (i32::from_ne_bytes((u_string::unit_count(&(w))).to_ne_bytes())) < (24);
                let mut has_internal_upper = false;
                if all_letters && !is_all_caps && !is_abbreviation {
                    for c_idx in 1..match u32::try_from(u_string::unit_count(&(w))) { Ok(value) => value, Err(_) => u32::MAX } {
                        if ParagraphShapingStage::paragraph_shaping_stage_is_upper_case(*(u_string::unit_at(&w, c_idx)).as_ref().unwrap()) {
                            has_internal_upper = true;
                            break;
                        }
                    }
                }
                let is_camel_case = all_letters && !is_all_caps && !is_abbreviation && has_internal_upper;
                let mut token_terms: Vec<f64> = vec![];
                for cl_idx in 0..match u32::try_from(shaped.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    token_terms.push(shaped.clusters[usize::try_from(cl_idx).unwrap_or(0)].advance);
                }
                let token_advance = AccurateSum::accurate_sum_of(&token_terms);
                let strong_reason = if is_latin { ParagraphShapingStage::paragraph_shaping_stage_strong_non_lexical_reason(w.as_str()) } else { None };
                let mut syllable_cuts: Vec<u32> = Vec::new();
                if all_letters && !is_abbreviation && !is_camel_case && (u32::from_ne_bytes((u_string::find_from(&w, "-", 0)).to_ne_bytes())) > 2147483647 && strong_reason.is_none() {
                    let hyphen_points = engine.hyphenator.hyphenate(w.as_str());
                    let mut unique_points: Vec<u32> = Vec::new();
                    for &pt in &hyphen_points {
                        let mut exists = false;
                        for q in 0..match u32::try_from(unique_points.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if unique_points[usize::try_from(q).unwrap_or(0)] == pt {
                                exists = true;
                                break;
                            }
                        }
                        if !exists {
                            unique_points.push(pt);
                        }
                    }
                    ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_points);
                    syllable_cuts = unique_points;
                }
                let mut longest_unhyphenated_letter_piece = 0u32;
                if all_letters {
                    let mut bounds = vec![0];
                    for p in 0..match u32::try_from(syllable_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        bounds.push(syllable_cuts[usize::try_from(p).unwrap_or(0)]);
                    }
                    bounds.push(u_string::unit_count(&(w)));
                    let mut max_len = 0u32;
                    for p in 0..u32::try_from((bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
                        let diff = u32::wrapping_sub(bounds[usize::try_from(u32::wrapping_add(p, 1)).unwrap_or(0)], bounds[usize::try_from(p).unwrap_or(0)]);
                        if i32::from_ne_bytes((diff).to_ne_bytes()) > (i32::from_ne_bytes((max_len).to_ne_bytes())) {
                            max_len = diff;
                        }
                    }
                    longest_unhyphenated_letter_piece = max_len;
                }
                let is_long_unhyphenated_letter_token = all_letters && !is_abbreviation && !is_camel_case && (i32::from_ne_bytes((longest_unhyphenated_letter_piece).to_ne_bytes())) >= 24;
                let is_long_opaque_latin_token = strong_reason.is_some() || is_long_unhyphenated_letter_token || is_latin && !all_letters && (i32::from_ne_bytes((u_string::unit_count(&(w))).to_ne_bytes())) >= 24;
                let mut technical_structural_cuts: Vec<u32> = Vec::new();
                if match &(progressive_span) { Some(__option6) => is_latin, None => false } {
                    let mut cuts_set: Vec<u32> = Vec::new();
                    let cc = ParagraphShapingStage::paragraph_shaping_stage_camel_case_cuts(text, (segment_range).clone());
                    for c in 0..match u32::try_from(cc.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        cuts_set.push(cc[usize::try_from(c).unwrap_or(0)]);
                    }
                    let at = ParagraphShapingStage::paragraph_shaping_stage_alpha_numeric_transition_cuts(text, (segment_range).clone());
                    for &v in &at {
                        let mut exists = false;
                        for q in 0..match u32::try_from(cuts_set.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if cuts_set[usize::try_from(q).unwrap_or(0)] == v {
                                exists = true;
                                break;
                            }
                        }
                        if !exists {
                            cuts_set.push(v);
                        }
                    }
                    for i in 0..u_string::unit_count(&(w)).saturating_sub(1) {
                        if ParagraphShapingStage::paragraph_shaping_stage_is_progressive_technical_break_after_char(*(u_string::unit_at(&w, i)).as_ref().unwrap()) {
                            let v = u32::wrapping_add(u32::wrapping_add(segment_range.start, i), 1);
                            let mut exists = false;
                            for q in 0..match u32::try_from(cuts_set.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                if cuts_set[usize::try_from(q).unwrap_or(0)] == v {
                                    exists = true;
                                    break;
                                }
                            }
                            if !exists {
                                cuts_set.push(v);
                            }
                        }
                    }
                    ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut cuts_set);
                    technical_structural_cuts = cuts_set;
                }
                let mut raw_technical_syllable_cuts: Vec<u32> = Vec::new();
                if match &(progressive_span) { Some(__option9) => is_latin, None => false } {
                    let mut preferred_bounds = vec![segment_range.start];
                    for c in 0..match u32::try_from(technical_structural_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        preferred_bounds.push(technical_structural_cuts[usize::try_from(c).unwrap_or(0)]);
                    }
                    preferred_bounds.push(segment_range.end);
                    let mut unique_bounds: Vec<u32> = Vec::new();
                    for &pt in &preferred_bounds {
                        let mut exists = false;
                        for q in 0..match u32::try_from(unique_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if unique_bounds[usize::try_from(q).unwrap_or(0)] == pt {
                                exists = true;
                                break;
                            }
                        }
                        if !exists {
                            unique_bounds.push(pt);
                        }
                    }
                    ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_bounds);
                    let mut cuts_list: Vec<u32> = Vec::new();
                    for p in 0..u32::try_from((unique_bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
                        let piece_start = unique_bounds[usize::try_from(p).unwrap_or(0)];
                        let piece_end = unique_bounds[usize::try_from(u32::wrapping_add(p, 1)).unwrap_or(0)];
                        let piece = u_string::substring(&text, { let v: u32 = piece_start; i32::from_ne_bytes(v.to_ne_bytes()) }, { let v: u32 = piece_end; i32::from_ne_bytes(v.to_ne_bytes()) });
                        if ParagraphShapingStage::paragraph_shaping_stage_strong_non_lexical_reason(piece.as_str()).is_some() {
                            continue;
                        }
                        let mut run_start = piece_start;
                        while ({ let v: u32 = run_start; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = piece_end; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                            while ({ let v: u32 = run_start; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = piece_end; i32::from_ne_bytes(v.to_ne_bytes()) }) && !ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at(&text,
run_start)).as_ref().unwrap()) {
                                run_start = u32::wrapping_add(run_start, 1);
                            }
                            let mut run_end = run_start;
                            while ({ let v: u32 = run_end; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = piece_end; i32::from_ne_bytes(v.to_ne_bytes()) }) && ParagraphShapingStage::paragraph_shaping_stage_is_letter(*(u_string::unit_at(&text,
run_end)).as_ref().unwrap()) {
                                run_end = u32::wrapping_add(run_end, 1);
                            }
                            if ({ let v: u32 = run_end; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = run_start; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                let word = u_string::substring(&text, { let v: u32 = run_start; i32::from_ne_bytes(v.to_ne_bytes()) }, { let v: u32 = run_end; i32::from_ne_bytes(v.to_ne_bytes()) });
                                let word_hyphs = engine.hyphenator.hyphenate(word.as_str());
                                for &off in &word_hyphs {
                                    if ({ let v: u32 = off; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 1 && ({ let v: u32 = off; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u_string::unit_count(&(word))).to_ne_bytes())) {
                                        cuts_list.push(u32::wrapping_add(run_start, off));
                                    }
                                }
                            }
                            run_start = if ({ let v: u32 = run_end; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = run_start; i32::from_ne_bytes(v.to_ne_bytes()) }) { run_end } else { u32::wrapping_add(run_start, 1) };
                        }
                    }
                    let mut unique_cuts: Vec<u32> = Vec::new();
                    for &pt in &cuts_list {
                        let mut exists = false;
                        for q in 0..match u32::try_from(unique_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if unique_cuts[usize::try_from(q).unwrap_or(0)] == pt {
                                exists = true;
                                break;
                            }
                        }
                        if !exists {
                            unique_cuts.push(pt);
                        }
                    }
                    ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_cuts);
                    raw_technical_syllable_cuts = unique_cuts;
                }
                let mut technical_syllable_cuts: Vec<u32> = Vec::new();
                for &cut in &raw_technical_syllable_cuts {
                    let mut in_structural = false;
                    for j in 0..match u32::try_from(technical_structural_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if technical_structural_cuts[usize::try_from(j).unwrap_or(0)] == cut {
                            in_structural = true;
                            break;
                        }
                    }
                    if !in_structural {
                        technical_syllable_cuts.push(cut);
                    }
                }
                let mut technical_emergency_cuts: Vec<u32> = Vec::new();
                match &(progressive_span) {
                    Some(__option11) => {
                        if is_latin {
                        let rejected_tiers = get_rejected_tiers((__option11.range).clone());
                        let exposed_for_current_line = match &(rejected_tiers) { Some(__option12) => i32::from_ne_bytes((u32::from_ne_bytes((__option12.size()).to_ne_bytes())).to_ne_bytes()) > (0), None => false };
                        let mut preferred_bounds = vec![segment_range.start];
                        for c in 0..match u32::try_from(technical_structural_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            preferred_bounds.push(technical_structural_cuts[usize::try_from(c).unwrap_or(0)]);
                        }
                        for c in 0..match u32::try_from(technical_syllable_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            preferred_bounds.push(technical_syllable_cuts[usize::try_from(c).unwrap_or(0)]);
                        }
                        preferred_bounds.push(segment_range.end);
                        let mut unique_bounds: Vec<u32> = Vec::new();
                        for &pt in &preferred_bounds {
                            let mut exists = false;
                            for q in 0..match u32::try_from(unique_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                if unique_bounds[usize::try_from(q).unwrap_or(0)] == pt {
                                    exists = true;
                                    break;
                                }
                            }
                            if !exists {
                                unique_bounds.push(pt);
                            }
                        }
                        ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_bounds);
                        let mut interior_emergency_cuts: Vec<u32> = Vec::new();
                        for p in 0..u32::try_from((unique_bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
                            let p_start = unique_bounds[usize::try_from(p).unwrap_or(0)];
                            let p_end = unique_bounds[usize::try_from(u32::wrapping_add(p, 1)).unwrap_or(0)];
                            let piece_range = TextRange::new(p_start, p_end).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
                            let piece_shaped = shape_segment(((decision).as_ref().unwrap().clone()).clone(), (piece_range).clone());
                            let mut piece_terms: Vec<f64> = vec![];
                            for cl_idx in 0..match u32::try_from(piece_shaped.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                piece_terms.push(piece_shaped.clusters[usize::try_from(cl_idx).unwrap_or(0)].advance);
                            }
                            let piece_advance = AccurateSum::accurate_sum_of(&piece_terms);
                            if !exposed_for_current_line && (piece_advance) <= measure && (progressive_span_advance((__option11.range).clone())) <= measure {
                            } else {
                                let graphemes = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text, (piece_range).clone());
                                for &pt in &graphemes {
                                    if ({ let v: u32 = pt; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = p_start; i32::from_ne_bytes(v.to_ne_bytes()) }) && ({ let v: u32 = pt; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = p_end;
i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                        interior_emergency_cuts.push(pt);
                                    }
                                }
                            }
                        }
                        let mut rejected_clean_boundaries: Vec<u32> = Vec::new();
                        match &(rejected_tiers) {
                            Some(__option13) => {
                                if __option13.has(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Structural))) {
                                    for c in 0..match u32::try_from(technical_structural_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                        rejected_clean_boundaries.push(technical_structural_cuts[usize::try_from(c).unwrap_or(0)]);
                                    }
                                }
                                if __option13.has(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(ProgressiveBreakTier::Syllable))) {
                                    for c in 0..match u32::try_from(technical_syllable_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                        rejected_clean_boundaries.push(technical_syllable_cuts[usize::try_from(c).unwrap_or(0)]);
                                    }
                                }
                            }
                            None => {
                            }
                        }
                        let capacity = interior_emergency_cuts.len();
                        let mut combined = Vec::with_capacity(capacity);
                        for c in 0..match u32::try_from(interior_emergency_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            combined.push(interior_emergency_cuts[usize::try_from(c).unwrap_or(0)]);
                        }
                        for c in 0..match u32::try_from(rejected_clean_boundaries.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            combined.push(rejected_clean_boundaries[usize::try_from(c).unwrap_or(0)]);
                        }
                        let mut unique_combined: Vec<u32> = Vec::new();
                        for &pt in &combined {
                            let mut exists = false;
                            for q in 0..match u32::try_from(unique_combined.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                if unique_combined[usize::try_from(q).unwrap_or(0)] == pt {
                                    exists = true;
                                    break;
                                }
                            }
                            if !exists {
                                unique_combined.push(pt);
                            }
                        }
                        ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_combined);
                        technical_emergency_cuts = unique_combined;
                        }
                    }
                    None => {
                    }
                }
                match &(progressive_span) {
                    Some(__option14) => {
                        if i32::from_ne_bytes((u32::try_from((technical_emergency_cuts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                            let rej = get_rejected_tiers((__option14.range).clone());
                            let mut reason = "ProgressiveTechnicalSpan".to_string();
                            match &(rej) {
                                Some(__option15) => {
                                    if i32::from_ne_bytes((u32::from_ne_bytes((__option15.size()).to_ne_bytes())).to_ne_bytes()) > (0) {
                                    let capacity = usize::try_from(u32::from_ne_bytes((__option15.size()).to_ne_bytes())).unwrap_or(0);
                                    let mut tiers = Vec::with_capacity(capacity);
                                    for i in 0..u32::from_ne_bytes((__option15.size()).to_ne_bytes()) {
                                        tiers.push(__option15.at(i32::from_ne_bytes((i).to_ne_bytes())));
                                    }
                                    let mut t_idx = 1u32;
                                    while (i32::from_ne_bytes((t_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((tiers.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                        let curr = tiers[usize::try_from(t_idx).unwrap_or(0)];
                                        let mut j = t_idx;
                                        while (j) > (0) && ({ let v: u32 = tiers[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = curr; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                            { while tiers.len() <= usize::try_from(j).unwrap_or(0) { tiers.push(0); } tiers[usize::try_from(j).unwrap_or(0)] = tiers[usize::try_from(u32::wrapping_sub(j, 1)).unwrap_or(0)]; };
                                            j = u32::wrapping_sub(j, 1);
                                        }
                                        { while tiers.len() <= usize::try_from(j).unwrap_or(0) { tiers.push(0); } tiers[usize::try_from(j).unwrap_or(0)] = curr; };
                                        t_idx = u32::wrapping_add(t_idx, 1);
                                    }
                                    let mut joined = String::new();
                                    for i in 0..match u32::try_from(tiers.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                        if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                                            joined += &("+");
                                        }
                                        joined += &(ParagraphShapingStage::paragraph_shaping_stage_tier_name(ProgressiveBreakTierPriority::progressive_break_tier_priority_from_priority(tiers[usize::try_from(i).unwrap_or(0)])));
                                    }
                                    reason = format!("{}{}",
            "CurrentLineTechnicalTierRejection:",
            joined
        );
                                    }
                                }
                                None => {
                                }
                            }
                            register_emergency_tracking_eligibility((__option14.range).clone(), reason.as_str());
                        }
                        let tier_arrays = vec![
    (technical_structural_cuts).clone(),
    (technical_syllable_cuts).clone(),
    (technical_emergency_cuts).clone(),
];
                        let tier_types = vec![
    ProgressiveBreakTier::Structural,
    ProgressiveBreakTier::Syllable,
    ProgressiveBreakTier::Emergency,
];
                        let mut __loop_guard15 = break_opportunity_decisions.lock().unwrap();
                        for t_idx in 0..3 {
                            let tier = tier_types[usize::try_from(t_idx).unwrap_or(0)];
                            let offsets = (tier_arrays[usize::try_from(t_idx).unwrap_or(0)]).clone();
                            let rej = get_rejected_tiers((__option14.range).clone());
                            match &(rej) {
                                Some(__option16) => {
                                    if __option16.has(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(tier))) {
                                    continue;
                                    }
                                }
                                None => {
                                }
                            }
                            let mut unique_offsets: Vec<u32> = Vec::new();
                            for &pt in &offsets {
                                let mut exists = false;
                                for q in 0..match u32::try_from(unique_offsets.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                    if unique_offsets[usize::try_from(q).unwrap_or(0)] == pt {
                                        exists = true;
                                        break;
                                    }
                                }
                                if !exists {
                                    unique_offsets.push(pt);
                                }
                            }
                            ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_offsets);
                            if i32::from_ne_bytes((u32::try_from((unique_offsets.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                                let rej_not_empty = match &(rej) { Some(__option17) => i32::from_ne_bytes((u32::from_ne_bytes((__option17.size()).to_ne_bytes())).to_ne_bytes()) > (0), None => false };
                                let break_reason = if tier == ProgressiveBreakTier::Emergency && rej_not_empty { "CurrentLineTechnicalEmergencyBreak".to_string() } else { "ProgressiveTechnicalBreak".to_string() };
                                __loop_guard15.push(BreakOpportunityDecisionInfo::new((*segment_range).clone(), w.as_str(), ParagraphShapingStage::paragraph_shaping_stage_copy_ints(&unique_offsets).to_vec(), break_reason.as_str(),
Some((ParagraphShapingStage::paragraph_shaping_stage_tier_name(tier)).to_string())));
                            }
                            for &offset in &unique_offsets {
                                put_progressive_break(offset, ProgressiveBreakOpportunity::new(tier, (__option14.range).clone(), Some(0.0)));
                            }
                        }
                        drop(__loop_guard15);
                        let mut boundary_tier = ProgressiveBreakTier::WholeToken;
                        if i32::from_ne_bytes((segment_range.start).to_ne_bytes()) > (i32::from_ne_bytes((__option14.range.start).to_ne_bytes())) && ParagraphShapingStage::paragraph_shaping_stage_is_whitespace(*(u_string::unit_at(&text, u32::wrapping_sub(segment_range.start,
1))).as_ref().unwrap()) {
                            boundary_tier = ProgressiveBreakTier::Whitespace;
                        }
                        let rej = get_rejected_tiers((__option14.range).clone());
                        if match &(rej) { None => true, Some(__option18) => !__option18.has(&(ProgressiveBreakTierPriority::progressive_break_tier_priority_priority(boundary_tier))) } {
                            let whole_token = ProgressiveBreakOpportunity::new(boundary_tier, (__option14.range).clone(), Some(0.0));
                            put_progressive_break(segment_range.start, (whole_token).clone());
                            let wrap_reason = if boundary_tier == ProgressiveBreakTier::Whitespace { "ProgressiveTechnicalWhitespaceBreak".to_string() } else { "ProgressiveTechnicalWholeTokenWrap".to_string() };
                            break_opportunity_decisions.lock().unwrap().push(BreakOpportunityDecisionInfo::new((*segment_range).clone(), w.as_str(), vec![segment_range.start].to_vec(), wrap_reason.as_str(),
Some((ParagraphShapingStage::paragraph_shaping_stage_tier_name(boundary_tier)).to_string())));
                        }
                    }
                    None => {
                    }
                }
                let mut clean_cuts: Vec<u32> = Vec::new();
                if progressive_span.is_some() {
                    for c in 0..match u32::try_from(technical_structural_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        clean_cuts.push(technical_structural_cuts[usize::try_from(c).unwrap_or(0)]);
                    }
                    for c in 0..match u32::try_from(technical_syllable_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        clean_cuts.push(technical_syllable_cuts[usize::try_from(c).unwrap_or(0)]);
                    }
                    for c in 0..match u32::try_from(technical_emergency_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        clean_cuts.push(technical_emergency_cuts[usize::try_from(c).unwrap_or(0)]);
                    }
                } else {
                    if !is_latin {
                    } else {
                        if u32::from_ne_bytes((u_string::find_from(&w, "-", 0)).to_ne_bytes()) <= 2147483647 {
                            let existing = ParagraphShapingStage::paragraph_shaping_stage_existing_hyphen_cuts(text, (segment_range).clone());
                            for c in 0..match u32::try_from(existing.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                clean_cuts.push(existing[usize::try_from(c).unwrap_or(0)]);
                            }
                            let sep = latin_separator_cuts((segment_range).clone(), token_advance, is_long_opaque_latin_token);
                            for c in 0..match u32::try_from(sep.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                clean_cuts.push(sep[usize::try_from(c).unwrap_or(0)]);
                            }
                        } else {
                            if is_camel_case {
                                clean_cuts = ParagraphShapingStage::paragraph_shaping_stage_camel_case_cuts(text, (segment_range).clone());
                            } else {
                                if !all_letters {
                                    clean_cuts = latin_separator_cuts((segment_range).clone(), token_advance, is_long_opaque_latin_token);
                                }
                            }
                        }
                    }
                }
                let mut hyphen_cuts: Vec<u32> = Vec::new();
                if progressive_span.is_none() && all_letters && !is_abbreviation && !is_camel_case && !is_long_unhyphenated_letter_token && (u32::from_ne_bytes((u_string::find_from(&w, "-", 0)).to_ne_bytes())) > 2147483647 && u32::try_from((clean_cuts.len()) &
0xFFFF_FFFF).unwrap_or(0) == 0 {
                    hyphen_cuts = latin_word_cuts(((decision).as_ref().unwrap().clone()).clone(), (segment_range).clone(), &syllable_cuts);
                }
                let mut opaque_hard_cuts: Vec<u32> = Vec::new();
                if progressive_span.is_none() && is_latin && (!all_letters || is_long_unhyphenated_letter_token) && ((token_advance) > (measure) || is_long_opaque_latin_token) {
                    opaque_hard_cuts = latin_opaque_hard_cuts(((decision).as_ref().unwrap().clone()).clone(), (segment_range).clone(), &clean_cuts, is_long_opaque_latin_token);
                }
                if progressive_span.is_none() && (i32::from_ne_bytes((u32::try_from((opaque_hard_cuts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) {
                    let mut clean_bounds = vec![segment_range.start];
                    for c in 0..match u32::try_from(clean_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        clean_bounds.push(clean_cuts[usize::try_from(c).unwrap_or(0)]);
                    }
                    clean_bounds.push(segment_range.end);
                    let mut unique_bounds: Vec<u32> = Vec::new();
                    for &pt in &clean_bounds {
                        let mut exists = false;
                        for q in 0..match u32::try_from(unique_bounds.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            if unique_bounds[usize::try_from(q).unwrap_or(0)] == pt {
                                exists = true;
                                break;
                            }
                        }
                        if !exists {
                            unique_bounds.push(pt);
                        }
                    }
                    ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut unique_bounds);
                    for p in 0..u32::try_from((unique_bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
                        let piece_start = unique_bounds[usize::try_from(p).unwrap_or(0)];
                        let piece_end = unique_bounds[usize::try_from(u32::wrapping_add(p, 1)).unwrap_or(0)];
                        let mut has_piece_hard_cuts = false;
                        for &cut in &opaque_hard_cuts {
                            if ({ let v: u32 = cut; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = piece_start; i32::from_ne_bytes(v.to_ne_bytes()) }) && ({ let v: u32 = cut; i32::from_ne_bytes(v.to_ne_bytes()) }) < ({ let v: u32 = piece_end;
i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                has_piece_hard_cuts = true;
                                break;
                            }
                        }
                        if !has_piece_hard_cuts {
                            continue;
                        }
                        let piece_range = TextRange::new(piece_start, piece_end).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?;
                        let piece_reason = ParagraphShapingStage::paragraph_shaping_stage_strong_non_lexical_reason(u_string::substring(&text, { let v: u32 = piece_start; i32::from_ne_bytes(v.to_ne_bytes()) }, { let v: u32 = piece_end; i32::from_ne_bytes(v.to_ne_bytes())
}).as_str());
                        match &(piece_reason) {
                            Some(__option20) => {
                                register_emergency_tracking_eligibility((piece_range).clone(), __option20.as_str());
                            }
                            None => {
                            }
                        }
                    }
                }
                let capacity = clean_cuts.len();
                let mut all_cuts_combined = Vec::with_capacity(capacity);
                for c in 0..match u32::try_from(clean_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    all_cuts_combined.push(clean_cuts[usize::try_from(c).unwrap_or(0)]);
                }
                for c in 0..match u32::try_from(hyphen_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    all_cuts_combined.push(hyphen_cuts[usize::try_from(c).unwrap_or(0)]);
                }
                for c in 0..match u32::try_from(opaque_hard_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    all_cuts_combined.push(opaque_hard_cuts[usize::try_from(c).unwrap_or(0)]);
                }
                let mut all_cuts: Vec<u32> = Vec::new();
                for &pt in &all_cuts_combined {
                    let mut exists = false;
                    for q in 0..match u32::try_from(all_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if all_cuts[usize::try_from(q).unwrap_or(0)] == pt {
                            exists = true;
                            break;
                        }
                    }
                    if !exists {
                        all_cuts.push(pt);
                    }
                }
                ParagraphShapingStage::paragraph_shaping_stage_sort_ints(&mut all_cuts);
                if u32::try_from((all_cuts.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                    shaping_results.push(shaped.clone());
                } else {
                    if i32::from_ne_bytes((u32::try_from((hyphen_cuts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                        for &cut in &hyphen_cuts {
                            let mut exists = false;
                            for q in 0..match u32::try_from(hyphen_offsets.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                if hyphen_offsets[usize::try_from(q).unwrap_or(0)] == cut {
                                    exists = true;
                                    break;
                                }
                            }
                            if !exists {
                                hyphen_offsets.push(cut);
                            }
                        }
                        if hyphen_advance_or_null.is_none() {
                            let hyphen_shaped = engine.text_shaper.shape(ShapingInput::new("-", TextRange::new(0u32, 1u32).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?, (input.text_style).clone(), (decision).as_ref().unwrap().clone(),
Some("-".to_string()), Some(vec![]))).map_err(|e| ParagraphShapingStageShapeParagraphFault::TextShaperShapeFaultFault(e))?;
                            hyphen_advance_or_null = if u32::try_from((hyphen_shaped.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0) == 1 { Some(hyphen_shaped.clusters[0usize].advance) } else { Some(0.5f64 * font_size) };
                            hyphen_glyphs = Vec::new();
                            for r in 0..match u32::try_from(hyphen_shaped.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                let run = (hyphen_shaped.glyph_runs[usize::try_from(r).unwrap_or(0)]).clone();
                                for g in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                                    hyphen_glyphs.push((run.glyphs[usize::try_from(g).unwrap_or(0)]).clone());
                                }
                            }
                        }
                    }
                    let mut bounds = vec![segment_range.start];
                    for c in 0..match u32::try_from(all_cuts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        bounds.push(all_cuts[usize::try_from(c).unwrap_or(0)]);
                    }
                    bounds.push(segment_range.end);
                    for k in 0..u32::try_from((bounds.len()) & 0xFFFF_FFFF).unwrap_or(0).saturating_sub(1) {
                        let pieces = shape_segment_with_point_mark_prefix(((decision).as_ref().unwrap().clone()).clone(), TextRange::new(bounds[usize::try_from(k).unwrap_or(0)], bounds[usize::try_from(u32::wrapping_add(k, 1)).unwrap_or(0)]).map_err(|e|
ParagraphShapingStageShapeParagraphFault::TextRangeErrorFault(e))?);
                        for p in 0..match u32::try_from(pieces.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                            shaping_results.push((pieces[usize::try_from(p).unwrap_or(0)]).clone());
                        }
                    }
                }
            }
        }
        let hyphen_advance = (hyphen_advance_or_null).unwrap_or(0.0f64);
        let mut hyphen_offsets_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for i in 0..match u32::try_from(hyphen_offsets.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            hyphen_offsets_builder.put(&(hyphen_offsets[usize::try_from(i).unwrap_or(0)]));
        }
        let hyphen_offsets_set: SortedSetTable<u32> = hyphen_offsets_builder.clone().build();
        let mut rollbacks_builder: SortedMapTableBuilder<TextRange, String> = SortedTable::sorted_table_map_builder::<TextRange, String>(Arc::new(compare_text_range));
        let mut __loop_guard16 = substitution_rollback_keys.lock().unwrap();
        let mut __loop_guard17 = substitution_rollback_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard16.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            rollbacks_builder.put(&((__loop_guard16[usize::try_from(i).unwrap_or(0)]).clone()), &(__loop_guard17[usize::try_from(i).unwrap_or(0)]).clone());
        }
        drop(__loop_guard16);
        drop(__loop_guard17);
        let rollbacks_map: SortedMapTable<TextRange, String> = rollbacks_builder.clone().build();
        let mut prog_offsets_builder: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32, ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut __loop_guard18 = prog_break_keys.lock().unwrap();
        let mut __loop_guard19 = prog_break_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard18.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            prog_offsets_builder.put(&(__loop_guard18[usize::try_from(i).unwrap_or(0)]), &((__loop_guard19[usize::try_from(i).unwrap_or(0)]).clone()));
        }
        drop(__loop_guard18);
        drop(__loop_guard19);
        let prog_offsets_map: SortedMapTable<u32, ProgressiveBreakOpportunity> = prog_offsets_builder.clone().build();
        let mut segment_cache_builder: SortedMapTableBuilder<TextRange, ShapingResult> = SortedTable::sorted_table_map_builder::<TextRange, ShapingResult>(Arc::new(compare_text_range));
        let mut __loop_guard20 = segment_shaping_cache_keys.lock().unwrap();
        let mut __loop_guard21 = segment_shaping_cache_values.lock().unwrap();
        for i in 0..match u32::try_from(__loop_guard20.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            segment_cache_builder.put(&((__loop_guard20[usize::try_from(i).unwrap_or(0)]).clone()), &((__loop_guard21[usize::try_from(i).unwrap_or(0)]).clone()));
        }
        drop(__loop_guard20);
        drop(__loop_guard21);
        let segment_cache_map: SortedMapTable<TextRange, ShapingResult> = segment_cache_builder.clone().build();
        return Ok(ParagraphShapingStageResult::new(shaping_results.to_vec(), (hyphen_offsets_set).clone(), hyphen_advance, hyphen_glyphs.to_vec(), (rollbacks_map).clone(), break_opportunity_decisions.lock().unwrap().to_vec(),
emergency_tracking_eligibility_decisions.lock().unwrap().to_vec(), (prog_offsets_map).clone(), Some((segment_cache_map).clone())));
    }
}
