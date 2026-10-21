use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::layout_queries::LayoutQueries;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::punctuation_decision_info::PunctuationDecisionInfo;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::protocol::plan::Plan;
use crate::org::tiqian::protocol::plan::PlanBopomofo;
use crate::org::tiqian::protocol::plan::PlanBopomofoPlacement;
use crate::org::tiqian::protocol::plan::PlanCell;
use crate::org::tiqian::protocol::plan::PlanDecorationSegment;
use crate::org::tiqian::protocol::plan::PlanEmphasisDot;
use crate::org::tiqian::protocol::plan::PlanEmphasisRange;
use crate::org::tiqian::protocol::plan::PlanInlineEdge;
use crate::org::tiqian::protocol::plan::PlanLine;
use crate::org::tiqian::protocol::plan::PlanRuby;
use crate::org::tiqian::protocol::plan_end_reason::PlanEndReason;
use crate::org::tiqian::protocol::plan_style_delta::PlanStyleDelta;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct PlanLowering;

impl PlanLowering {
    pub(crate) fn plan_lowering_f32(v: f64) -> f64 {
        return v;
    }

    pub(crate) fn plan_lowering_range_key(r: TextRange) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(r.start)).as_str()).as_ustr(); __s += &(UString::from(":")); __s += UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(r.end)).as_str()).as_ustr(); __s }).as_str());
    }

    pub(crate) fn plan_lowering_style_at(result: LayoutResult, offset: u32) -> TextStyle {
        let span_src = (((result.input).clone().content).clone().spans).clone();
        for span in &span_src {
            if i32::from_ne_bytes(((offset) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((span.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((offset) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((span.range).clone().end) as i32).to_ne_bytes())) {
                return ((span.style).clone()).clone();
            }
        }
        return (((result.input).clone().text_style).clone()).clone();
    }

    pub fn plan_lowering_to_plan(result: LayoutResult, render_evidence: bool) -> Result<Plan, TextRangeError> {
        let mut natural_b: SortedMapTableBuilder<UString, f64> = SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let mut features_b: SortedMapTableBuilder<UString, Vec<UString>> = SortedTable::sorted_table_map_builder::<UString, Vec<UString>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let mut fonts_b: SortedMapTableBuilder<UString, UString> = SortedTable::sorted_table_map_builder::<UString, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let mut glyph_ids_b: SortedMapTableBuilder<UString, Vec<UString>> = SortedTable::sorted_table_map_builder::<UString, Vec<UString>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        for ri in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let run = (result.glyph_runs[usize::try_from(ri).unwrap_or(0)]).clone();
            for gi in 0..match u32::try_from(run.glyphs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let glyph = (run.glyphs[usize::try_from(gi).unwrap_or(0)]).clone();
                let k = PlanLowering::plan_lowering_range_key((glyph.cluster_range).clone());
                let prior_natural = natural_b.get(&(k).to_ustring());
                let prior_n = (prior_natural).unwrap_or(0.0f64);
                natural_b.put(&(k).to_ustring(), &(prior_n + glyph.advance));
                if i32::from_ne_bytes(((u32::try_from((run.open_type_features.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                    let mut fs = features_b.get(&(k).to_ustring());
                    if fs.is_none() {
                        fs = Some(vec![].clone());
                    }
                    for fi in 0..match u32::try_from(run.open_type_features.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let f = (run.open_type_features[usize::try_from(fi).unwrap_or(0)]).clone();
                        if u32::from_ne_bytes(((match (fs).as_ref().unwrap().iter().position(|e| e == &f) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) > 2147483647 {
                            (fs).as_mut().unwrap().push(f.clone());
                        }
                    }
                    features_b.put(&(k).to_ustring(), (fs).as_ref().unwrap());
                }
                let rfk = glyph.render_font_key.clone();
                match &(rfk) {
                    Some(__option) => {
                        fonts_b.put(&(k).to_ustring(), &__option);
                    }
                    None => {
                    }
                }
                let mut ids = glyph_ids_b.get(&(k).to_ustring());
                if ids.is_none() {
                    ids = Some(vec![].clone());
                }
                (ids).as_mut().unwrap().push(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(glyph.id)).as_str()));
                glyph_ids_b.put(&(k).to_ustring(), (ids).as_ref().unwrap());
            }
        }
        let mut zero_b: SortedMapTableBuilder<UString, bool> = SortedTable::sorted_table_map_builder::<UString, bool>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let zero_src = ((result.debug).clone().zero_width_break_decisions).clone();
        for zi in 0..match u32::try_from(zero_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            zero_b.put(&PlanLowering::plan_lowering_range_key(((zero_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &(true));
        }
        let mut shaping_b: SortedMapTableBuilder<UString, ShapingDecisionInfo> = SortedTable::sorted_table_map_builder::<UString, ShapingDecisionInfo>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let shaping_src = ((result.debug).clone().shaping_decisions).clone();
        for zi in 0..match u32::try_from(shaping_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            shaping_b.put(&PlanLowering::plan_lowering_range_key(((shaping_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &((shaping_src[usize::try_from(zi).unwrap_or(0)]).clone()));
        }
        let mut punct_b: SortedMapTableBuilder<UString, PunctuationDecisionInfo> = SortedTable::sorted_table_map_builder::<UString, PunctuationDecisionInfo>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let punct_src = ((result.debug).clone().punctuation_decisions).clone();
        for zi in 0..match u32::try_from(punct_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            punct_b.put(&PlanLowering::plan_lowering_range_key(((punct_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &((punct_src[usize::try_from(zi).unwrap_or(0)]).clone()));
        }
        let mut inline_advance_b: SortedMapTableBuilder<UString, f64> = SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let inline_obj_src = ((result.input).clone().inline_objects).clone();
        for zi in 0..match u32::try_from(inline_obj_src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            inline_advance_b.put(&PlanLowering::plan_lowering_range_key(((inline_obj_src[usize::try_from(zi).unwrap_or(0)]).clone().range).clone()), &(inline_obj_src[usize::try_from(zi).unwrap_or(0)].advance));
        }
        let mut edge_start_b: SortedMapTableBuilder<UString, f64> = SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let mut edge_end_b: SortedMapTableBuilder<UString, f64> = SortedTable::sorted_table_map_builder::<UString, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        let box_src = ((result.input).clone().inline_boxes).clone();
        for b in &box_src {
            if b.inline_start != 0 as f64 {
                let sk = UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((b.range).clone().start)).as_str());
                let prior_start = edge_start_b.get(&(sk).to_ustring());
                let p_start = (prior_start).unwrap_or(0.0f64);
                edge_start_b.put(&(sk).to_ustring(), &(p_start + b.inline_start));
            }
            if b.inline_end != 0 as f64 {
                let ek = UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((b.range).clone().end)).as_str());
                let prior_end = edge_end_b.get(&(ek).to_ustring());
                let p_end = (prior_end).unwrap_or(0.0f64);
                edge_end_b.put(&(ek).to_ustring(), &(p_end + b.inline_end));
            }
        }
        let natural: SortedMapTable<UString, f64> = natural_b.clone().build();
        let features: SortedMapTable<UString, Vec<UString>> = features_b.clone().build();
        let fonts: SortedMapTable<UString, UString> = fonts_b.clone().build();
        let glyph_ids: SortedMapTable<UString, Vec<UString>> = glyph_ids_b.clone().build();
        let zero: SortedMapTable<UString, bool> = zero_b.clone().build();
        let shaping: SortedMapTable<UString, ShapingDecisionInfo> = shaping_b.clone().build();
        let punct: SortedMapTable<UString, PunctuationDecisionInfo> = punct_b.clone().build();
        let inline_advance: SortedMapTable<UString, f64> = inline_advance_b.clone().build();
        let edge_start: SortedMapTable<UString, f64> = edge_start_b.clone().build();
        let edge_end: SortedMapTable<UString, f64> = edge_end_b.clone().build();
        let mut lines: Vec<PlanLine> = vec![];
        for li in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (result.lines[usize::try_from(li).unwrap_or(0)]).clone();
            let mut cells: Vec<PlanCell> = vec![];
            {
                let _g1 = LayoutQueries::layout_queries_positioned_clusters_for_line((result).clone(), (line).clone())?;
                for p in &_g1 {
                    let c = (result.clusters[usize::try_from(p.cluster_index).unwrap_or(0)]).clone();
                    let ck = PlanLowering::plan_lowering_range_key((c.range).clone());
                    if !((i32::from_ne_bytes(((u_string::unit_count(&((c.display_text).to_ustring()))) as i32).to_ne_bytes())) > (0) || zero.has(&(ck).to_ustring()) || render_evidence && inline_advance.has(&(ck).to_ustring())) {
                        continue;
                    }
                    let nat_w = if natural.has(&(ck).to_ustring()) { (natural.get(&(ck).to_ustring())).unwrap() } else { c.advance };
                    let mut inline_obj: Option<f64> = None;
                    let mut adv_override: Option<f64> = None;
                    let mut render_fam: Option<UString> = None;
                    let mut dash: Option<UString> = None;
                    let mut lang: Option<UString> = None;
                    let mut face: Option<UString> = None;
                    let mut g_ids: Option<UString> = None;
                    let mut ev: Option<UString> = None;
                    let mut ink_floor: Option<f64> = None;
                    let mut body_w: Option<f64> = None;
                    let mut latin = false;
                    let mut style_delta: Option<PlanStyleDelta> = None;
                    if render_evidence {
                        inline_obj = if inline_advance.has(&(ck).to_ustring()) { inline_advance.get(&(ck).to_ustring()) } else { None };
                        let glyph_w = match &(inline_obj) { None => nat_w, Some(__option1) => *__option1 };
                        adv_override = if c.advance != glyph_w { Some(c.advance) } else { None };
                        render_fam = if fonts.has(&(ck).to_ustring()) { fonts.get(&(ck).to_ustring()) } else { None };
                        let sd = shaping.get(&(ck).to_ustring());
                        dash = match &(sd) { Some(__option2) => (__option2.strategy).clone(), None => None };
                        lang = match &(sd) { Some(__option3) => (__option3.language).clone(), None => None };
                        face = match &(sd) { Some(__option4) => (__option4.resolved_face).clone(), None => None };
                        g_ids = if glyph_ids.has(&(ck).to_ustring()) && (u32::try_from(((glyph_ids.get(&(ck).to_ustring())).as_ref().map_or(0, |v| v.len())) & 0xFFFF_FFFF).unwrap_or(0)) > (0) { Some(UString::from(format!("{}", { let _jtmp = glyph_ids.get(&(ck).to_ustring()); let joined = _jtmp.as_ref().unwrap(); let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(","); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str())) } else { None };
                        ev = match &(sd) { Some(__option5) => (Some((__option5.reason).to_ustring())).clone(), None => None };
                        let pd = punct.get(&(ck).to_ustring());
                        if match &(pd) { Some(__option6) => __option6.ink_containment_applied && __option6.ink_containment_body_floor.is_some(), None => false } {
                            ink_floor = (pd).as_ref().unwrap().ink_containment_body_floor;
                            body_w = Some((pd).as_ref().unwrap().body_width);
                        }
                        let fd_src = ((result.debug).clone().font_decisions).clone();
                        for fd in &fd_src {
                            if i32::from_ne_bytes((((c.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((fd.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes((((c.range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((fd.range).clone().end) as i32).to_ne_bytes()) && (fd.role).to_ustring() == UString::from(FontRole::LatinText.name()) {
                                latin = true;
                            }
                        }
                        let cs = PlanLowering::plan_lowering_style_at((result).clone(), (c.range).clone().start);
                        style_delta = if cs != ((result.input).clone().text_style).clone() { Some(PlanStyleDelta { font_size: if cs.font_size != ((result.input).clone().text_style).clone().font_size { Some(cs.font_size) } else { None }, font_weight: if cs.font_weight != ((result.input).clone().text_style).clone().font_weight { Some(cs.font_weight) } else { None }, italic: if cs.italic != ((result.input).clone().text_style).clone().italic { Some(cs.italic) } else { None } }) } else { None };
                    }
                    let feats = features.get(&(ck).to_ustring());
                    cells.push(PlanCell { range_start: (c.range).clone().start, range_end: (c.range).clone().end, source: (c.text).to_ustring().clone(), display: (c.display_text).to_ustring().clone(), draw_x: p.draw_x, natural_width: nat_w, leading_layout_advance: c.leading_layout_advance, shaping_boundary: (i32::from_ne_bytes(((u32::wrapping_sub((c.range).clone().end, (c.range).clone().start)) as i32).to_ne_bytes())) > (1), open_type_features: match &(feats) { Some(__option8) => (*__option8).clone(), None => vec![] }, render_font_family: render_fam.clone(), dash_strategy: dash.clone(), shaping_language: lang.clone(), resolved_face: face.clone(), glyph_ids: g_ids.clone(), shaping_evidence: ev.clone(), punctuation_ink_floor: ink_floor, punctuation_body_width: body_w, latin: latin, advance: adv_override, inline_object: inline_obj, style_delta: style_delta });
                }
            }
            lines.push(PlanLine { range_start: (line.range).clone().start, range_end: (line.range).clone().end, top: line.top, bottom: line.bottom, baseline: line.baseline, indent: line.indent, visual_width: line.visual_width, hyphen_advance: line.hyphen_advance, end_reason: PlanLowering::plan_lowering_end_reason_from(line.end_reason), cells: cells });
        }
        let mut emphasis_ranges: Vec<PlanEmphasisRange> = vec![];
        let mut inline_edges: Vec<PlanInlineEdge> = vec![];
        let mut ruby_decisions: Vec<PlanRuby> = vec![];
        let mut bopomofo_decisions: Vec<PlanBopomofo> = vec![];
        let mut decoration_segments: Vec<PlanDecorationSegment> = vec![];
        let mut emphasis_dots: Vec<PlanEmphasisDot> = vec![];
        let mut font_size: Option<f64> = None;
        let mut overlay_width: Option<f64> = None;
        if render_evidence {
            font_size = Some(((result.input).clone().text_style).clone().font_size);
            overlay_width = Some((result.size).clone().width);
            let emph_src = ((result.input).clone().decorations).clone();
            for d in &emph_src {
                if d.kind == DecorationKind::Emphasis {
                    emphasis_ranges.push(PlanEmphasisRange { start: (d.range).clone().start, end: (d.range).clone().end });
                }
            }
            let mut offsets: Vec<u32> = vec![];
            for oi in 0..u32::from_ne_bytes(((edge_start.size()) as u32).to_ne_bytes()) {
                offsets.push(u32::from_ne_bytes(((u_string::parse_i32(&(edge_start.key_at({ let v: u32 = oi; i32::from_ne_bytes(v.to_ne_bytes()) }))).unwrap()) as u32).to_ne_bytes()));
            }
            for ei in 0..u32::from_ne_bytes(((edge_end.size()) as u32).to_ne_bytes()) {
                let ev = u_string::parse_i32(&(edge_end.key_at({ let v: u32 = ei; i32::from_ne_bytes(v.to_ne_bytes()) })));
                if u32::from_ne_bytes(((match offsets.iter().position(|e| e == &u32::from_ne_bytes(((ev.unwrap_or(-1)) as u32).to_ne_bytes())) { Some(v) => i32::from_ne_bytes(u32::try_from(v).unwrap_or(0).to_ne_bytes()), None => -1 }) as u32).to_ne_bytes()) > 2147483647 {
                    offsets.push(u32::from_ne_bytes(((ev.unwrap()) as u32).to_ne_bytes()));
                }
            }
            let mut oi = 1u32;
            while (i32::from_ne_bytes(((oi) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((offsets.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                let ov = offsets[usize::try_from(oi).unwrap_or(0)];
                let mut oj = oi;
                while (oj) > (0) && ({ let v: u32 = offsets[usize::try_from(u32::wrapping_sub(oj, 1)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = ov; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                    { while offsets.len() <= usize::try_from(oj).unwrap_or(0) { offsets.push(0); } offsets[usize::try_from(oj).unwrap_or(0)] = offsets[usize::try_from(u32::wrapping_sub(oj, 1)).unwrap_or(0)]; };
                    oj = u32::wrapping_sub(oj, 1);
                }
                { while offsets.len() <= usize::try_from(oj).unwrap_or(0) { offsets.push(0); } offsets[usize::try_from(oj).unwrap_or(0)] = ov; };
                oi = u32::wrapping_add(oi, 1);
            }
            for &offset in &offsets {
                let sk = UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(offset)).as_str());
                inline_edges.push(PlanInlineEdge { offset: offset, inline_start: if edge_start.has(&(sk).to_ustring()) { edge_start.get(&(sk).to_ustring()) } else { None }, inline_end: if edge_end.has(&(sk).to_ustring()) { edge_end.get(&(sk).to_ustring()) } else { None } });
            }
            let ruby_src = ((result.debug).clone().ruby_decisions).clone();
            for r in &ruby_src {
                ruby_decisions.push(PlanRuby { base_range_start: (r.base_range).clone().start, base_range_end: (r.base_range).clone().end, text: (r.text).to_ustring().clone(), center_x: r.center_x, baseline_y: r.baseline_y, font_size: r.font_size, font_weight: r.font_weight, font_families: PlanLowering::plan_lowering_to_array(&(r.font_families).clone()), ascent: Some(r.ascent) });
            }
            let bopo_src = ((result.debug).clone().bopomofo_decisions).clone();
            for z in &bopo_src {
                let mut placements: Vec<PlanBopomofoPlacement> = vec![];
                for pj in 0..match u32::try_from((z.placements).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let pl = (z.placements[usize::try_from(pj).unwrap_or(0)]).clone();
                    placements.push(PlanBopomofoPlacement { text: (pl.text).to_ustring().clone(), role: UString::from(pl.role.name()).clone(), left: pl.left, top: pl.top, width: pl.width, height: pl.height });
                }
                bopomofo_decisions.push(PlanBopomofo { base_range_start: (z.base_range).clone().start, base_range_end: (z.base_range).clone().end, text: (z.text).to_ustring().clone(), font_weight: z.font_weight, font_families: PlanLowering::plan_lowering_to_array(&(z.font_families).clone()), placements: placements });
            }
            let seg_src = ((result.debug).clone().decoration_segments).clone();
            for s in &seg_src {
                if s.kind.to_ustring() == UString::from(DecorationKind::ProperNoun.name()) || (s.kind).to_ustring() == UString::from(DecorationKind::BookTitle.name()) {
                    decoration_segments.push(PlanDecorationSegment { kind: (s.kind).to_ustring().clone(), left: s.left, top: s.top, right: s.right, source_range_start: (s.source_range).clone().start, source_range_end: (s.source_range).clone().end });
                }
            }
            let dot_src = ((result.debug).clone().decoration_decisions).clone();
            for d in &dot_src {
                if d.applied && (d.kind).to_ustring() == UString::from(DecorationKind::Emphasis.name()) && (d.dot_diameter) > (0 as f64) {
                    emphasis_dots.push(PlanEmphasisDot { cluster_range_start: Some(PlanLowering::plan_lowering_f32(i32::from_ne_bytes((((d.cluster_range).clone().start) as i32).to_ne_bytes()) as f64)), anchor_x: d.anchor_x, anchor_y: d.anchor_y, dot_diameter: d.dot_diameter });
                }
            }
        }
        return Ok(Plan { width: ((result.input).clone().constraints).clone().max_width, height: (result.size).clone().height, lines: lines, emphasis_ranges: emphasis_ranges, inline_edges: inline_edges, ruby_decisions: ruby_decisions, bopomofo_decisions: bopomofo_decisions, font_size: font_size, overlay_width: overlay_width, decoration_segments: decoration_segments, emphasis_dots: emphasis_dots });
    }

    pub(crate) fn plan_lowering_end_reason_from(reason: LineEndReason) -> PlanEndReason {
        return match reason {
            LineEndReason::AutoWrap => PlanEndReason::AutoWrap,
            LineEndReason::MandatoryBreak => PlanEndReason::MandatoryBreak,
            LineEndReason::ParagraphEnd => PlanEndReason::ParagraphEnd,
        };
    }

    pub(crate) fn plan_lowering_to_array(src: &[UString]) -> Vec<UString> {
        let mut out: Vec<UString> = vec![];
        for i in 0..match u32::try_from(src.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            out.push((src[usize::try_from(i).unwrap_or(0)]).clone());
        }
        return out;
    }
}
