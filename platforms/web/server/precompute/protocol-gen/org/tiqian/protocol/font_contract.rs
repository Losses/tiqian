use crate::org::tiqian::protocol::canonical::Canonical;
use crate::org::tiqian::protocol::js_coerce::JsCoerce;
use crate::org::tiqian::protocol::snapshot_typography::SnapshotTypography;
use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct FontContract;

impl FontContract {
    pub fn font_contract_required_cjk_dash_contract_input(key: WireValue, text: WireValue, text_spans: WireValue, typography: SnapshotTypography) -> Option<WireValue> {
        let text_string = if FontContract::font_contract_is_wire_null((text).clone()) { UString::from("") } else { JsCoerce::js_coerce_to_string((text).clone()).to_ustring() };
        let spans = FontContract::font_contract_arr_of((text_spans).clone());
        let mut order: Vec<u32> = vec![];
        let mut signatures: Vec<UString> = vec![];
        let mut styles: Vec<DashStyle> = vec![];
        let mut dashes_per_group: Vec<Vec<UString>> = vec![];
        let mut offset = 0.0f64;
        let mut i = 0u32;
        let len = u_string::unit_count(&(text_string));
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((len) as i32).to_ne_bytes())) {
            let code = u_string::unit_at(&text_string, i).unwrap_or(0);
            let mut dash = UString::new();
            if code == 8212 && (i32::from_ne_bytes(((u32::wrapping_add(i, 1)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((len) as i32).to_ne_bytes())) && matches!(u_string::unit_at(&text_string, u32::wrapping_add(i, 1)).as_ref(), Some(__v) if __v == &(8212)) {
                dash = UString::from("——").to_ustring();
            } else {
                if code == 11834 {
                    dash = UString::from("⸺").to_ustring();
                }
            }
            if dash != UString::from("") {
                let style = FontContract::font_contract_resolved_style_at(offset, &spans, (typography).clone());
                let signature = JsCoerce::js_coerce_render_json(FontContract::font_contract_dash_style_to_json((style).clone()));
                let mut index = 0u32;
                let mut found = false;
                let mut search_idx = 0u32;
                while (i32::from_ne_bytes(((search_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((signatures.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    if signatures[usize::try_from(search_idx).unwrap_or(0)].clone() == signature {
                        index = search_idx;
                        found = true;
                        break;
                    }
                    search_idx = u32::wrapping_add(search_idx, 1);
                }
                if !found {
                    index = u32::try_from((styles.len()) & 0xFFFF_FFFF).unwrap_or(0);
                    styles.push(style.clone());
                    dashes_per_group.push(vec![]);
                    order.push(index);
                    signatures.push(signature.clone());
                }
                let mut seen = false;
                let mut dash_idx = 0u32;
                while (i32::from_ne_bytes(((dash_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((dashes_per_group[usize::try_from(index).unwrap_or(0)]).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    if dashes_per_group[usize::try_from(index).unwrap_or(0)].clone()[usize::try_from(dash_idx).unwrap_or(0)].clone() == dash {
                        seen = true;
                        break;
                    }
                    dash_idx = u32::wrapping_add(dash_idx, 1);
                }
                if !seen {
                    dashes_per_group[usize::try_from(index).unwrap_or(0)].push(dash.clone());
                }
                if dash == UString::from("——") {
                    offset += 2.0f64;
                    i = u32::wrapping_add(i, 2);
                } else {
                    offset += 1.0f64;
                    i = u32::wrapping_add(i, 1);
                }
            } else {
                if i32::from_ne_bytes(((code) as i32).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes(((code) as i32).to_ne_bytes())) <= 56319 {
                    offset += 2.0f64;
                    i = u32::wrapping_add(i, 2);
                } else {
                    offset += 1.0f64;
                    i = u32::wrapping_add(i, 1);
                }
            }
        }
        if u32::try_from((styles.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return None;
        }
        let mut probe = UString::new();
        let mut probe_len = 0.0f64;
        let mut out_spans: Vec<WireValue> = vec![];
        for &index in &order {
            {
                let mut _g = 0u32;
                let _g1 = (dashes_per_group[usize::try_from(index).unwrap_or(0)]).clone();
                while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let dash = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                    _g = u32::wrapping_add(_g, 1);
                    let start = probe_len;
                    probe += &(dash);
                    probe_len += if dash == UString::from("——") { 2.0f64 } else { 1.0f64 };
                    let end = probe_len;
                    let style_fields = FontContract::font_contract_obj_fields(FontContract::font_contract_dash_style_to_json((styles[usize::try_from(index).unwrap_or(0)]).clone()));
                    let mut fields = vec![
    (WireField { name: UString::from("start").to_ustring(), value: WireValue::WNum { value: start } }).clone(),
    (WireField { name: UString::from("end").to_ustring(), value: WireValue::WNum { value: end } }).clone(),
];
                    {
                        let mut _g = 0u32;
                        while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((style_fields.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                            let sf = (style_fields[usize::try_from(_g).unwrap_or(0)]).clone();
                            _g = u32::wrapping_add(_g, 1);
                            fields.push(sf.clone());
                        }
                    }
                    out_spans.push(WireValue::WObj { fields: fields.to_vec() });
                }
            }
        }
        return Some(WireValue::WObj { fields: vec![
    (WireField { name: UString::from("key").to_ustring(), value: WireValue::WStr { value: if FontContract::font_contract_is_wire_null((key).clone()) { UString::from("") } else { JsCoerce::js_coerce_to_string((key).clone()).to_ustring() }.to_ustring() } }).clone(),
    (WireField { name: UString::from("text").to_ustring(), value: WireValue::WStr { value: probe.to_ustring() } }).clone(),
    (WireField { name: UString::from("textSpans").to_ustring(), value: WireValue::WArr { items: out_spans.to_vec() } }).clone(),
].to_vec() });
    }

    pub(crate) fn font_contract_matched_span_at(text_offset: f64, spans: &Vec<WireValue>) -> WireValue {
        let n = u32::try_from((spans.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let mut k = 0u32;
        while (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((n) as i32).to_ne_bytes())) {
            let candidate = (spans[usize::try_from(u32::wrapping_sub(u32::wrapping_sub(n, 1), k)).unwrap_or(0)]).clone();
            let start_raw = Canonical::canonical_member((candidate).clone(), UStr::new(&[115,116,97,114,116]));
            let end_raw = Canonical::canonical_member((candidate).clone(), UStr::new(&[101,110,100]));
            if FontContract::font_contract_is_raw_number((start_raw).clone()) && FontContract::font_contract_is_raw_number((end_raw).clone()) {
                let start = FontContract::font_contract_raw_number((start_raw).clone());
                let end = FontContract::font_contract_raw_number((end_raw).clone());
                if FontContract::font_contract_is_safe_integer(start) && FontContract::font_contract_is_safe_integer(end) && (text_offset) >= start && (text_offset) < (end) {
                    return candidate;
                }
            }
            k = u32::wrapping_add(k, 1);
        }
        return (WireValue::WNull).clone();
    }

    pub(crate) fn font_contract_resolved_style_at(text_offset: f64, spans: &Vec<WireValue>, typography: SnapshotTypography) -> DashStyle {
        let matched = FontContract::font_contract_matched_span_at(text_offset, &spans);
        let mut font_families = typography.font_families.clone();
        let families_raw = Canonical::canonical_member((matched).clone(), UStr::new(&[102,111,110,116,70,97,109,105,108,105,101,115]));
        if FontContract::font_contract_is_wire_arr((families_raw).clone()) {
            font_families = FontContract::font_contract_map_strings(&FontContract::font_contract_arr_of((families_raw).clone()));
        }
        let font_size_px = FontContract::font_contract_coerce_number_or(Canonical::canonical_member((matched).clone(), UStr::new(&[102,111,110,116,83,105,122,101,80,120])), typography.font_size_px);
        let font_weight = FontContract::font_contract_coerce_number_or(Canonical::canonical_member((matched).clone(), UStr::new(&[102,111,110,116,87,101,105,103,104,116])), format!("{}", (i32::from_ne_bytes(((typography.font_weight) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 1.0f64);
        let italic = FontContract::font_contract_italic_or(Canonical::canonical_member((matched).clone(), UStr::new(&[105,116,97,108,105,99])), typography.italic);
        let baseline_shift_px = FontContract::font_contract_coerce_number_or(Canonical::canonical_member((matched).clone(), UStr::new(&[98,97,115,101,108,105,110,101,83,104,105,102,116,80,120])), 0.0f64);
        return DashStyle { font_families: font_families, font_size_px: font_size_px, font_weight: font_weight, italic: italic, baseline_shift_px: baseline_shift_px };
    }

    pub(crate) fn font_contract_dash_style_to_json(style: DashStyle) -> WireValue {
        let mut families: Vec<WireValue> = vec![];
        {
            let _g1 = style.font_families.clone();
            for family in &_g1 {
                families.push(WireValue::WStr { value: family.to_ustring() });
            }
        }
        return WireValue::WObj { fields: vec![
    (WireField { name: UString::from("fontFamilies").to_ustring(), value: WireValue::WArr { items: families.to_vec() } }).clone(),
    (WireField { name: UString::from("fontSizePx").to_ustring(), value: WireValue::WNum { value: style.font_size_px } }).clone(),
    (WireField { name: UString::from("fontWeight").to_ustring(), value: WireValue::WNum { value: style.font_weight } }).clone(),
    (WireField { name: UString::from("italic").to_ustring(), value: style.italic }).clone(),
    (WireField { name: UString::from("baselineShiftPx").to_ustring(), value: WireValue::WNum { value: style.baseline_shift_px } }).clone(),
].to_vec() };
    }

    pub(crate) fn font_contract_map_strings(items: &Vec<WireValue>) -> Vec<UString> {
        let mut out: Vec<UString> = vec![];
        for item in items {
            out.push(JsCoerce::js_coerce_to_string((*item).clone()));
        }
        return out;
    }

    pub(crate) fn font_contract_is_wire_null(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => true,
            WireValue::WBool { .. } => false,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => false,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn font_contract_is_wire_arr(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => false,
            WireValue::WBool { .. } => false,
            WireValue::WNum { .. } => false,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => true,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn font_contract_arr_of(value: WireValue) -> Vec<WireValue> {
        return match value {
            WireValue::WNull => Vec::new(),
            WireValue::WBool { .. } => Vec::new(),
            WireValue::WNum { .. } => Vec::new(),
            WireValue::WStr { .. } => Vec::new(),
            WireValue::WArr { items: _p0 } => _p0,
            WireValue::WObj { .. } => Vec::new(),
        };
    }

    pub(crate) fn font_contract_obj_fields(value: WireValue) -> Vec<WireField> {
        return match value {
            WireValue::WNull => Vec::new(),
            WireValue::WBool { .. } => Vec::new(),
            WireValue::WNum { .. } => Vec::new(),
            WireValue::WStr { .. } => Vec::new(),
            WireValue::WArr { .. } => Vec::new(),
            WireValue::WObj { fields: _p0 } => _p0,
        };
    }

    pub(crate) fn font_contract_is_raw_number(value: WireValue) -> bool {
        return match value {
            WireValue::WNull => false,
            WireValue::WBool { .. } => false,
            WireValue::WNum { .. } => true,
            WireValue::WStr { .. } => false,
            WireValue::WArr { .. } => false,
            WireValue::WObj { .. } => false,
        };
    }

    pub(crate) fn font_contract_raw_number(value: WireValue) -> f64 {
        return match value {
            WireValue::WNull => 0.0f64,
            WireValue::WBool { .. } => 0.0f64,
            WireValue::WNum { value: _p0 } => _p0,
            WireValue::WStr { .. } => 0.0f64,
            WireValue::WArr { .. } => 0.0f64,
            WireValue::WObj { .. } => 0.0f64,
        };
    }

    pub(crate) fn font_contract_coerce_number_or(value: WireValue, fallback: f64) -> f64 {
        return match value {
            WireValue::WNull => fallback,
            WireValue::WBool { value: _p0 } => if _p0 { 1.0f64 } else { 0.0f64 },
            WireValue::WNum { value: _p0 } => _p0,
            WireValue::WStr { value: _p0 } => JsCoerce::js_coerce_to_number_from_string(_p0.as_ustr()),
            WireValue::WArr { .. } => JsCoerce::js_coerce_to_number((value).clone()),
            WireValue::WObj { .. } => JsCoerce::js_coerce_to_number((value).clone()),
        };
    }

    pub(crate) fn font_contract_italic_or(value: WireValue, fallback: bool) -> WireValue {
        return match value {
            WireValue::WNull => WireValue::WBool { value: fallback },
            WireValue::WBool { value: _p0 } => WireValue::WBool { value: _p0 },
            WireValue::WNum { value: _p0 } => WireValue::WNum { value: _p0 },
            WireValue::WStr { value: _p0 } => WireValue::WStr { value: _p0.to_ustring() },
            WireValue::WArr { items: _p0 } => WireValue::WArr { items: _p0.to_vec() },
            WireValue::WObj { fields: _p0 } => WireValue::WObj { fields: _p0.to_vec() },
        };
    }

    pub(crate) fn font_contract_is_safe_integer(value: f64) -> bool {
        return value % 1.0f64 == 0.0f64 && ((value).abs()) <= 9007199254740991.0f64;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DashStyle {
    pub font_families: Vec<UString>,
    pub font_size_px: f64,
    pub font_weight: f64,
    pub italic: WireValue,
    pub baseline_shift_px: f64,
}
