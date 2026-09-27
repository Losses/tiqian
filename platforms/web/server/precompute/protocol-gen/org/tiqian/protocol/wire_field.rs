use crate::org::tiqian::protocol::wire_value::WireValue;


#[derive(Debug, Clone, PartialEq)]
pub struct WireField {
    pub name: String,
    pub value: WireValue,
}
