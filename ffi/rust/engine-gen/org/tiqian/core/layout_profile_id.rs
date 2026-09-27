use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutProfileId {
    pub value: String,
}

impl LayoutProfileId {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}",
            "LayoutProfileId(",
            "value=",
            (self.value).to_string(),
            ")"
        );
    }
}

pub fn compare_layout_profile_id(a: &LayoutProfileId, b: &LayoutProfileId) -> i32 {
    let cmp_value = SortedTable::sorted_table_compare_strings(a.value.as_str(), b.value.as_str());
    if cmp_value != 0 { return cmp_value; }
    0
}
