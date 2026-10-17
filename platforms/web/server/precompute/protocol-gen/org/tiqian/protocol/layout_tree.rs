use crate::org::tiqian::protocol::layout_attribute::LayoutAttribute;
use crate::org::tiqian::protocol::layout_float_shape::LayoutFloatShape;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutTree {
    Block { children: Vec<LayoutTree> },
    Inline { tag_name: Option<UString>, attributes: Vec<LayoutAttribute>, continue_adjacent: bool, word_breakable: bool, line_breakable: bool, text: UString, children: Vec<LayoutTree> },
    Float { x: f64, y: f64, shape: LayoutFloatShape },
}

impl LayoutTree {
    pub fn to_string(&self) -> String {
        match self {
            LayoutTree::Block { children } => format!("Block(children={})", {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < children.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", (children[j0]).to_string());
                j0 += 1;
            }
            out.push(']');
            out
        }),
            LayoutTree::Inline { tag_name, attributes, continue_adjacent, word_breakable, line_breakable, text,
children } => format!("Inline(tag_name={}, attributes={}, continue_adjacent={}, word_breakable={}, line_breakable={}, text={}, children={})", match tag_name { Some(__o) => format!("{}", (__o).clone()), None => "null".to_string() }, {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < attributes.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", format!("{{name={}, value={}}}",
            (attributes[j0].name).clone(),
            (attributes[j0].value).clone()
        ));
                j0 += 1;
            }
            out.push(']');
            out
        }, (continue_adjacent).to_string(), (word_breakable).to_string(), (line_breakable).to_string(), (text).clone(), {
            let mut out = String::new();
            out.push('[');
            let mut j0 = 0usize;
            while j0 < children.len() {
                if j0 > 0 { out.push_str(", "); }
                let _ = write!(out, "{}", (children[j0]).to_string());
                j0 += 1;
            }
            out.push(']');
            out
        }),
            LayoutTree::Float { x, y, shape } => format!("Float(x={}, y={}, shape={})", (x).to_string(), (y).to_string(), (shape).to_string()),
        }
    }
}
