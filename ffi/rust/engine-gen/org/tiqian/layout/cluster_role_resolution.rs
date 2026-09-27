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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ResolvedClusterRange(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "role=",
            self.role.name(),
            ", ",
            "mandatoryBreak=",
            self.mandatory_break,
            ", ",
            "zeroWidthSoftBreak=",
            self.zero_width_soft_break,
            ", ",
            "roleOverride=",
            (match &(self.role_override) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ")"
        );
    }
}

#[derive(Clone, Copy)]
pub struct ClusterRoleResolution;

impl ClusterRoleResolution {
    pub fn cluster_role_resolution_cluster_role_ranges(text: &str, classifier: Box<dyn FontRoleClassifier>, context: FontRoleContext, profile: ClreqProfile, span_boundaries: SortedSetTable<u32>, emoji_shaping_boundaries: SortedSetTable<u32>, inline_objects_by_start:
Option<SortedMapTable<u32, InlineObjectSpan>>) -> Result<Vec<ResolvedClusterRange>, TextRangeError> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let graphemes = SourceInteractionBoundaries::source_interaction_boundaries_source_grapheme_boundaries(text, TextRange::new(0u32, __count)?);
        let mut out: Vec<ResolvedClusterRange> = vec![];
        let mut index = 0u32;
        let mut gi = if __count == 0 { 0 } else { 1 };
        let mut gs = graphemes[0usize];
        let mut ge = if gi < (i32::from_ne_bytes(u32::try_from(((graphemes).len()) & 4294967295).unwrap_or(0).to_ne_bytes())) { graphemes[usize::try_from(gi).unwrap_or(0)] } else { __count };
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            while (i32::from_ne_bytes((index).to_ne_bytes())) >= { let v: u32 = ge; i32::from_ne_bytes(v.to_ne_bytes()) } && (gi) < (i32::wrapping_sub(i32::from_ne_bytes((i32::from_ne_bytes(u32::try_from(((graphemes).len()) &
4294967295).unwrap_or(0).to_ne_bytes())).to_ne_bytes()), 1)) {
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
            let count = if i32::from_ne_bytes((cp).to_ne_bytes()) > (65535) { 2 } else { 1 };
            let start = index;
            if ClusterRoleResolution::cluster_role_resolution_is_mandatory(text, index, cp) {
                let end = if cp == 13 && (i32::from_ne_bytes((u32::wrapping_add(index, 1)).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && u_string::unit_at_from(&__units, u32::wrapping_add(index, 1)).as_ref().map_or(false, |v| v == &(10)) {
u32::wrapping_add(index, 2) } else { u32::wrapping_add(index, count) };
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
            let attached = role == FontRole::LatinText && ClusterRoleResolution::cluster_role_resolution_is_ascii_point(cp) && prev.is_some() && (prev).as_ref().unwrap().role != FontRole::Unknown &&
!ClusterRoleResolution::cluster_role_resolution_is_ws(*(u_string::unit_at_from(&__units, u32::wrapping_sub(((prev).as_ref().unwrap().range).clone().end, 1))).as_ref().unwrap()) && ((prev).as_ref().unwrap().range).clone().end == start;
            index = u32::wrapping_add(index, count);
            if role == FontRole::Emoji {
                let mut b = ge;
                let mut bi = 0u32;
                while (i32::from_ne_bytes((bi).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes((emoji_shaping_boundaries.size()).to_ne_bytes())).to_ne_bytes())) {
                    let k = emoji_shaping_boundaries.at(i32::from_ne_bytes((bi).to_ne_bytes()));
                    if i32::from_ne_bytes((k).to_ne_bytes()) > (i32::from_ne_bytes((start).to_ne_bytes())) && (i32::from_ne_bytes((k).to_ne_bytes())) < ({ let v: u32 = ge; i32::from_ne_bytes(v.to_ne_bytes()) }) && (b == ge || (i32::from_ne_bytes((k).to_ne_bytes())) < ({ let v:
u32 = b; i32::from_ne_bytes(v.to_ne_bytes()) })) {
                        b = k;
                    }
                    bi = u32::wrapping_add(bi, 1);
                }
                index = b;
            } else {
                if role == FontRole::LatinText {
                    if cp == 8212 || cp == 8230 {
                        if ClusterRoleResolution::cluster_role_resolution_contains(&profile.coalesce_repeatable_punctuation, cp) {
                            while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && !span_boundaries.has(&(index)) && ClusterRoleResolution::cluster_role_resolution_code_point(text, index) == cp {
                                index = u32::wrapping_add(index, if i32::from_ne_bytes((cp).to_ne_bytes()) > (65535) { 2 } else { 1 });
                            }
                        }
                    } else {
                        if attached {
                            while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && !span_boundaries.has(&(index)) &&
ClusterRoleResolution::cluster_role_resolution_is_ascii_point(ClusterRoleResolution::cluster_role_resolution_code_point(text, index)) {
                                index = u32::wrapping_add(index, 1);
                            }
                        } else {
                            while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && !span_boundaries.has(&(index)) {
                                let n = ClusterRoleResolution::cluster_role_resolution_code_point(text, index);
                                if n == 8212 || n == 8230 || classifier.classify(text, TextRange::new(index, u32::wrapping_add(index, if i32::from_ne_bytes((n).to_ne_bytes()) > (65535) { 2 } else { 1 }))?, Some((context).clone())) != FontRole::LatinText ||
ClusterRoleResolution::cluster_role_resolution_emoji_promotion(text, index, __count).is_some() {
                                    break;
                                }
                                index = u32::wrapping_add(index, if i32::from_ne_bytes((n).to_ne_bytes()) > (65535) { 2 } else { 1 });
                            }
                        }
                    }
                } else {
                    if role == FontRole::CjkPunctuation && ClusterRoleResolution::cluster_role_resolution_contains(&profile.coalesce_repeatable_punctuation, cp) {
                        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && !span_boundaries.has(&(index)) && ClusterRoleResolution::cluster_role_resolution_code_point(text, index) == cp {
                            index = u32::wrapping_add(index, if i32::from_ne_bytes((cp).to_ne_bytes()) > (65535) { 2 } else { 1 });
                        }
                    }
                }
            }
            while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) && !span_boundaries.has(&(index)) {
                let e = ClusterRoleResolution::cluster_role_resolution_code_point(text, index);
                if !ClusterRoleResolution::cluster_role_resolution_is_combining(e) && !ClusterRoleResolution::cluster_role_resolution_is_variation(e) {
                    break;
                }
                index = u32::wrapping_add(index, if i32::from_ne_bytes((e).to_ne_bytes()) > (65535) { 2 } else { 1 });
            }
            let r = TextRange::new(start, index)?;
            let mut role_info: Option<RoleOverrideInfo> = None;
            if role == FontRole::Emoji && classified != FontRole::Emoji {
                role_info = Some(RoleOverrideInfo::new((r).clone(), u_string::substring(&text, i32::from_ne_bytes((r.start).to_ne_bytes()), i32::from_ne_bytes((r.end).to_ne_bytes())).as_str(), classified.name().to_string().as_str(), role.name().to_string().as_str(),
"UnicodeEmojiSequenceRolePromotion", match &(promotion) { None => "EmojiPresentationCodePoint".to_string(), Some(__option3) => __option3.to_string() }.as_str()).clone());
            }
            out.push(ResolvedClusterRange::new((r).clone(), role, Some(false), Some(false), (role_info).clone()));
        }
        return Ok(out);
    }

    pub fn cluster_role_resolution_require_covered_by(clusters: &Vec<Cluster>, decisions: &Vec<FontDecision>) -> Result<(), TextRangeError> {
        let mut ci = 0u32;
        let mut di = 0u32;
        while (i32::from_ne_bytes((di).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let d = (decisions[usize::try_from({ let t = di; di = u32::wrapping_add(di, 1); t }).unwrap_or(0)]).clone();
            while (i32::from_ne_bytes((ci).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && (i32::from_ne_bytes((((clusters[usize::try_from(ci).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <=
i32::from_ne_bytes(((d.range).clone().start).to_ne_bytes()) {
                ci = u32::wrapping_add(ci, 1);
            }
            let mut cursor = (d.range).clone().start;
            while (i32::from_ne_bytes((ci).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) && (i32::from_ne_bytes((((clusters[usize::try_from(ci).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes())) <
(i32::from_ne_bytes(((d.range).clone().end).to_ne_bytes())) {
                let c = (clusters[usize::try_from(ci).unwrap_or(0)]).clone();
                if !((i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes())) >= i32::from_ne_bytes(((d.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes(((c.range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((d.range).clone().end).to_ne_bytes())) {
                    return Err(TextRangeError::Message { text: format!("{}{}{}{}",
            "TextShaper returned cluster ",
            (c.range).clone().to_string(),
            " crossing ",
            (d.range).clone().to_string()
        ).to_string() });
                }
                if c.range.clone().start != cursor {
                    return Err(TextRangeError::Message { text: format!("{}{}{}{}{}{}",
            "TextShaper returned non-contiguous clusters for ",
            (d.range).clone().to_string(),
            "; expected start=",
            crate::runtime::int_text::IntText::int_text(cursor),
            ", actual=",
            (c.range).clone().to_string()
        ).to_string() });
                }
                cursor = (c.range).clone().end;
                ci = u32::wrapping_add(ci, 1);
            }
            if cursor != (d.range).clone().end {
                return Err(TextRangeError::Message { text: format!("{}{}{}{}",
            "TextShaper must return clusters covering ",
            (d.range).clone().to_string(),
            "; coveredUntil=",
            crate::runtime::int_text::IntText::int_text(cursor)
        ).to_string() });
            }
        }
        Ok(())
    }

    pub(crate) fn cluster_role_resolution_code_point(t: &str, i: u32) -> u32 {
    let __units1 = u_string::units(&t);
    let __count1 = u_string::unit_count(&t);
        let h = u_string::unit_at_from(&__units1, i).unwrap_or(0);
        if i32::from_ne_bytes((h).to_ne_bytes()) < (55296) || (i32::from_ne_bytes((h).to_ne_bytes())) > (56319) || (i32::from_ne_bytes((u32::wrapping_add(i, 1)).to_ne_bytes())) >= i32::from_ne_bytes((__count1).to_ne_bytes()) {
            return h;
        }
        let l = u_string::unit_at_from(&__units1, u32::wrapping_add(i, 1)).unwrap_or(0);
        return if i32::from_ne_bytes((l).to_ne_bytes()) < (56320) || (i32::from_ne_bytes((l).to_ne_bytes())) > (57343) { h } else { u32::wrapping_add(u32::wrapping_add(65536, (u32::wrapping_sub(h, 55296)) << (10)), u32::wrapping_sub(l, 56320)) };
    }

    pub(crate) fn cluster_role_resolution_is_mandatory(t: &str, i: u32, c: u32) -> bool {
        return LineBreakFns::line_break_fns_is_mandatory_break_code_point(c) && !(c == 10 && (i32::from_ne_bytes((i).to_ne_bytes())) > (0) && u_string::unit_at(&t, u32::wrapping_sub(i, 1)).as_ref().map_or(false, |v| v == &(13)));
    }

    pub(crate) fn cluster_role_resolution_contains(a: &[u32], v: u32) -> bool {
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if a[usize::try_from(i).unwrap_or(0)] == v {
                return true;
            }
            i = u32::wrapping_add(i, 1);
        }
        return false;
    }

    pub(crate) fn cluster_role_resolution_is_variation(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) >= 65024 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 65039 || (i32::from_ne_bytes((c).to_ne_bytes())) >= 917760 && (i32::from_ne_bytes((c).to_ne_bytes())) <= 917999;
    }

    pub(crate) fn cluster_role_resolution_is_combining(c: u32) -> bool {
        return (i32::from_ne_bytes((c).to_ne_bytes())) <= 65535 && UnicodeCombiningMarkData::unicode_combining_mark_data_contains(c);
    }

    pub(crate) fn cluster_role_resolution_is_ws(cp: u32) -> bool {
        return (i32::from_ne_bytes((cp).to_ne_bytes())) <= 65535 && ((i32::from_ne_bytes((cp).to_ne_bytes())) >= 9 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 13 || (i32::from_ne_bytes((cp).to_ne_bytes())) >= 28 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 32 || cp == 160 ||
cp == 5760 || (i32::from_ne_bytes((cp).to_ne_bytes())) >= 8192 && (i32::from_ne_bytes((cp).to_ne_bytes())) <= 8202 || cp == 8232 || cp == 8233 || cp == 8239 || cp == 8287 || cp == 12288);
    }

    pub(crate) fn cluster_role_resolution_is_ascii_point(c: u32) -> bool {
        return c == 44 || c == 46 || c == 59 || c == 58 || c == 33 || c == 63;
    }

    pub(crate) fn cluster_role_resolution_emoji_promotion(t: &str, s: u32, e: u32) -> Option<String> {
        let b = ClusterRoleResolution::cluster_role_resolution_code_point(t, s);
        let mut n = u32::wrapping_add(s, if i32::from_ne_bytes((b).to_ne_bytes()) > (65535) { 2 } else { 1 });
        if b == 35 || b == 42 || (i32::from_ne_bytes((b).to_ne_bytes())) >= 48 && (i32::from_ne_bytes((b).to_ne_bytes())) <= 57 {
            if i32::from_ne_bytes((n).to_ne_bytes()) < (i32::from_ne_bytes((e).to_ne_bytes())) && ClusterRoleResolution::cluster_role_resolution_code_point(t, n) == 65039 {
                n = u32::wrapping_add(n, 1);
            }
            if i32::from_ne_bytes((n).to_ne_bytes()) < (i32::from_ne_bytes((e).to_ne_bytes())) && ClusterRoleResolution::cluster_role_resolution_code_point(t, n) == 8419 {
                return Some("KeycapSequence".to_string());
            }
        }
        if UnicodeEmojiData::unicode_emoji_data_contains(b) && UnicodeEmojiStyleVariationData::unicode_emoji_style_variation_data_contains(b) && (i32::from_ne_bytes((n).to_ne_bytes())) < (i32::from_ne_bytes((e).to_ne_bytes())) &&
ClusterRoleResolution::cluster_role_resolution_code_point(t, n) == 65039 {
            return Some("EmojiStyleVariationSequence".to_string());
        }
        if UnicodeEmojiModifierBaseData::unicode_emoji_modifier_base_data_contains(b) {
            while (i32::from_ne_bytes((n).to_ne_bytes())) < (i32::from_ne_bytes((e).to_ne_bytes())) && (ClusterRoleResolution::cluster_role_resolution_is_combining(ClusterRoleResolution::cluster_role_resolution_code_point(t, n)) ||
ClusterRoleResolution::cluster_role_resolution_is_variation(ClusterRoleResolution::cluster_role_resolution_code_point(t, n))) {
                n = u32::wrapping_add(n, if i32::from_ne_bytes((ClusterRoleResolution::cluster_role_resolution_code_point(t, n)).to_ne_bytes()) > (65535) { 2 } else { 1 });
            }
            let m = if i32::from_ne_bytes((n).to_ne_bytes()) < (i32::from_ne_bytes((e).to_ne_bytes())) { ClusterRoleResolution::cluster_role_resolution_code_point(t, n) } else { 4294967295u32 };
            if ({ let v: u32 = m; i32::from_ne_bytes(v.to_ne_bytes()) }) >= 127995 && ({ let v: u32 = m; i32::from_ne_bytes(v.to_ne_bytes()) }) <= 127999 {
                return Some("EmojiModifierSequence".to_string());
            }
        }
        return None;
    }
}
