use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::decoration_decision_info::DecorationDecisionInfo;
use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_segment_info::DecorationSegmentInfo;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::fp_helper::FPHelper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphToPreparedParagraphJsonFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}

impl From<PreparedParagraphToPreparedParagraphJsonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphToPreparedParagraphJsonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphToPlanWithDiagnosticsJsonFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ToPreparedParagraphJsonFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}

impl From<PreparedParagraphToPlanWithDiagnosticsJsonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphToPlanWithDiagnosticsJsonFault) -> Self {
        match value {
            PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphToPlanWithDiagnosticsJsonFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphToPlanWithDiagnosticsJsonFault) -> Self {
        match value {
            PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphToPlanWithDiagnosticsJsonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphToPlanWithDiagnosticsJsonFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(value)
    }
}

static PREPARED_PARAGRAPH_FNS_FIVE_POWERS_BUILDER: Mutex<Option<SortedMapTableBuilder<u32, String>>> = Mutex::new(None);
static PREPARED_PARAGRAPH_FNS_FIVE_POWERS: Mutex<Option<SortedMapTable<u32, String>>> = Mutex::new(None);
static PREPARED_PARAGRAPH_FNS_TWO_POWERS_BUILDER: Mutex<Option<SortedMapTableBuilder<u32, String>>> = Mutex::new(None);
static PREPARED_PARAGRAPH_FNS_TWO_POWERS: Mutex<Option<SortedMapTable<u32, String>>> = Mutex::new(None);

#[derive(Clone, Copy)]
pub struct PreparedParagraphFns;

impl PreparedParagraphFns {

    pub fn prepared_paragraph_fns_to_prepared_paragraph_json(result: LayoutResult, render_evidence: bool) -> Result<String, PreparedParagraphToPreparedParagraphJsonFault> {
        let mut natural_b: SortedMapTableBuilder<String, f64> = SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut features_b: SortedMapTableBuilder<String, Vec<String>> = SortedTable::sorted_table_map_builder::<String, Vec<String>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut fonts_b: SortedMapTableBuilder<String, String> = SortedTable::sorted_table_map_builder::<String, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut glyph_ids_b: SortedMapTableBuilder<String, Vec<String>> = SortedTable::sorted_table_map_builder::<String, Vec<String>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        for ri in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (result.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let glyph = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                let k = PreparedParagraphFns::prepared_paragraph_fns_range_key((glyph.cluster_range).clone());
                let natural_prior = natural_b.get(&(k).to_string());
                let prior_natural = (natural_prior).unwrap_or(0.0f64);
                natural_b.put(&(k).to_string(), &(prior_natural + glyph.advance));
                if i32::from_ne_bytes((u32::try_from((run.open_type_features.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                    let mut fs = features_b.get(&(k).to_string());
                    if fs.is_none() {
                        fs = Some(vec![].clone());
                    }
                    for fi in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let f = (run.open_type_features[usize::try_from(fi).unwrap_or(0)]).clone();
                        if u32::from_ne_bytes((match (fs).as_ref().unwrap().iter().position(|e| e == &f) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) > 2147483647 {
                            (fs).as_mut().unwrap().push(f.clone());
                        }
                    }
                    features_b.put(&(k).to_string(), (fs).as_ref().unwrap());
                }
                let render_font_key = glyph.render_font_key.clone();
                match &(render_font_key) {
                    Some(__option) => {
                        fonts_b.put(&(k).to_string(), &__option);
                    }
                    None => {
                    }
                }
                let mut ids = glyph_ids_b.get(&(k).to_string());
                if ids.is_none() {
                    ids = Some(vec![].clone());
                }
                (ids).as_mut().unwrap().push(crate::runtime::int_text::IntText::int_text(glyph.id));
                glyph_ids_b.put(&(k).to_string(), (ids).as_ref().unwrap());
            }
        }
        let mut zero_b: SortedMapTableBuilder<String, bool> = SortedTable::sorted_table_map_builder::<String, bool>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let zero_src = ((result.debug).clone().zero_width_break_decisions).clone();
        for zi in 0..match u32::try_from(zero_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            zero_b.put(&PreparedParagraphFns::prepared_paragraph_fns_range_key(((zero_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &(true));
        }
        let mut shaping_b: SortedMapTableBuilder<String, ShapingDecisionInfo> = SortedTable::sorted_table_map_builder::<String, ShapingDecisionInfo>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let shaping_src = ((result.debug).clone().shaping_decisions).clone();
        for zi in 0..match u32::try_from(shaping_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            shaping_b.put(&PreparedParagraphFns::prepared_paragraph_fns_range_key(((shaping_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &((shaping_src[usize::try_from(zi).unwrap_or(0)]).clone()));
        }
        let mut punct_b: SortedMapTableBuilder<String, PunctuationDecisionInfo> = SortedTable::sorted_table_map_builder::<String, PunctuationDecisionInfo>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let punct_src = ((result.debug).clone().punctuation_decisions).clone();
        for zi in 0..match u32::try_from(punct_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            punct_b.put(&PreparedParagraphFns::prepared_paragraph_fns_range_key(((punct_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &((punct_src[usize::try_from(zi).unwrap_or(0)]).clone()));
        }
        let mut inline_advance_b: SortedMapTableBuilder<String, f64> = SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let inline_obj_src = ((result.input).clone().inline_objects).clone();
        for zi in 0..match u32::try_from(inline_obj_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            inline_advance_b.put(&PreparedParagraphFns::prepared_paragraph_fns_range_key(((inline_obj_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &(inline_obj_src[usize::try_from(zi).unwrap_or(0)].advance));
        }
        let mut edge_start_b: SortedMapTableBuilder<String, f64> = SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut edge_end_b: SortedMapTableBuilder<String, f64> = SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let box_src = ((result.input).clone().inline_boxes).clone();
        for b in &box_src {
            if b.inline_start != 0 as f64 {
                let sk = crate::runtime::int_text::IntText::int_text((b.range).clone().start);
                let start_prior = edge_start_b.get(&(sk).to_string());
                let prior_start = (start_prior).unwrap_or(0.0f64);
                edge_start_b.put(&(sk).to_string(), &(prior_start + b.inline_start));
            }
            if b.inline_end != 0 as f64 {
                let ek = crate::runtime::int_text::IntText::int_text((b.range).clone().end);
                let end_prior = edge_end_b.get(&(ek).to_string());
                let prior_end = (end_prior).unwrap_or(0.0f64);
                edge_end_b.put(&(ek).to_string(), &(prior_end + b.inline_end));
            }
        }
        let natural: SortedMapTable<String, f64> = natural_b.clone().build();
        let features: SortedMapTable<String, Vec<String>> = features_b.clone().build();
        let fonts: SortedMapTable<String, String> = fonts_b.clone().build();
        let glyph_ids: SortedMapTable<String, Vec<String>> = glyph_ids_b.clone().build();
        let zero: SortedMapTable<String, bool> = zero_b.clone().build();
        let shaping: SortedMapTable<String, ShapingDecisionInfo> = shaping_b.clone().build();
        let punct: SortedMapTable<String, PunctuationDecisionInfo> = punct_b.clone().build();
        let inline_advance: SortedMapTable<String, f64> = inline_advance_b.clone().build();
        let edge_start: SortedMapTable<String, f64> = edge_start_b.clone().build();
        let edge_end: SortedMapTable<String, f64> = edge_end_b.clone().build();
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"{".is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("{".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"\"schema\":1,\"layoutRevision\":\"tiqian-layout-v2\",\"width\":".is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("\"schema\":1,\"layoutRevision\":\"tiqian-layout-v2\",\"width\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(((result.input).clone().constraints).clone().max_width).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(((result.input).clone().constraints).clone().max_width).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"height\":".is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend(",\"height\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number((result.size).clone().height).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number((result.size).clone().height).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"lines\":[".is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend(",\"lines\":[".encode_utf16());
        for li in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ({ let v: u32 = li; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                        return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(",".encode_utf16());
            }
            let line = (result.lines[usize::try_from(li).unwrap_or(0)]).clone();
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"{\"rangeStart\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend("{\"rangeStart\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((line.range).clone().start).is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(crate::runtime::int_text::IntText::int_text((line.range).clone().start).encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"rangeEnd\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"rangeEnd\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((line.range).clone().end).is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(crate::runtime::int_text::IntText::int_text((line.range).clone().end).encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"top\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"top\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.top).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.top).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"bottom\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"bottom\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.bottom).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.bottom).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"baseline\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"baseline\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.baseline).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.baseline).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"indent\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"indent\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.indent).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.indent).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"visualWidth\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"visualWidth\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.visual_width).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.visual_width).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"hyphenAdvance\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"hyphenAdvance\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.hyphen_advance).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(line.hyphen_advance).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"endReason\":".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"endReason\":".encode_utf16());
            let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, line.end_reason.name().to_string().as_str()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?;
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"cells\":[".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend(",\"cells\":[".encode_utf16());
            let mut ci = 0u32;
            {
                let _g1 = LayoutQueries::layout_queries_positioned_clusters_for_line((result).clone(), (line).clone()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(e))?;
                for p in &_g1 {
                    let c = (result.clusters[usize::try_from(p.cluster_index).unwrap_or(0)]).clone();
                    let ck = PreparedParagraphFns::prepared_paragraph_fns_range_key((c.range).clone());
                    if !((i32::from_ne_bytes((u_string::unit_count(&((c.display_text).to_string()))).to_ne_bytes())) > (0) || zero.has(&(ck).to_string()) || render_evidence && inline_advance.has(&(ck).to_string())) {
                        continue;
                    }
                    if i32::from_ne_bytes({ let t = ci; ci = u32::wrapping_add(ci, 1); t }.to_ne_bytes()) > (0) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"rangeStart\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend("{\"rangeStart\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((c.range).clone().start).is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text((c.range).clone().start).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"rangeEnd\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"rangeEnd\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((c.range).clone().end).is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text((c.range).clone().end).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"source\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"source\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, (c.text).to_string().as_str()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"display\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"display\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, (c.display_text).to_string().as_str()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"drawX\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"drawX\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.draw_x).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.draw_x).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"naturalWidth\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"naturalWidth\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(if natural.has(&(ck).to_string()) { (natural.get(&(ck).to_string())).unwrap() } else { c.advance }).map_err(|e|
PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(if natural.has(&(ck).to_string()) { (natural.get(&(ck).to_string())).unwrap() } else { c.advance }).map_err(|e|
PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"leadingLayoutAdvance\":".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"leadingLayoutAdvance\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(c.leading_layout_advance).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(c.leading_layout_advance).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?.encode_utf16());
                    if i32::from_ne_bytes((u32::wrapping_sub((c.range).clone().end, (c.range).clone().start)).to_ne_bytes()) > (1) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"shapingBoundary\":true".is_empty() {
                                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                        out.extend(",\"shapingBoundary\":true".encode_utf16());
                    }
                    if features.has(&(ck).to_string()) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"openTypeFeatures\":[".is_empty() {
                                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                        out.extend(",\"openTypeFeatures\":[".encode_utf16());
                        let mut fi = 0u32;
                        {
                            let mut _g = 0u32;
                            let _g1 = (features.get(&(ck).to_string())).unwrap();
                            while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                let f = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if i32::from_ne_bytes({ let t = fi; fi = u32::wrapping_add(fi, 1); t }.to_ne_bytes()) > (0) {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                                        }
                                    }
                                    out.extend(",".encode_utf16());
                                }
                                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, f.as_str()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?;
                            }
                        }
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                        out.extend("]".encode_utf16());
                    }
                    if render_evidence {
                        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_cell_evidence(&mut out, (result).clone(), (c).clone(), (natural).clone(), (fonts).clone(), (glyph_ids).clone(), (shaping).clone(), (punct).clone(), (inline_advance).clone()).map_err(|e|
PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?;
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend("}".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]}".is_empty() {
                    return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
            out.extend("]}".encode_utf16());
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("]".encode_utf16());
        if render_evidence {
            let _ = PreparedParagraphFns::prepared_paragraph_fns_append_paragraph_evidence(&mut out, (result).clone(), (edge_start).clone(), (edge_end).clone()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?;
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                return Err(PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("}".encode_utf16());
        return Ok(String::from_utf16(out.as_slice()).map_err(|_| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) }))?);
    }

    pub fn prepared_paragraph_fns_to_plan_with_diagnostics_json(result: LayoutResult, render_evidence: bool, zero_advance_epsilon_px: f64) -> Result<String, PreparedParagraphToPlanWithDiagnosticsJsonFault> {
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"{\"plan\":".is_empty() {
                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("{\"plan\":".encode_utf16());
        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((result).clone(), render_evidence).map_err(|e|
PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(e))?.as_str()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"diagnostics\":{\"capabilityIssues\":[".is_empty() {
                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend(",\"diagnostics\":{\"capabilityIssues\":[".encode_utf16());
        let mut first = true;
        let shaping_diag_src = ((result.debug).clone().shaping_decisions).clone();
        for d in &shaping_diag_src {
            match &((d.capability_issue).clone()) {
                Some(__option1) => {
                    if !first {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"name\":".is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend("{\"name\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, (*__option1).clone().as_str()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"reason\":".is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"reason\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, (d.reason).to_string().as_str()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"rangeStart\":".is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"rangeStart\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((d.range).clone().start).is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text((d.range).clone().start).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"rangeEnd\":".is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",\"rangeEnd\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((d.range).clone().end).is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text((d.range).clone().end).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend("}".encode_utf16());
                }
                None => {
                }
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"],\"advanceSuspects\":[".is_empty() {
                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("],\"advanceSuspects\":[".encode_utf16());
        first = true;
        for d in &shaping_diag_src {
            if !((d.advance).is_finite() && (d.advance) > (zero_advance_epsilon_px)) {
                if !first {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                first = false;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"{\"displayText\":".is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend("{\"displayText\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, (d.display_text).to_string().as_str()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"advance\":\"".is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(",\"advance\":\"".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !if d.advance.is_finite() { PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.advance).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?.to_string() } else {
crate::runtime::fp_helper::FPHelper::format_float(d.advance).to_string() }.is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(if d.advance.is_finite() { PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.advance).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?.to_string() } else {
crate::runtime::fp_helper::FPHelper::format_float(d.advance).to_string() }.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\",\"reason\":".is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend("\",\"reason\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(&mut out, (d.reason).to_string().as_str()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"rangeStart\":".is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(",\"rangeStart\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((d.range).clone().start).is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((d.range).clone().start).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"rangeEnd\":".is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(",\"rangeEnd\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((d.range).clone().end).is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((d.range).clone().end).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                        return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                    }
                }
                out.extend("}".encode_utf16());
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"]}}".is_empty() {
                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
            }
        }
        out.extend("]}}".encode_utf16());
        return Ok(String::from_utf16(out.as_slice()).map_err(|_| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) }))?);
    }

    pub(crate) fn prepared_paragraph_fns_range_key(r: TextRange) -> String {
        return format!("{}{}{}",
            crate::runtime::int_text::IntText::int_text(r.start),
            ":",
            crate::runtime::int_text::IntText::int_text(r.end)
        );
    }

    pub(crate) fn prepared_paragraph_fns_append_json_string(out: &mut Vec<u16>, value: &str) -> Result<(), UStringFault> {
    let __units = u_string::units(&value);
    let __count = u_string::unit_count(&value);
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"\"".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("\"".encode_utf16());
        for i in 0..match u32::try_from(u_string::unit_count(&(value))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units, i).unwrap_or(0);
            if c == 34 {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\\\"".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\\\"".encode_utf16());
            } else {
                if c == 92 {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"\\\\".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("\\\\".encode_utf16());
                } else {
                    if c == 8 {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !"\\b".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend("\\b".encode_utf16());
                    } else {
                        if c == 12 {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !"\\f".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend("\\f".encode_utf16());
                        } else {
                            if c == 10 {
                                if let Some(&unit) = out.last() {
                                    if unit >= 55296 && unit <= 56319 && !"\\n".is_empty() {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                                out.extend("\\n".encode_utf16());
                            } else {
                                if c == 13 {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !"\\r".is_empty() {
                                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                        }
                                    }
                                    out.extend("\\r".encode_utf16());
                                } else {
                                    if c == 9 {
                                        if let Some(&unit) = out.last() {
                                            if unit >= 55296 && unit <= 56319 && !"\\t".is_empty() {
                                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                            }
                                        }
                                        out.extend("\\t".encode_utf16());
                                    } else {
                                        if i32::from_ne_bytes((c).to_ne_bytes()) < (32) {
                                            if let Some(&unit) = out.last() {
                                                if unit >= 55296 && unit <= 56319 && !format!("{}{}",
            "\\u",
            format!("{:0w$X}", c, w = usize::try_from(4).unwrap_or_default()).to_lowercase()
        ).is_empty() {
                                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                                }
                                            }
                                            out.extend(format!("{}{}",
            "\\u",
            format!("{:0w$X}", c, w = usize::try_from(4).unwrap_or_default()).to_lowercase()
        ).encode_utf16());
                                        } else {
                                            if c >= 56320 && c <= 57343 {
                                                match out.last() {
                                                    Some(&last) if last >= 55296 && last <= 56319 => {}
                                                    _ => return Err(UStringFault::UnpairedSurrogate { unit: c }),
                                                }
                                            } else if let Some(&last) = out.last() {
                                                if last >= 55296 && last <= 56319 {
                                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                                                }
                                            }
                                            out.push(u16::try_from((c) & 0xFFFF).unwrap_or(0));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"\"".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("\"".encode_utf16());
        Ok(())
    }

    pub(crate) fn prepared_paragraph_fns_append_cell_evidence(out: &mut Vec<u16>, result: LayoutResult, c: Cluster, natural: SortedMapTable<String, f64>, fonts: SortedMapTable<String, String>, ids: SortedMapTable<String, Vec<String>>, shaping: SortedMapTable<String,
ShapingDecisionInfo>, punct: SortedMapTable<String, PunctuationDecisionInfo>, inline_advance: SortedMapTable<String, f64>) -> Result<(), UStringFault> {
        let k = PreparedParagraphFns::prepared_paragraph_fns_range_key((c.range).clone());
        if inline_advance.has(&(k).to_string()) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"inlineObject\":".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"inlineObject\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(inline_advance.get(&(k).to_string()).unwrap())?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(inline_advance.get(&(k).to_string()).unwrap())?.encode_utf16());
        }
        let width = if inline_advance.has(&(k).to_string()) { (inline_advance.get(&(k).to_string())).unwrap() } else { if natural.has(&(k).to_string()) { (natural.get(&(k).to_string())).unwrap() } else { c.advance } };
        if c.advance != width {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"advance\":".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"advance\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(c.advance)?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(c.advance)?.encode_utf16());
        }
        if fonts.has(&(k).to_string()) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"renderFontFamily\":".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"renderFontFamily\":".encode_utf16());
            let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (fonts.get(&(k).to_string())).as_deref().unwrap_or(""))?;
        }
        let sd = shaping.get(&(k).to_string());
        match &(sd) {
            Some(__option2) => {
                if __option2.strategy.is_some() {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"dashStrategy\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"dashStrategy\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (__option2.strategy).as_deref().unwrap_or(""))?;
                match &(__option2.language) {
                    Some(__option3) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"shapingLanguage\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"shapingLanguage\":".encode_utf16());
                        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (*__option3).clone().as_str())?;
                    }
                    None => {
                    }
                }
                match &(__option2.resolved_face) {
                    Some(__option4) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"resolvedFace\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"resolvedFace\":".encode_utf16());
                        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (*__option4).clone().as_str())?;
                    }
                    None => {
                    }
                }
                if ids.has(&(k).to_string()) && (u32::try_from(((ids.get(&(k).to_string())).as_ref().map_or(0, |v| v.len())) & 0xFFFF_FFFF).unwrap_or(0)) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"glyphIds\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"glyphIds\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, { let _jtmp2 = ids.get(&(k).to_string()); let joined2 = _jtmp2.as_ref().unwrap(); let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if
index2 > 0 { out.push_str(&(",")); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } out }.as_str())?;
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"shapingEvidence\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"shapingEvidence\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (__option2.reason).to_string().as_str())?;
                }
            }
            None => {
            }
        }
        let pd = punct.get(&(k).to_string());
        if match &(pd) { Some(__option5) => __option5.ink_containment_applied && __option5.ink_containment_body_floor.is_some(), None => false } {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"punctuationInkFloor\":".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"punctuationInkFloor\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(((pd).as_ref().unwrap().ink_containment_body_floor).unwrap())?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(((pd).as_ref().unwrap().ink_containment_body_floor).unwrap())?.encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"punctuationBodyWidth\":".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"punctuationBodyWidth\":".encode_utf16());
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number((pd).as_ref().unwrap().body_width)?.is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number((pd).as_ref().unwrap().body_width)?.encode_utf16());
        }
        let mut latin = false;
        {
            let fd_src = ((result.debug).clone().font_decisions).clone();
            for fd in &fd_src {
                if i32::from_ne_bytes(((c.range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes(((fd.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes(((c.range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((fd.range).clone().end).to_ne_bytes()) &&
(fd.role).to_string() == FontRole::LatinText.name().to_string() {
                    latin = true;
                }
            }
        }
        if latin {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"latin\":true".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"latin\":true".encode_utf16());
        }
        let style = PreparedParagraphFns::prepared_paragraph_fns_style_at((result).clone(), (c.range).clone().start);
        if style != ((result.input).clone().text_style).clone() {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"style\":{".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"style\":{".encode_utf16());
            let mut n = 0u32;
            if style.font_size != ((result.input).clone().text_style).clone().font_size {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\"fontSize\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\"fontSize\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(style.font_size)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(style.font_size)?.encode_utf16());
                n = u32::wrapping_add(n, 1);
            }
            if style.font_weight != ((result.input).clone().text_style).clone().font_weight {
                if i32::from_ne_bytes({ let t = n; n = u32::wrapping_add(n, 1); t }.to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\"fontWeight\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\"fontWeight\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(style.font_weight).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(style.font_weight).encode_utf16());
            }
            if style.italic != ((result.input).clone().text_style).clone().italic {
                if i32::from_ne_bytes({ let t = n; n = u32::wrapping_add(n, 1); t }.to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\"italic\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\"italic\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !if style.italic { "true".to_string() } else { "false".to_string() }.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(if style.italic { "true".to_string() } else { "false".to_string() }.encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("}".encode_utf16());
        }
        Ok(())
    }

    pub(crate) fn prepared_paragraph_fns_style_at(result: LayoutResult, offset: u32) -> TextStyle {
        let mut found: Option<TextStyle> = None;
        {
            let span_src = (((result.input).clone().content).clone().spans).clone();
            for span in &span_src {
                if i32::from_ne_bytes((offset).to_ne_bytes()) >= i32::from_ne_bytes(((span.range).clone().start).to_ne_bytes()) && (i32::from_ne_bytes((offset).to_ne_bytes())) < (i32::from_ne_bytes(((span.range).clone().end).to_ne_bytes())) {
                    found = Some((span.style).clone());
                }
            }
        }
        return match &(found) { None => ((result.input).clone().text_style).clone(), Some(__option6) => (*__option6).clone() };
    }

    pub(crate) fn prepared_paragraph_fns_append_paragraph_evidence(out: &mut Vec<u16>, result: LayoutResult, starts: SortedMapTable<String, f64>, ends: SortedMapTable<String, f64>) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"fontSize\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"fontSize\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(((result.input).clone().text_style).clone().font_size)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(((result.input).clone().text_style).clone().font_size)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"overlayWidth\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"overlayWidth\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number((result.size).clone().width)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number((result.size).clone().width)?.encode_utf16());
        let mut emph: Vec<DecorationSpan> = vec![];
        {
            let dec_src = ((result.input).clone().decorations).clone();
            for d in &dec_src {
                if d.kind == DecorationKind::Emphasis {
                    emph.push(d.clone());
                }
            }
        }
        if i32::from_ne_bytes((u32::try_from((emph.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"emphasisRanges\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"emphasisRanges\":[".encode_utf16());
            for i in 0..match u32::try_from(emph.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("[".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(((emph[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(((emph[usize::try_from(i).unwrap_or(0)]).clone().range).clone().start).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(((emph[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(((emph[usize::try_from(i).unwrap_or(0)]).clone().range).clone().end).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("]".encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        let mut offsets: Vec<u32> = vec![];
        for oi in 0..u32::from_ne_bytes((starts.size()).to_ne_bytes()) {
            offsets.push(u32::from_ne_bytes((u_string::parse_i32(&(starts.key_at({ let v: u32 = oi; i32::from_ne_bytes(v.to_ne_bytes()) }))).unwrap()).to_ne_bytes()));
        }
        for ei in 0..u32::from_ne_bytes((ends.size()).to_ne_bytes()) {
            let ev = u_string::parse_i32(&(ends.key_at({ let v: u32 = ei; i32::from_ne_bytes(v.to_ne_bytes()) })));
            if u32::from_ne_bytes((match offsets.iter().position(|e| e == &u32::from_ne_bytes((ev.unwrap_or(-1)).to_ne_bytes())) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }).to_ne_bytes()) > 2147483647 {
                offsets.push(u32::from_ne_bytes((ev.unwrap()).to_ne_bytes()));
            }
        }
        let mut oi = 1u32;
        while (i32::from_ne_bytes((oi).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((offsets.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let ov = offsets[usize::try_from(oi).unwrap_or(0)];
            let mut oj = oi;
            while (oj) > (0) && ({ let v: u32 = offsets[usize::try_from(u32::wrapping_sub(oj, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = ov; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                { while offsets.len() <= usize::try_from(oj).unwrap_or(0) { offsets.push(0); } offsets[usize::try_from(oj).unwrap_or(0)] = offsets[usize::try_from(u32::wrapping_sub(oj, 1)).unwrap_or(0)]; };
                oj = u32::wrapping_sub(oj, 1);
            }
            { while offsets.len() <= usize::try_from(oj).unwrap_or(0) { offsets.push(0); } offsets[usize::try_from(oj).unwrap_or(0)] = ov; };
            oi = u32::wrapping_add(oi, 1);
        }
        if i32::from_ne_bytes((u32::try_from((offsets.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"inlineEdges\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"inlineEdges\":[".encode_utf16());
            for i in 0..match u32::try_from(offsets.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                let k = crate::runtime::int_text::IntText::int_text(offsets[usize::try_from(i).unwrap_or(0)]);
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"{\"offset\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("{\"offset\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(offsets[usize::try_from(i).unwrap_or(0)]).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(offsets[usize::try_from(i).unwrap_or(0)]).encode_utf16());
                if starts.has(&(k).to_string()) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"inlineStart\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"inlineStart\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(starts.get(&(k).to_string()).unwrap())?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(starts.get(&(k).to_string()).unwrap())?.encode_utf16());
                }
                if ends.has(&(k).to_string()) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"inlineEnd\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"inlineEnd\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(ends.get(&(k).to_string()).unwrap())?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(ends.get(&(k).to_string()).unwrap())?.encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("}".encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from(((result.debug).clone().ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"rubyDecisions\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"rubyDecisions\":[".encode_utf16());
            for i in 0..match u32::try_from((result.debug).clone().ruby_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                let r = ((result.debug).clone().ruby_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"{\"baseRangeStart\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("{\"baseRangeStart\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((r.base_range).clone().start).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((r.base_range).clone().start).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"baseRangeEnd\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"baseRangeEnd\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((r.base_range).clone().end).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((r.base_range).clone().end).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"text\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"text\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (r.text).to_string().as_str())?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"centerX\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"centerX\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.center_x)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.center_x)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"baselineY\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"baselineY\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.baseline_y)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.baseline_y)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"fontSize\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"fontSize\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.font_size)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.font_size)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"ascent\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"ascent\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.ascent)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(r.ascent)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"fontWeight\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"fontWeight\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(r.font_weight).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(r.font_weight).encode_utf16());
                if i32::from_ne_bytes((u32::try_from((r.font_families.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"fontFamilies\":[".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"fontFamilies\":[".encode_utf16());
                    for j in 0..match u32::try_from(r.font_families.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if ({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",".encode_utf16());
                        }
                        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (r.font_families[usize::try_from(j).unwrap_or(0)]).clone().as_str())?;
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("]".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("}".encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_remaining_paragraph_evidence(out, (result).clone())?;
        Ok(())
    }

    pub(crate) fn prepared_paragraph_fns_append_remaining_paragraph_evidence(out: &mut Vec<u16>, result: LayoutResult) -> Result<(), UStringFault> {
        if i32::from_ne_bytes((u32::try_from(((result.debug).clone().bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"bopomofoDecisions\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"bopomofoDecisions\":[".encode_utf16());
            for i in 0..match u32::try_from((result.debug).clone().bopomofo_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                let z = ((result.debug).clone().bopomofo_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"{\"baseRangeStart\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("{\"baseRangeStart\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((z.base_range).clone().start).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((z.base_range).clone().start).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"baseRangeEnd\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"baseRangeEnd\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((z.base_range).clone().end).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((z.base_range).clone().end).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"text\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"text\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (z.text).to_string().as_str())?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"fontWeight\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"fontWeight\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(z.font_weight).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(z.font_weight).encode_utf16());
                if i32::from_ne_bytes((u32::try_from((z.font_families.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"fontFamilies\":[".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"fontFamilies\":[".encode_utf16());
                    for j in 0..match u32::try_from(z.font_families.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        if ({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",".encode_utf16());
                        }
                        let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (z.font_families[usize::try_from(j).unwrap_or(0)]).clone().as_str())?;
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("]".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"placements\":[".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"placements\":[".encode_utf16());
                for j in 0..match u32::try_from(z.placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    if ({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    let p = (z.placements[usize::try_from(j).unwrap_or(0)]).clone();
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"text\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("{\"text\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (p.text).to_string().as_str())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"left\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"left\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.left)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.left)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"top\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"top\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.top)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.top)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"width\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"width\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.width)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.width)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"height\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"height\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.height)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(p.height)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"role\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"role\":".encode_utf16());
                    let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, p.role.name().to_string().as_str())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("}".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"]}".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("]}".encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        let mut segs: Vec<DecorationSegmentInfo> = vec![];
        {
            let seg_src = ((result.debug).clone().decoration_segments).clone();
            for s in &seg_src {
                if s.kind.to_string() == DecorationKind::ProperNoun.name().to_string() || (s.kind).to_string() == DecorationKind::BookTitle.name().to_string() {
                    segs.push(s.clone());
                }
            }
        }
        if i32::from_ne_bytes((u32::try_from((segs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"decorationSegments\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"decorationSegments\":[".encode_utf16());
            for i in 0..match u32::try_from(segs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                let s = (segs[usize::try_from(i).unwrap_or(0)]).clone();
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"{\"kind\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("{\"kind\":".encode_utf16());
                let _ = PreparedParagraphFns::prepared_paragraph_fns_append_json_string(out, (s.kind).to_string().as_str())?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"left\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"left\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(s.left)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(s.left)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"top\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"top\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(s.top)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(s.top)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"right\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"right\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(s.right)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(s.right)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"sourceRangeStart\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"sourceRangeStart\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((s.source_range).clone().start).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((s.source_range).clone().start).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"sourceRangeEnd\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"sourceRangeEnd\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((s.source_range).clone().end).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((s.source_range).clone().end).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("}".encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        let mut dots: Vec<DecorationDecisionInfo> = vec![];
        {
            let dot_src = ((result.debug).clone().decoration_decisions).clone();
            for d in &dot_src {
                if d.applied && (d.kind).to_string() == DecorationKind::Emphasis.name().to_string() && (d.dot_diameter) > (0 as f64) {
                    dots.push(d.clone());
                }
            }
        }
        if i32::from_ne_bytes((u32::try_from((dots.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"emphasisDots\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"emphasisDots\":[".encode_utf16());
            for i in 0..match u32::try_from(dots.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                let d = (dots[usize::try_from(i).unwrap_or(0)]).clone();
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"{\"clusterRangeStart\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("{\"clusterRangeStart\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text((d.cluster_range).clone().start).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text((d.cluster_range).clone().start).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"anchorX\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"anchorX\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.anchor_x)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.anchor_x)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"anchorY\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"anchorY\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.anchor_y)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.anchor_y)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"dotDiameter\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"dotDiameter\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.dot_diameter)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PreparedParagraphFns::prepared_paragraph_fns_ecma_json_number(d.dot_diameter)?.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("}".encode_utf16());
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        Ok(())
    }

    pub fn prepared_paragraph_fns_ecma_json_number(float_value: f64) -> Result<String, UStringFault> {
        if float_value.is_nan() {
            return Ok("NaN".to_string());
        }
        if float_value.is_finite() == false {
            return Ok(if float_value < (0 as f64) { "-Infinity".to_string() } else { "Infinity".to_string() });
        }
        if float_value == 0 as f64 {
            return Ok("0".to_string());
        }
        let negative = float_value < (0 as f64);
        let magnitude_value = if negative { -float_value } else { float_value };
        let shortest = PreparedParagraphFns::prepared_paragraph_fns_shortest_round_trip_digits(magnitude_value)?;
        let digits = PreparedParagraphFns::prepared_paragraph_fns_canonical_float_digits(magnitude_value, (shortest.digits).to_string().as_str())?;
        let k = u_string::unit_count(&(digits));
        let n = shortest.exponent;
        let sign = if negative { "-".to_string() } else { "".to_string() };
        if i32::from_ne_bytes((k).to_ne_bytes()) <= i32::from_ne_bytes((n).to_ne_bytes()) && (i32::from_ne_bytes((n).to_ne_bytes())) <= 21 {
            return Ok(format!("{}{}{}",
            sign,
            digits,
            PreparedParagraphFns::prepared_paragraph_fns_zeros(u32::wrapping_sub(n, k))
        ));
        }
        if 0 < (i32::from_ne_bytes((n).to_ne_bytes())) && (i32::from_ne_bytes((n).to_ne_bytes())) <= 21 {
            return Ok(format!("{}{}{}{}",
            sign,
            u_string::substr(&digits, 0i32, Some(i32::from_ne_bytes((n).to_ne_bytes()))),
            ".",
            u_string::substr(&digits, i32::from_ne_bytes((n).to_ne_bytes()), None)
        ));
        }
        if i32::from_ne_bytes((4294967290u32).to_ne_bytes()) < (i32::from_ne_bytes((n).to_ne_bytes())) && (i32::from_ne_bytes((n).to_ne_bytes())) <= 0 {
            return Ok(format!("{}{}{}{}",
            sign,
            "0.",
            PreparedParagraphFns::prepared_paragraph_fns_zeros(u32::from_ne_bytes((-(n as i32)).to_ne_bytes())),
            digits
        ));
        }
        let mantissa = if i32::from_ne_bytes((k).to_ne_bytes()) > (1) { format!("{}{}{}",
            u_string::substr(&digits, 0i32, Some(1i32)),
            ".",
            u_string::substr(&digits, 1i32, None)
        ).to_string() } else { digits.to_string() };
        let ev = i32::wrapping_sub(i32::from_ne_bytes((n).to_ne_bytes()), 1);
        let esign = if ev < (0) { "-".to_string() } else { "+".to_string() };
        let absolute_exponent = if ev < (0) { u32::from_ne_bytes((-(ev as i32)).to_ne_bytes()) } else { u32::from_ne_bytes((ev).to_ne_bytes()) };
        return Ok(format!("{}{}{}{}{}",
            sign,
            mantissa,
            "e",
            esign,
            crate::runtime::int_text::IntText::int_text(absolute_exponent)
        ));
    }

    pub fn prepared_paragraph_fns_shortest_round_trip_digits(magnitude: f64) -> Result<PreparedParagraphDigitsAndExponent, UStringFault> {
        let d = PreparedParagraphFns::prepared_paragraph_fns_decompose(magnitude)?;
        let f = d.exp2;
        let expansion = PreparedParagraphFns::prepared_paragraph_fns_dyadic_decimal((d.mantissa).to_string().as_str(), f)?;
        let exact = PreparedParagraphFns::prepared_paragraph_fns_trim_zeros((expansion.digits).to_string().as_str());
        let n = expansion.exponent;
        let base = (d.mantissa).to_string().clone();
        let doubled = PreparedParagraphFns::prepared_paragraph_fns_times_small(base.as_str(), 2)?;
        let hi = PreparedParagraphFns::prepared_paragraph_fns_dyadic_decimal_string(PreparedParagraphFns::prepared_paragraph_fns_add_decimal(doubled.as_str(), &"1")?.as_str(), u32::wrapping_sub(d.exp2, 1))?;
        let lo: PreparedParagraphDigitsAndExponent;
        if base == "4503599627370496" {
            lo = PreparedParagraphFns::prepared_paragraph_fns_dyadic_decimal_string(PreparedParagraphFns::prepared_paragraph_fns_decrement_decimal(PreparedParagraphFns::prepared_paragraph_fns_times_small(base.as_str(), 4)?.as_str()).as_str(), u32::wrapping_sub(d.exp2, 2))?;
        } else {
            lo = PreparedParagraphFns::prepared_paragraph_fns_dyadic_decimal_string(PreparedParagraphFns::prepared_paragraph_fns_decrement_decimal(doubled.as_str()).as_str(), u32::wrapping_sub(d.exp2, 1))?;
        }
        let inclusive = u32::from_ne_bytes((i32::from_ne_bytes((u32::wrapping_sub(u_string::unit_at(&base, u32::wrapping_sub(u_string::unit_count(&(base)), 1)).unwrap_or(0), 48)).to_ne_bytes()) % 2i32).to_ne_bytes()) == 0;
        let mut length = 1u32;
        while (i32::from_ne_bytes((length).to_ne_bytes())) <= 17 {
            let keep = u_string::substr(&exact, 0i32, Some(i32::from_ne_bytes((length).to_ne_bytes())));
            let up = PreparedParagraphFns::prepared_paragraph_fns_increment_decimal(keep.as_str());
            let up_n = u32::wrapping_sub(u32::wrapping_add(n, u_string::unit_count(&(up))), length);
            if PreparedParagraphFns::prepared_paragraph_fns_in_interval(keep.as_str(), n, (lo).clone(), (hi).clone(), inclusive) || PreparedParagraphFns::prepared_paragraph_fns_in_interval(up.as_str(), up_n, (lo).clone(), (hi).clone(), inclusive) {
                let rounded = PreparedParagraphFns::prepared_paragraph_fns_round_to_significant(exact.as_str(), length);
                return Ok(PreparedParagraphDigitsAndExponent { digits: (rounded.digits).to_string().clone(), exponent: u32::wrapping_add(n, rounded.exponent) });
            }
            length = u32::wrapping_add(length, 1);
        }
        return Ok(PreparedParagraphDigitsAndExponent { digits: exact.clone(), exponent: n });
    }

    pub(crate) fn prepared_paragraph_fns_in_interval(digits: &str, n: u32, lo: PreparedParagraphDigitsAndExponent, hi: PreparedParagraphDigitsAndExponent, inclusive: bool) -> bool {
        let low = PreparedParagraphFns::prepared_paragraph_fns_compare_decimal(digits, n, (lo.digits).to_string().as_str(), lo.exponent);
        let high = PreparedParagraphFns::prepared_paragraph_fns_compare_decimal(digits, n, (hi.digits).to_string().as_str(), hi.exponent);
        return ((i32::from_ne_bytes((low).to_ne_bytes())) > (0) || low == 0 && inclusive) && ((high) > 2147483647 || high == 0 && inclusive);
    }

    pub(crate) fn prepared_paragraph_fns_canonical_float_digits(magnitude: f64, double_digits: &str) -> Result<String, UStringFault> {
    let __units1 = u_string::units(&double_digits);
    let __count1 = u_string::unit_count(&double_digits);
        let d = PreparedParagraphFns::prepared_paragraph_fns_decompose_f32(magnitude);
        let exact = if d.mant24 == 0 { "0".to_string() } else { if d.exp2 <= 2147483647 { PreparedParagraphFns::prepared_paragraph_fns_times_long(PreparedParagraphFns::prepared_paragraph_fns_two_to_the(d.exp2)?.as_str(), d.mant24)?.to_string() } else {
PreparedParagraphFns::prepared_paragraph_fns_times_long(PreparedParagraphFns::prepared_paragraph_fns_five_to_the(u32::from_ne_bytes((-(d.exp2 as i32)).to_ne_bytes()))?.as_str(), d.mant24)?.to_string() }.to_string() };
        let stripped = PreparedParagraphFns::prepared_paragraph_fns_trim_zeros(exact.as_str());
        if i32::from_ne_bytes((u_string::unit_count(&(stripped))).to_ne_bytes()) <= i32::from_ne_bytes((__count1).to_ne_bytes()) {
            return Ok(double_digits.to_string());
        }
        let rounded = PreparedParagraphFns::prepared_paragraph_fns_round_to_significant(stripped.as_str(), __count1);
        return Ok(if u_string::unit_count(&((rounded.digits).to_string())) == __count1 { (rounded.digits).to_string() } else { double_digits.to_string() });
    }

    pub(crate) fn prepared_paragraph_fns_dyadic_decimal(p: &str, f: u32) -> Result<PreparedParagraphDigitsAndExponent, UStringFault> {
        return Ok(PreparedParagraphFns::prepared_paragraph_fns_dyadic_decimal_string(p, f)?);
    }

    pub(crate) fn prepared_paragraph_fns_dyadic_decimal_string(p: &str, f: u32) -> Result<PreparedParagraphDigitsAndExponent, UStringFault> {
        let digits = if f > 2147483647 { PreparedParagraphFns::prepared_paragraph_fns_multiply_decimal(PreparedParagraphFns::prepared_paragraph_fns_five_to_the(u32::from_ne_bytes((-(f as i32)).to_ne_bytes()))?.as_str(), p)?.to_string() } else {
PreparedParagraphFns::prepared_paragraph_fns_multiply_decimal(PreparedParagraphFns::prepared_paragraph_fns_two_to_the(u32::from_ne_bytes((f).to_ne_bytes()))?.as_str(), p)?.to_string() };
        return Ok(PreparedParagraphDigitsAndExponent { digits: digits.clone(), exponent: if f > 2147483647 { u32::wrapping_add(u_string::unit_count(&(digits)), f) } else { u_string::unit_count(&(digits)) } });
    }

    pub(crate) fn prepared_paragraph_fns_multiply_decimal(a: &str, b: &str) -> Result<String, UStringFault> {
    let __units2 = u_string::units(&b);
    let __count2 = u_string::unit_count(&b);
        let mut result = "0".to_string();
        let mut shift = 0u32;
        let mut i = __count2;
        let __units3 = u_string::units(&b);
        let __count3 = u_string::unit_count(&b);
        while (i) > (0) {
            let digit = u32::wrapping_sub(u_string::unit_at_from(&__units3, u32::wrapping_sub(i, 1)).unwrap_or(0), 48);
            if digit != 0 {
                let mut part = PreparedParagraphFns::prepared_paragraph_fns_times_small(a, digit)?;
                if i32::from_ne_bytes((shift).to_ne_bytes()) > (0) {
                    part += &(PreparedParagraphFns::prepared_paragraph_fns_zeros(shift));
                }
                result = PreparedParagraphFns::prepared_paragraph_fns_add_decimal(result.as_str(), part.as_str())?;
            }
            shift = u32::wrapping_add(shift, 1);
            i = u32::wrapping_sub(i, 1);
        }
        return Ok(result);
    }

    pub(crate) fn prepared_paragraph_fns_five_to_the(k: u32) -> Result<String, UStringFault> {
        if { let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.is_none() {
            *PREPARED_PARAGRAPH_FNS_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()) = Some(SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes())))).clone());
            { let __rhs_value = Some({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.unwrap().clone().build().clone()); *PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()) =
__rhs_value; };
        }
        let cached = ({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().get(&(k));
        match &(cached) {
            Some(__option7) => {
                return Ok((*__option7).clone());
            }
            None => {
            }
        }
        let mut anchor = 0u32;
        let mut digits = "1".to_string();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.as_ref().unwrap().size().to_ne_bytes())).to_ne_bytes())) {
            if i32::from_ne_bytes({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.as_ref().unwrap().key_at(i32::from_ne_bytes((i).to_ne_bytes())).to_ne_bytes()) < (i32::from_ne_bytes((k).to_ne_bytes())) &&
(i32::from_ne_bytes({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.as_ref().unwrap().key_at(i32::from_ne_bytes((i).to_ne_bytes())).to_ne_bytes())) > (i32::from_ne_bytes((anchor).to_ne_bytes())) {
                anchor = ({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes((i).to_ne_bytes()));
                digits = ({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().value_at(i32::from_ne_bytes((i).to_ne_bytes()));
            }
            i = u32::wrapping_add(i, 1);
        }
        for _ in anchor..k {
            digits = PreparedParagraphFns::prepared_paragraph_fns_times_small(digits.as_str(), 5)?;
        }
        ({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_mut().unwrap().put(&(k), &(digits).to_string());
        { let __rhs_value1 = Some({ let __guard = PREPARED_PARAGRAPH_FNS_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.unwrap().clone().build().clone()); *PREPARED_PARAGRAPH_FNS_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()) =
__rhs_value1; };
        return Ok(digits);
    }

    pub(crate) fn prepared_paragraph_fns_two_to_the(k: u32) -> Result<String, UStringFault> {
        if { let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.is_none() {
            *PREPARED_PARAGRAPH_FNS_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()) = Some(SortedTable::sorted_table_map_builder::<u32, String>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes((*a).to_ne_bytes()),
i32::from_ne_bytes((*b).to_ne_bytes())))).clone());
            { let __rhs_value2 = Some({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.unwrap().clone().build().clone()); *PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()) =
__rhs_value2; };
        }
        let cached = ({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().get(&(k));
        match &(cached) {
            Some(__option8) => {
                return Ok((*__option8).clone());
            }
            None => {
            }
        }
        let mut anchor = 0u32;
        let mut digits = "1".to_string();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u32::from_ne_bytes({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.as_ref().unwrap().size().to_ne_bytes())).to_ne_bytes())) {
            if i32::from_ne_bytes({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.as_ref().unwrap().key_at(i32::from_ne_bytes((i).to_ne_bytes())).to_ne_bytes()) < (i32::from_ne_bytes((k).to_ne_bytes())) &&
(i32::from_ne_bytes({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.as_ref().unwrap().key_at(i32::from_ne_bytes((i).to_ne_bytes())).to_ne_bytes())) > (i32::from_ne_bytes((anchor).to_ne_bytes())) {
                anchor = ({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes((i).to_ne_bytes()));
                digits = ({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().value_at(i32::from_ne_bytes((i).to_ne_bytes()));
            }
            i = u32::wrapping_add(i, 1);
        }
        for _ in anchor..k {
            digits = PreparedParagraphFns::prepared_paragraph_fns_times_small(digits.as_str(), 2)?;
        }
        ({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_mut().unwrap().put(&(k), &(digits).to_string());
        { let __rhs_value3 = Some({ let __guard = PREPARED_PARAGRAPH_FNS_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.unwrap().clone().build().clone()); *PREPARED_PARAGRAPH_FNS_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()) = __rhs_value3;
};
        return Ok(digits);
    }

    pub(crate) fn prepared_paragraph_fns_times_long(digits: &str, factor: u32) -> Result<String, UStringFault> {
        if factor == 0 {
            return Ok("0".to_string());
        }
        let mut result: Option<String> = None;
        let mut shift = 0u32;
        let mut remaining = factor;
        while (i32::from_ne_bytes((remaining).to_ne_bytes())) > (0) {
            let chunk = u32::from_ne_bytes((i32::from_ne_bytes((remaining).to_ne_bytes()) % 100000000i32).to_ne_bytes());
            remaining = remaining / (100000000);
            if chunk != 0 {
                let mut part = PreparedParagraphFns::prepared_paragraph_fns_times_small(digits, chunk)?;
                if i32::from_ne_bytes((shift).to_ne_bytes()) > (0) {
                    part += &(PreparedParagraphFns::prepared_paragraph_fns_zeros(shift));
                }
                result = Some(match &(result) { None => part.to_string(), Some(__option9) => PreparedParagraphFns::prepared_paragraph_fns_add_decimal(__option9.as_str(), part.as_str())?.to_string() }.clone());
            }
            shift = u32::wrapping_add(shift, 8);
        }
        return Ok(match &(result) { None => "0".to_string(), Some(__option10) => __option10.to_string() });
    }

    pub(crate) fn prepared_paragraph_fns_add_decimal(a: &str, b: &str) -> Result<String, UStringFault> {
    let __units5 = u_string::units(&b);
    let __units4 = u_string::units(&a);
    let __count5 = u_string::unit_count(&b);
    let __count4 = u_string::unit_count(&a);
        let mut out = Vec::<u16>::new();
        let mut i = i32::wrapping_sub(i32::from_ne_bytes((__count4).to_ne_bytes()), 1);
        let mut j = i32::wrapping_sub(i32::from_ne_bytes((__count5).to_ne_bytes()), 1);
        let mut carry = 0u32;
        while (i) >= 0 || (j) >= 0 || (i32::from_ne_bytes((carry).to_ne_bytes())) > (0) {
            let sum = u32::wrapping_add(u32::wrapping_add(if i >= 0 { u32::wrapping_sub(u_string::unit_at_from(&__units4, u32::from_ne_bytes((i).to_ne_bytes())).unwrap_or(0), 48) } else { 0 }, if j >= 0 { u32::wrapping_sub(u_string::unit_at_from(&__units5,
u32::from_ne_bytes((j).to_ne_bytes())).unwrap_or(0), 48) } else { 0 }), carry);
            if u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((sum).to_ne_bytes()) % 10i32).to_ne_bytes())) >= 56320 && u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((sum).to_ne_bytes()) % 10i32).to_ne_bytes())) <= 57343 {
                match out.last() {
                    Some(&last) if last >= 55296 && last <= 56319 => {}
                    _ => return Err(UStringFault::UnpairedSurrogate { unit: u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((sum).to_ne_bytes()) % 10i32).to_ne_bytes())) }),
                }
            } else if let Some(&last) = out.last() {
                if last >= 55296 && last <= 56319 {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                }
            }
            out.push(u16::try_from((u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((sum).to_ne_bytes()) % 10i32).to_ne_bytes()))) & 0xFFFF).unwrap_or(0));
            carry = sum / (10);
            i = i32::wrapping_sub(i, 1);
            j = i32::wrapping_sub(j, 1);
        }
        let text = String::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?;
        return Ok(PreparedParagraphFns::prepared_paragraph_fns_reverse(text.as_str()));
    }

    pub(crate) fn prepared_paragraph_fns_round_to_significant(exact: &str, length: u32) -> PreparedParagraphDigitsAndExponent {
        if i32::from_ne_bytes((length).to_ne_bytes()) >= i32::from_ne_bytes((u_string::unit_count(&(exact))).to_ne_bytes()) {
            return PreparedParagraphDigitsAndExponent { digits: exact.to_string(), exponent: 0 };
        }
        let keep = u_string::substr(&exact, 0i32, Some(i32::from_ne_bytes((length).to_ne_bytes())));
        let rem = u_string::substr(&exact, i32::from_ne_bytes((length).to_ne_bytes()), None);
        let mut up = false;
        if i32::from_ne_bytes((u_string::unit_at(&rem, 0u32).unwrap_or(0)).to_ne_bytes()) > (53) {
            up = true;
        } else {
            if u_string::unit_at(&rem, 0u32).as_ref().map_or(false, |v| v == &(53)) {
                let mut tail = 1u32;
                let __units6 = u_string::units(&rem);
                let __count6 = u_string::unit_count(&rem);
                while (i32::from_ne_bytes((tail).to_ne_bytes())) < (i32::from_ne_bytes((__count6).to_ne_bytes())) && u_string::unit_at_from(&__units6, tail).as_ref().map_or(false, |v| v == &(48)) {
                    tail = u32::wrapping_add(tail, 1);
                }
                up = i32::from_ne_bytes((tail).to_ne_bytes()) < (i32::from_ne_bytes((u_string::unit_count(&(rem))).to_ne_bytes())) || u32::from_ne_bytes((i32::from_ne_bytes((u32::wrapping_sub(u_string::unit_at(&keep, u32::wrapping_sub(u_string::unit_count(&(keep)),
1)).unwrap_or(0), 48)).to_ne_bytes()) % 2i32).to_ne_bytes()) != 0;
            }
        }
        let rounded = if up { PreparedParagraphFns::prepared_paragraph_fns_increment_decimal(keep.as_str()).to_string() } else { keep.to_string() };
        return PreparedParagraphDigitsAndExponent { digits: PreparedParagraphFns::prepared_paragraph_fns_trim_zeros(rounded.as_str()).clone(), exponent: u32::wrapping_sub(u_string::unit_count(&(rounded)), length) };
    }

    pub(crate) fn prepared_paragraph_fns_compare_decimal(a: &str, n_a: u32, b: &str, n_b: u32) -> u32 {
        let e_a = u32::wrapping_sub(n_a, u_string::unit_count(&(a)));
        let e_b = u32::wrapping_sub(n_b, u_string::unit_count(&(b)));
        let aa = if i32::from_ne_bytes((e_a).to_ne_bytes()) >= i32::from_ne_bytes((e_b).to_ne_bytes()) { format!("{}{}",
            a,
            PreparedParagraphFns::prepared_paragraph_fns_zeros(u32::wrapping_sub(e_a, e_b))
        ).to_string() } else { a.to_string() };
        let bb = if i32::from_ne_bytes((e_b).to_ne_bytes()) >= i32::from_ne_bytes((e_a).to_ne_bytes()) { format!("{}{}",
            b,
            PreparedParagraphFns::prepared_paragraph_fns_zeros(u32::wrapping_sub(e_b, e_a))
        ).to_string() } else { b.to_string() };
        if u_string::unit_count(&(aa)) != u_string::unit_count(&(bb)) {
            return u32::wrapping_sub(u_string::unit_count(&(aa)), u_string::unit_count(&(bb)));
        }
        return PreparedParagraphFns::prepared_paragraph_fns_compare_digit_strings(aa.as_str(), bb.as_str());
    }

    pub(crate) fn prepared_paragraph_fns_times_small(digits: &str, factor: u32) -> Result<String, UStringFault> {
    let __units7 = u_string::units(&digits);
    let __count7 = u_string::unit_count(&digits);
        let mut out = Vec::<u16>::new();
        let mut carry = 0u32;
        let mut i = __count7;
        let __units8 = u_string::units(&digits);
        let __count8 = u_string::unit_count(&digits);
        while (i) > (0) {
            let product = u32::wrapping_add(u32::wrapping_mul(u32::wrapping_sub(u_string::unit_at_from(&__units8, u32::wrapping_sub(i, 1)).unwrap_or(0), 48), factor), carry);
            if u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((product).to_ne_bytes()) % 10i32).to_ne_bytes())) >= 56320 && u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((product).to_ne_bytes()) % 10i32).to_ne_bytes())) <= 57343 {
                match out.last() {
                    Some(&last) if last >= 55296 && last <= 56319 => {}
                    _ => return Err(UStringFault::UnpairedSurrogate { unit: u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((product).to_ne_bytes()) % 10i32).to_ne_bytes())) }),
                }
            } else if let Some(&last) = out.last() {
                if last >= 55296 && last <= 56319 {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                }
            }
            out.push(u16::try_from((u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((product).to_ne_bytes()) % 10i32).to_ne_bytes()))) & 0xFFFF).unwrap_or(0));
            carry = product / (10);
            i = u32::wrapping_sub(i, 1);
        }
        while (i32::from_ne_bytes((carry).to_ne_bytes())) > (0) {
            if u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((carry).to_ne_bytes()) % 10i32).to_ne_bytes())) >= 56320 && u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((carry).to_ne_bytes()) % 10i32).to_ne_bytes())) <= 57343 {
                match out.last() {
                    Some(&last) if last >= 55296 && last <= 56319 => {}
                    _ => return Err(UStringFault::UnpairedSurrogate { unit: u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((carry).to_ne_bytes()) % 10i32).to_ne_bytes())) }),
                }
            } else if let Some(&last) = out.last() {
                if last >= 55296 && last <= 56319 {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                }
            }
            out.push(u16::try_from((u32::wrapping_add(48, u32::from_ne_bytes((i32::from_ne_bytes((carry).to_ne_bytes()) % 10i32).to_ne_bytes()))) & 0xFFFF).unwrap_or(0));
            carry = carry / (10);
        }
        let text = String::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?;
        return Ok(PreparedParagraphFns::prepared_paragraph_fns_reverse(text.as_str()));
    }

    pub(crate) fn prepared_paragraph_fns_increment_decimal(digits: &str) -> String {
        let mut a = u_string::split(&digits, &"");
        let mut i = u32::wrapping_sub(u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
        loop {
            if a[usize::try_from(i).unwrap_or(0)].clone() != "9" {
                { while a.len() <= usize::try_from(i).unwrap_or(0) { a.push(String::new()); } a[usize::try_from(i).unwrap_or(0)] = if u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1) > 0xFFFF { String::from_utf16(&[0xD800 +
(((u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) & 0x3FF) as
u16]).unwrap() } else { String::from_utf16_lossy(&[(u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) as u16]) }; };
                return { let joined3 = a; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(&("")); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } out };
            }
            { while a.len() <= usize::try_from(i).unwrap_or(0) { a.push(String::new()); } a[usize::try_from(i).unwrap_or(0)] = "0".to_string(); };
            if i == 0 {
                return format!("{}{}",
            "1",
            { let joined4 = a; let mut out = String::new(); let n = joined4.len(); let mut index4 = 0usize; while index4 < n { if index4 > 0 { out.push_str(&("")); } let _ = write!(out, "{}", joined4[index4]); index4 += 1; } out }
        );
            }
            i = u32::wrapping_sub(i, 1);
        }
    }

    pub(crate) fn prepared_paragraph_fns_decrement_decimal(digits: &str) -> String {
        let mut a = u_string::split(&digits, &"");
        let mut i = u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i) > (0) && (a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone() == "0" {
            { while a.len() <= usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0) { a.push(String::new()); } a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = "9".to_string(); };
            i = u32::wrapping_sub(i, 1);
        }
        if u32::wrapping_sub(i, 1) <= 2147483647 {
            { while a.len() <= usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0) { a.push(String::new()); } a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = if u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(),
0u32).unwrap_or(0), 1) > 0xFFFF { String::from_utf16(&[0xD800 + (((u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) >> 10) as u16, 0xDC00 +
(((u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) & 0x3FF) as u16]).unwrap() } else {
String::from_utf16_lossy(&[(u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) as u16]) }; };
        }
        let out = { let joined5 = a; let mut out = String::new(); let n = joined5.len(); let mut index5 = 0usize; while index5 < n { if index5 > 0 { out.push_str(&("")); } let _ = write!(out, "{}", joined5[index5]); index5 += 1; } out };
        let mut start = 0;
        while (start) < (i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(out))).to_ne_bytes()), 1)) && u_string::unit_at(&out, u32::from_ne_bytes((start).to_ne_bytes())).as_ref().map_or(false, |v| v == &(48)) {
            start = i32::wrapping_add(start, 1);
        }
        return u_string::substr(&out, i32::from_ne_bytes((start).to_ne_bytes()), None);
    }

    pub(crate) fn prepared_paragraph_fns_decompose(v: f64) -> Result<PreparedParagraphDecomposed, UStringFault> {
        let f = PreparedParagraphFns::prepared_paragraph_fns_decompose_f32(v);
        let mut mantissa = crate::runtime::int_text::IntText::int_text(f.mant24);
        let mut exponent = f.exp2;
        if i32::from_ne_bytes((f.mant24).to_ne_bytes()) >= 8388608 {
            mantissa = PreparedParagraphFns::prepared_paragraph_fns_times_long(mantissa.as_str(), 536870912)?;
            exponent = u32::wrapping_sub(exponent, 29);
        }
        while (i32::from_ne_bytes((u_string::unit_count(&(mantissa))).to_ne_bytes())) < (16) || u_string::unit_count(&(mantissa)) == 16 && (PreparedParagraphFns::prepared_paragraph_fns_compare_digit_strings(mantissa.as_str(), &"4503599627370496")) > 2147483647 {
            mantissa = PreparedParagraphFns::prepared_paragraph_fns_times_small(mantissa.as_str(), 2)?;
            exponent = u32::wrapping_sub(exponent, 1);
        }
        return Ok(PreparedParagraphDecomposed { mantissa: mantissa.clone(), exp2: exponent });
    }

    pub(crate) fn prepared_paragraph_fns_decompose_f32(v: f64) -> PreparedParagraphF32Decomposed {
        let bits = FPHelper::float_to_i32(v);
        let raw_exponent = bits >> 23 & 255;
        let raw_mantissa = bits & 8388607;
        if u32::from_ne_bytes((raw_exponent).to_ne_bytes()) == 0 {
            return PreparedParagraphF32Decomposed { mant24: u32::from_ne_bytes((raw_mantissa).to_ne_bytes()), exp2: 4294967147u32 };
        }
        return PreparedParagraphF32Decomposed { mant24: u32::from_ne_bytes((raw_mantissa).to_ne_bytes()) | 8388608, exp2: u32::from_ne_bytes((i32::wrapping_sub(raw_exponent, 150)).to_ne_bytes()) };
    }

    pub(crate) fn prepared_paragraph_fns_compare_digit_strings(a: &str, b: &str) -> u32 {
    let __units9 = u_string::units(&a);
    let __units10 = u_string::units(&b);
    let __count9 = u_string::unit_count(&a);
    let __count10 = u_string::unit_count(&b);
        if __count9 != __count10 {
            return u32::wrapping_sub(__count9, __count10);
        }
        let mut i = 0u32;
        let __units11 = u_string::units(&a);
        let __units12 = u_string::units(&b);
        let __count11 = u_string::unit_count(&a);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count9).to_ne_bytes())) {
            let da = u_string::unit_at_from(&__units11, i).unwrap_or(0);
            let db = u_string::unit_at_from(&__units12, i).unwrap_or(0);
            if da != db {
                return u32::wrapping_sub(da, db);
            }
            i = u32::wrapping_add(i, 1);
        }
        return 0;
    }

    pub(crate) fn prepared_paragraph_fns_zeros(n: u32) -> String {
        let mut s = String::new();
        for _ in 0..n {
            s += &("0");
        }
        return s;
    }

    pub(crate) fn prepared_paragraph_fns_trim_zeros(s: &str) -> String {
    let __units13 = u_string::units(&s);
    let __count12 = u_string::unit_count(&s);
        let mut e = __count12;
        while (i32::from_ne_bytes((e).to_ne_bytes())) > (1) && u_string::unit_at_from(&__units13, u32::wrapping_sub(e, 1)).as_ref().map_or(false, |v| v == &(48)) {
            e = u32::wrapping_sub(e, 1);
        }
        return u_string::substr(&s, 0i32, Some(i32::from_ne_bytes((e).to_ne_bytes())));
    }

    pub(crate) fn prepared_paragraph_fns_reverse(s: &str) -> String {
    let __units14 = u_string::units(&s);
    let __count13 = u_string::unit_count(&s);
        let mut r = String::new();
        let mut i = __count13;
        let __units15 = u_string::units(&s);
        let __count14 = u_string::unit_count(&s);
        while (i) > (0) {
            r += &(u_string::char_at_from(&__units15, u32::wrapping_sub(i, 1)));
            i = u32::wrapping_sub(i, 1);
        }
        return r;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedParagraphDigitsAndExponent {
    pub digits: String,
    pub exponent: u32,
}

pub fn compare_prepared_paragraph_digits_and_exponent(a: &PreparedParagraphDigitsAndExponent, b: &PreparedParagraphDigitsAndExponent) -> i32 {
    let cmp_digits = SortedTable::sorted_table_compare_strings(a.digits.as_str(), b.digits.as_str());
    if cmp_digits != 0 { return cmp_digits; }
    let cmp_exponent = if a.exponent < b.exponent { -1 } else if a.exponent > b.exponent { 1 } else { 0 };
    if cmp_exponent != 0 { return cmp_exponent; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedParagraphDecomposed {
    pub mantissa: String,
    pub exp2: u32,
}

pub fn compare_prepared_paragraph_decomposed(a: &PreparedParagraphDecomposed, b: &PreparedParagraphDecomposed) -> i32 {
    let cmp_mantissa = SortedTable::sorted_table_compare_strings(a.mantissa.as_str(), b.mantissa.as_str());
    if cmp_mantissa != 0 { return cmp_mantissa; }
    let cmp_exp2 = if a.exp2 < b.exp2 { -1 } else if a.exp2 > b.exp2 { 1 } else { 0 };
    if cmp_exp2 != 0 { return cmp_exp2; }
    0
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreparedParagraphF32Decomposed {
    pub mant24: u32,
    pub exp2: u32,
}

pub fn compare_prepared_paragraph_f32_decomposed(a: &PreparedParagraphF32Decomposed, b: &PreparedParagraphF32Decomposed) -> i32 {
    let cmp_mant24 = if a.mant24 < b.mant24 { -1 } else if a.mant24 > b.mant24 { 1 } else { 0 };
    if cmp_mant24 != 0 { return cmp_mant24; }
    let cmp_exp2 = if a.exp2 < b.exp2 { -1 } else if a.exp2 > b.exp2 { 1 } else { 0 };
    if cmp_exp2 != 0 { return cmp_exp2; }
    0
}
