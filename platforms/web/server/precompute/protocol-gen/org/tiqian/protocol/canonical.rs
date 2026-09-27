use crate::haxe::crypto::sha256::Sha256;
use crate::org::tiqian::protocol::encode_result::EncodeResult;
use crate::org::tiqian::protocol::js_coerce::JsCoerce;
use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::runtime::bytes_buffer::BytesBuffer;
use crate::runtime::fp_helper::FPHelper;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct Canonical;

impl Canonical {
    pub fn canonical_digest(data: &[u8]) -> Vec<u8> {
        return Sha256::sha256_make(&data);
    }

    pub fn canonical_encode(input: WireValue, kind: u32) -> EncodeResult {
        let mut writer = Writer::new();
        writer.begin(kind);
        let text = Canonical::canonical_member((input).clone(), UStr::new(&[116,101,120,116]));
        let mut text_value = UString::new();
        if !Canonical::canonical_is_wire_null((text).clone()) {
            text_value = JsCoerce::js_coerce_to_string((text).clone());
        }
        writer.str(text_value.as_ustr());
        if kind == 0 {
            Canonical::canonical_number_field(&mut writer, Canonical::canonical_number_member((input).clone(), UStr::new(&[109,97,120,87,105,100,116,104,80,120])));
        }
        let semantics = Canonical::canonical_encode_semantics(&mut writer, Canonical::canonical_member((input).clone(), UStr::new(&[115,101,109,97,110,116,105,99,115])));
        if semantics != UString::from("") {
            return EncodeResult::CErr { issue: semantics.to_ustring() };
        }
        let spans = Canonical::canonical_encode_text_spans(&mut writer, Canonical::canonical_member((input).clone(), UStr::new(&[116,101,120,116,83,112,97,110,115])));
        if spans != UString::from("") {
            return EncodeResult::CErr { issue: spans.to_ustring() };
        }
        let boxes = Canonical::canonical_encode_inline_boxes(&mut writer, Canonical::canonical_member((input).clone(), UStr::new(&[105,110,108,105,110,101,66,111,120,101,115])));
        if boxes != UString::from("") {
            return EncodeResult::CErr { issue: boxes.to_ustring() };
        }
        let boundaries = Canonical::canonical_arr_of_nullable(Canonical::canonical_member((input).clone(), UStr::new(&[115,111,117,114,99,101,66,111,117,110,100,97,114,105,101,115])));
        writer.u32(u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut boundary_index_idx = 0u32;
        while (i32::from_ne_bytes(((boundary_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let boundary = (boundaries[usize::try_from(boundary_index_idx).unwrap_or(0)]).clone();
            let value = Canonical::canonical_canonical_f64(JsCoerce::js_coerce_to_number((boundary).clone()));
            writer.f64(match &(value) { None => 0.0f64, Some(__option1) => *__option1 });
            boundary_index_idx = u32::wrapping_add(boundary_index_idx, 1);
        }
        return EncodeResult::COk { bytes: writer.finish() };
    }

    pub fn canonical_member(input: WireValue, name: &UStr) -> WireValue {
        if Canonical::canonical_is_wire_obj((input).clone()) {
            return Canonical::canonical_member_of_fields(&Canonical::canonical_obj_fields((input).clone()), name);
        }
        return (WireValue::WNull).clone();
    }

    pub(crate) fn canonical_member_of_fields(fields: &Vec<WireField>, name: &UStr) -> WireValue {
        let mut field_index_idx = 0u32;
        while (i32::from_ne_bytes(((field_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let field = (fields[usize::try_from(field_index_idx).unwrap_or(0)]).clone();
            if field.name.to_ustring() == name {
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

    pub(crate) fn canonical_number_member(input: WireValue, name: &UStr) -> Option<f64> {
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

    pub(crate) fn canonical_encode_semantics(writer: &mut Writer, value: WireValue) -> UString {
        let shape = Canonical::canonical_list_member((value).clone());
        if Canonical::canonical_is_bad_shape((shape).clone()) {
            return UString::from("InvalidSnapshotSemantics").to_ustring();
        }
        let items = Canonical::canonical_shape_items((shape).clone());
        writer.u32(u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut span_index_idx = 0u32;
        while (i32::from_ne_bytes(((span_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let span = (items[usize::try_from(span_index_idx).unwrap_or(0)]).clone();
            let attributes = Canonical::canonical_member((span).clone(), UStr::new(&[97,116,116,114,105,98,117,116,101,115]));
            let order = Canonical::canonical_number_member((span).clone(), UStr::new(&[111,114,100,101,114]));
            let tag_name = Canonical::canonical_member((span).clone(), UStr::new(&[116,97,103,78,97,109,101]));
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
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), UStr::new(&[115,116,97,114,116])));
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), UStr::new(&[101,110,100])));
            if tag_name != WireValue::WNull {
                writer.str(JsCoerce::js_coerce_to_string((tag_name).clone()).as_ustr());
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
        return UString::new();
    }

    pub(crate) fn canonical_encode_text_spans(writer: &mut Writer, value: WireValue) -> UString {
        let shape = Canonical::canonical_list_member((value).clone());
        if Canonical::canonical_is_bad_shape((shape).clone()) {
            return UString::from("InvalidSnapshotTextSpans").to_ustring();
        }
        let items = Canonical::canonical_shape_items((shape).clone());
        writer.u32(u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut span_index_idx = 0u32;
        while (i32::from_ne_bytes(((span_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let span = (items[usize::try_from(span_index_idx).unwrap_or(0)]).clone();
            let has_families = Canonical::canonical_families_present((span).clone());
            let families = Canonical::canonical_families_of((span).clone());
            let font_size_px = Canonical::canonical_number_member((span).clone(), UStr::new(&[102,111,110,116,83,105,122,101,80,120]));
            let font_weight = Canonical::canonical_number_member((span).clone(), UStr::new(&[102,111,110,116,87,101,105,103,104,116]));
            let has_italic = Canonical::canonical_italic_present((span).clone());
            let italic_value = Canonical::canonical_italic_value_of((span).clone());
            let baseline_shift_px = Canonical::canonical_number_member((span).clone(), UStr::new(&[98,97,115,101,108,105,110,101,83,104,105,102,116,80,120]));
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
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), UStr::new(&[115,116,97,114,116])));
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((span).clone(), UStr::new(&[101,110,100])));
            if has_families {
                writer.u32(u32::try_from((families.len()) & 0xFFFF_FFFF).unwrap_or(0));
                let mut name_index_idx = 0u32;
                while (i32::from_ne_bytes(((name_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((families.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    writer.str((families[usize::try_from(name_index_idx).unwrap_or(0)]).clone().as_ustr());
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
        return UString::new();
    }

    pub(crate) fn canonical_encode_inline_boxes(writer: &mut Writer, value: WireValue) -> UString {
        let shape = Canonical::canonical_list_member((value).clone());
        if Canonical::canonical_is_bad_shape((shape).clone()) {
            return UString::from("InvalidSnapshotInlineBoxes").to_ustring();
        }
        let items = Canonical::canonical_shape_items((shape).clone());
        writer.u32(u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut item_index_idx = 0u32;
        while (i32::from_ne_bytes(((item_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let item = (items[usize::try_from(item_index_idx).unwrap_or(0)]).clone();
            let inline_start_px = Canonical::canonical_number_member((item).clone(), UStr::new(&[105,110,108,105,110,101,83,116,97,114,116,80,120]));
            let inline_end_px = Canonical::canonical_number_member((item).clone(), UStr::new(&[105,110,108,105,110,101,69,110,100,80,120]));
            let outer_spacing = Canonical::canonical_member((item).clone(), UStr::new(&[111,117,116,101,114,83,112,97,99,105,110,103]));
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
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((item).clone(), UStr::new(&[115,116,97,114,116])));
            Canonical::canonical_number_field(writer, Canonical::canonical_number_member((item).clone(), UStr::new(&[101,110,100])));
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
                writer.str(JsCoerce::js_coerce_to_string((outer_spacing).clone()).as_ustr());
            }
            item_index_idx = u32::wrapping_add(item_index_idx, 1);
        }
        return UString::new();
    }

    pub(crate) fn canonical_families_present(span: WireValue) -> bool {
        return Canonical::canonical_is_wire_arr(Canonical::canonical_member((span).clone(), UStr::new(&[102,111,110,116,70,97,109,105,108,105,101,115])));
    }

    pub(crate) fn canonical_families_of(span: WireValue) -> Vec<UString> {
        return Canonical::canonical_families_from_list(&Canonical::canonical_arr_of(Canonical::canonical_member((span).clone(), UStr::new(&[102,111,110,116,70,97,109,105,108,105,101,115]))));
    }

    pub(crate) fn canonical_families_from_list(list: &Vec<WireValue>) -> Vec<UString> {
        let mut names: Vec<UString> = Vec::new();
        let mut item_index_idx = 0u32;
        while (i32::from_ne_bytes(((item_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((list.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            names.push(JsCoerce::js_coerce_to_string((list[usize::try_from(item_index_idx).unwrap_or(0)]).clone()));
            item_index_idx = u32::wrapping_add(item_index_idx, 1);
        }
        return names;
    }

    pub(crate) fn canonical_italic_present(span: WireValue) -> bool {
        return Canonical::canonical_is_wire_bool(Canonical::canonical_member((span).clone(), UStr::new(&[105,116,97,108,105,99])));
    }

    pub(crate) fn canonical_italic_value_of(span: WireValue) -> bool {
        return Canonical::canonical_bool_of(Canonical::canonical_member((span).clone(), UStr::new(&[105,116,97,108,105,99])));
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
            writer.str(JsCoerce::js_coerce_render_json((value).clone()).as_ustr());
            return;
        }
        writer.u8(0);
    }

    pub(crate) fn canonical_encode_object_attributes(writer: &mut Writer, fields: &Vec<WireField>) {
        writer.u8(1);
        writer.u32(u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut field_index_idx = 0u32;
        while (i32::from_ne_bytes(((field_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let field = (fields[usize::try_from(field_index_idx).unwrap_or(0)]).clone();
            writer.str((field.name).to_ustring().as_ustr());
            writer.str(JsCoerce::js_coerce_to_string((field.value).clone()).as_ustr());
            field_index_idx = u32::wrapping_add(field_index_idx, 1);
        }
    }
}

pub struct Writer {
    pub(crate) buf: BytesBuffer,
}

impl Writer {
    pub fn new() -> Self {
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
        self.buf.add(&UString::from("TQCS").as_bytes().to_vec());
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

    pub fn str(&mut self, value: &UStr) {
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
