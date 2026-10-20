use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::test::shaping_evidence::MetricsEvidenceEntry;
use crate::org::tiqian::test::shaping_evidence::MetricsEvidenceKey;
use crate::org::tiqian::test::shaping_evidence::RecordedFontMetrics;
use crate::org::tiqian::test::shaping_evidence::RecordedGlyph;
use crate::org::tiqian::test::shaping_evidence::RecordedShapingDecision;
use crate::org::tiqian::test::shaping_evidence::RecordedShapingResult;
use crate::org::tiqian::test::shaping_evidence::ShapingEvidence;
use crate::org::tiqian::test::shaping_evidence::ShapingEvidenceEntry;
use crate::org::tiqian::test::shaping_evidence::ShapingEvidenceKey;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Debug, Clone, PartialEq)]
pub struct JsonMember {
    pub name: UString,
    pub value: JsonValue,
}

impl JsonMember {
    pub fn new(name: &UStr, value: JsonValue) -> Self {
        Self {
            name: name.to_ustring(),
            value,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("JsonMember(")); __s += &(UString::from("name=")); __s += (self.name).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("value=")); __s += UString::from(format!("{}", { fn std_string_json_value0(v: &JsonValue) -> String { { let mut out = String::new(); match v { JsonValue::JNull => out.push_str("JNull"), JsonValue::JBool { v } => { let _ = write!(out, "JBool(v={})", v); }, JsonValue::JNum { v } => { let _ = write!(out, "JNum(v={})", v); }, JsonValue::JStr { v } => { let _ = write!(out, "JStr(v={})", v); }, JsonValue::JArr { v } => { let _ = write!(out, "JArr(v={})", {
        let mut out = String::new();
        out.push('[');
        let arr = v;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", std_string_json_value0(&arr[i]));
            i += 1;
        }
        out.push(']');
        out
    }); }, JsonValue::JObj { v } => { let _ = write!(out, "JObj(v={})", {
        let mut out = String::new();
        out.push('[');
        let arr = v;
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }); } }; out } } std_string_json_value0(&(self.value).clone()) }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

fn json_member_value_order(v: &JsonValue) -> i32 {
    match v {
        JsonValue::JStr { .. } => 3,
        JsonValue::JObj { .. } => 5,
        JsonValue::JNum { .. } => 2,
        JsonValue::JNull => 0,
        JsonValue::JBool { .. } => 1,
        JsonValue::JArr { .. } => 4,
    }
}
pub fn compare_json_member(a: &JsonMember, b: &JsonMember) -> i32 {
    let cmp_name = SortedTable::sorted_table_compare_strings(a.name.as_ustr(), b.name.as_ustr());
    if cmp_name != 0 { return cmp_name; }
    let cmp_value = match json_member_value_order(&a.value).cmp(&json_member_value_order(&b.value)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_value != 0 { return cmp_value; }
    0
}

#[derive(Clone, PartialEq)]
pub struct JsonCursor {
    pub pos: u32,
}

impl JsonCursor {
    pub fn new(pos: u32) -> Self {
        Self {
            pos,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ShapingEvidenceJson;

impl ShapingEvidenceJson {
    pub fn shaping_evidence_json_parse(text: &UStr) -> Result<ShapingEvidence, TextRangeError> {
        let mut cur = JsonCursor::new(0u32);
        ShapingEvidenceJson::shaping_evidence_json_skip_ws(&mut cur, text);
        let root = ShapingEvidenceJson::shaping_evidence_json_obj_required(ShapingEvidenceJson::shaping_evidence_json_parse_value(&mut cur, text)?, UStr::new(&[60,114,111,111,116,62]))?;
        let meta_obj = ShapingEvidenceJson::shaping_evidence_json_obj_required(ShapingEvidenceJson::shaping_evidence_json_field(&root, UStr::new(&[109,101,116,97]))?, UStr::new(&[109,101,116,97]))?;
        let mut meta: SortedMapTableBuilder<UString, UString> = SortedTable::sorted_table_map_builder::<UString, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        for i in 0..match u32::try_from(meta_obj.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            meta.put(&((meta_obj[usize::try_from(i).unwrap_or(0)]).clone().name).to_ustring(), &ShapingEvidenceJson::shaping_evidence_json_str_required(((meta_obj[usize::try_from(i).unwrap_or(0)]).clone().value).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("meta.")); __s += ((meta_obj[usize::try_from(i).unwrap_or(0)]).clone().name).to_ustring().as_ustr(); __s }).as_str()).as_ustr())?);
        }
        let shaping_arr = ShapingEvidenceJson::shaping_evidence_json_arr_required(ShapingEvidenceJson::shaping_evidence_json_field(&root, UStr::new(&[115,104,97,112,105,110,103]))?, UStr::new(&[115,104,97,112,105,110,103]))?;
        let mut shaping: Vec<ShapingEvidenceEntry> = vec![];
        for i in 0..match u32::try_from(shaping_arr.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let entry = ShapingEvidenceJson::shaping_evidence_json_obj_required((shaping_arr[usize::try_from(i).unwrap_or(0)]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("shaping[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("]")); __s }).as_str()).as_ustr())?;
            let key = ShapingEvidenceJson::shaping_evidence_json_parse_shaping_key(&ShapingEvidenceJson::shaping_evidence_json_obj_required(ShapingEvidenceJson::shaping_evidence_json_field(&entry, UStr::new(&[107,101,121]))?, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("shaping[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("].key")); __s }).as_str()).as_ustr())?)?;
            let result = ShapingEvidenceJson::shaping_evidence_json_parse_shaping_result(&ShapingEvidenceJson::shaping_evidence_json_obj_required(ShapingEvidenceJson::shaping_evidence_json_field(&entry, UStr::new(&[114,101,115,117,108,116]))?, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("shaping[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("].result")); __s }).as_str()).as_ustr())?)?;
            shaping.push(ShapingEvidenceEntry::new((key).clone(), (result).clone()));
        }
        let metrics_arr = ShapingEvidenceJson::shaping_evidence_json_arr_required(ShapingEvidenceJson::shaping_evidence_json_field(&root, UStr::new(&[109,101,116,114,105,99,115]))?, UStr::new(&[109,101,116,114,105,99,115]))?;
        let mut metrics: Vec<MetricsEvidenceEntry> = vec![];
        for i in 0..match u32::try_from(metrics_arr.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let entry = ShapingEvidenceJson::shaping_evidence_json_obj_required((metrics_arr[usize::try_from(i).unwrap_or(0)]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("metrics[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("]")); __s }).as_str()).as_ustr())?;
            let key = ShapingEvidenceJson::shaping_evidence_json_parse_metrics_key(&ShapingEvidenceJson::shaping_evidence_json_obj_required(ShapingEvidenceJson::shaping_evidence_json_field(&entry, UStr::new(&[107,101,121]))?, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("metrics[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("].key")); __s }).as_str()).as_ustr())?)?;
            let result = ShapingEvidenceJson::shaping_evidence_json_parse_font_metrics(&ShapingEvidenceJson::shaping_evidence_json_obj_required(ShapingEvidenceJson::shaping_evidence_json_field(&entry, UStr::new(&[114,101,115,117,108,116]))?, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("metrics[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("].result")); __s }).as_str()).as_ustr())?)?;
            metrics.push(MetricsEvidenceEntry::new((key).clone(), (result).clone()));
        }
        return Ok(ShapingEvidence::new(meta.clone().build(), shaping.to_vec(), metrics.to_vec()));
    }

    pub(crate) fn shaping_evidence_json_parse_shaping_key(obj: &Vec<JsonMember>) -> Result<ShapingEvidenceKey, TextRangeError> {
        return Ok(ShapingEvidenceKey::new(ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[100,105,115,112,108,97,121,84,101,120,116]))?, UStr::new(&[100,105,115,112,108,97,121,84,101,120,116]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,75,101,121]))?, UStr::new(&[102,111,110,116,75,101,121]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,70,97,109,105,108,121]))?, UStr::new(&[102,111,110,116,70,97,109,105,108,121]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[114,111,108,101]))?, UStr::new(&[114,111,108,101]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_array_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[115,116,121,108,101,70,111,110,116,70,97,109,105,108,105,101,115]))?, UStr::new(&[115,116,121,108,101,70,111,110,116,70,97,109,105,108,105,101,115]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,83,105,122,101]))?, UStr::new(&[102,111,110,116,83,105,122,101]))?, ShapingEvidenceJson::shaping_evidence_json_int_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,87,101,105,103,104,116]))?, UStr::new(&[102,111,110,116,87,101,105,103,104,116]))?, ShapingEvidenceJson::shaping_evidence_json_bool_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[105,116,97,108,105,99]))?, UStr::new(&[105,116,97,108,105,99]))?, ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[108,111,99,97,108,101]))?, UStr::new(&[108,111,99,97,108,101]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_array_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[111,112,101,110,84,121,112,101,70,101,97,116,117,114,101,115]))?, UStr::new(&[111,112,101,110,84,121,112,101,70,101,97,116,117,114,101,115]))?));
    }

    pub(crate) fn shaping_evidence_json_parse_shaping_result(obj: &Vec<JsonMember>) -> Result<RecordedShapingResult, TextRangeError> {
        let glyph_values = ShapingEvidenceJson::shaping_evidence_json_arr_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[103,108,121,112,104,115]))?, UStr::new(&[103,108,121,112,104,115]))?;
        let mut glyphs: Vec<RecordedGlyph> = vec![];
        for i in 0..match u32::try_from(glyph_values.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let g = ShapingEvidenceJson::shaping_evidence_json_obj_required((glyph_values[usize::try_from(i).unwrap_or(0)]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("glyphs[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("]")); __s }).as_str()).as_ustr())?;
            glyphs.push(RecordedGlyph::new(ShapingEvidenceJson::shaping_evidence_json_int_required(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[105,100]))?, UStr::new(&[105,100]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[97,100,118,97,110,99,101]))?, UStr::new(&[97,100,118,97,110,99,101]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[120]))?, UStr::new(&[120]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[121]))?, UStr::new(&[121]))?, ShapingEvidenceJson::shaping_evidence_json_rect_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[98,111,117,110,100,115]))?, UStr::new(&[98,111,117,110,100,115]))?, ShapingEvidenceJson::shaping_evidence_json_float_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[104,97,108,116,65,100,118,97,110,99,101]))?, UStr::new(&[104,97,108,116,65,100,118,97,110,99,101]))?, ShapingEvidenceJson::shaping_evidence_json_float_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&g, UStr::new(&[104,97,108,116,80,108,97,99,101,109,101,110,116,88]))?, UStr::new(&[104,97,108,116,80,108,97,99,101,109,101,110,116,88]))?));
        }
        let decision_values = ShapingEvidenceJson::shaping_evidence_json_arr_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[100,101,99,105,115,105,111,110,115]))?, UStr::new(&[100,101,99,105,115,105,111,110,115]))?;
        let mut decisions: Vec<RecordedShapingDecision> = vec![];
        for i in 0..match u32::try_from(decision_values.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ShapingEvidenceJson::shaping_evidence_json_obj_required((decision_values[usize::try_from(i).unwrap_or(0)]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("decisions[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("]")); __s }).as_str()).as_ustr())?;
            decisions.push(RecordedShapingDecision::new(ShapingEvidenceJson::shaping_evidence_json_int_required(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[103,108,121,112,104,67,111,117,110,116]))?, UStr::new(&[103,108,121,112,104,67,111,117,110,116]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[97,100,118,97,110,99,101]))?, UStr::new(&[97,100,118,97,110,99,101]))?, ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[115,111,117,114,99,101]))?, UStr::new(&[115,111,117,114,99,101]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[114,101,97,115,111,110]))?, UStr::new(&[114,101,97,115,111,110]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_int_required(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[103,108,121,112,104,115,87,105,116,104,111,117,116,73,110,107,66,111,117,110,100,115]))?, UStr::new(&[103,108,121,112,104,115,87,105,116,104,111,117,116,73,110,107,66,111,117,110,100,115]))?, ShapingEvidenceJson::shaping_evidence_json_int_required(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[109,105,115,115,105,110,103,71,108,121,112,104,115]))?, UStr::new(&[109,105,115,115,105,110,103,71,108,121,112,104,115]))?, ShapingEvidenceJson::shaping_evidence_json_str_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[114,101,115,111,108,118,101,100,70,97,99,101]))?, UStr::new(&[114,101,115,111,108,118,101,100,70,97,99,101]))?.clone(), ShapingEvidenceJson::shaping_evidence_json_str_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[115,99,114,105,112,116]))?, UStr::new(&[115,99,114,105,112,116]))?.clone(), ShapingEvidenceJson::shaping_evidence_json_str_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[108,97,110,103,117,97,103,101]))?, UStr::new(&[108,97,110,103,117,97,103,101]))?.clone(), ShapingEvidenceJson::shaping_evidence_json_str_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[115,116,114,97,116,101,103,121]))?, UStr::new(&[115,116,114,97,116,101,103,121]))?.clone(), ShapingEvidenceJson::shaping_evidence_json_str_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[102,101,97,116,117,114,101,69,118,105,100,101,110,99,101]))?, UStr::new(&[102,101,97,116,117,114,101,69,118,105,100,101,110,99,101]))?.clone(), ShapingEvidenceJson::shaping_evidence_json_str_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&d, UStr::new(&[99,97,112,97,98,105,108,105,116,121,73,115,115,117,101]))?, UStr::new(&[99,97,112,97,98,105,108,105,116,121,73,115,115,117,101]))?.clone()));
        }
        return Ok(RecordedShapingResult::new(ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[99,108,117,115,116,101,114,65,100,118,97,110,99,101]))?, UStr::new(&[99,108,117,115,116,101,114,65,100,118,97,110,99,101]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[114,117,110,65,100,118,97,110,99,101]))?, UStr::new(&[114,117,110,65,100,118,97,110,99,101]))?, ShapingEvidenceJson::shaping_evidence_json_str_array_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[114,117,110,70,101,97,116,117,114,101,115]))?, UStr::new(&[114,117,110,70,101,97,116,117,114,101,115]))?.to_vec(), glyphs.to_vec(), decisions.to_vec()));
    }

    pub(crate) fn shaping_evidence_json_parse_metrics_key(obj: &Vec<JsonMember>) -> Result<MetricsEvidenceKey, TextRangeError> {
        return Ok(MetricsEvidenceKey::new(ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,75,101,121]))?, UStr::new(&[102,111,110,116,75,101,121]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,83,105,122,101]))?, UStr::new(&[102,111,110,116,83,105,122,101]))?, ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[114,111,108,101]))?, UStr::new(&[114,111,108,101]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[108,111,99,97,108,101]))?, UStr::new(&[108,111,99,97,108,101]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_str_array_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,70,97,109,105,108,105,101,115]))?, UStr::new(&[102,111,110,116,70,97,109,105,108,105,101,115]))?, ShapingEvidenceJson::shaping_evidence_json_int_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,111,110,116,87,101,105,103,104,116]))?, UStr::new(&[102,111,110,116,87,101,105,103,104,116]))?, ShapingEvidenceJson::shaping_evidence_json_bool_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[105,116,97,108,105,99]))?, UStr::new(&[105,116,97,108,105,99]))?, ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[102,97,99,101,83,101,108,101,99,116,105,111,110,84,101,120,116]))?, UStr::new(&[102,97,99,101,83,101,108,101,99,116,105,111,110,84,101,120,116]))?.as_ustr()));
    }

    pub(crate) fn shaping_evidence_json_parse_font_metrics(obj: &Vec<JsonMember>) -> Result<RecordedFontMetrics, TextRangeError> {
        return Ok(RecordedFontMetrics::new(ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[97,115,99,101,110,116]))?, UStr::new(&[97,115,99,101,110,116]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[100,101,115,99,101,110,116]))?, UStr::new(&[100,101,115,99,101,110,116]))?, ShapingEvidenceJson::shaping_evidence_json_num_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[108,101,97,100,105,110,103]))?, UStr::new(&[108,101,97,100,105,110,103]))?, ShapingEvidenceJson::shaping_evidence_json_str_required(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[115,111,117,114,99,101]))?, UStr::new(&[115,111,117,114,99,101]))?.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_float_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[116,121,112,111,65,115,99,101,110,116]))?, UStr::new(&[116,121,112,111,65,115,99,101,110,116]))?, ShapingEvidenceJson::shaping_evidence_json_float_or_null(ShapingEvidenceJson::shaping_evidence_json_field(&obj, UStr::new(&[116,121,112,111,68,101,115,99,101,110,116]))?, UStr::new(&[116,121,112,111,68,101,115,99,101,110,116]))?));
    }

    pub(crate) fn shaping_evidence_json_field(obj: &Vec<JsonMember>, name: &UStr) -> Result<JsonValue, TextRangeError> {
        let mut found: Option<JsonValue> = None;
        for i in 0..match u32::try_from(obj.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if obj[usize::try_from(i).unwrap_or(0)].clone().name.to_ustring() == name {
                found = Some(((obj[usize::try_from(i).unwrap_or(0)]).clone().value).clone());
                break;
            }
        }
        if found == None {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Missing JSON field: ")); __s += name; __s }).as_str()) });
        }
        return Ok((found).unwrap());
    }

    pub(crate) fn shaping_evidence_json_str_required(v: JsonValue, name: &UStr) -> Result<UString, TextRangeError> {
        let value = match v {
    JsonValue::JNull => None,
    JsonValue::JBool { .. } => None,
    JsonValue::JNum { .. } => None,
    JsonValue::JStr { v: _p0 } => Some(_p0),
    JsonValue::JArr { .. } => None,
    JsonValue::JObj { .. } => None,
};
        if value.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected string")); __s }).as_str()) });
        }
        return Ok((value).as_deref().unwrap_or(UStr::new(&[])).to_ustring());
    }

    pub(crate) fn shaping_evidence_json_num_required(v: JsonValue, name: &UStr) -> Result<f64, TextRangeError> {
        let matched: bool;
        let mut value = 0.0f64;
        let _ = match v {
    JsonValue::JNull => matched = false,
    JsonValue::JBool { .. } => matched = false,
    JsonValue::JNum { v: _p0 } => {
    value = _p0;
    matched = true
},
    JsonValue::JStr { .. } => matched = false,
    JsonValue::JArr { .. } => matched = false,
    JsonValue::JObj { .. } => matched = false,
};
        if !matched {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected number")); __s }).as_str()) });
        }
        return Ok(value);
    }

    pub(crate) fn shaping_evidence_json_int_required(v: JsonValue, name: &UStr) -> Result<u32, TextRangeError> {
        return Ok(u32::from_ne_bytes(((match f64::from(ShapingEvidenceJson::shaping_evidence_json_num_required((v).clone(), name)?) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes()));
    }

    pub(crate) fn shaping_evidence_json_bool_required(v: JsonValue, name: &UStr) -> Result<bool, TextRangeError> {
        let matched: bool;
        let mut value = false;
        let _ = match v {
    JsonValue::JNull => matched = false,
    JsonValue::JBool { v: _p0 } => {
    value = _p0;
    matched = true
},
    JsonValue::JNum { .. } => matched = false,
    JsonValue::JStr { .. } => matched = false,
    JsonValue::JArr { .. } => matched = false,
    JsonValue::JObj { .. } => matched = false,
};
        if !matched {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected bool")); __s }).as_str()) });
        }
        return Ok(value);
    }

    pub(crate) fn shaping_evidence_json_str_array_required(v: JsonValue, name: &UStr) -> Result<Vec<UString>, TextRangeError> {
        let values = ShapingEvidenceJson::shaping_evidence_json_arr_required((v).clone(), name)?;
        let mut out: Vec<UString> = vec![];
        for i in 0..match u32::try_from(values.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            out.push(ShapingEvidenceJson::shaping_evidence_json_str_required((values[usize::try_from(i).unwrap_or(0)]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += name; __s += &(UString::from("[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("]")); __s }).as_str()).as_ustr())?);
        }
        return Ok(out);
    }

    pub(crate) fn shaping_evidence_json_str_or_null(v: JsonValue, name: &UStr) -> Result<Option<UString>, TextRangeError> {
        let mut null_kind = false;
        let mut value: Option<UString> = None;
        let _ = match v {
    JsonValue::JNull => null_kind = true,
    JsonValue::JBool { .. } => value = None,
    JsonValue::JNum { .. } => value = None,
    JsonValue::JStr { v: _p0 } => value = Some(_p0.clone()),
    JsonValue::JArr { .. } => value = None,
    JsonValue::JObj { .. } => value = None,
};
        if !null_kind && value.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected string or null")); __s }).as_str()) });
        }
        return Ok(value);
    }

    pub(crate) fn shaping_evidence_json_float_or_null(v: JsonValue, name: &UStr) -> Result<Option<f64>, TextRangeError> {
        let mut null_kind = false;
        let mut value: Option<f64> = None;
        let _ = match v {
    JsonValue::JNull => null_kind = true,
    JsonValue::JBool { .. } => value = None,
    JsonValue::JNum { v: _p0 } => value = Some(_p0),
    JsonValue::JStr { .. } => value = None,
    JsonValue::JArr { .. } => value = None,
    JsonValue::JObj { .. } => value = None,
};
        if !null_kind && value.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected number or null")); __s }).as_str()) });
        }
        return Ok(value);
    }

    pub(crate) fn shaping_evidence_json_rect_or_null(v: JsonValue, name: &UStr) -> Result<Option<Rect>, TextRangeError> {
        let values = ShapingEvidenceJson::shaping_evidence_json_arr_or_null((v).clone(), name)?;
        if values.is_none() {
            return Ok(None);
        }
        if values.as_ref().map_or(0, |v| v.len()) != 4 {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected 4 bounds values, got ")); __s += &(UString::from(format!("{}", ((values).as_ref().map_or(0, |v| v.len())).to_string()).as_str())); __s }).as_str()) });
        }
        return Ok(Some(Rect::new(ShapingEvidenceJson::shaping_evidence_json_num_required(((values).as_ref().unwrap()[0usize]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += name; __s += &(UString::from("[0]")); __s }).as_str()).as_ustr())?, ShapingEvidenceJson::shaping_evidence_json_num_required(((values).as_ref().unwrap()[1usize]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += name; __s += &(UString::from("[1]")); __s }).as_str()).as_ustr())?, ShapingEvidenceJson::shaping_evidence_json_num_required(((values).as_ref().unwrap()[2usize]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += name; __s += &(UString::from("[2]")); __s }).as_str()).as_ustr())?, ShapingEvidenceJson::shaping_evidence_json_num_required(((values).as_ref().unwrap()[3usize]).clone(), UString::from(format!("{}", { let mut __s = UString::new(); __s += name; __s += &(UString::from("[3]")); __s }).as_str()).as_ustr())?)));
    }

    pub(crate) fn shaping_evidence_json_arr_or_null(v: JsonValue, name: &UStr) -> Result<Option<Vec<JsonValue>>, TextRangeError> {
        let mut null_kind = false;
        let mut value: Option<Vec<JsonValue>> = None;
        let _ = match v {
    JsonValue::JNull => null_kind = true,
    JsonValue::JBool { .. } => value = None,
    JsonValue::JNum { .. } => value = None,
    JsonValue::JStr { .. } => value = None,
    JsonValue::JArr { v: _p0 } => value = Some(_p0.clone()),
    JsonValue::JObj { .. } => value = None,
};
        if !null_kind && value.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected array or null")); __s }).as_str()) });
        }
        return Ok(value);
    }

    pub(crate) fn shaping_evidence_json_arr_required(v: JsonValue, name: &UStr) -> Result<Vec<JsonValue>, TextRangeError> {
        let value = match v {
    JsonValue::JNull => None,
    JsonValue::JBool { .. } => None,
    JsonValue::JNum { .. } => None,
    JsonValue::JStr { .. } => None,
    JsonValue::JArr { v: _p0 } => Some(_p0),
    JsonValue::JObj { .. } => None,
};
        if value.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected array")); __s }).as_str()) });
        }
        return Ok((value).unwrap());
    }

    pub(crate) fn shaping_evidence_json_obj_required(v: JsonValue, name: &UStr) -> Result<Vec<JsonMember>, TextRangeError> {
        let value = match v {
    JsonValue::JNull => None,
    JsonValue::JBool { .. } => None,
    JsonValue::JNum { .. } => None,
    JsonValue::JStr { .. } => None,
    JsonValue::JArr { .. } => None,
    JsonValue::JObj { v: _p0 } => Some(_p0),
};
        if value.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Field ")); __s += name; __s += &(UString::from(": expected object")); __s }).as_str()) });
        }
        return Ok((value).unwrap());
    }

    pub(crate) fn shaping_evidence_json_skip_ws(cur: &mut JsonCursor, text: &UStr) {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut scanning = true;
        while scanning {
            if i32::from_ne_bytes(((cur.pos) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((__count) as i32).to_ne_bytes()) {
            }
            let c = u_string::unit_at_from(&__units, cur.pos).unwrap_or(0);
            if c == 32 || c == 9 || c == 10 || c == 13 {
                cur.pos = u32::wrapping_add(cur.pos, 1);
            } else {
                scanning = false;
            }
        }
    }

    pub(crate) fn shaping_evidence_json_parse_value(cur: &mut JsonCursor, text: &UStr) -> Result<JsonValue, TextRangeError> {
        ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
        let c = u_string::unit_at(&text, cur.pos).unwrap_or(0);
        if c == 123 {
            return Ok(JsonValue::JObj { v: ShapingEvidenceJson::shaping_evidence_json_parse_object(cur, text)?.to_vec() });
        }
        if c == 91 {
            return Ok(JsonValue::JArr { v: ShapingEvidenceJson::shaping_evidence_json_parse_array(cur, text)?.to_vec() });
        }
        if c == 34 {
            return Ok(JsonValue::JStr { v: ShapingEvidenceJson::shaping_evidence_json_parse_string(cur, text)?.to_ustring() });
        }
        if u_string::substr(&text, i32::from_ne_bytes(((cur.pos) as i32).to_ne_bytes()), Some(4i32)) == UString::from("true") {
            cur.pos = u32::wrapping_add(cur.pos, 4);
            return Ok(JsonValue::JBool { v: true });
        }
        if u_string::substr(&text, i32::from_ne_bytes(((cur.pos) as i32).to_ne_bytes()), Some(5i32)) == UString::from("false") {
            cur.pos = u32::wrapping_add(cur.pos, 5);
            return Ok(JsonValue::JBool { v: false });
        }
        if u_string::substr(&text, i32::from_ne_bytes(((cur.pos) as i32).to_ne_bytes()), Some(4i32)) == UString::from("null") {
            cur.pos = u32::wrapping_add(cur.pos, 4);
            return Ok((JsonValue::JNull).clone());
        }
        return Ok(JsonValue::JNum { v: ShapingEvidenceJson::shaping_evidence_json_parse_number(cur, text)? });
    }

    pub(crate) fn shaping_evidence_json_parse_object(cur: &mut JsonCursor, text: &UStr) -> Result<Vec<JsonMember>, TextRangeError> {
    let __units1 = u_string::units(&text);
    let __count1 = u_string::unit_count(&text);
        let mut members: Vec<JsonMember> = vec![];
        cur.pos = u32::wrapping_add(cur.pos, 1);
        ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
        let mut closed = false;
        if u_string::unit_at_from(&__units1, cur.pos).as_ref().map_or(false, |v| v == &(125)) {
            cur.pos = u32::wrapping_add(cur.pos, 1);
            closed = true;
        }
        while !closed {
            ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
            if !(u_string::unit_at_from(&__units1, cur.pos).as_ref().map_or(false, |v| v == &(34))) {
                return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: expected member name at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cur.pos)).as_str())); __s }).as_str()) });
            }
            let name = ShapingEvidenceJson::shaping_evidence_json_parse_string(cur, text)?;
            ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
            if !(u_string::unit_at_from(&__units1, cur.pos).as_ref().map_or(false, |v| v == &(58))) {
                return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: expected ':' at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cur.pos)).as_str())); __s }).as_str()) });
            }
            cur.pos = u32::wrapping_add(cur.pos, 1);
            ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
            members.push(JsonMember::new(name.as_ustr(), ShapingEvidenceJson::shaping_evidence_json_parse_value(cur, text)?));
            ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
            let c = u_string::unit_at_from(&__units1, cur.pos).unwrap_or(0);
            if c == 44 {
                cur.pos = u32::wrapping_add(cur.pos, 1);
            } else {
                if c == 125 {
                    cur.pos = u32::wrapping_add(cur.pos, 1);
                    closed = true;
                } else {
                    return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: expected ',' or '}' at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cur.pos)).as_str())); __s }).as_str()) });
                }
            }
        }
        return Ok(members);
    }

    pub(crate) fn shaping_evidence_json_parse_array(cur: &mut JsonCursor, text: &UStr) -> Result<Vec<JsonValue>, TextRangeError> {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        let mut values: Vec<JsonValue> = vec![];
        cur.pos = u32::wrapping_add(cur.pos, 1);
        ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
        let mut closed = false;
        if u_string::unit_at_from(&__units2, cur.pos).as_ref().map_or(false, |v| v == &(93)) {
            cur.pos = u32::wrapping_add(cur.pos, 1);
            closed = true;
        }
        while !closed {
            ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
            values.push(ShapingEvidenceJson::shaping_evidence_json_parse_value(cur, text)?);
            ShapingEvidenceJson::shaping_evidence_json_skip_ws(cur, text);
            let c = u_string::unit_at_from(&__units2, cur.pos).unwrap_or(0);
            if c == 44 {
                cur.pos = u32::wrapping_add(cur.pos, 1);
            } else {
                if c == 93 {
                    cur.pos = u32::wrapping_add(cur.pos, 1);
                    closed = true;
                } else {
                    return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: expected ',' or ']' at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cur.pos)).as_str())); __s }).as_str()) });
                }
            }
        }
        return Ok(values);
    }

    pub(crate) fn shaping_evidence_json_parse_string(cur: &mut JsonCursor, text: &UStr) -> Result<UString, TextRangeError> {
    let __units3 = u_string::units(&text);
    let __count3 = u_string::unit_count(&text);
        cur.pos = u32::wrapping_add(cur.pos, 1);
        let mut buf_b = UString::new();
        let mut done = false;
        while !done {
            let c = u_string::unit_at_from(&__units3, cur.pos).unwrap_or(0);
            if c == 34 {
                cur.pos = u32::wrapping_add(cur.pos, 1);
                done = true;
            } else {
                if c == 92 {
                    cur.pos = u32::wrapping_add(cur.pos, 1);
                    let e = u_string::unit_at_from(&__units3, cur.pos).unwrap_or(0);
                    if e == 34 {
                        buf_b += &(UString::from("\""));
                        cur.pos = u32::wrapping_add(cur.pos, 1);
                    } else {
                        if e == 92 {
                            buf_b += &(UString::from("\\"));
                            cur.pos = u32::wrapping_add(cur.pos, 1);
                        } else {
                            if e == 47 {
                                buf_b += &(UString::from("/"));
                                cur.pos = u32::wrapping_add(cur.pos, 1);
                            } else {
                                if e == 98 {
                                    buf_b += &(UString::from(""));
                                    cur.pos = u32::wrapping_add(cur.pos, 1);
                                } else {
                                    if e == 102 {
                                        buf_b += &(UString::from(""));
                                        cur.pos = u32::wrapping_add(cur.pos, 1);
                                    } else {
                                        if e == 110 {
                                            buf_b += &(UString::from(concat!("\n",
"")));
                                            cur.pos = u32::wrapping_add(cur.pos, 1);
                                        } else {
                                            if e == 114 {
                                                buf_b += &(UString::from("\r"));
                                                cur.pos = u32::wrapping_add(cur.pos, 1);
                                            } else {
                                                if e == 116 {
                                                    buf_b += &(UString::from("\t"));
                                                    cur.pos = u32::wrapping_add(cur.pos, 1);
                                                } else {
                                                    if e == 117 {
                                                        {
                                                            let c = ShapingEvidenceJson::shaping_evidence_json_parse_hex4(text, u32::wrapping_add(cur.pos, 1))?;
                                                            buf_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
                                                        }
                                                        cur.pos = u32::wrapping_add(cur.pos, 5);
                                                    } else {
                                                        return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: unsupported escape at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cur.pos)).as_str())); __s }).as_str()) });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let c = c;
                        buf_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
                    }
                    cur.pos = u32::wrapping_add(cur.pos, 1);
                }
            }
        }
        return Ok(buf_b);
    }

    pub(crate) fn shaping_evidence_json_parse_hex4(text: &UStr, start: u32) -> Result<u32, TextRangeError> {
        let mut value = 0u32;
        for i in 0..4 {
            let c = u_string::unit_at(&text, u32::wrapping_add(start, i)).unwrap_or(0);
            let digit: u32;
            if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= 48 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 57 {
                digit = u32::wrapping_sub(c, 48);
            } else {
                if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= 97 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 102 {
                    digit = u32::wrapping_add(u32::wrapping_sub(c, 97), 10);
                } else {
                    if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= 65 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 70 {
                        digit = u32::wrapping_add(u32::wrapping_sub(c, 65), 10);
                    } else {
                        return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: bad \\u escape at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(start)).as_str())); __s }).as_str()) });
                    }
                }
            }
            value = u32::wrapping_add(u32::wrapping_mul(value, 16), digit);
        }
        return Ok(value);
    }

    pub(crate) fn shaping_evidence_json_parse_number(cur: &mut JsonCursor, text: &UStr) -> Result<f64, TextRangeError> {
    let __units4 = u_string::units(&text);
    let __count4 = u_string::unit_count(&text);
        let start = cur.pos;
        let mut scanning = true;
        while scanning {
            if i32::from_ne_bytes(((cur.pos) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((__count4) as i32).to_ne_bytes()) {
            }
            let c = u_string::unit_at_from(&__units4, cur.pos).unwrap_or(0);
            if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) >= 48 && (i32::from_ne_bytes(((c) as i32).to_ne_bytes())) <= 57 || c == 45 || c == 43 || c == 46 || c == 101 || c == 69 {
                cur.pos = u32::wrapping_add(cur.pos, 1);
            } else {
                scanning = false;
            }
        }
        if cur.pos == start {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Malformed JSON: expected value at offset ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(start)).as_str())); __s }).as_str()) });
        }
        return Ok(u_string::parse_f64(&(u_string::substring(&text, i32::from_ne_bytes(((start) as i32).to_ne_bytes()), i32::from_ne_bytes(((cur.pos) as i32).to_ne_bytes())))));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    JNull,
    JBool { v: bool },
    JNum { v: f64 },
    JStr { v: UString },
    JArr { v: Vec<JsonValue> },
    JObj { v: Vec<JsonMember> },
}

impl JsonValue {
    pub fn to_string(&self) -> String {
        match self {
            JsonValue::JNull => "JNull".to_string(),
            JsonValue::JBool { v } => format!("JBool(v={})", (v).to_string()),
            JsonValue::JNum { v } => format!("JNum(v={})", (v).to_string()),
            JsonValue::JStr { v } => format!("JStr(v={})", (v).clone()),
            JsonValue::JArr { v } => format!("JArr(v={})", {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < v.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", (v[j0]).to_string());
                j0 += 1;
            }
            out.push(']');
            out
        }),
            JsonValue::JObj { v } => format!("JObj(v={})", {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < v.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", (v[j0]).to_string());
                j0 += 1;
            }
            out.push(']');
            out
        }),
        }
    }
}
