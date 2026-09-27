use crate::org::tiqian::protocol::canonical::Canonical;
use crate::org::tiqian::protocol::encode_result::EncodeResult;
use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct CanonicalTestSupport;

impl CanonicalTestSupport {
    pub fn canonical_test_support_bytes_of(result: EncodeResult) -> Option<Vec<u8>> {
        return match result {
            EncodeResult::COk { bytes: _p0 } => Some(_p0),
            EncodeResult::CErr { .. } => None,
        };
    }

    pub fn canonical_test_support_hex_bytes(bytes: &[u8]) -> String {
        let digits = "0123456789abcdef".to_string();
        let mut out_b = String::new();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let byte = u32::from(bytes[usize::try_from(index_idx).unwrap_or(0)]);
            {
                let c = u_string::unit_at(&digits, byte >> 4 & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { String::from_utf16(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(c) as u16]) });
            }
            {
                let c = u_string::unit_at(&digits, byte & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { String::from_utf16(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(c) as u16]) });
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return out_b;
    }

    pub fn canonical_test_support_hex_of(result: EncodeResult) -> String {
        return match result {
            EncodeResult::COk { bytes: _p0 } => CanonicalTestSupport::canonical_test_support_hex_bytes(&_p0),
            EncodeResult::CErr { .. } => "<encode failed>".to_string(),
        };
    }

    pub fn canonical_test_support_hex_of_digest(result: EncodeResult) -> String {
        return match result {
            EncodeResult::COk { bytes: _p0 } => CanonicalTestSupport::canonical_test_support_hex_bytes(&Canonical::canonical_digest(&_p0)),
            EncodeResult::CErr { .. } => "<encode failed>".to_string(),
        };
    }

    pub fn canonical_test_support_wire_obj(key: &str, text: &str, width: WireValue) -> WireValue {
        let mut fields = vec![
    (WireField { name: "key".to_string(), value: WireValue::WStr { value: key.to_string() } }).clone(),
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: text.to_string() } }).clone(),
];
        if true {
            fields.push(WireField { name: "maxWidthPx".to_string(), value: width });
        }
        return WireValue::WObj { fields: fields.to_vec() };
    }

    pub fn canonical_test_support_full_vector_input() -> WireValue {
        return WireValue::WObj { fields: vec![
    (WireField { name: "key".to_string(), value: WireValue::WStr { value: "p-2".to_string() } }).clone(),
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: "中文字排版".to_string() } }).clone(),
    (WireField { name: "maxWidthPx".to_string(), value: WireValue::WNum { value: 144 } }).clone(),
    (WireField { name: "semantics".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: "tagName".to_string(), value: WireValue::WStr { value: "a".to_string() } }).clone(),
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 2 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 4 } }).clone(),
    (WireField { name: "attributes".to_string(), value: WireValue::WObj { fields: vec![
    (WireField { name: "href".to_string(), value: WireValue::WStr { value: "https://example.com".to_string() } }).clone(),
    (WireField { name: "class".to_string(), value: WireValue::WStr { value: "link".to_string() } }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "order".to_string(), value: WireValue::WNum { value: 1 } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: "tagName".to_string(), value: WireValue::WStr { value: "em".to_string() } }).clone(),
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 0 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 1 } }).clone(),
].to_vec() }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "textSpans".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 0 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 2 } }).clone(),
    (WireField { name: "fontFamilies".to_string(), value: WireValue::WArr { items: vec![(WireValue::WStr { value: "Dela Gothic One".to_string() }).clone()].to_vec() } }).clone(),
    (WireField { name: "fontSizePx".to_string(), value: WireValue::WNum { value: 18 } }).clone(),
    (WireField { name: "fontWeight".to_string(), value: WireValue::WNum { value: 400 } }).clone(),
    (WireField { name: "italic".to_string(), value: WireValue::WBool { value: true } }).clone(),
    (WireField { name: "baselineShiftPx".to_string(), value: WireValue::WNum { value: -0.5f64 } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 2 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 4 } }).clone(),
    (WireField { name: "fontFamilies".to_string(), value: WireValue::WArr { items: vec![].to_vec() } }).clone(),
].to_vec() }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "inlineBoxes".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 1 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 2 } }).clone(),
    (WireField { name: "inlineStartPx".to_string(), value: WireValue::WNum { value: 8 } }).clone(),
    (WireField { name: "inlineEndPx".to_string(), value: WireValue::WNum { value: 4 } }).clone(),
    (WireField { name: "outerSpacing".to_string(), value: WireValue::WStr { value: "Source".to_string() } }).clone(),
].to_vec() }).clone(),
    (WireValue::WObj { fields: vec![
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 3 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 4 } }).clone(),
].to_vec() }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "sourceBoundaries".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WNum { value: 0 }).clone(),
    (WireValue::WNum { value: 18 }).clone(),
    (WireValue::WNum { value: 36 }).clone(),
].to_vec() } }).clone(),
].to_vec() };
    }

    pub fn canonical_test_support_loose_vector_input() -> WireValue {
        return WireValue::WObj { fields: vec![
    (WireField { name: "key".to_string(), value: WireValue::WStr { value: "p-3".to_string() } }).clone(),
    (WireField { name: "text".to_string(), value: WireValue::WStr { value: " coerce ".to_string() } }).clone(),
    (WireField { name: "maxWidthPx".to_string(), value: WireValue::WStr { value: "144.5".to_string() } }).clone(),
    (WireField { name: "semantics".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: "tagName".to_string(), value: WireValue::WStr { value: "i".to_string() } }).clone(),
    (WireField { name: "start".to_string(), value: WireValue::WStr { value: "1".to_string() } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 2 } }).clone(),
    (WireField { name: "attributes".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WArr { items: vec![
    (WireValue::WStr { value: "a".to_string() }).clone(),
    (WireValue::WStr { value: "1".to_string() }).clone(),
].to_vec() }).clone(),
    (WireValue::WArr { items: vec![(WireValue::WStr { value: "b".to_string() }).clone(), (WireValue::WNum { value: 2 }).clone()].to_vec() }).clone(),
].to_vec() } }).clone(),
].to_vec() }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "textSpans".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 0 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 1 } }).clone(),
    (WireField { name: "italic".to_string(), value: WireValue::WStr { value: "no".to_string() } }).clone(),
].to_vec() }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "inlineBoxes".to_string(), value: WireValue::WArr { items: vec![
    (WireValue::WObj { fields: vec![
    (WireField { name: "start".to_string(), value: WireValue::WNum { value: 0 } }).clone(),
    (WireField { name: "end".to_string(), value: WireValue::WNum { value: 1 } }).clone(),
    (WireField { name: "outerSpacing".to_string(), value: WireValue::WNum { value: 7 } }).clone(),
].to_vec() }).clone(),
].to_vec() } }).clone(),
    (WireField { name: "sourceBoundaries".to_string(), value: WireValue::WArr { items: vec![(WireValue::WStr { value: "3".to_string() }).clone(), (WireValue::WNull).clone()].to_vec() } }).clone(),
].to_vec() };
    }

    pub fn canonical_test_support_assert_hex(recorder: TestTraceRecorder, label: &str, expected: &str, result: EncodeResult) -> Result<(), TracedAssertionsFailFault> {
        let _ = recorder.record(label).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected, CanonicalTestSupport::canonical_test_support_hex_of((result).clone()).as_str(), Some((label).to_string()))?;
        Ok(())
    }
}
