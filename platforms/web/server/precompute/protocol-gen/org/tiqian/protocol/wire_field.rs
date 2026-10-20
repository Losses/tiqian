use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct WireField {
    pub name: UString,
    pub value: WireValue,
}
