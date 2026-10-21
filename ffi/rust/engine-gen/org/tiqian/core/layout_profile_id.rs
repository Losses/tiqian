use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutProfileId {
    pub value: UString,
}

impl LayoutProfileId {
    pub fn new(value: &UStr) -> Self {
        Self {
            value: value.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LayoutProfileId(")); __s += &(UString::from("value=")); __s += (self.value).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_layout_profile_id(a: &LayoutProfileId, b: &LayoutProfileId) -> i32 {
    let cmp_value = SortedTable::sorted_table_compare_strings(a.value.as_ustr(), b.value.as_ustr());
    if cmp_value != 0 { return cmp_value; }
    0
}
