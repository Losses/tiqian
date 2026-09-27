use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::core::unicode_number::UnicodeNumber;
use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_classifier::UnicodeScriptEvidenceClassifier;
use crate::org::tiqian::core::unicode_word_character::UnicodeWordCharacter;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteRoleDecision;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Clone, PartialEq)]
pub struct ContextualQuoteRoleResolver {
    pub(crate) text: String,
    pub(crate) pairs: Vec<QuotePair>,
    pub(crate) context: FontRoleContext,
    pub(crate) pair_by_open: SortedMapTable<u32, QuotePair>,
    pub(crate) pair_by_close: SortedMapTable<u32, QuotePair>,
    pub(crate) parent_by_pair: SortedMapTable<u32, Option<QuotePair>>,
}

impl ContextualQuoteRoleResolver {
    pub fn new(text: &str, pairs: Vec<QuotePair>, context: FontRoleContext) -> Self {
        let mut __self = Self {
            text: text.to_string(),
            pairs: pairs.clone(),
            context: context.clone(),
            pair_by_open: SortedTable::sorted_table_map_builder::<u32, QuotePair>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).build(),
            pair_by_close: SortedTable::sorted_table_map_builder::<u32, QuotePair>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).build(),
            parent_by_pair: SortedTable::sorted_table_map_builder::<u32, Option<QuotePair>>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes())))).build(),
        };
        let mut open_builder: SortedMapTableBuilder<u32, QuotePair> = SortedTable::sorted_table_map_builder::<u32, QuotePair>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut close_builder: SortedMapTableBuilder<u32, QuotePair> = SortedTable::sorted_table_map_builder::<u32, QuotePair>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut pi = 0u32;
        while (i32::from_ne_bytes((pi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pairs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let pair = (pairs[usize::try_from(pi).unwrap_or(0)]).clone();
            pi = u32::wrapping_add(pi, 1);
            open_builder.put(&(pair.open_index), &(pair));
            close_builder.put(&(pair.close_index), &(pair));
        }
        let mut parent_builder: SortedMapTableBuilder<u32, Option<QuotePair>> = SortedTable::sorted_table_map_builder::<u32, Option<QuotePair>>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes()))));
        pi = 0u32;
        while (i32::from_ne_bytes((pi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pairs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let pair = (pairs[usize::try_from(pi).unwrap_or(0)]).clone();
            pi = u32::wrapping_add(pi, 1);
            parent_builder.put(&(pair.open_index), &(__self.find_parent((pair).clone())));
        }
            __self.pair_by_open = open_builder.clone().build();
            __self.pair_by_close = close_builder.clone().build();
            __self.parent_by_pair = parent_builder.clone().build();
        return __self;
    }

    pub fn resolve(&self) -> Result<Vec<QuoteRoleDecision>, TextRangeError> {
        let mut decisions: Vec<QuoteRoleDecision> = vec![];
        let mut resolved_pairs: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let pipeline_result = {
    let mut _sorted = (self.pairs).clone().to_vec();
    _sorted.sort_by_key(|pair| pair.open_index);
    _sorted
};
        let ordered = (pipeline_result).clone();
        for pair in &ordered {
            let resolution = self.resolve_pair((pair).clone(), (resolved_pairs).clone())?;
            resolved_pairs.put(&(pair.open_index), &(resolution.role));
            decisions.push(QuoteRoleDecision::new(pair.open_index, resolution.role, (resolution.source).to_string().as_str(), (resolution.reason).to_string().as_str()));
            decisions.push(QuoteRoleDecision::new(pair.close_index, resolution.role, (resolution.source).to_string().as_str(), (resolution.reason).to_string().as_str()));
        }
        let mut paired_indices_builder: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut pi = 0u32;
        while (i32::from_ne_bytes((pi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from(((self.pairs).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let pair = (self.pairs[usize::try_from(pi).unwrap_or(0)]).clone();
            pi = u32::wrapping_add(pi, 1);
            paired_indices_builder.put(&(pair.open_index));
            paired_indices_builder.put(&(pair.close_index));
        }
        let paired_indices: SortedSetTable<u32> = paired_indices_builder.clone().build();
        for index in 0..match u32::try_from(u_string::unit_count(&((self.text).to_string()))) { Ok(value) => value, Err(_) => u32::MAX } {
            if paired_indices.has(&(index)) || !ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_ambiguous_curly_quote(*(u_string::unit_at(&(self.text).to_string(), index)).as_ref().unwrap()) {
                continue;
            }
            let resolution = self.resolve_unmatched(index)?;
            decisions.push(QuoteRoleDecision::new(index, resolution.role, (resolution.source).to_string().as_str(), (resolution.reason).to_string().as_str()));
        }
        let pipeline_result1 = {
    let mut _sorted = decisions.to_vec();
    _sorted.sort_by_key(|d| d.index);
    _sorted
};
        let sorted = (pipeline_result1).clone();
        return Ok(sorted);
    }

    fn resolve_pair(&self, pair: QuotePair, resolved_pairs: SortedMapTableBuilder<u32, FontRole>) -> Result<Resolution, TextRangeError> {
        let parent = (self.parent_by_pair).clone().get(&(pair.open_index)).flatten();
        let enclosing_start = match &(parent) { None => 0, Some(__option) => u32::wrapping_add(__option.open_index, 1) };
        let enclosing_end = match &(parent) { None => u_string::unit_count(&((self.text).to_string())), Some(__option1) => __option1.close_index };
        let mut outer_evidence = ScriptEvidence::new((self).clone());
        let _ = outer_evidence.add_range(enclosing_start, pair.open_index)?;
        let _ = outer_evidence.add_range(u32::wrapping_add(pair.close_index, 1), enclosing_end)?;
        let mut content_evidence = ScriptEvidence::new((self).clone());
        let _ = content_evidence.add_range(u32::wrapping_add(pair.open_index, 1), pair.close_index)?;
        if match &(self.char_at(u32::wrapping_sub(pair.open_index, 1))) { Some(__option2) => ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_ascii_space_or_tab(Some(*__option2)) && content_evidence.has_western && !content_evidence.has_cjk, None => false } {
            return Ok(Resolution::new(FontRole::LatinText, "DelimitedWesternQuotationRun", "whitespace-delimited-wholly-western-quotation"));
        }
        if self.is_non_cjk_word_internal_quote_pair((pair).clone())? {
            return Ok(Resolution::new(FontRole::LatinText, "NonCjkWordInternalQuotePair", "non-cjk-word-internal-quotation"));
        }
        let outer_role = outer_evidence.unambiguous_role();
        match &(outer_role) {
            Some(__option3) => {
                return Ok(Resolution::new(*__option3, "PairedPunctuationOuterScriptContext", "quote-pair-inherits-enclosing-level-script"));
            }
            None => {
            }
        }
        if outer_evidence.get_is_mixed() {
            return Ok(self.paragraph_language_resolution(&"mixed-enclosing-level-script"));
        }
        match &(parent) {
            Some(__option4) => {
                let enclosing_role = resolved_pairs.get(&(__option4.open_index));
                match &(enclosing_role) {
                    Some(__option5) => {
                        return Ok(Resolution::new(*__option5, "PairedPunctuationEnclosingQuoteContext", "quote-pair-inherits-enclosing-quotation"));
                    }
                    None => {
                    }
                }
            }
            None => {
            }
        }
        let content_role = content_evidence.unambiguous_role();
        match &(content_role) {
            Some(__option6) => {
                return Ok(Resolution::new(*__option6, "PairedPunctuationContentScriptContext", "quoted-content-script"));
            }
            None => {
            }
        }
        return Ok(self.paragraph_language_resolution(if content_evidence.get_is_mixed() { "mixed-quoted-content".to_string() } else { "no-strong-script-context".to_string() }.as_str()));
    }

    fn resolve_unmatched(&self, index: u32) -> Result<Resolution, TextRangeError> {
        if u_string::unit_at(&(self.text).to_string(), index).as_ref().map_or(false, |v| v == &(8217)) && self.is_non_cjk_in_word_apostrophe(index)? {
            return Ok(Resolution::new(FontRole::LatinText, "NonCjkInWordApostrophe", "non-cjk-in-word-apostrophe"));
        }
        if self.is_digit_bound_closing_quote(index)? {
            return Ok(Resolution::new(FontRole::LatinText, "NumericPrimeUnmatchedQuote", "digit-bound-unmatched-quote-as-prime"));
        }
        let left_role = self.nearest_strong_script_role(u32::wrapping_sub(index, 1), 4294967295u32)?;
        let right_role = self.nearest_strong_script_role(u32::wrapping_add(index, 1), 1)?;
        if ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_ascii_space_or_tab(self.char_at(u32::wrapping_sub(index, 1))) && right_role == Some(FontRole::LatinText) {
            return Ok(Resolution::new(FontRole::LatinText, "DelimitedUnmatchedWesternQuote", "whitespace-delimited-unmatched-western-quote"));
        }
        match &(left_role) {
            Some(__option7) => {
                if match &(right_role) { None => true, Some(__option8) => Some(*__option8) == Some(*__option7) } {
                return Ok(Resolution::new(*__option7, "UnmatchedQuoteSurroundingScriptContext", "unmatched-quote-surrounding-script"));
                }
            }
            None => {
            }
        }
        match &(right_role) {
            Some(__option9) => {
                if left_role == None {
                return Ok(Resolution::new(*__option9, "UnmatchedQuoteSurroundingScriptContext", "unmatched-quote-surrounding-script"));
                }
            }
            None => {
            }
        }
        let reason = if match &(left_role) { Some(__option12) => right_role != None, None => false } { "conflicting-unmatched-quote-context".to_string() } else { "no-unmatched-quote-context".to_string() };
        return Ok(self.paragraph_language_resolution(reason.as_str()));
    }

    fn nearest_strong_script_role(&self, start_index: u32, direction: u32) -> Result<Option<FontRole>, TextRangeError> {
        let mut index = start_index;
        while (index) <= 2147483647 && (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&((self.text).to_string()))).to_ne_bytes())) {
            if direction > 2147483647 {
                let pair = (self.pair_by_close).clone().get(&(index));
                match &(pair) {
                    Some(__option13) => {
                        index = u32::wrapping_sub(__option13.open_index, 1);
                        continue;
                    }
                    None => {
                    }
                }
            } else {
                let pair = (self.pair_by_open).clone().get(&(index));
                match &(pair) {
                    Some(__option14) => {
                        index = u32::wrapping_add(__option14.close_index, 1);
                        continue;
                    }
                    None => {
                    }
                }
            }
            let scalar_start = if direction > 2147483647 && ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_low_surrogate(*(u_string::unit_at(&(self.text).to_string(), index)).as_ref().unwrap()) && (i32::from_ne_bytes((index).to_ne_bytes())) > (0) &&
ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_high_surrogate(*(u_string::unit_at(&(self.text).to_string(), u32::wrapping_sub(index, 1))).as_ref().unwrap()) { u32::wrapping_sub(index, 1) } else { index };
            let length = self.code_point_length_at(scalar_start, u_string::unit_count(&((self.text).to_string())));
            let role = self.strong_script_role(scalar_start, length)?;
            match &(role) {
                Some(__option15) => {
                    return Ok(Some(*__option15));
                }
                None => {
                }
            }
            index = if direction > 2147483647 { u32::wrapping_sub(scalar_start, 1) } else { u32::wrapping_add(scalar_start, length) };
        }
        return Ok(None);
    }

    fn paragraph_language_resolution(&self, reason: &str) -> Resolution {
        let role = if UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(((self.context).clone().locale).to_string().as_str()) { FontRole::CjkPunctuation } else { FontRole::LatinText };
        return Resolution::new(role, "ParagraphLanguageQuoteContext", format!("{}{}{}",
            reason,
            "; paragraph-language=",
            ((self.context).clone().locale).to_string()
        ).as_str());
    }

    fn find_parent(&self, pair: QuotePair) -> Option<QuotePair> {
        let mut result: Option<QuotePair> = None;
        let mut ci = 0u32;
        while (i32::from_ne_bytes((ci).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from(((self.pairs).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let candidate = (self.pairs[usize::try_from(ci).unwrap_or(0)]).clone();
            ci = u32::wrapping_add(ci, 1);
            if candidate != pair && (i32::from_ne_bytes((candidate.open_index).to_ne_bytes())) < (i32::from_ne_bytes((pair.open_index).to_ne_bytes())) && (i32::from_ne_bytes((candidate.close_index).to_ne_bytes())) > (i32::from_ne_bytes((pair.close_index).to_ne_bytes())) && (match
&(result) { None => true, Some(__option16) => i32::from_ne_bytes((u32::wrapping_sub(candidate.close_index, candidate.open_index)).to_ne_bytes()) < (i32::from_ne_bytes((u32::wrapping_sub(__option16.close_index, __option16.open_index)).to_ne_bytes())) }) {
                result = Some(candidate.clone());
            }
        }
        return result;
    }

    fn strong_script_role(&self, index: u32, length: u32) -> Result<Option<FontRole>, TextRangeError> {
        let evidence = UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(self.code_point_at(index, u32::wrapping_add(index, length)))?;
        return Ok(match evidence {
            UnicodeScriptEvidence::Neutral => None,
            UnicodeScriptEvidence::EastAsian => Some(FontRole::CjkPunctuation),
            UnicodeScriptEvidence::Other => Some(FontRole::LatinText),
        });
    }

    fn is_non_cjk_in_word_apostrophe(&self, index: u32) -> Result<bool, TextRangeError> {
        let before = self.code_point_before(index);
        let after = self.code_point_at_or_null(u32::wrapping_add(index, 1));
        return Ok(match &(before) { Some(__option17) => after.is_some() && self.is_non_cjk_word_character(*__option17)? && self.is_non_cjk_word_character(*(after).as_ref().unwrap())? && (self.is_non_cjk_non_numeric_word_character(*__option17)? ||
self.is_non_cjk_non_numeric_word_character(*(after).as_ref().unwrap())?), None => false });
    }

    fn is_digit_bound_closing_quote(&self, index: u32) -> Result<bool, TextRangeError> {
        let before = self.code_point_before(index);
        return Ok((u_string::unit_at(&(self.text).to_string(), index).as_ref().map_or(false, |v| v == &(8217)) || u_string::unit_at(&(self.text).to_string(), index).as_ref().map_or(false, |v| v == &(8221))) && before.is_some() &&
UnicodeNumber::unicode_number_contains(*(before).as_ref().unwrap())?);
    }

    fn is_non_cjk_word_internal_quote_pair(&self, pair: QuotePair) -> Result<bool, TextRangeError> {
        let before = self.code_point_before(pair.open_index);
        let after = self.code_point_at_or_null(u32::wrapping_add(pair.close_index, 1));
        if match &(before) { None => true, Some(__option18) => after.is_none() } || !self.is_non_cjk_non_numeric_word_character(*(before).as_ref().unwrap())? || !self.is_non_cjk_non_numeric_word_character(*(after).as_ref().unwrap())? {
            return Ok(false);
        }
        let mut index = u32::wrapping_add(pair.open_index, 1);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((pair.close_index).to_ne_bytes())) {
            let code_point = self.code_point_at_or_null(index);
            if match &(code_point) { None => true, Some(__option19) => !self.is_non_cjk_word_character(*__option19)? } {
                return Ok(false);
            }
            index = u32::wrapping_add(index, if i32::from_ne_bytes((code_point.unwrap_or(0)).to_ne_bytes()) > (65535) { 2 } else { 1 });
        }
        return Ok(true);
    }

    fn is_non_cjk_word_character(&self, code_point: u32) -> Result<bool, TextRangeError> {
        return Ok(UnicodeWordCharacter::unicode_word_character_contains(code_point)? && UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(code_point)? != UnicodeScriptEvidence::EastAsian);
    }

    fn is_non_cjk_non_numeric_word_character(&self, code_point: u32) -> Result<bool, TextRangeError> {
        return Ok(self.is_non_cjk_word_character(code_point)? && !UnicodeNumber::unicode_number_contains(code_point)? && !self.is_fullwidth(code_point));
    }

    fn is_fullwidth(&self, code_point: u32) -> bool {
        return code_point == 12288 || (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 65281 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 65376 || (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 65504 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 65510;
    }

    fn char_at(&self, index: u32) -> Option<u32> {
        return if index <= 2147483647 && (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&((self.text).to_string()))).to_ne_bytes())) { u_string::unit_at(&(self.text).to_string(), index) } else { None };
    }

    fn code_point_at(&self, index: u32, end: u32) -> u32 {
        let high = u_string::unit_at(&(self.text).to_string(), index).unwrap_or(0);
        if !ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_high_surrogate(high) || (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) >= i32::from_ne_bytes((end).to_ne_bytes()) {
            return high;
        }
        let low = u_string::unit_at(&(self.text).to_string(), u32::wrapping_add(index, 1)).unwrap_or(0);
        return if ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_low_surrogate(low) { u32::wrapping_sub(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, 55296)) << (10)), low), 56320) } else { high };
    }

    fn code_point_length_at(&self, index: u32, end: u32) -> u32 {
        return if ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_high_surrogate(*(u_string::unit_at(&(self.text).to_string(), index)).as_ref().unwrap()) && (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) <
(i32::from_ne_bytes((end).to_ne_bytes())) && ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_low_surrogate(*(u_string::unit_at(&(self.text).to_string(), u32::wrapping_add(index, 1))).as_ref().unwrap()) { 2 } else { 1 };
    }

    fn code_point_before(&self, index: u32) -> Option<u32> {
        if i32::from_ne_bytes((index).to_ne_bytes()) <= 0 {
            return None;
        }
        let low = u_string::unit_at(&(self.text).to_string(), u32::wrapping_sub(index, 1)).unwrap_or(0);
        if !ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_low_surrogate(low) || (i32::from_ne_bytes((index).to_ne_bytes())) < (2) {
            return Some(low);
        }
        let high = u_string::unit_at(&(self.text).to_string(), u32::wrapping_sub(index, 2)).unwrap_or(0);
        return if ContextualQuoteRoleResolver::contextual_quote_role_resolver_is_high_surrogate(high) { Some(u32::wrapping_sub(u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, 55296)) << (10)), low), 56320)) } else { Some(low) };
    }

    fn code_point_at_or_null(&self, index: u32) -> Option<u32> {
        return if index > 2147483647 || (i32::from_ne_bytes((index).to_ne_bytes())) >= i32::from_ne_bytes((u_string::unit_count(&((self.text).to_string()))).to_ne_bytes()) { None } else { Some(self.code_point_at(u32::from_ne_bytes((index).to_ne_bytes()),
u_string::unit_count(&((self.text).to_string())))) };
    }

    pub(crate) fn contextual_quote_role_resolver_is_ambiguous_curly_quote(code_point: u32) -> bool {
        return code_point == 8216 || code_point == 8217 || code_point == 8220 || code_point == 8221;
    }

    pub(crate) fn contextual_quote_role_resolver_is_ascii_space_or_tab(code_point: Option<u32>) -> bool {
        return match &(code_point) { Some(__option20) => *__option20 == 32 || *__option20 == 9, None => false };
    }

    pub(crate) fn contextual_quote_role_resolver_is_high_surrogate(code_point: u32) -> bool {
        return (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 55296 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 56319;
    }

    pub(crate) fn contextual_quote_role_resolver_is_low_surrogate(code_point: u32) -> bool {
        return (i32::from_ne_bytes((code_point).to_ne_bytes())) >= 56320 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 57343;
    }
}

#[derive(Clone, PartialEq)]
pub struct ScriptEvidence {
    pub has_cjk: bool,
    pub has_western: bool,
    pub(crate) resolver: ContextualQuoteRoleResolver,
}

impl ScriptEvidence {
    pub fn new(resolver: ContextualQuoteRoleResolver) -> Self {
        Self {
            resolver,
            has_cjk: false,
            has_western: false,
        }
    }

    pub fn get_is_mixed(&self) -> bool {
        return self.has_cjk && self.has_western;
    }

    pub fn add_range(&mut self, start: u32, end: u32) -> Result<(), TextRangeError> {
        let mut index = start;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((end).to_ne_bytes())) {
            let nested = self.resolver.pair_by_open.get(&(index));
            match &(nested) {
                Some(__option21) => {
                    if i32::from_ne_bytes((__option21.close_index).to_ne_bytes()) < (i32::from_ne_bytes((end).to_ne_bytes())) {
                    index = u32::wrapping_add(__option21.close_index, 1);
                    continue;
                    }
                }
                None => {
                }
            }
            let length = self.resolver.code_point_length_at(index, end);
            let role = self.resolver.strong_script_role(index, length)?;
            if role == Some(FontRole::CjkPunctuation) {
                self.has_cjk = true;
            } else {
                if role == Some(FontRole::LatinText) {
                    self.has_western = true;
                }
            }
            index = u32::wrapping_add(index, length);
        }
        Ok(())
    }

    pub fn unambiguous_role(&self) -> Option<FontRole> {
        return if self.has_cjk && !self.has_western { Some(FontRole::CjkPunctuation) } else { if self.has_western && !self.has_cjk { Some(FontRole::LatinText) } else { None } };
    }
}

#[derive(Clone, PartialEq)]
pub struct Resolution {
    pub role: FontRole,
    pub source: String,
    pub reason: String,
}

impl Resolution {
    pub fn new(role: FontRole, source: &str, reason: &str) -> Self {
        Self {
            role,
            source: source.to_string(),
            reason: reason.to_string(),
        }
    }
}
