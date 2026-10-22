use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::inline_object_span::InlineObjectSpan;
use crate::org::tiqian::core::role_override_info::RoleOverrideInfo;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_combining_mark_data::UnicodeCombiningMarkData;
use crate::org::tiqian::core::unicode_emoji_modifier_base_data::UnicodeEmojiModifierBaseData;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::font::font_role_context::FontRoleClassifier;
use crate::org::tiqian::font::font_role_context::FontRoleContext;
use crate::org::tiqian::font::unicode_emoji_data::UnicodeEmojiData;
use crate::org::tiqian::font::unicode_emoji_style_variation_data::UnicodeEmojiStyleVariationData;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedClusterRange {
    pub range: TextRange,
    pub role: FontRole,
    pub mandatory_break: bool,
    pub zero_width_soft_break: bool,
    pub role_override: Option<RoleOverrideInfo>,
}

impl ResolvedClusterRange {
    pub fn new(range: TextRange, role: FontRole, mandatory_break: Option<bool>, zero_width_soft_break: Option<bool>, role_override: Option<RoleOverrideInfo>) -> Self {
        let mandatory_break = mandatory_break.unwrap_or_else(|| false);
        let zero_width_soft_break = zero_width_soft_break.unwrap_or_else(|| false);
        Self {
            range,
            role,
            mandatory_break: mandatory_break,
            zero_width_soft_break: zero_width_soft_break,
            role_override,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ResolvedClusterRange(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += UString::from(self.role.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("mandatoryBreak=")); __s += UString::from(format!("{}", (self.mandatory_break).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("zeroWidthSoftBreak=")); __s += UString::from(format!("{}", (self.zero_width_soft_break).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("roleOverride=")); __s += (match &(self.role_override) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, Copy)]
pub struct ClusterRoleResolution;

impl ClusterRoleResolution {
    pub fn cluster_role_resolution_cluster_role_ranges(text: &UStr, classifier: Box<dyn FontRoleClassifier>, context: FontRoleContext, profile: ClreqProfile, span_boundaries: SortedSetTable<u32>, emoji_shaping_boundaries: SortedSetTable<u32>, inline_objects_by_start: Option<SortedMapTable<u32, InlineObjectSpan>>) -> Result<Vec<ResolvedClusterRange>, TextRangeError> {
    let __units = text.as_slice();
    let __count = u_string::unit_count(&text);
        let graphemes = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text, TextRange::new(0u32, __count)?);
        let mut out: Vec<ResolvedClusterRange> = vec![];
        let mut index = 0u32;
        let mut gi = if __count == 0 { 0 } else { 1 };
        let mut gs = graphemes[0usize];
        let mut ge = if gi < ((u32::try_from(((graphemes).len()) & 4294967295).unwrap_or(0)) as i32 as i32) { graphemes[usize::try_from(gi).unwrap_or(0)] } else { __count };
        while ((index) as i32) < ((__count) as i32) {
            while ((index) as i32) >= { let v: u32 = ge; v as i32 } && (gi) < (i32::wrapping_sub((u32::try_from(((graphemes).len()) & 4294967295).unwrap_or(0)) as i32 as i32, 1)) {
                gs = ge;
                gi = i32::wrapping_add(gi, 1);
                ge = graphemes[usize::try_from(gi).unwrap_or(0)];
            }
            let io = match &(inline_objects_by_start) { None => None, Some(__option1) => __option1.get(&(index)) };
            match &(io) {
                Some(__option2) => {
                    out.push(ResolvedClusterRange::new((__option2.range).clone(), FontRole::Unknown, Some(false), Some(false), None));
                    index = __option2.range.end;
                    continue;
                }
                None => {
                }
            }
            let cp = ClusterRoleResolution::cluster_role_resolution_code_point(text, index);
            let count = if cp as i32 > (65535) { 2 } else { 1 };
            let start = index;
            if ClusterRoleResolution::cluster_role_resolution_is_mandatory(text, index, cp) {
                let end = if cp == 13 && ((u32::wrapping_add(index, 1)) as i32) < ((__count) as i32) && matches!(u_string::unit_at_from(__units, u32::wrapping_add(index, 1)).as_ref(), Some(__v) if __v == &(10)) { u32::wrapping_add(index, 2) } else { u32::wrapping_add(index, count) };
                out.push(ResolvedClusterRange::new(TextRange::new(start, end)?, FontRole::Unknown, Some(true), Some(false), None));
                index = end;
                continue;
            }
            if LineBreakFns::line_break_fns_is_zero_width_space_code_point(cp) {
                out.push(ResolvedClusterRange::new(TextRange::new(start, u32::wrapping_add(index, count))?, FontRole::Unknown, Some(false), Some(true), None));
                index = u32::wrapping_add(index, count);
                continue;
            }
            let classified = classifier.classify(text, TextRange::new(start, u32::wrapping_add(start, count))?, Some((context).clone()));
            let promotion = ClusterRoleResolution::cluster_role_resolution_emoji_promotion(text, start, ge);
            let role = if classified == FontRole::Emoji || promotion.is_some() { FontRole::Emoji } else { classified };
            let prev = if u32::try_from((out.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { None } else { Some((out[usize::try_from(u32::wrapping_sub(u32::try_from((out.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone()) };
            let attached = role == FontRole::LatinText && ClusterRoleResolution::cluster_role_resolution_is_ascii_point(cp) && prev.is_some() && (prev).as_ref().unwrap().role != FontRole::Unknown && !ClusterRoleResolution::cluster_role_resolution_is_ws(*(u_string::unit_at_from(__units, u32::wrapping_sub(((prev).as_ref().unwrap().range).clone().end, 1))).as_ref().unwrap()) && ((prev).as_ref().unwrap().range).clone().end == start;
            index = u32::wrapping_add(index, count);
            if role == FontRole::Emoji {
                let mut b = ge;
                let mut bi = 0u32;
                while ((bi) as i32) < ((emoji_shaping_boundaries.size()) as u32 as i32) {
                    let k = emoji_shaping_boundaries.at((bi) as i32);
                    if k as i32 > ((start) as i32) && ((k) as i32) < ({ let v: u32 = ge; v as i32 }) && (b == ge || ((k) as i32) < ({ let v: u32 = b; v as i32 })) {
                        b = k;
                    }
                    bi = u32::wrapping_add(bi, 1);
                }
                index = b;
            } else {
                if role == FontRole::LatinText {
                    if cp == 8212 || cp == 8230 {
                        if ClusterRoleResolution::cluster_role_resolution_contains(&profile.coalesce_repeatable_punctuation, cp) {
                            while ((index) as i32) < ((__count) as i32) && !span_boundaries.has(&(index)) && ClusterRoleResolution::cluster_role_resolution_code_point(text, index) == cp {
                                index = u32::wrapping_add(index, if cp as i32 > (65535) { 2 } else { 1 });
                            }
                        }
                    } else {
                        if attached {
                            while ((index) as i32) < ((__count) as i32) && !span_boundaries.has(&(index)) && ClusterRoleResolution::cluster_role_resolution_is_ascii_point(ClusterRoleResolution::cluster_role_resolution_code_point(text, index)) {
                                index = u32::wrapping_add(index, 1);
                            }
                        } else {
                            while ((index) as i32) < ((__count) as i32) && !span_boundaries.has(&(index)) {
                                let n = ClusterRoleResolution::cluster_role_resolution_code_point(text, index);
                                if n == 8212 || n == 8230 || classifier.classify(text, TextRange::new(index, u32::wrapping_add(index, if n as i32 > (65535) { 2 } else { 1 }))?, Some((context).clone())) != FontRole::LatinText || ClusterRoleResolution::cluster_role_resolution_emoji_promotion(text, index, __count).is_some() {
                                    break;
                                }
                                index = u32::wrapping_add(index, if n as i32 > (65535) { 2 } else { 1 });
                            }
                        }
                    }
                } else {
                    if role == FontRole::CjkPunctuation && ClusterRoleResolution::cluster_role_resolution_contains(&profile.coalesce_repeatable_punctuation, cp) {
                        while ((index) as i32) < ((__count) as i32) && !span_boundaries.has(&(index)) && ClusterRoleResolution::cluster_role_resolution_code_point(text, index) == cp {
                            index = u32::wrapping_add(index, if cp as i32 > (65535) { 2 } else { 1 });
                        }
                    }
                }
            }
            while ((index) as i32) < ((__count) as i32) && !span_boundaries.has(&(index)) {
                let e = ClusterRoleResolution::cluster_role_resolution_code_point(text, index);
                if !ClusterRoleResolution::cluster_role_resolution_is_combining(e) && !ClusterRoleResolution::cluster_role_resolution_is_variation(e) {
                    break;
                }
                index = u32::wrapping_add(index, if e as i32 > (65535) { 2 } else { 1 });
            }
            let r = TextRange::new(start, index)?;
            let mut role_info: Option<RoleOverrideInfo> = None;
            if role == FontRole::Emoji && classified != FontRole::Emoji {
                role_info = Some(RoleOverrideInfo::new((r).clone(), u_string::substring(&text, (r.start) as i32, (r.end) as i32).as_ustr(), UString::from(classified.name()).as_ustr(), UString::from(role.name()).as_ustr(), &(UStr::new(&[85,110,105,99,111,100,101,69,109,111,106,105,83,101,113,117,101,110,99,101,82,111,108,101,80,114,111,109,111,116,105,111,110])), match &(promotion) { None => UString::from("EmojiPresentationCodePoint"), Some(__option3) => __option3.to_ustring() }.as_ustr()).clone());
            }
            out.push(ResolvedClusterRange::new((r).clone(), role, Some(false), Some(false), (role_info).clone()));
        }
        return Ok(out);
    }

    pub fn cluster_role_resolution_require_covered_by(clusters: &Vec<Cluster>, decisions: &Vec<FontDecision>) -> Result<(), TextRangeError> {
        let mut ci = 0u32;
        let mut di = 0u32;
        while ((di) as i32) < ((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32) {
            let d = (decisions[usize::try_from({ let t = di; di = u32::wrapping_add(di, 1); t }).unwrap_or(0)]).clone();
            while ((ci) as i32) < ((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32) && ((((clusters[usize::try_from(ci).unwrap_or(0)]).clone().range).clone().end) as i32) <= (d.range).clone().start as i32 {
                ci = u32::wrapping_add(ci, 1);
            }
            let mut cursor = (d.range).clone().start;
            while ((ci) as i32) < ((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32) && ((((clusters[usize::try_from(ci).unwrap_or(0)]).clone().range).clone().start) as i32) < (((d.range).clone().end) as i32) {
                let c = (clusters[usize::try_from(ci).unwrap_or(0)]).clone();
                if !(((c.range).clone().start) as i32 >= (d.range).clone().start as i32 && (((c.range).clone().end) as i32) <= (d.range).clone().end as i32) {
                    return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextShaper returned cluster ")); __s += UString::from(format!("{}", (c.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(" crossing ")); __s += UString::from(format!("{}", (d.range).clone().to_string()).as_str()).as_ustr(); __s }).as_str()) });
                }
                if c.range.clone().start != cursor {
                    return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextShaper returned non-contiguous clusters for ")); __s += UString::from(format!("{}", (d.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from("; expected start=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cursor)).as_str())); __s += &(UString::from(", actual=")); __s += UString::from(format!("{}", (c.range).clone().to_string()).as_str()).as_ustr(); __s }).as_str()) });
                }
                cursor = (c.range).clone().end;
                ci = u32::wrapping_add(ci, 1);
            }
            if cursor != (d.range).clone().end {
                return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextShaper must return clusters covering ")); __s += UString::from(format!("{}", (d.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from("; coveredUntil=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cursor)).as_str())); __s }).as_str()) });
            }
        }
        Ok(())
    }

    pub(crate) fn cluster_role_resolution_code_point(t: &UStr, i: u32) -> u32 {
    let __units1 = t.as_slice();
    let __count1 = u_string::unit_count(&t);
        let h = u_string::unit_at_from(__units1, i).unwrap_or(0);
        if h as i32 < (55296) || ((h) as i32) > (56319) || ((u32::wrapping_add(i, 1)) as i32) >= __count1 as i32 {
            return h;
        }
        let l = u_string::unit_at_from(__units1, u32::wrapping_add(i, 1)).unwrap_or(0);
        return if l as i32 < (56320) || ((l) as i32) > (57343) { h } else { u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(h, 55296)) << (10)), u32::wrapping_sub(l, 56320)) };
    }

    pub(crate) fn cluster_role_resolution_is_mandatory(t: &UStr, i: u32, c: u32) -> bool {
        return LineBreakFns::line_break_fns_is_mandatory_break_code_point(c) && !(c == 10 && ((i) as i32) > (0) && matches!(u_string::unit_at(&t, u32::wrapping_sub(i, 1)).as_ref(), Some(__v) if __v == &(13)));
    }

    pub(crate) fn cluster_role_resolution_contains(a: &[u32], v: u32) -> bool {
        let mut i = 0u32;
        while ((i) as i32) < ((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32) {
            if a[usize::try_from(i).unwrap_or(0)] == v {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub(crate) fn cluster_role_resolution_is_variation(c: u32) -> bool {
        return ((c) as i32) >= 65024 && ((c) as i32) <= 65039 || ((c) as i32) >= 917760 && ((c) as i32) <= 917999;
    }

    pub(crate) fn cluster_role_resolution_is_combining(c: u32) -> bool {
        return ((c) as i32) <= 65535 && UnicodeCombiningMarkData::unicode_combining_mark_data_contains(c);
    }

    pub(crate) fn cluster_role_resolution_is_ws(cp: u32) -> bool {
        return ((cp) as i32) <= 65535 && ((cp) as i32 >= 9 && ((cp) as i32) <= 13 || ((cp) as i32) >= 28 && ((cp) as i32) <= 32 || cp == 160 || cp == 5760 || ((cp) as i32) >= 8192 && ((cp) as i32) <= 8202 || cp == 8232 || cp == 8233 || cp == 8239 || cp == 8287 || cp == 12288);
    }

    pub(crate) fn cluster_role_resolution_is_ascii_point(c: u32) -> bool {
        return c == 44 || c == 46 || c == 59 || c == 58 || c == 33 || c == 63;
    }

    pub(crate) fn cluster_role_resolution_emoji_promotion(t: &UStr, s: u32, e: u32) -> Option<UString> {
        let b = ClusterRoleResolution::cluster_role_resolution_code_point(t, s);
        let mut n = u32::wrapping_add(s, if b as i32 > (65535) { 2 } else { 1 });
        if b == 35 || b == 42 || ((b) as i32) >= 48 && ((b) as i32) <= 57 {
            if n as i32 < ((e) as i32) && ClusterRoleResolution::cluster_role_resolution_code_point(t, n) == 65039 {
                n = u32::wrapping_add(n, 1);
            }
            if n as i32 < ((e) as i32) && ClusterRoleResolution::cluster_role_resolution_code_point(t, n) == 8419 {
                return Some(UString::from("KeycapSequence").to_ustring());
            }
        }
        if UnicodeEmojiData::unicode_emoji_data_contains(b) && UnicodeEmojiStyleVariationData::unicode_emoji_style_variation_data_contains(b) && ((n) as i32) < ((e) as i32) && ClusterRoleResolution::cluster_role_resolution_code_point(t, n) == 65039 {
            return Some(UString::from("EmojiStyleVariationSequence").to_ustring());
        }
        if UnicodeEmojiModifierBaseData::unicode_emoji_modifier_base_data_contains(b) {
            while ((n) as i32) < ((e) as i32) && (ClusterRoleResolution::cluster_role_resolution_is_combining(ClusterRoleResolution::cluster_role_resolution_code_point(t, n)) || ClusterRoleResolution::cluster_role_resolution_is_variation(ClusterRoleResolution::cluster_role_resolution_code_point(t, n))) {
                n = u32::wrapping_add(n, if ClusterRoleResolution::cluster_role_resolution_code_point(t, n) as i32 > (65535) { 2 } else { 1 });
            }
            let m = if n as i32 < ((e) as i32) { ClusterRoleResolution::cluster_role_resolution_code_point(t, n) } else { 4294967295u32 };
            if ({ let v: u32 = m; v as i32 }) >= 127995 && ({ let v: u32 = m; v as i32 }) <= 127999 {
                return Some(UString::from("EmojiModifierSequence").to_ustring());
            }
        }
        return None;
    }
}
