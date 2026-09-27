use crate::org::tiqian::protocol::wire_field::WireField;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum WireValue {
    WNull,
    WBool { value: bool },
    WNum { value: f64 },
    WStr { value: UString },
    WArr { items: Vec<WireValue> },
    WObj { fields: Vec<WireField> },
}

impl WireValue {
    pub fn to_string(&self) -> String {
        match self {
            WireValue::WNull => "WNull".to_string(),
            WireValue::WBool { value } => format!("WBool(value={})", (value).to_string()),
            WireValue::WNum { value } => format!("WNum(value={})", (value).to_string()),
            WireValue::WStr { value } => format!("WStr(value={})", (value).clone()),
            WireValue::WArr { items } => format!("WArr(items={})", {
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
            WireValue::WObj { fields } => format!("WObj(fields={})", {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < fields.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", format!("{{name={}, value={}}}",
            (fields[j0].name).clone(),
            (fields[j0].value).to_string()
        ));
                j0 += 1;
            }
            out.push(']');
            out
        }),
        }
    }
}
