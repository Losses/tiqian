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
use crate::runtime::bytes_buffer::BytesBuffer;
use crate::runtime::fp_helper::FPHelper;
use std::sync::Arc;
use std::sync::Mutex;


pub struct PlanPackedWriter {
    pub(crate) buf: BytesBuffer,
}

impl PlanPackedWriter {
    pub fn new() -> Self {
        Self {
            buf: BytesBuffer::new(),
        }
    }

    pub fn u8(&mut self, v: u32) {
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
    }

    pub fn u16(&mut self, v: u32) {
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[2])) & 0xFF).unwrap_or(0));
    }

    pub fn u32(&mut self, v: u32) {
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[2])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[1])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(v.to_be_bytes()[0])) & 0xFF).unwrap_or(0));
    }

    pub fn f64(&mut self, v: f64) {
        let bits = FPHelper::double_to_i64(v);
        self.u32(bits.low);
        self.u32(bits.high);
    }

    pub fn raw(&mut self, bytes: &[u8]) {
        for i in 0..match u32::try_from(bytes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            self.buf.add_byte(u8::try_from((u32::from(bytes[usize::try_from(i).unwrap_or(0)])) & 0xFF).unwrap_or(0));
        }
    }

    pub fn finish(&mut self) -> Vec<u8> {
        return self.buf.get_bytes();
    }
}

pub struct PlanPackedReader<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) pos: u32,
    pub failed: bool,
}

impl<'a> PlanPackedReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            pos: 0,
            failed: false,
        }
    }

    pub fn u8(&mut self) -> u32 {
        if !self.need(1) {
            return 0;
        }
        let v = u32::from(self.bytes[usize::try_from(self.pos).unwrap_or(0)]);
        self.pos += 1;
        return v;
    }

    pub fn u32(&mut self) -> u32 {
        if !self.need(4) {
            return 0;
        }
        let v = u32::wrapping_add(u32::from(self.bytes[usize::try_from(self.pos).unwrap_or(0)]) | (u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, 1)).unwrap_or(0)])) << (8) | (u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, 2)).unwrap_or(0)]))
<< (16), u32::wrapping_mul(u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, 3)).unwrap_or(0)]), 16777216));
        self.pos += 4;
        return v;
    }

    pub fn f64(&mut self) -> f64 {
        let low = self.u32();
        let high = self.u32();
        if self.failed {
            return 0.0f64;
        }
        return FPHelper::i64_to_double(low, high);
    }

    pub fn string(&mut self, len: u32) -> String {
        if !self.need(len) {
            return String::new();
        }
        let s = String::from_utf8_lossy(&self.bytes[usize::try_from(self.pos).unwrap_or(0)..usize::try_from(self.pos + len).unwrap_or(0)]).into_owned();
        self.pos += len;
        return s;
    }

    fn need(&mut self, length: u32) -> bool {
        if self.failed {
            return false;
        }
        if length > 2147483647 || (i32::from_ne_bytes((u32::wrapping_add(self.pos, length)).to_ne_bytes())) > (i32::from_ne_bytes((u32::try_from((self.bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            self.failed = true;
            return false;
        }
        return true;
    }
}

#[derive(Clone, PartialEq)]
pub struct StringPool {
    pub ordered: Vec<String>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            ordered: vec![],
        }
    }

    pub fn intern(&mut self, value: &str) -> u32 {
        for i in 0..match u32::try_from(self.ordered.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if self.ordered[usize::try_from(i).unwrap_or(0)].clone() == value {
                return i;
            }
        }
        let idx = u32::try_from((self.ordered.len()) & 0xFFFF_FFFF).unwrap_or(0);
        self.ordered.push(value.to_string());
        return idx;
    }

    pub fn index_of(&self, value: &str) -> u32 {
        for i in 0..match u32::try_from((self.ordered).clone().len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if self.ordered[usize::try_from(i).unwrap_or(0)].clone() == value {
                return i;
            }
        }
        return 4294967295u32;
    }
}

#[derive(Clone, Copy)]
pub struct PlanPacked;

impl PlanPacked {
    pub fn plan_packed_encode(plan: Plan) -> Vec<u8> {
        let mut pool = StringPool::new();
        let mut writer = PlanPackedWriter::new();
        let mut cell_range_start: Vec<u32> = vec![];
        let mut cell_range_end: Vec<u32> = vec![];
        let mut cell_source_ref: Vec<u32> = vec![];
        let mut cell_display_ref: Vec<u32> = vec![];
        let mut cell_draw_x: Vec<f64> = vec![];
        let mut cell_natural_width: Vec<f64> = vec![];
        let mut cell_leading_advance: Vec<f64> = vec![];
        let mut cell_shaping_boundary: Vec<u32> = vec![];
        let mut cell_latin: Vec<u32> = vec![];
        let mut cell_render_family_ref: Vec<u32> = vec![];
        let mut cell_dash_ref: Vec<u32> = vec![];
        let mut cell_language_ref: Vec<u32> = vec![];
        let mut cell_resolved_face_ref: Vec<u32> = vec![];
        let mut cell_glyph_ids_ref: Vec<u32> = vec![];
        let mut cell_evidence_ref: Vec<u32> = vec![];
        let mut cell_ink_floor: Vec<f64> = vec![];
        let mut cell_body_width: Vec<f64> = vec![];
        let mut cell_advance: Vec<f64> = vec![];
        let mut cell_inline_object: Vec<f64> = vec![];
        let mut cell_style_font_size: Vec<f64> = vec![];
        let mut cell_style_font_weight: Vec<f64> = vec![];
        let mut cell_style_italic: Vec<u32> = vec![];
        let mut cell_feature_offset: Vec<u32> = vec![];
        let mut cell_feature_count: Vec<u32> = vec![];
        let mut feature_pool: Vec<u32> = vec![];
        let mut cell_count = 0u32;
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                cell_count = u32::wrapping_add(cell_count, u32::try_from(((line.cells).clone().len()) & 0xFFFF_FFFF).unwrap_or(0));
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = (line.cells).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let cell = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        cell_range_start.push(cell.range_start);
                        cell_range_end.push(cell.range_end);
                        cell_source_ref.push(pool.intern((cell.source).to_string().as_str()));
                        cell_display_ref.push(pool.intern((cell.display).to_string().as_str()));
                        cell_draw_x.push(cell.draw_x);
                        cell_natural_width.push(cell.natural_width);
                        cell_leading_advance.push(cell.leading_layout_advance);
                        cell_shaping_boundary.push(if cell.shaping_boundary { 1 } else { 0 });
                        cell_latin.push(if cell.latin { 1 } else { 0 });
                        cell_render_family_ref.push(match &(cell.render_font_family) { Some(__option1) => pool.intern((*__option1).clone().as_str()), None => 4294967295u32 });
                        cell_dash_ref.push(match &(cell.dash_strategy) { Some(__option4) => pool.intern((*__option4).clone().as_str()), None => 4294967295u32 });
                        cell_language_ref.push(match &(cell.shaping_language) { Some(__option7) => pool.intern((*__option7).clone().as_str()), None => 4294967295u32 });
                        cell_resolved_face_ref.push(match &(cell.resolved_face) { Some(__option10) => pool.intern((*__option10).clone().as_str()), None => 4294967295u32 });
                        cell_glyph_ids_ref.push(match &(cell.glyph_ids) { Some(__option13) => pool.intern((*__option13).clone().as_str()), None => 4294967295u32 });
                        cell_evidence_ref.push(match &(cell.shaping_evidence) { Some(__option16) => pool.intern((*__option16).clone().as_str()), None => 4294967295u32 });
                        cell_ink_floor.push(match &(cell.punctuation_ink_floor) { Some(__option19) => *__option19, None => f64::NAN });
                        cell_body_width.push(match &(cell.punctuation_body_width) { Some(__option21) => *__option21, None => f64::NAN });
                        cell_advance.push(match &(cell.advance) { Some(__option23) => *__option23, None => f64::NAN });
                        cell_inline_object.push(match &(cell.inline_object) { Some(__option25) => *__option25, None => f64::NAN });
                        let style = cell.style_delta;
                        let mut style_font_size = f64::NAN;
                        let mut style_font_weight = f64::NAN;
                        let mut style_italic = 2u32;
                        match &(style) {
                            Some(__option26) => {
                                let fs = __option26.font_size;
                                match &(fs) {
                                    Some(__option27) => {
                                        style_font_size = *__option27;
                                    }
                                    None => {
                                    }
                                }
                                let fw = __option26.font_weight;
                                match &(fw) {
                                    Some(__option28) => {
                                        style_font_weight = *__option28;
                                    }
                                    None => {
                                    }
                                }
                                let it = __option26.italic;
                                match &(it) {
                                    Some(__option29) => {
                                        style_italic = if *__option29.unwrap_or(false) { 1 } else { 0 };
                                    }
                                    None => {
                                    }
                                }
                            }
                            None => {
                            }
                        }
                        cell_style_font_size.push(style_font_size);
                        cell_style_font_weight.push(style_font_weight);
                        cell_style_italic.push(style_italic);
                        let feats = cell.open_type_features.clone();
                        {
                            let mut _g = 0u32;
                            while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((feats.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                let f = (feats[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                pool.intern(f.as_str());
                            }
                        }
                        cell_feature_offset.push(u32::try_from((feature_pool.len()) & 0xFFFF_FFFF).unwrap_or(0));
                        cell_feature_count.push(u32::try_from((feats.len()) & 0xFFFF_FFFF).unwrap_or(0));
                        {
                            let mut _g = 0u32;
                            while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((feats.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                let f = (feats[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                feature_pool.push(pool.index_of(f.as_str()));
                            }
                        }
                    }
                }
            }
        }
        let mut ruby_base_start: Vec<u32> = vec![];
        let mut ruby_base_end: Vec<u32> = vec![];
        let mut ruby_text_ref: Vec<u32> = vec![];
        let mut ruby_center_x: Vec<f64> = vec![];
        let mut ruby_baseline_y: Vec<f64> = vec![];
        let mut ruby_font_size: Vec<f64> = vec![];
        let mut ruby_font_weight: Vec<u32> = vec![];
        let mut ruby_family_offset: Vec<u32> = vec![];
        let mut ruby_family_count: Vec<u32> = vec![];
        let mut ruby_ascent: Vec<f64> = vec![];
        let mut ruby_family_pool: Vec<u32> = vec![];
        {
            let _g1 = plan.ruby_decisions.clone();
            for ruby in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = (ruby.font_families).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let fam = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        pool.intern(fam.as_str());
                    }
                }
                pool.intern((ruby.text).to_string().as_str());
            }
        }
        {
            let _g1 = plan.ruby_decisions.clone();
            for ruby in &_g1 {
                ruby_base_start.push(ruby.base_range_start);
                ruby_base_end.push(ruby.base_range_end);
                ruby_text_ref.push(pool.index_of((ruby.text).to_string().as_str()));
                ruby_center_x.push(ruby.center_x);
                ruby_baseline_y.push(ruby.baseline_y);
                ruby_font_size.push(ruby.font_size);
                ruby_font_weight.push(ruby.font_weight);
                ruby_family_offset.push(u32::try_from((ruby_family_pool.len()) & 0xFFFF_FFFF).unwrap_or(0));
                ruby_family_count.push(u32::try_from(((ruby.font_families).clone().len()) & 0xFFFF_FFFF).unwrap_or(0));
                ruby_ascent.push(match &(ruby.ascent) { Some(__option31) => *__option31, None => f64::NAN });
                {
                    let mut _g = 0u32;
                    let _g1 = (ruby.font_families).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let fam = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        ruby_family_pool.push(pool.index_of(fam.as_str()));
                    }
                }
            }
        }
        let mut bopomofo_base_start: Vec<u32> = vec![];
        let mut bopomofo_base_end: Vec<u32> = vec![];
        let mut bopomofo_text_ref: Vec<u32> = vec![];
        let mut bopomofo_font_weight: Vec<u32> = vec![];
        let mut bopomofo_family_offset: Vec<u32> = vec![];
        let mut bopomofo_family_count: Vec<u32> = vec![];
        let mut bopomofo_place_offset: Vec<u32> = vec![];
        let mut bopomofo_place_count: Vec<u32> = vec![];
        let mut bopomofo_family_pool: Vec<u32> = vec![];
        let mut bopomofo_place_text_ref: Vec<u32> = vec![];
        let mut bopomofo_place_role_ref: Vec<u32> = vec![];
        let mut bopomofo_place_left: Vec<f64> = vec![];
        let mut bopomofo_place_top: Vec<f64> = vec![];
        let mut bopomofo_place_width: Vec<f64> = vec![];
        let mut bopomofo_place_height: Vec<f64> = vec![];
        {
            let _g1 = plan.bopomofo_decisions.clone();
            for bopomofo in &_g1 {
                {
                    let mut _g = 0u32;
                    let _g1 = (bopomofo.font_families).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let fam = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        pool.intern(fam.as_str());
                    }
                }
                pool.intern((bopomofo.text).to_string().as_str());
                {
                    let mut _g = 0u32;
                    let _g1 = (bopomofo.placements).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let pl = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        pool.intern((pl.text).to_string().as_str());
                        pool.intern((pl.role).to_string().as_str());
                    }
                }
            }
        }
        {
            let _g1 = plan.bopomofo_decisions.clone();
            for bopomofo in &_g1 {
                bopomofo_base_start.push(bopomofo.base_range_start);
                bopomofo_base_end.push(bopomofo.base_range_end);
                bopomofo_text_ref.push(pool.index_of((bopomofo.text).to_string().as_str()));
                bopomofo_font_weight.push(bopomofo.font_weight);
                bopomofo_family_offset.push(u32::try_from((bopomofo_family_pool.len()) & 0xFFFF_FFFF).unwrap_or(0));
                bopomofo_family_count.push(u32::try_from(((bopomofo.font_families).clone().len()) & 0xFFFF_FFFF).unwrap_or(0));
                bopomofo_place_offset.push(u32::try_from((bopomofo_place_text_ref.len()) & 0xFFFF_FFFF).unwrap_or(0));
                bopomofo_place_count.push(u32::try_from(((bopomofo.placements).clone().len()) & 0xFFFF_FFFF).unwrap_or(0));
                {
                    let mut _g = 0u32;
                    let _g1 = (bopomofo.font_families).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let fam = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        bopomofo_family_pool.push(pool.index_of(fam.as_str()));
                    }
                }
                {
                    let mut _g = 0u32;
                    let _g1 = (bopomofo.placements).clone().clone();
                    while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                        let pl = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                        _g = u32::wrapping_add(_g, 1);
                        bopomofo_place_text_ref.push(pool.index_of((pl.text).to_string().as_str()));
                        bopomofo_place_role_ref.push(pool.index_of((pl.role).to_string().as_str()));
                        bopomofo_place_left.push(pl.left);
                        bopomofo_place_top.push(pl.top);
                        bopomofo_place_width.push(pl.width);
                        bopomofo_place_height.push(pl.height);
                    }
                }
            }
        }
        let mut decoration_kind_ref: Vec<u32> = vec![];
        let mut decoration_left: Vec<f64> = vec![];
        let mut decoration_top: Vec<f64> = vec![];
        let mut decoration_right: Vec<f64> = vec![];
        {
            let _g1 = plan.decoration_segments.clone();
            for seg in &_g1 {
                pool.intern((seg.kind).to_string().as_str());
            }
        }
        {
            let _g1 = plan.decoration_segments.clone();
            for seg in &_g1 {
                decoration_kind_ref.push(pool.index_of((seg.kind).to_string().as_str()));
                decoration_left.push(seg.left);
                decoration_top.push(seg.top);
                decoration_right.push(seg.right);
            }
        }
        let emphasis_range_count = u32::try_from((plan.emphasis_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let inline_edge_count = u32::try_from((plan.inline_edges.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let ruby_count = u32::try_from((plan.ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let bopomofo_count = u32::try_from((plan.bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let bopomofo_placement_total = u32::try_from((bopomofo_place_text_ref.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let decoration_segment_count = u32::try_from((plan.decoration_segments.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let emphasis_dot_count = u32::try_from((plan.emphasis_dots.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let string_count = u32::try_from((pool.ordered.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let feature_total = u32::try_from((feature_pool.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let ruby_family_total = u32::try_from((ruby_family_pool.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let bopomofo_family_total = u32::try_from((bopomofo_family_pool.len()) & 0xFFFF_FFFF).unwrap_or(0);
        writer.u32(1414615120);
        writer.u32(1);
        writer.f64(plan.width);
        writer.f64(plan.height);
        let font_size = plan.font_size;
        writer.f64(match &(font_size) { Some(__option33) => *__option33, None => f64::NAN });
        let overlay_width = plan.overlay_width;
        writer.f64(match &(overlay_width) { Some(__option35) => *__option35, None => f64::NAN });
        writer.u32(u32::try_from((plan.lines.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(cell_count);
        writer.u32(emphasis_range_count);
        writer.u32(inline_edge_count);
        writer.u32(ruby_count);
        writer.u32(bopomofo_count);
        writer.u32(bopomofo_placement_total);
        writer.u32(decoration_segment_count);
        writer.u32(emphasis_dot_count);
        writer.u32(string_count);
        writer.u32(feature_total);
        writer.u32(ruby_family_total);
        writer.u32(bopomofo_family_total);
        {
            let _g1 = pool.ordered.clone();
            for s in &_g1 {
                let encoded = s.as_bytes().to_vec();
                writer.u32(u32::try_from((encoded.len()) & 0xFFFF_FFFF).unwrap_or(0));
            }
        }
        {
            let _g1 = pool.ordered.clone();
            for s in &_g1 {
                writer.raw(&s.as_bytes().to_vec());
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.u32(line.range_start);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.u32(line.range_end);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.f64(line.top);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.f64(line.bottom);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.f64(line.baseline);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.f64(line.indent);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.f64(line.visual_width);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.f64(line.hyphen_advance);
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.u8(PlanPacked::plan_packed_end_reason_code(line.end_reason));
            }
        }
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                writer.u32(u32::try_from(((line.cells).clone().len()) & 0xFFFF_FFFF).unwrap_or(0));
            }
        }
        for &v in &cell_range_start {
            writer.u32(v);
        }
        for &v in &cell_range_end {
            writer.u32(v);
        }
        for &v in &cell_source_ref {
            writer.u32(v);
        }
        for &v in &cell_display_ref {
            writer.u32(v);
        }
        for &v in &cell_draw_x {
            writer.f64(v);
        }
        for &v in &cell_natural_width {
            writer.f64(v);
        }
        for &v in &cell_leading_advance {
            writer.f64(v);
        }
        for &v in &cell_shaping_boundary {
            writer.u8(v);
        }
        for &v in &cell_latin {
            writer.u8(v);
        }
        for &v in &cell_render_family_ref {
            writer.u32(v);
        }
        for &v in &cell_dash_ref {
            writer.u32(v);
        }
        for &v in &cell_language_ref {
            writer.u32(v);
        }
        for &v in &cell_resolved_face_ref {
            writer.u32(v);
        }
        for &v in &cell_glyph_ids_ref {
            writer.u32(v);
        }
        for &v in &cell_evidence_ref {
            writer.u32(v);
        }
        for &v in &cell_ink_floor {
            writer.f64(v);
        }
        for &v in &cell_body_width {
            writer.f64(v);
        }
        for &v in &cell_advance {
            writer.f64(v);
        }
        for &v in &cell_inline_object {
            writer.f64(v);
        }
        for &v in &cell_style_font_size {
            writer.f64(v);
        }
        for &v in &cell_style_font_weight {
            writer.f64(v);
        }
        for &v in &cell_style_italic {
            writer.u8(v);
        }
        for &v in &cell_feature_offset {
            writer.u32(v);
        }
        for &v in &cell_feature_count {
            writer.u32(v);
        }
        for &v in &feature_pool {
            writer.u32(v);
        }
        {
            let _g1 = plan.emphasis_ranges.clone();
            for range in &_g1 {
                writer.f64(i32::from_ne_bytes((range.start).to_ne_bytes()) as f64);
            }
        }
        {
            let _g1 = plan.emphasis_ranges.clone();
            for range in &_g1 {
                writer.f64(i32::from_ne_bytes((range.end).to_ne_bytes()) as f64);
            }
        }
        {
            let _g1 = plan.inline_edges.clone();
            for edge in &_g1 {
                writer.f64(i32::from_ne_bytes((edge.offset).to_ne_bytes()) as f64);
            }
        }
        {
            let _g1 = plan.inline_edges.clone();
            for edge in &_g1 {
                writer.f64(match &(edge.inline_start) { Some(__option37) => *__option37, None => f64::NAN });
            }
        }
        {
            let _g1 = plan.inline_edges.clone();
            for edge in &_g1 {
                writer.f64(match &(edge.inline_end) { Some(__option39) => *__option39, None => f64::NAN });
            }
        }
        for &v in &ruby_base_start {
            writer.u32(v);
        }
        for &v in &ruby_base_end {
            writer.u32(v);
        }
        for &v in &ruby_text_ref {
            writer.u32(v);
        }
        for &v in &ruby_center_x {
            writer.f64(v);
        }
        for &v in &ruby_baseline_y {
            writer.f64(v);
        }
        for &v in &ruby_font_size {
            writer.f64(v);
        }
        for &v in &ruby_font_weight {
            writer.u32(v);
        }
        for &v in &ruby_family_offset {
            writer.u32(v);
        }
        for &v in &ruby_family_count {
            writer.u32(v);
        }
        for &v in &ruby_ascent {
            writer.f64(v);
        }
        for &v in &ruby_family_pool {
            writer.u32(v);
        }
        for &v in &bopomofo_base_start {
            writer.u32(v);
        }
        for &v in &bopomofo_base_end {
            writer.u32(v);
        }
        for &v in &bopomofo_text_ref {
            writer.u32(v);
        }
        for &v in &bopomofo_font_weight {
            writer.u32(v);
        }
        for &v in &bopomofo_family_offset {
            writer.u32(v);
        }
        for &v in &bopomofo_family_count {
            writer.u32(v);
        }
        for &v in &bopomofo_place_offset {
            writer.u32(v);
        }
        for &v in &bopomofo_place_count {
            writer.u32(v);
        }
        for &v in &bopomofo_family_pool {
            writer.u32(v);
        }
        for &v in &bopomofo_place_text_ref {
            writer.u32(v);
        }
        for &v in &bopomofo_place_role_ref {
            writer.u32(v);
        }
        for &v in &bopomofo_place_left {
            writer.f64(v);
        }
        for &v in &bopomofo_place_top {
            writer.f64(v);
        }
        for &v in &bopomofo_place_width {
            writer.f64(v);
        }
        for &v in &bopomofo_place_height {
            writer.f64(v);
        }
        for &v in &decoration_kind_ref {
            writer.u32(v);
        }
        for &v in &decoration_left {
            writer.f64(v);
        }
        for &v in &decoration_top {
            writer.f64(v);
        }
        for &v in &decoration_right {
            writer.f64(v);
        }
        {
            let _g1 = plan.emphasis_dots.clone();
            for dot in &_g1 {
                let cs = dot.cluster_range_start;
                writer.f64(match &(cs) { Some(__option41) => *__option41, None => f64::NAN });
            }
        }
        {
            let _g1 = plan.emphasis_dots.clone();
            for dot in &_g1 {
                writer.f64(dot.anchor_x);
            }
        }
        {
            let _g1 = plan.emphasis_dots.clone();
            for dot in &_g1 {
                writer.f64(dot.anchor_y);
            }
        }
        {
            let _g1 = plan.emphasis_dots.clone();
            for dot in &_g1 {
                writer.f64(dot.dot_diameter);
            }
        }
        return writer.finish();
    }

    pub(crate) fn plan_packed_end_reason_code(reason: PlanEndReason) -> u32 {
        return match reason {
            PlanEndReason::AutoWrap => 0,
            PlanEndReason::MandatoryBreak => 1,
            PlanEndReason::ParagraphEnd => 2,
        };
    }

    pub fn plan_packed_decode(bytes: &[u8]) -> Option<Plan> {
        let mut r = PlanPackedReader::new(bytes);
        if r.u32() != 1414615120 {
            return None;
        }
        if r.u32() != 1 {
            return None;
        }
        let width = r.f64();
        let height = r.f64();
        let font_sz = r.f64();
        let overlay_w = r.f64();
        let lc = r.u32();
        let cc = r.u32();
        let erc = r.u32();
        let iec = r.u32();
        let rc = r.u32();
        let bc = r.u32();
        let bpt = r.u32();
        let dsc = r.u32();
        let edc = r.u32();
        let sc = r.u32();
        let ft = r.u32();
        let rft = r.u32();
        let bft = r.u32();
        if r.failed {
            return None;
        }
        let mut deltas: Vec<u32> = vec![];
        for _ in 0..sc {
            deltas.push(r.u32());
        }
        let pool: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        for &d in &deltas {
            pool.lock().unwrap().push(r.string(d));
        }
        if r.failed {
            return None;
        }
        let sref: Arc<dyn Fn(u32) -> Option<String> + Send + Sync + 'static> = { let pool = (pool).clone(); Arc::new({ let pool = Arc::clone(&pool); move |r#ref| {
        return if r#ref != 4294967295u32 { Some((pool.lock().unwrap()[usize::try_from(r#ref).unwrap_or(0)]).clone()) } else { None };
} }) };
        let mut lrs: Vec<u32> = vec![];
        for _ in 0..lc {
            lrs.push(r.u32());
        }
        let mut lre: Vec<u32> = vec![];
        for _ in 0..lc {
            lre.push(r.u32());
        }
        let mut lt: Vec<f64> = vec![];
        for _ in 0..lc {
            lt.push(r.f64());
        }
        let mut lb: Vec<f64> = vec![];
        for _ in 0..lc {
            lb.push(r.f64());
        }
        let mut lbl: Vec<f64> = vec![];
        for _ in 0..lc {
            lbl.push(r.f64());
        }
        let mut lin: Vec<f64> = vec![];
        for _ in 0..lc {
            lin.push(r.f64());
        }
        let mut lvw: Vec<f64> = vec![];
        for _ in 0..lc {
            lvw.push(r.f64());
        }
        let mut lha: Vec<f64> = vec![];
        for _ in 0..lc {
            lha.push(r.f64());
        }
        let mut ler: Vec<u32> = vec![];
        for _ in 0..lc {
            ler.push(r.u8());
        }
        let mut lcc: Vec<u32> = vec![];
        for _ in 0..lc {
            lcc.push(r.u32());
        }
        if r.failed {
            return None;
        }
        let mut crs: Vec<u32> = vec![];
        for _ in 0..cc {
            crs.push(r.u32());
        }
        let mut cre: Vec<u32> = vec![];
        for _ in 0..cc {
            cre.push(r.u32());
        }
        let mut csr: Vec<u32> = vec![];
        for _ in 0..cc {
            csr.push(r.u32());
        }
        let mut cdr: Vec<u32> = vec![];
        for _ in 0..cc {
            cdr.push(r.u32());
        }
        let mut cdx: Vec<f64> = vec![];
        for _ in 0..cc {
            cdx.push(r.f64());
        }
        let mut cnw: Vec<f64> = vec![];
        for _ in 0..cc {
            cnw.push(r.f64());
        }
        let mut cla: Vec<f64> = vec![];
        for _ in 0..cc {
            cla.push(r.f64());
        }
        let mut csb: Vec<u32> = vec![];
        for _ in 0..cc {
            csb.push(r.u8());
        }
        let mut cla2: Vec<u32> = vec![];
        for _ in 0..cc {
            cla2.push(r.u8());
        }
        let mut crf: Vec<u32> = vec![];
        for _ in 0..cc {
            crf.push(r.u32());
        }
        let mut cd2: Vec<u32> = vec![];
        for _ in 0..cc {
            cd2.push(r.u32());
        }
        let mut cl2: Vec<u32> = vec![];
        for _ in 0..cc {
            cl2.push(r.u32());
        }
        let mut crf2: Vec<u32> = vec![];
        for _ in 0..cc {
            crf2.push(r.u32());
        }
        let mut cgi: Vec<u32> = vec![];
        for _ in 0..cc {
            cgi.push(r.u32());
        }
        let mut cev: Vec<u32> = vec![];
        for _ in 0..cc {
            cev.push(r.u32());
        }
        let mut cif: Vec<f64> = vec![];
        for _ in 0..cc {
            cif.push(r.f64());
        }
        let mut cbw: Vec<f64> = vec![];
        for _ in 0..cc {
            cbw.push(r.f64());
        }
        let mut cad: Vec<f64> = vec![];
        for _ in 0..cc {
            cad.push(r.f64());
        }
        let mut cio: Vec<f64> = vec![];
        for _ in 0..cc {
            cio.push(r.f64());
        }
        let mut csf: Vec<f64> = vec![];
        for _ in 0..cc {
            csf.push(r.f64());
        }
        let mut csw: Vec<f64> = vec![];
        for _ in 0..cc {
            csw.push(r.f64());
        }
        let mut csi: Vec<u32> = vec![];
        for _ in 0..cc {
            csi.push(r.u8());
        }
        let mut cfo: Vec<u32> = vec![];
        for _ in 0..cc {
            cfo.push(r.u32());
        }
        let mut cfc: Vec<u32> = vec![];
        for _ in 0..cc {
            cfc.push(r.u32());
        }
        if r.failed {
            return None;
        }
        let mut fp: Vec<u32> = vec![];
        for _ in 0..ft {
            fp.push(r.u32());
        }
        let mut cursor = 0u32;
        let mut lines: Vec<PlanLine> = vec![];
        let mut __loop_guard = pool.lock().unwrap();
        for li in 0..lc {
            let cell_n = lcc[usize::try_from(li).unwrap_or(0)];
            let mut cells: Vec<PlanCell> = vec![];
            for _ in 0..cell_n {
                let i = cursor;
                cursor = u32::wrapping_add(cursor, 1);
                let fo = cfo[usize::try_from(i).unwrap_or(0)];
                let r#fn = cfc[usize::try_from(i).unwrap_or(0)];
                let mut feats: Vec<String> = vec![];
                for k in 0..r#fn {
                    feats.push((__loop_guard[usize::try_from(fp[usize::try_from(u32::wrapping_add(fo, k)).unwrap_or(0)]).unwrap_or(0)]).clone());
                }
                let sf = csf[usize::try_from(i).unwrap_or(0)];
                let sw = csw[usize::try_from(i).unwrap_or(0)];
                let si = csi[usize::try_from(i).unwrap_or(0)];
                let style = if sf.is_nan() && (sw).is_nan() && si == 2 { None } else { Some(PlanStyleDelta { font_size: if sf.is_nan() { None } else { Some(sf) }, font_weight: if sw.is_nan() { None } else { Some(u32::from_ne_bytes((match f64::from(sw) { v if v.is_nan() => 0i32, v
if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) |
4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0)
}.to_ne_bytes()) }).to_ne_bytes())) }, italic: if si == 2 { None } else { Some(si != 0) } }) };
                cells.push(PlanCell { range_start: crs[usize::try_from(i).unwrap_or(0)], range_end: cre[usize::try_from(i).unwrap_or(0)], source: (__loop_guard[usize::try_from(csr[usize::try_from(i).unwrap_or(0)]).unwrap_or(0)]).clone().clone(), display:
(__loop_guard[usize::try_from(cdr[usize::try_from(i).unwrap_or(0)]).unwrap_or(0)]).clone().clone(), draw_x: cdx[usize::try_from(i).unwrap_or(0)], natural_width: cnw[usize::try_from(i).unwrap_or(0)], leading_layout_advance: cla[usize::try_from(i).unwrap_or(0)], shaping_boundary:
csb[usize::try_from(i).unwrap_or(0)] != 0, open_type_features: feats, render_font_family: sref(crf[usize::try_from(i).unwrap_or(0)]).clone(), dash_strategy: sref(cd2[usize::try_from(i).unwrap_or(0)]).clone(), shaping_language: sref(cl2[usize::try_from(i).unwrap_or(0)]).clone(),
resolved_face: sref(crf2[usize::try_from(i).unwrap_or(0)]).clone(), glyph_ids: sref(cgi[usize::try_from(i).unwrap_or(0)]).clone(), shaping_evidence: sref(cev[usize::try_from(i).unwrap_or(0)]).clone(), punctuation_ink_floor: if cif[usize::try_from(i).unwrap_or(0)].is_nan() { None
} else { Some(cif[usize::try_from(i).unwrap_or(0)]) }, punctuation_body_width: if cbw[usize::try_from(i).unwrap_or(0)].is_nan() { None } else { Some(cbw[usize::try_from(i).unwrap_or(0)]) }, latin: cla2[usize::try_from(i).unwrap_or(0)] != 0, advance: if
cad[usize::try_from(i).unwrap_or(0)].is_nan() { None } else { Some(cad[usize::try_from(i).unwrap_or(0)]) }, inline_object: if cio[usize::try_from(i).unwrap_or(0)].is_nan() { None } else { Some(cio[usize::try_from(i).unwrap_or(0)]) }, style_delta: style });
            }
            let er = ler[usize::try_from(li).unwrap_or(0)];
            let end_r = if er == 1 { PlanEndReason::MandatoryBreak } else { if er == 2 { PlanEndReason::ParagraphEnd } else { PlanEndReason::AutoWrap } };
            lines.push(PlanLine { range_start: lrs[usize::try_from(li).unwrap_or(0)], range_end: lre[usize::try_from(li).unwrap_or(0)], top: lt[usize::try_from(li).unwrap_or(0)], bottom: lb[usize::try_from(li).unwrap_or(0)], baseline: lbl[usize::try_from(li).unwrap_or(0)],
indent: lin[usize::try_from(li).unwrap_or(0)], visual_width: lvw[usize::try_from(li).unwrap_or(0)], hyphen_advance: lha[usize::try_from(li).unwrap_or(0)], end_reason: end_r, cells: cells });
        }
        drop(__loop_guard);
        let mut emphasis: Vec<PlanEmphasisRange> = vec![];
        for _ in 0..erc {
            let s = r.f64();
            let e = r.f64();
            emphasis.push(PlanEmphasisRange { start: u32::from_ne_bytes((match f64::from(s) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 -
v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e =>
u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes()), end: u32::from_ne_bytes((match f64::from(e) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 =>
-2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v =>
i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes()) });
        }
        let mut edges: Vec<PlanInlineEdge> = vec![];
        for _ in 0..iec {
            let off = r.f64();
            let s = r.f64();
            let e = r.f64();
            edges.push(PlanInlineEdge { offset: u32::from_ne_bytes((match f64::from(off) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 -
v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e =>
u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes()), inline_start: if s.is_nan() { None } else { Some(s) }, inline_end: if e.is_nan() { None } else { Some(e) } });
        }
        let mut rb_s: Vec<u32> = vec![];
        for _ in 0..rc {
            rb_s.push(r.u32());
        }
        let mut rb_e: Vec<u32> = vec![];
        for _ in 0..rc {
            rb_e.push(r.u32());
        }
        let mut rb_t: Vec<u32> = vec![];
        for _ in 0..rc {
            rb_t.push(r.u32());
        }
        let mut rb_cx: Vec<f64> = vec![];
        for _ in 0..rc {
            rb_cx.push(r.f64());
        }
        let mut rb_by: Vec<f64> = vec![];
        for _ in 0..rc {
            rb_by.push(r.f64());
        }
        let mut rb_fs: Vec<f64> = vec![];
        for _ in 0..rc {
            rb_fs.push(r.f64());
        }
        let mut rb_fw: Vec<u32> = vec![];
        for _ in 0..rc {
            rb_fw.push(r.u32());
        }
        let mut rb_fo: Vec<u32> = vec![];
        for _ in 0..rc {
            rb_fo.push(r.u32());
        }
        let mut rb_fc: Vec<u32> = vec![];
        for _ in 0..rc {
            rb_fc.push(r.u32());
        }
        let mut rb_as: Vec<f64> = vec![];
        for _ in 0..rc {
            rb_as.push(r.f64());
        }
        let mut ruby_fam: Vec<u32> = vec![];
        for _ in 0..rft {
            ruby_fam.push(r.u32());
        }
        let mut rubys: Vec<PlanRuby> = vec![];
        let mut __loop_guard1 = pool.lock().unwrap();
        for i in 0..rc {
            let mut fams: Vec<String> = vec![];
            for k in 0..rb_fc[usize::try_from(i).unwrap_or(0)] {
                fams.push((__loop_guard1[usize::try_from(ruby_fam[usize::try_from(u32::wrapping_add(rb_fo[usize::try_from(i).unwrap_or(0)], k)).unwrap_or(0)]).unwrap_or(0)]).clone());
            }
            rubys.push(PlanRuby { base_range_start: rb_s[usize::try_from(i).unwrap_or(0)], base_range_end: rb_e[usize::try_from(i).unwrap_or(0)], text: (__loop_guard1[usize::try_from(rb_t[usize::try_from(i).unwrap_or(0)]).unwrap_or(0)]).clone().clone(), center_x:
rb_cx[usize::try_from(i).unwrap_or(0)], baseline_y: rb_by[usize::try_from(i).unwrap_or(0)], font_size: rb_fs[usize::try_from(i).unwrap_or(0)], font_weight: rb_fw[usize::try_from(i).unwrap_or(0)], font_families: fams, ascent: if rb_as[usize::try_from(i).unwrap_or(0)].is_nan() {
None } else { Some(rb_as[usize::try_from(i).unwrap_or(0)]) } });
        }
        drop(__loop_guard1);
        let mut bb_s: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_s.push(r.u32());
        }
        let mut bb_e: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_e.push(r.u32());
        }
        let mut bb_t: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_t.push(r.u32());
        }
        let mut bb_fw: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_fw.push(r.u32());
        }
        let mut bb_fo: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_fo.push(r.u32());
        }
        let mut bb_fc: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_fc.push(r.u32());
        }
        let mut bb_po: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_po.push(r.u32());
        }
        let mut bb_pc: Vec<u32> = vec![];
        for _ in 0..bc {
            bb_pc.push(r.u32());
        }
        let mut bopo_fam: Vec<u32> = vec![];
        for _ in 0..bft {
            bopo_fam.push(r.u32());
        }
        let mut bp_t: Vec<u32> = vec![];
        for _ in 0..bpt {
            bp_t.push(r.u32());
        }
        let mut bp_r: Vec<u32> = vec![];
        for _ in 0..bpt {
            bp_r.push(r.u32());
        }
        let mut bp_l: Vec<f64> = vec![];
        for _ in 0..bpt {
            bp_l.push(r.f64());
        }
        let mut bp_t2: Vec<f64> = vec![];
        for _ in 0..bpt {
            bp_t2.push(r.f64());
        }
        let mut bp_w: Vec<f64> = vec![];
        for _ in 0..bpt {
            bp_w.push(r.f64());
        }
        let mut bp_h: Vec<f64> = vec![];
        for _ in 0..bpt {
            bp_h.push(r.f64());
        }
        let mut bopos: Vec<PlanBopomofo> = vec![];
        let mut __loop_guard2 = pool.lock().unwrap();
        for i in 0..bc {
            let mut fams: Vec<String> = vec![];
            for k in 0..bb_fc[usize::try_from(i).unwrap_or(0)] {
                fams.push((__loop_guard2[usize::try_from(bopo_fam[usize::try_from(u32::wrapping_add(bb_fo[usize::try_from(i).unwrap_or(0)], k)).unwrap_or(0)]).unwrap_or(0)]).clone());
            }
            let mut placements: Vec<PlanBopomofoPlacement> = vec![];
            for k in 0..bb_pc[usize::try_from(i).unwrap_or(0)] {
                let pidx = u32::wrapping_add(bb_po[usize::try_from(i).unwrap_or(0)], k);
                placements.push(PlanBopomofoPlacement { text: (__loop_guard2[usize::try_from(bp_t[usize::try_from(pidx).unwrap_or(0)]).unwrap_or(0)]).clone().clone(), role: (__loop_guard2[usize::try_from(bp_r[usize::try_from(pidx).unwrap_or(0)]).unwrap_or(0)]).clone().clone(),
left: bp_l[usize::try_from(pidx).unwrap_or(0)], top: bp_t2[usize::try_from(pidx).unwrap_or(0)], width: bp_w[usize::try_from(pidx).unwrap_or(0)], height: bp_h[usize::try_from(pidx).unwrap_or(0)] });
            }
            bopos.push(PlanBopomofo { base_range_start: bb_s[usize::try_from(i).unwrap_or(0)], base_range_end: bb_e[usize::try_from(i).unwrap_or(0)], text: (__loop_guard2[usize::try_from(bb_t[usize::try_from(i).unwrap_or(0)]).unwrap_or(0)]).clone().clone(), font_weight:
bb_fw[usize::try_from(i).unwrap_or(0)], font_families: fams, placements: placements });
        }
        drop(__loop_guard2);
        let mut decos: Vec<PlanDecorationSegment> = vec![];
        let mut __loop_guard3 = pool.lock().unwrap();
        for _ in 0..dsc {
            let kr = r.u32();
            decos.push(PlanDecorationSegment { kind: (__loop_guard3[usize::try_from(kr).unwrap_or(0)]).clone().clone(), left: r.f64(), top: r.f64(), right: r.f64() });
        }
        drop(__loop_guard3);
        let mut dots: Vec<PlanEmphasisDot> = vec![];
        for _ in 0..edc {
            let ds = r.f64();
            dots.push(PlanEmphasisDot { cluster_range_start: if ds.is_nan() { None } else { Some(ds) }, anchor_x: r.f64(), anchor_y: r.f64(), dot_diameter: r.f64() });
        }
        if r.failed {
            return None;
        }
        return Some(Plan { width: width, height: height, lines: lines, emphasis_ranges: emphasis, inline_edges: edges, ruby_decisions: rubys, bopomofo_decisions: bopos, font_size: if font_sz.is_nan() { None } else { Some(font_sz) }, overlay_width: if overlay_w.is_nan() { None }
else { Some(overlay_w) }, decoration_segments: decos, emphasis_dots: dots });
    }
}
