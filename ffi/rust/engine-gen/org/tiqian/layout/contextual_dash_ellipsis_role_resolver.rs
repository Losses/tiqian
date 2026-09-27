use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_classifier::UnicodeScriptEvidenceClassifier;
use crate::org::tiqian::core::unicode_word_character::UnicodeWordCharacter;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub struct DashEllipsisRoleDecision {
    pub range: TextRange,
    pub role: FontRole,
    pub source: String,
    pub reason: String,
}

impl DashEllipsisRoleDecision {
    pub fn new(range: TextRange, role: FontRole, source: &str, reason: &str) -> Result<Self, TextRangeError> {
        Ok(Self {
            range,
            role,
            source: source.to_string(),
            reason: reason.to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "DashEllipsisRoleDecision(",
            "range=",
            (self.range).clone().to_string(),
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

fn dash_ellipsis_role_decision_role_order(v: &FontRole) -> i32 {
    match v {
        FontRole::Unknown => 5,
        FontRole::Symbol => 3,
        FontRole::LatinText => 2,
        FontRole::Emoji => 4,
        FontRole::CjkText => 0,
        FontRole::CjkPunctuation => 1,
    }
}
pub fn compare_dash_ellipsis_role_decision(a: &DashEllipsisRoleDecision, b: &DashEllipsisRoleDecision) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_role = match dash_ellipsis_role_decision_role_order(&a.role).cmp(&dash_ellipsis_role_decision_role_order(&b.role)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_role != 0 { return cmp_role; }
    let cmp_source = SortedTable::sorted_table_compare_strings(a.source.as_str(), b.source.as_str());
    if cmp_source != 0 { return cmp_source; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}

#[derive(Clone, PartialEq)]
pub struct ContextualDashEllipsisRoleResolver {
}

impl ContextualDashEllipsisRoleResolver {
    pub fn new() -> Result<Self, TextRangeError> {
        Ok(Self {
        })
    }

    pub fn resolve(&self, text: &str, context: Option<FontRoleContext>) -> Result<Vec<DashEllipsisRoleDecision>, TextRangeError> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let actual_context = match &(context) { None => FontRoleContext::new(Some("zh-Hans".to_string()), None), Some(__option) => (*__option).clone() };
        let mut has_mark = false;
        let mut ci = 0u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((ci).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            if ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_is_contextual_dash_or_ellipsis(*(u_string::unit_at_from(&__units1, ci)).as_ref().unwrap()) {
                has_mark = true;
                break;
            }
            ci = u32::wrapping_add(ci, 1);
        }
        if !has_mark {
            return Ok(vec![]);
        }
        let strong_script_context = StrongScriptContextIndex::new(text)?;
        let runs = self.collect_runs(text)?;
        let pair_resolutions: SortedMapTable<u32, Resolution> = self.resolve_parenthetical_pairs(text, &runs, (strong_script_context).clone(), (actual_context).clone())?;
        let mut result: Vec<DashEllipsisRoleDecision> = vec![];
        let mut ri = 0u32;
        while (i32::from_ne_bytes((ri).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((runs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let range = (runs[usize::try_from(ri).unwrap_or(0)]).clone();
            let paired = pair_resolutions.get(&(range.start));
            let resolution = match &(paired) { None => self.resolve_single_run((range).clone(), (strong_script_context).clone(), (actual_context).clone())?, Some(__option1) => (*__option1).clone() };
            result.push(DashEllipsisRoleDecision::new((range).clone(), resolution.role, (resolution.source).to_string().as_str(), (resolution.reason).to_string().as_str())?);
            ri = u32::wrapping_add(ri, 1);
        }
        return Ok(result);
    }

    fn collect_runs(&self, text: &str) -> Result<Vec<TextRange>, TextRangeError> {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        let mut runs: Vec<TextRange> = vec![];
        let mut index = 0u32;
        let __units3 = u_string::units(&text);
        let __count3 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
            if !ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_is_contextual_dash_or_ellipsis(*(u_string::unit_at_from(&__units3, index)).as_ref().unwrap()) {
                index = u32::wrapping_add(index, ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_code_point_length_at(text, index));
                continue;
            }
            let start = index;
            let __units4 = u_string::units(&text);
            let __count4 = u_string::unit_count(&text);
            while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) && ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_is_contextual_dash_or_ellipsis(*(u_string::unit_at_from(&__units3,
index)).as_ref().unwrap()) {
                index = u32::wrapping_add(index, 1);
            }
            runs.push(TextRange::new(start, index)?);
        }
        return Ok(runs);
    }

    fn resolve_single_run(&self, range: TextRange, strong_script_context: StrongScriptContextIndex, context: FontRoleContext) -> Result<Resolution, TextRangeError> {
        let left_role = strong_script_context.left_of(range.start);
        let right_role = strong_script_context.right_of(range.end);
        match &(left_role) {
            Some(__option2) => {
                if right_role == Some(*__option2) {
                return Ok(Resolution::new(*__option2, "DashEllipsisSurroundingScriptContext", "matching-surrounding-script")?);
                }
            }
            None => {
            }
        }
        match &(left_role) {
            Some(__option3) => {
                if right_role == None {
                return Ok(Resolution::new(*__option3, "DashEllipsisSurroundingScriptContext", "only-left-strong-script")?);
                }
            }
            None => {
            }
        }
        match &(right_role) {
            Some(__option4) => {
                if left_role == None {
                return Ok(Resolution::new(*__option4, "DashEllipsisSurroundingScriptContext", "only-right-strong-script")?);
                }
            }
            None => {
            }
        }
        let reason = if match &(left_role) { Some(__option7) => right_role != None, None => false } { "conflicting-surrounding-script".to_string() } else { "no-strong-script-context".to_string() };
        return Ok(self.paragraph_language_resolution((context).clone(), reason.as_str())?);
    }

    fn resolve_parenthetical_pairs(&self, text: &str, runs: &Vec<TextRange>, strong_script_context: StrongScriptContextIndex, context: FontRoleContext) -> Result<SortedMapTable<u32, Resolution>, TextRangeError> {
        let mut resolutions: SortedMapTableBuilder<u32, Resolution> = SortedTable::sorted_table_map_builder::<u32, Resolution>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut index = 0u32;
        while (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((runs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let first = (runs[usize::try_from(index).unwrap_or(0)]).clone();
            let second = (runs[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]).clone();
            if !ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_is_parenthetical_dash_pair(text, (first).clone(), (second).clone())? {
                index = u32::wrapping_add(index, 1);
                continue;
            }
            let left_role = strong_script_context.left_of(first.start);
            let right_role = strong_script_context.right_of(second.end);
            let resolution = self.parenthetical_pair_resolution(left_role, right_role, (context).clone())?;
            resolutions.put(&(first.start), &(resolution));
            resolutions.put(&(second.start), &(resolution));
            index = u32::wrapping_add(index, 2);
        }
        return Ok(resolutions.clone().build());
    }

    fn parenthetical_pair_resolution(&self, left_role: Option<FontRole>, right_role: Option<FontRole>, context: FontRoleContext) -> Result<Resolution, TextRangeError> {
        match &(left_role) {
            Some(__option8) => {
                if right_role == Some(*__option8) {
                return Ok(Resolution::new(*__option8, "ParentheticalDashPairContext", "matching-outer-script")?);
                }
            }
            None => {
            }
        }
        match &(left_role) {
            Some(__option9) => {
                if right_role == None {
                return Ok(Resolution::new(*__option9, "ParentheticalDashPairContext", "only-left-outer-script")?);
                }
            }
            None => {
            }
        }
        match &(right_role) {
            Some(__option10) => {
                if left_role == None {
                return Ok(Resolution::new(*__option10, "ParentheticalDashPairContext", "only-right-outer-script")?);
                }
            }
            None => {
            }
        }
        let reason = if match &(left_role) { Some(__option13) => right_role != None, None => false } { "parenthetical-pair-conflicting-outer-script".to_string() } else { "parenthetical-pair-no-outer-context".to_string() };
        return Ok(self.paragraph_language_resolution((context).clone(), reason.as_str())?);
    }

    fn paragraph_language_resolution(&self, context: FontRoleContext, reason: &str) -> Result<Resolution, TextRangeError> {
        let role = if UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context((context.locale).to_string().as_str()) { FontRole::CjkPunctuation } else { FontRole::LatinText };
        return Ok(Resolution::new(role, "ParagraphLanguageDashEllipsisContext", format!("{}{}{}",
            reason,
            "; paragraph-language=",
            (context.locale).to_string()
        ).as_str())?);
    }

    pub(crate) fn contextual_dash_ellipsis_role_resolver_is_contextual_dash_or_ellipsis(code_point: u32) -> bool {
        return code_point == 8212 || code_point == 8230;
    }

    pub(crate) fn contextual_dash_ellipsis_role_resolver_is_parenthetical_dash_pair(text: &str, first: TextRange, second: TextRange) -> Result<bool, TextRangeError> {
        if !ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_is_pure_dash_run(text, (first).clone()) || !ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_is_pure_dash_run(text, (second).clone()) {
            return Ok(false);
        }
        if u32::wrapping_sub(first.end, first.start) != u32::wrapping_sub(second.end, second.start) {
            return Ok(false);
        }
        let mut index = first.end;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((second.start).to_ne_bytes())) {
            let cp = ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_code_point_at_compat(text, index);
            if cp != 32 && !UnicodeWordCharacter::unicode_word_character_contains(cp)? {
                return Ok(false);
            }
            index = u32::wrapping_add(index, ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_char_count(cp));
        }
        return Ok(true);
    }

    pub(crate) fn contextual_dash_ellipsis_role_resolver_is_pure_dash_run(text: &str, range: TextRange) -> bool {
        let mut index = range.start;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((range.end).to_ne_bytes())) {
            if !(u_string::unit_at(&text, index).as_ref().map_or(false, |v| v == &(8212))) {
                return false;
            }
            index = u32::wrapping_add(index, 1);
        }
        return true;
    }

    pub fn contextual_dash_ellipsis_role_resolver_code_point_at_compat(text: &str, index: u32) -> u32 {
    let __units5 = u_string::units(&text);
    let __count5 = u_string::unit_count(&text);
        let high = u_string::unit_at_from(&__units5, index).unwrap_or(0);
        if i32::from_ne_bytes((high).to_ne_bytes()) < (55296) || (i32::from_ne_bytes((high).to_ne_bytes())) > (56319) || (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) >= i32::from_ne_bytes((__count5).to_ne_bytes()) {
            return high;
        }
        let low = u_string::unit_at_from(&__units5, u32::wrapping_add(index, 1)).unwrap_or(0);
        if i32::from_ne_bytes((low).to_ne_bytes()) < (56320) || (i32::from_ne_bytes((low).to_ne_bytes())) > (57343) {
            return high;
        }
        return u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(high, 55296)) << (10)), u32::wrapping_sub(low, 56320));
    }

    pub fn contextual_dash_ellipsis_role_resolver_code_point_length_at(text: &str, index: u32) -> u32 {
        return ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_char_count(ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_code_point_at_compat(text, index));
    }

    pub fn contextual_dash_ellipsis_role_resolver_char_count(code_point: u32) -> u32 {
        return if i32::from_ne_bytes((code_point).to_ne_bytes()) > (65535) { 2 } else { 1 };
    }
}

#[derive(Clone)]
pub struct ContextualDashEllipsisAwareFontRoleClassifier {
    pub(crate) delegate: Box<dyn FontRoleClassifier>,
    pub(crate) role_by_index: SortedMapTable<u32, FontRole>,
}

impl ContextualDashEllipsisAwareFontRoleClassifier {
    pub fn new(delegate: Box<dyn FontRoleClassifier>, decisions: Vec<DashEllipsisRoleDecision>) -> Result<Self, TextRangeError> {
        let mut builder: SortedMapTableBuilder<u32, FontRole> = SortedTable::sorted_table_map_builder::<u32, FontRole>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()), i32::from_ne_bytes((*b).to_ne_bytes()))));
        let mut di = 0u32;
        while (i32::from_ne_bytes((di).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let d = (decisions[usize::try_from(di).unwrap_or(0)]).clone();
            let mut index = (d.range).clone().start;
            while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes(((d.range).clone().end).to_ne_bytes())) {
                builder.put(&(index), &(d.role));
                index = u32::wrapping_add(index, 1);
            }
            di = u32::wrapping_add(di, 1);
        }
        Ok(Self {
            delegate,
            role_by_index: builder.clone().build(),
        })
    }

    pub fn classify(&self, text: &str, range: TextRange, context: Option<FontRoleContext>) -> FontRole {
        let role = (self.role_by_index).clone().get(&(range.start));
        return match &(role) { None => self.delegate.classify(text, (range).clone(), (context).clone()), Some(__option14) => *__option14 };
    }
}

impl FontRoleClassifier for ContextualDashEllipsisAwareFontRoleClassifier {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.ContextualDashEllipsisRoleResolver.ContextualDashEllipsisAwareFontRoleClassifier"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontRoleClassifier> {
        Box::new(self.clone())
    }

    fn classify(&self, text: &str, range: TextRange, context: Option<FontRoleContext>) -> FontRole {
        let role = (self.role_by_index).clone().get(&(range.start));
        return match &(role) { None => self.delegate.classify(text, (range).clone(), (context).clone()), Some(__option15) => *__option15 };
    }
}

#[derive(Clone, Copy)]
pub struct ContextualDashEllipsisRoles;

impl ContextualDashEllipsisRoles {
    pub fn contextual_dash_ellipsis_roles_with_contextual_dash_ellipsis_roles(base: Box<dyn FontRoleClassifier>, text: &str, context: Option<FontRoleContext>) -> Result<Box<dyn FontRoleClassifier>, TextRangeError> {
        let decisions = ContextualDashEllipsisRoleResolver::new()?.resolve(text, (context).clone())?;
        return Ok(if u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { base.clone() } else { Box::new(ContextualDashEllipsisAwareFontRoleClassifier::new(base.clone(), decisions.to_vec())?) });
    }

    pub fn contextual_dash_ellipsis_roles_to_role_override_infos(decisions: &Vec<DashEllipsisRoleDecision>, text: &str, base_classifier: Box<dyn FontRoleClassifier>, context: FontRoleContext) -> Result<Vec<RoleOverrideInfo>, TextRangeError> {
        let mut result: Vec<RoleOverrideInfo> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let d = (decisions[usize::try_from(index).unwrap_or(0)]).clone();
            let first = TextRange::new((d.range).clone().start, u32::wrapping_add((d.range).clone().start, 1))?;
            result.push(RoleOverrideInfo::new((d.range).clone(), u_string::substring(&text, i32::from_ne_bytes(((d.range).clone().start).to_ne_bytes()), i32::from_ne_bytes(((d.range).clone().end).to_ne_bytes())).as_str(), base_classifier.classify(text, (first).clone(),
Some((context).clone())).name().to_string().as_str(), d.role.name().to_string().as_str(), (d.source).to_string().as_str(), (d.reason).to_string().as_str()));
            index = u32::wrapping_add(index, 1);
        }
        return Ok(result);
    }
}

#[derive(Clone, PartialEq)]
pub struct Resolution {
    pub role: FontRole,
    pub source: String,
    pub reason: String,
}

impl Resolution {
    pub fn new(role: FontRole, source: &str, reason: &str) -> Result<Self, TextRangeError> {
        Ok(Self {
            role,
            source: source.to_string(),
            reason: reason.to_string(),
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct StrongScriptContextIndex {
    pub(crate) left_role_before_boundary: Vec<Option<FontRole>>,
    pub(crate) right_role_from_boundary: Vec<Option<FontRole>>,
}

impl StrongScriptContextIndex {
    pub fn new(text: &str) -> Result<Self, TextRangeError> {
    let mut left_role_before_boundary = vec![];
    let mut right_role_from_boundary = vec![];
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) <= i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes()) {
            { while left_role_before_boundary.len() <= usize::try_from(i).unwrap_or(0) { left_role_before_boundary.push(None); } left_role_before_boundary[usize::try_from(i).unwrap_or(0)] = None; };
            { while right_role_from_boundary.len() <= usize::try_from(i).unwrap_or(0) { right_role_from_boundary.push(None); } right_role_from_boundary[usize::try_from(i).unwrap_or(0)] = None; };
            i = u32::wrapping_add(i, 1);
        }
        let mut current_role: Option<FontRole> = None;
        let mut scalar_start = 0u32;
        while (i32::from_ne_bytes((scalar_start).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(text))).to_ne_bytes())) {
            let cp = ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_code_point_at_compat(text, scalar_start);
            let scalar_end = u32::wrapping_add(scalar_start, ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_char_count(cp));
            current_role = StrongScriptContextIndex::strong_script_context_index_next_strong_script_role(cp, current_role)?;
            let mut boundary = u32::wrapping_add(scalar_start, 1);
            while (i32::from_ne_bytes((boundary).to_ne_bytes())) <= i32::from_ne_bytes((scalar_end).to_ne_bytes()) {
                { while left_role_before_boundary.len() <= usize::try_from(boundary).unwrap_or(0) { left_role_before_boundary.push(None); } left_role_before_boundary[usize::try_from(boundary).unwrap_or(0)] = current_role; };
                boundary = u32::wrapping_add(boundary, 1);
            }
            scalar_start = scalar_end;
        }
        current_role = None;
        let mut scalar_end = u_string::unit_count(&(text));
        while (i32::from_ne_bytes((scalar_end).to_ne_bytes())) > (0) {
            scalar_start = StrongScriptContextIndex::strong_script_context_index_scalar_start_before(text, scalar_end);
            let cp = ContextualDashEllipsisRoleResolver::contextual_dash_ellipsis_role_resolver_code_point_at_compat(text, scalar_start);
            current_role = StrongScriptContextIndex::strong_script_context_index_next_strong_script_role(cp, current_role)?;
            let mut boundary = scalar_start;
            while (i32::from_ne_bytes((boundary).to_ne_bytes())) < (i32::from_ne_bytes((scalar_end).to_ne_bytes())) {
                { while right_role_from_boundary.len() <= usize::try_from(boundary).unwrap_or(0) { right_role_from_boundary.push(None); } right_role_from_boundary[usize::try_from(boundary).unwrap_or(0)] = current_role; };
                boundary = u32::wrapping_add(boundary, 1);
            }
            scalar_end = scalar_start;
        }
        Ok(Self {
            left_role_before_boundary: left_role_before_boundary,
            right_role_from_boundary: right_role_from_boundary,
        })
    }

    pub fn left_of(&self, boundary: u32) -> Option<FontRole> {
        return self.left_role_before_boundary[usize::try_from(boundary).unwrap_or(0)];
    }

    pub fn right_of(&self, boundary: u32) -> Option<FontRole> {
        return self.right_role_from_boundary[usize::try_from(boundary).unwrap_or(0)];
    }

    pub(crate) fn strong_script_context_index_next_strong_script_role(code_point: u32, current_role: Option<FontRole>) -> Result<Option<FontRole>, TextRangeError> {
        if LineBreakFns::line_break_fns_is_mandatory_break_code_point(code_point) {
            return Ok(None);
        }
        let evidence = UnicodeScriptEvidenceClassifier::unicode_script_evidence_classifier_classify(code_point)?;
        return Ok(match evidence {
            UnicodeScriptEvidence::Neutral => current_role,
            UnicodeScriptEvidence::EastAsian => Some(FontRole::CjkPunctuation),
            UnicodeScriptEvidence::Other => Some(FontRole::LatinText),
        });
    }

    pub(crate) fn strong_script_context_index_scalar_start_before(text: &str, end_exclusive: u32) -> u32 {
    let __units6 = u_string::units(&text);
    let __count6 = u_string::unit_count(&text);
        let last_index = u32::wrapping_sub(end_exclusive, 1);
        let last = u_string::unit_at_from(&__units6, last_index).unwrap_or(0);
        if i32::from_ne_bytes((last).to_ne_bytes()) >= 56320 && (i32::from_ne_bytes((last).to_ne_bytes())) <= 57343 && (i32::from_ne_bytes((last_index).to_ne_bytes())) > (0) {
            let before = u_string::unit_at_from(&__units6, u32::wrapping_sub(last_index, 1)).unwrap_or(0);
            if i32::from_ne_bytes((before).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((before).to_ne_bytes())) <= 56319 {
                return u32::wrapping_sub(last_index, 1);
            }
        }
        return last_index;
    }
}
