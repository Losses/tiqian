use crate::haxe::crypto::sha256::Sha256;
use crate::org::tiqian::protocol::encode_result::EncodeResult;
use crate::org::tiqian::protocol::js_coerce::JsCoerce;
use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::runtime::bytes_buffer::BytesBuffer;
use crate::runtime::fp_helper::FPHelper;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct Canonical;

impl Canonical {
    pub fn canonical_digest(data: &[u8]) -> Vec<u8> {
        return Sha256::sha256_make(&data);
    }

    pub fn canonical_encode(input: WireValue, kind: u32) -> EncodeResult {
        let mut writer = Writer::new(kind);
        writer.begin(kind);
        let text = Canonical::canonical_member((input).clone(), &"text");
        let mut text_value = String::new();
        if !Canonical::canonical_is_wire_null((text).clone()) {
            text_value = JsCoerce::js_coerce_to_string((text).clone());
        }
        writer.str(text_value.as_str());
        if kind == 0 {
            Canonical::canonical_number_field(&mut writer, Canonical::canonical_number_member((input).clone(), &"maxWidthPx"));
        }
        let semantics = Canonical::canonical_encode_semantics(&mut writer, Canonical::canonical_member((input).clone(), &"semantics"));
        if semantics != "" {
            return EncodeResult::CErr { issue: semantics.to_string() };
        }
        let spans = Canonical::canonical_encode_text_spans(&mut writer, Canonical::canonical_member((input).clone(), &"textSpans"));
        if spans != "" {
            return EncodeResult::CErr { issue: spans.to_string() };
        }
        let boxes = Canonical::canonical_encode_inline_boxes(&mut writer, Canonical::canonical_member((input).clone(), &"inlineBoxes"));
        if boxes != "" {
            return EncodeResult::CErr { issue: boxes.to_string() };
        }
        let boundaries = Canonical::canonical_arr_of_nullable(Canonical::canonical_member((input).clone(), &"sourceBoundaries"));
        writer.u32(u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut boundary_index_idx = 0u32;
        while (i32::from_ne_bytes((boundary_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let boundary = (boundaries[usize::try_from(boundary_index_idx).unwrap_or(0)]).clone();
            let value = Canonical::canonical_canonical_f64(JsCoerce::js_coerce_to_number((boundary).clone()));
            writer.f64(match &(value) { None => 0.0f64, Some(__option1) => *__option1 });
            boundary_index_idx = u32::wrapping_add(boundary_index_idx, 1);
        }
        return EncodeResult::COk { bytes: writer.finish() };
    }

    pub fn canonical_member(input: WireValue, name: &str) -> WireValue {
        if Canonical::canonical_is_wire_obj((input).clone()) {
            return Canonical::canonical_member_of_fields(&Canonical::canonical_obj_fields((input).clone()), name);
        }
        return (WireValue::WNull).clone();
    }

    pub(crate) fn canonical_member_of_fields(fields: &Vec<WireField>, name: &str) -> WireValue {
        let mut field_index_idx = 0u32;
        while (i32::from_ne_bytes((field_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let field = (fields[usize::try_from(field_index_idx).unwrap_or(0)]).clone();
            if field.name.to_string() == name {
                return (field.value).clone();
            }
            field_index_idx = u32::wrapping_add(field_index_idx, 1);
        }
        return (WireValue::WNull).clone();
    }

    pub(crate) fn canonical_canonical_f64(value: f64) -> Option<f64> {
        if !(value).is_finite() {
            return None;
        }
        return if value == 0.0f64 { Some(0.0f64) } else { Some(value) };
    }

    pub(crate) fn canonical_number_member(input: WireValue, name: &str) -> Option<f64> {
        let resolved = Canonical::canonical_member((input).clone(), name);
        if resolved == WireValue::WNull {
            return None;
        }
        return Canonical::canonical_canonical_f64(JsCoerce::js_coerce_to_number((resolved).clone()));
    }

    pub(crate) fn canonical_number_field(writer: &mut Writer, value: Option<f64>) {
        match &(value) {
            None => {
                writer.u8(0);
            }
            Some(__option2) => {
                writer.u8(1);
                writer.f64(*__option2);
            }
        }
    }

    pub(crate) fn canonical_list_member(value: WireValue) -> ListShape {
        if Canonical::canonical_is_wire_null((value).clone()) {
            return (ListShape::LAbsent).clone();
        }
        if Canonical::canonical_is_wire_arr((value).clone()) {
            return ListShape::LArr { items: Canonical::canonical_arr_of((value).clone()).to_vec() };
        }
        return (ListShape::LBad).clone();
    }

    pub(crate) fn canonical_is_wire_null(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => true,
            WireValue::WBool { .. } => false,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => false,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn canonical_is_wire_arr(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => false,
            WireValue::WBool { .. } => false,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => true,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn canonical_is_wire_obj(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => false,
            WireValue::WBool { .. } => false,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => false,
            WireValue::WObj { .. } => true,
        };
    }

    pub(crate) fn canonical_arr_of(value: WireValue) -> Vec<WireValue> {
        return match value {
            WireValue::WNull => Vec::new(),
            WireValue::WBool { .. } => Vec::new(),
            WireValue::WNum { .. } => Vec::new(),
            WireValue::WStr { .. } => Vec::new(),
            WireValue::WArr { items: _p0 } => _p0,
            WireValue::WObj { .. } => Vec::new(),
        };
    }

    pub(crate) fn canonical_arr_of_nullable(value: WireValue) -> Vec<WireValue> {
        return match value {
            WireValue::WNull => Vec::new(),
            WireValue::WBool { .. } => Vec::new(),
            WireValue::WNum { .. } => Vec::new(),
            WireValue::WStr { .. } => Vec::new(),
            WireValue::WArr { items: _p0 } => _p0,
            WireValue::WObj { .. } => Vec::new(),
        };
    }

    pub(crate) fn canonical_obj_fields(value: WireValue) -> Vec<WireField> {
        return match value {
            WireValue::WNull => Vec::new(),
            WireValue::WBool { .. } => Vec::new(),
            WireValue::WNum { .. } => Vec::new(),
            WireValue::WStr { .. } => Vec::new(),
            WireValue::WArr { .. } => Vec::new(),
            WireValue::WObj { fields: _p0 } => _p0,
        };
    }

    pub(crate) fn canonical_is_bad_shape(shape: ListShape) -> bool {
        return match shape {
            ListShape::LAbsent => false,
            ListShape::LArr { .. } => false,
            ListShape::LBad => true,
        };
    }

    pub(crate) fn canonical_shape_items(shape: ListShape) -> Vec<WireValue> {
        return match shape {
            ListShape::LAbsent => Vec::new(),
            ListShape::LArr { items: _p0 } => _p0,
            ListShape::LBad => Vec::new(),
        };
    }

    pub(crate) fn canonical_encode_semantics(writer: &mut Writer, value: WireValue) -> String {
        let shape = Canonical::canonical_list_member((value).clone());
        if Canonical::canonical_is_bad_shape((shape).clone()) {
            return "InvalidSnapshotSemantics".to_string();
        }
        let items = Canonical::canonical_shape_items((shape).clone());
        writer.u32(u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut span_index_idx = 0u32;
        while (i32::from_ne_bytes((span_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let span = (items[usize::try_from(span_index_idx).unwrap_or(0)]).clone();
            let attributes = Canonical::canonical_member((span).clone(), &"attributes");
            let order = Canonical::canonical_number_member((span).clone(), &"order");
            let tag_name = Canonical::canonical_member((span).clone(), &"tagName");
            let mut flags = 0u32;
            if attributes != WireValue::WNull {
                flags |= 1;
            }
            if order.is_some() {
                flags |= 2;
            }
            if tag_name != WireValue::WNull {
                flags |= 4;
            }
            writer.u8(flags);
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), &"start"));
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), &"end"));
            if tag_name != WireValue::WNull {
                writer.str(JsCoerce::js_coerce_to_string((tag_name).clone()).as_str());
            }
            match &(order) {
                Some(__option4) => {
                    writer.f64(*__option4);
                }
                None => {
                }
            }
            if attributes != WireValue::WNull {
                Canonical::canonical_encode_attributes(writer, (attributes).clone());
            }
            span_index_idx = u32::wrapping_add(span_index_idx, 1);
        }
        return String::new();
    }

    pub(crate) fn canonical_encode_text_spans(writer: &mut Writer, value: WireValue) -> String {
        let shape = Canonical::canonical_list_member((value).clone());
        if Canonical::canonical_is_bad_shape((shape).clone()) {
            return "InvalidSnapshotTextSpans".to_string();
        }
        let items = Canonical::canonical_shape_items((shape).clone());
        writer.u32(u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut span_index_idx = 0u32;
        while (i32::from_ne_bytes((span_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let span = (items[usize::try_from(span_index_idx).unwrap_or(0)]).clone();
            let has_families = Canonical::canonical_families_present((span).clone());
            let families = Canonical::canonical_families_of((span).clone());
            let font_size_px = Canonical::canonical_number_member((span).clone(), &"fontSizePx");
            let font_weight = Canonical::canonical_number_member((span).clone(), &"fontWeight");
            let has_italic = Canonical::canonical_italic_present((span).clone());
            let italic_value = Canonical::canonical_italic_value_of((span).clone());
            let baseline_shift_px = Canonical::canonical_number_member((span).clone(), &"baselineShiftPx");
            let mut flags = 0u32;
            if has_families {
                flags |= 1;
            }
            if font_size_px.is_some() {
                flags |= 2;
            }
            if font_weight.is_some() {
                flags |= 4;
            }
            if has_italic {
                flags |= 8;
            }
            if baseline_shift_px.is_some() {
                flags |= 16;
            }
            writer.u8(flags);
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), &"start"));
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), &"end"));
            if has_families {
                writer.u32(u32::try_from((families.len()) & 0xFFFF_FFFF).unwrap_or(0));
                let mut name_index_idx = 0u32;
                while (i32::from_ne_bytes((name_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((families.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                    writer.str((families[usize::try_from(name_index_idx).unwrap_or(0)]).clone().as_str());
                    name_index_idx = u32::wrapping_add(name_index_idx, 1);
                }
            }
            match &(font_size_px) {
                Some(__option8) => {
                    writer.f64(*__option8);
                }
                None => {
                }
            }
            match &(font_weight) {
                Some(__option9) => {
                    writer.f64(*__option9);
                }
                None => {
                }
            }
            if has_italic {
                writer.u8(if italic_value { 1 } else { 0 });
            }
            match &(baseline_shift_px) {
                Some(__option10) => {
                    writer.f64(*__option10);
                }
                None => {
                }
            }
            span_index_idx = u32::wrapping_add(span_index_idx, 1);
        }
        return String::new();
    }

    pub(crate) fn canonical_encode_inline_boxes(writer: &mut Writer, value: WireValue) -> String {
        let shape = Canonical::canonical_list_member((value).clone());
        if Canonical::canonical_is_bad_shape((shape).clone()) {
            return "InvalidSnapshotInlineBoxes".to_string();
        }
        let items = Canonical::canonical_shape_items((shape).clone());
        writer.u32(u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut item_index_idx = 0u32;
        while (i32::from_ne_bytes((item_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let item = (items[usize::try_from(item_index_idx).unwrap_or(0)]).clone();
            let inline_start_px = Canonical::canonical_number_member((item).clone(), &"inlineStartPx");
            let inline_end_px = Canonical::canonical_number_member((item).clone(), &"inlineEndPx");
            let outer_spacing = Canonical::canonical_member((item).clone(), &"outerSpacing");
            let mut flags = 0u32;
            if inline_start_px.is_some() {
                flags |= 1;
            }
            if inline_end_px.is_some() {
                flags |= 2;
            }
            if outer_spacing != WireValue::WNull {
                flags |= 4;
            }
            writer.u8(flags);
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((item).clone(), &"start"));
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((item).clone(), &"end"));
            match &(inline_start_px) {
                Some(__option13) => {
                    writer.f64(*__option13);
                }
                None => {
                }
            }
            match &(inline_end_px) {
                Some(__option14) => {
                    writer.f64(*__option14);
                }
                None => {
                }
            }
            if outer_spacing != WireValue::WNull {
                writer.str(JsCoerce::js_coerce_to_string((outer_spacing).clone()).as_str());
            }
            item_index_idx = u32::wrapping_add(item_index_idx, 1);
        }
        return String::new();
    }

    pub(crate) fn canonical_families_present(span: WireValue) -> bool {
        return Canonical::canonical_is_wire_arr(Canonical::canonical_member((span).clone(), &"fontFamilies"));
    }

    pub(crate) fn canonical_families_of(span: WireValue) -> Vec<String> {
        return Canonical::canonical_families_from_list(&Canonical::canonical_arr_of(Canonical::canonical_member((span).clone(), &"fontFamilies")));
    }

    pub(crate) fn canonical_families_from_list(list: &Vec<WireValue>) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        let mut item_index_idx = 0u32;
        while (i32::from_ne_bytes((item_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((list.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            names.push(JsCoerce::js_coerce_to_string((list[usize::try_from(item_index_idx).unwrap_or(0)]).clone()));
            item_index_idx = u32::wrapping_add(item_index_idx, 1);
        }
        return names;
    }

    pub(crate) fn canonical_italic_present(span: WireValue) -> bool {
        return Canonical::canonical_is_wire_bool(Canonical::canonical_member((span).clone(), &"italic"));
    }

    pub(crate) fn canonical_italic_value_of(span: WireValue) -> bool {
        return Canonical::canonical_bool_of(Canonical::canonical_member((span).clone(), &"italic"));
    }

    pub(crate) fn canonical_is_wire_bool(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => false,
            WireValue::WBool { .. } => true,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => false,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn canonical_bool_of(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => false,
            WireValue::WBool { value: _p0 } => _p0,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => false,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn canonical_encode_attributes(writer: &mut Writer, value: WireValue) {
        if Canonical::canonical_is_wire_obj((value).clone()) {
            Canonical::canonical_encode_object_attributes(writer, &Canonical::canonical_obj_fields((value).clone()));
            return;
        }
        if Canonical::canonical_is_wire_arr((value).clone()) {
            writer.u8(2);
            writer.str(JsCoerce::js_coerce_render_json((value).clone()).as_str());
            return;
        }
        writer.u8(0);
    }

    pub(crate) fn canonical_encode_object_attributes(writer: &mut Writer, fields: &Vec<WireField>) {
        writer.u8(1);
        writer.u32(u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut field_index_idx = 0u32;
        while (i32::from_ne_bytes((field_index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let field = (fields[usize::try_from(field_index_idx).unwrap_or(0)]).clone();
            writer.str((field.name).to_string().as_str());
            writer.str(JsCoerce::js_coerce_to_string((field.value).clone()).as_str());
            field_index_idx = u32::wrapping_add(field_index_idx, 1);
        }
    }
}

pub struct Writer {
    pub(crate) buf: BytesBuffer,
}

impl Writer {
    pub fn new(kind: u32) -> Self {
        Self {
            buf: BytesBuffer::new(),
        }
    }

    pub fn begin(&mut self, kind: u32) {
        self.write_magic();
        self.u8(1);
        self.u8(kind);
    }

    fn write_magic(&mut self) {
        self.buf.add(&"TQCS".as_bytes().to_vec());
    }

    pub fn u8(&mut self, value: u32) {
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
    }

    pub fn u32(&mut self, value: u32) {
        self.emit_bits(value);
    }

    pub fn f64(&mut self, value: f64) {
        let bits = FPHelper::double_to_i64(value);
        self.emit_bits(bits.low);
        self.emit_bits(bits.high);
    }

    fn emit_bits(&mut self, value: u32) {
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[2])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[1])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[0])) & 0xFF).unwrap_or(0));
    }

    pub fn str(&mut self, value: &str) {
        let encoded = value.as_bytes().to_vec();
        self.u32(u32::try_from((encoded.len()) & 0xFFFF_FFFF).unwrap_or(0));
        self.buf.add(&encoded);
    }

    pub fn finish(&mut self) -> Vec<u8> {
        return self.buf.get_bytes();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListShape {
    LAbsent,
    LArr { items: Vec<WireValue> },
    LBad,
}

impl ListShape {
    pub fn to_string(&self) -> String {
        match self {
            ListShape::LAbsent => "LAbsent".to_string(),
            ListShape::LArr { items } => format!("LArr(items={})", {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < items.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", (items[j0]).to_string());
                j0 += 1;
            }
            out.push(']');
            out
        }),
            ListShape::LBad => "LBad".to_string(),
        }
    }
}
