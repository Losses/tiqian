use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_number::UnicodeNumber;
use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_classifier::UnicodeScriptEvidenceClassifier;
use crate::org::tiqian::core::unicode_word_character::UnicodeWordCharacter;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::contextual_quote_role_resolver::ContextualQuoteRoleResolver;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub struct QuotePair {
    pub open_index: u32,
    pub close_index: u32,
    pub quote_type: QuoteType,
}

impl QuotePair {
    pub fn new(open_index: u32, close_index: u32, quote_type: QuoteType) -> Self {
        Self {
            open_index,
            close_index,
            quote_type,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "QuotePair(",
            "openIndex=",
            crate::runtime::int_text::IntText::int_text(self.open_index),
            ", ",
            "closeIndex=",
            crate::runtime::int_text::IntText::int_text(self.close_index),
            ", ",
            "quoteType=",
            self.quote_type.name(),
            ")"
        );
    }
}

fn quote_pair_quote_type_order(v: &QuoteType) -> i32 {
    match v {
        QuoteType::Single => 1,
        QuoteType::Double => 0,
    }
}
pub fn compare_quote_pair(a: &QuotePair, b: &QuotePair) -> i32 {
    let cmp_open_index = if a.open_index < b.open_index { -1 } else if a.open_index > b.open_index { 1 } else { 0 };
    if cmp_open_index != 0 { return cmp_open_index; }
    let cmp_close_index = if a.close_index < b.close_index { -1 } else if a.close_index > b.close_index { 1 } else { 0 };
    if cmp_close_index != 0 { return cmp_close_index; }
    let cmp_quote_type = match quote_pair_quote_type_order(&a.quote_type).cmp(&quote_pair_quote_type_order(&b.quote_type)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_quote_type != 0 { return cmp_quote_type; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteRoleDecision {
    pub index: u32,
    pub role: FontRole,
    pub source: String,
    pub reason: String,
}

impl QuoteRoleDecision {
    pub fn new(index: u32, role: FontRole, source: &str, reason: &str) -> Self {
        Self {
            index,
            role,
            source: source.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "QuoteRoleDecision(",
            "index=",
            crate::runtime::int_text::IntText::int_text(self.index),
            ", ",
            "role=",
            self.role.name(),
            ", ",
            "source=",
            (self.source).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

fn quote_role_decision_role_order(v: &FontRole) -> i32 {
    match v {
        FontRole::Unknown => 5,
        FontRole::Symbol => 3,
        FontRole::LatinText => 2,
        FontRole::Emoji => 4,
        FontRole::CjkText => 0,
        FontRole::CjkPunctuation => 1,
    }
}
pub fn compare_quote_role_decision(a: &QuoteRoleDecision, b: &QuoteRoleDecision) -> i32 {
    let cmp_index = if a.index < b.index { -1 } else if a.index > b.index { 1 } else { 0 };
    if cmp_index != 0 { return cmp_index; }
    let cmp_role = match quote_role_decision_role_order(&a.role).cmp(&quote_role_decision_role_order(&b.role)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_role != 0 { return cmp_role; }
    let cmp_source = SortedTable::sorted_table_compare_strings(a.source.as_str(), b.source.as_str());
    if cmp_source != 0 { return cmp_source; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}

#[derive(Clone, PartialEq)]
pub struct QuotePairAnalyzer {
}

impl QuotePairAnalyzer {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn analyze(&self, text: &str) -> Result<Vec<QuotePair>, TextRangeError> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut stack: Vec<OpenQuoteEntry> = vec![];
        let mut pairs: Vec<QuotePair> = vec![];
        let mut index = 0u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let c = u_string::unit_at_from(&__units1, index).unwrap_or(0);
            if c == 8220 {
                stack.push(OpenQuoteEntry { index: index, r#type: QuoteType::Double });
            } else {
                if c == 8216 {
                    stack.push(OpenQuoteEntry { index: index, r#type: QuoteType::Single });
                } else {
                    if c == 8221 {
                        if i32::from_ne_bytes((u32::try_from((stack.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) && stack[usize::try_from(u32::wrapping_sub(u32::try_from((stack.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].r#type == QuoteType::Double {
                            let r#match = stack.pop();
                            pairs.push(QuotePair::new(r#match.as_ref().unwrap().index, index, QuoteType::Double));
                        }
                    } else {
                        if c == 8217 {
                            if !QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_in_word_apostrophe(text, index)? && (i32::from_ne_bytes((u32::try_from((stack.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (0) &&
stack[usize::try_from(u32::wrapping_sub(u32::try_from((stack.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].r#type == QuoteType::Single {
                                let r#match = stack.pop();
                                pairs.push(QuotePair::new(r#match.as_ref().unwrap().index, index, QuoteType::Single));
                            }
                        }
                    }
                }
            }
            index = u32::wrapping_add(index, 1);
        }
        return Ok(pairs);
    }

    pub fn classify_pairs(&self, text: &str, pairs: &Vec<QuotePair>, context: Option<FontRoleContext>) -> Result<SortedMapTable<u32, FontRole>, TextRangeError> {
        return Ok(QuotePairAnalyzer::quote_pair_analyzer_map_decisions(&self.classify_quote_roles(text, &pairs, (context).clone())?));
    }

    pub fn classify_pairs_with_classifier(&self, text: &str, pairs: &Vec<QuotePair>, _font_role_classifier: Box<dyn FontRoleClassifier>, context: Option<FontRoleContext>) -> Result<SortedMapTable<u32, FontRole>, TextRangeError> {
        return Ok(self.classify_pairs(text, &pairs, (context).clone())?);
    }

    pub fn classify_quote_roles(&self, text: &str, pairs: &Vec<QuotePair>, context: Option<FontRoleContext>) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        return Ok(ContextualQuoteRoleResolver::new(text, pairs.to_vec(), match &(context) { None => FontRoleContext::new(Some("zh-Hans".to_string()), None), Some(__option) => (*__option).clone() }).resolve()?);
    }

    pub fn classify_quote_roles_with_classifier(&self, text: &str, pairs: &Vec<QuotePair>, _font_role_classifier: Box<dyn FontRoleClassifier>, context: Option<FontRoleContext>) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        return Ok(self.classify_quote_roles(text, &pairs, (context).clone())?);
    }

    pub(crate) fn quote_pair_analyzer_map_decisions(decisions: &Vec<QuoteRoleDecision>) -> SortedMapTable<u32, FontRole> {
        let mut out: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for d in decisions {
            out.put(&(d.index), &(d.role));
        }
        return out.clone().build();
    }

    pub fn quote_pair_analyzer_is_non_cjk_in_word_apostrophe(text: &str, index: u32) -> Result<bool, TextRangeError> {
        let before = QuotePairAnalyzer::quote_pair_analyzer_code_point_before(text, index);
        if before.is_none() {
            return Ok(false);
        }
        let after = QuotePairAnalyzer::quote_pair_analyzer_code_point_at_or_null(text, u32::wrapping_add(index, 1));
        if after.is_none() {
            return Ok(false);
        }
        return Ok(QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_word_character(*(before).as_ref().unwrap())? && QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_word_character(*(after).as_ref().unwrap())? &&
(QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_non_numeric_word_character(*(before).as_ref().unwrap())? || QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_non_numeric_word_character(*(after).as_ref().unwrap())?));
    }

    pub fn quote_pair_analyzer_is_digit_bound_closing_quote(text: &str, index: u32) -> Result<bool, TextRangeError> {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        return Ok((u_string::unit_at_from(&__units2, index).as_ref().map_or(false, |v| v == &(8217)) || u_string::unit_at_from(&__units2, index).as_ref().map_or(false, |v| v == &(8221))) && QuotePairAnalyzer::quote_pair_analyzer_code_point_before(text, index).is_some() &&
UnicodeNumber::unicode_number_contains(*(QuotePairAnalyzer::quote_pair_analyzer_code_point_before(text, index)).as_ref().unwrap())?);
    }

    pub fn quote_pair_analyzer_is_non_cjk_word_internal_quote_pair(text: &str, pair: QuotePair) -> Result<bool, TextRangeError> {
        let before = QuotePairAnalyzer::quote_pair_analyzer_code_point_before(text, pair.open_index);
        let after = QuotePairAnalyzer::quote_pair_analyzer_code_point_at_or_null(text, u32::wrapping_add(pair.close_index, 1));
        if match &(before) { None => true, Some(__option2) => after.is_none() } || !QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_non_numeric_word_character(*(before).as_ref().unwrap())? ||
!QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_non_numeric_word_character(*(after).as_ref().unwrap())? {
            return Ok(false);
        }
        let mut index = u32::wrapping_add(pair.open_index, 1);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((pair.close_index).to_ne_bytes())) {
            let cp = QuotePairAnalyzer::quote_pair_analyzer_code_point_at_or_null(text, index);
            if match &(cp) { None => true, Some(__option3) => !QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_word_character(*__option3)? } {
                return Ok(false);
            }
            index = u32::wrapping_add(index, if i32::from_ne_bytes((cp.unwrap_or(0)).to_ne_bytes()) > (65535) { 2 } else { 1 });
        }
        return Ok(true);
    }

    pub(crate) fn quote_pair_analyzer_is_non_cjk_word_character(cp: u32) -> Result<bool, TextRangeError> {
        return Ok(UnicodeWordCharacter::unicode_word_character_contains(cp)? && UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(cp)? != UnicodeScriptEvidence::EastAsian);
    }

    pub(crate) fn quote_pair_analyzer_is_non_cjk_non_numeric_word_character(cp: u32) -> Result<bool, TextRangeError> {
        return Ok(QuotePairAnalyzer::quote_pair_analyzer_is_non_cjk_word_character(cp)? && !UnicodeNumber::unicode_number_contains(cp)? && !QuotePairAnalyzer::quote_pair_analyzer_is_fullwidth_east_asian_width(cp));
    }

    pub(crate) fn quote_pair_analyzer_is_fullwidth_east_asian_width(cp: u32) -> bool {
        return cp == 12288 || (i32::from_ne_bytes((cp).to_ne_bytes())) >= 65281 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 65376 || (i32::from_ne_bytes((cp).to_ne_bytes())) >= 65504 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 65510;
    }

    pub fn quote_pair_analyzer_code_point_before(text: &str, index: u32) -> Option<u32> {
    let __units3 = u_string::units(&text);
    let __count3 = u_string::unit_count(&text);
        if i32::from_ne_bytes((index).to_ne_bytes()) <= 0 {
            return None;
        }
        let low = u_string::unit_at_from(&__units3, u32::wrapping_sub(index, 1)).unwrap_or(0);
        if i32::from_ne_bytes((low).to_ne_bytes()) < (56320) || (i32::from_ne_bytes((low).to_ne_bytes())) > (57343) || (i32::from_ne_bytes((index).to_ne_bytes())) < (2) {
            return Some(low);
        }
        let high = u_string::unit_at_from(&__units3, u32::wrapping_sub(index, 2)).unwrap_or(0);
        if i32::from_ne_bytes((high).to_ne_bytes()) < (55296) || (i32::from_ne_bytes((high).to_ne_bytes())) > (56319) {
            return Some(low);
        }
        return Some(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, 55296)) << (10)), u32::wrapping_sub(low, 56320)));
    }

    pub fn quote_pair_analyzer_code_point_at_or_null(text: &str, index: u32) -> Option<u32> {
    let __units4 = u_string::units(&text);
    let __count4 = u_string::unit_count(&text);
        if index > 2147483647 || (i32::from_ne_bytes((index).to_ne_bytes())) >= i32::from_ne_bytes((__count4).to_ne_bytes()) {
            return None;
        }
        let high = u_string::unit_at_from(&__units4, index).unwrap_or(0);
        if i32::from_ne_bytes((high).to_ne_bytes()) < (55296) || (i32::from_ne_bytes((high).to_ne_bytes())) > (56319) || (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) >= i32::from_ne_bytes((__count4).to_ne_bytes()) {
            return Some(high);
        }
        let low = u_string::unit_at_from(&__units4, u32::wrapping_add(index, 1)).unwrap_or(0);
        if i32::from_ne_bytes((low).to_ne_bytes()) < (56320) || (i32::from_ne_bytes((low).to_ne_bytes())) > (57343) {
            return Some(high);
        }
        return Some(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, 55296)) << (10)), u32::wrapping_sub(low, 56320)));
    }
}

#[derive(Clone)]
pub struct QuotePairAwareFontRoleClassifier {
    pub(crate) delegate: Box<dyn FontRoleClassifier>,
    pub(crate) quote_roles: SortedMapTable<u32, FontRole>,
}

impl QuotePairAwareFontRoleClassifier {
    pub fn new(delegate: Box<dyn FontRoleClassifier>, quote_roles: SortedMapTable<u32, FontRole>) -> Self {
        Self {
            delegate,
            quote_roles,
        }
    }

    pub fn classify(&self, text: &str, range: TextRange, context: Option<FontRoleContext>) -> FontRole {
        let role = (self.quote_roles).clone().get(&(range.start));
        return match &(role) { None => self.delegate.classify(text, (range).clone(), (context).clone()), Some(__option4) => *__option4 };
    }

    pub fn quote_pair_aware_font_role_classifier_with_contextual_quote_roles(base: Box<dyn FontRoleClassifier>, text: &str, context: Option<FontRoleContext>) -> Result<Box<dyn FontRoleClassifier>, TextRangeError> {
        let analyzer = QuotePairAnalyzer::new();
        let decisions = analyzer.classify_quote_roles(text, &analyzer.analyze(text)?, (context).clone())?;
        if u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(base.clone());
        }
        let mut roles: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        for decision in &decisions {
            roles.put(&(decision.index), &(decision.role));
        }
        return Ok(Box::new(QuotePairAwareFontRoleClassifier::new(base.clone(), roles.clone().build())));
    }
}

impl FontRoleClassifier for QuotePairAwareFontRoleClassifier {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.QuotePairAnalyzer.QuotePairAwareFontRoleClassifier"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontRoleClassifier> {
        Box::new(self.clone())
    }

    fn classify(&self, text: &str, range: TextRange, context: Option<FontRoleContext>) -> FontRole {
        let role = (self.quote_roles).clone().get(&(range.start));
        return match &(role) { None => self.delegate.classify(text, (range).clone(), (context).clone()), Some(__option5) => *__option5 };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuoteType {
    Double,
    Single,
}

pub fn compare_quote_type(a: &QuoteType, b: &QuoteType) -> i32 {
    if a == b { return 0; }
    fn rank(v: &QuoteType) -> i32 {
        match v {
            QuoteType::Double => 0,
            QuoteType::Single => 1,
        }
    }
    rank(a) - rank(b)
}

impl QuoteType {
    pub fn to_string(&self) -> String {
        match self {
            QuoteType::Double => "Double".to_string(),
            QuoteType::Single => "Single".to_string(),
        }
    }
}

impl QuoteType {
    pub fn name(&self) -> &'static str {
        match self {
            QuoteType::Double => "Double",
            QuoteType::Single => "Single",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenQuoteEntry {
    pub index: u32,
    pub r#type: QuoteType,
}
