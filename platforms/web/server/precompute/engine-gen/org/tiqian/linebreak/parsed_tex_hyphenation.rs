use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::u_string::UString;


#[derive(Clone, PartialEq)]
pub struct ParsedTexHyphenation {
    pub patterns: SortedMapTable<UString, Vec<u32>>,
    pub exceptions: SortedMapTable<UString, Vec<u32>>,
}

impl ParsedTexHyphenation {
    pub fn new(patterns: SortedMapTable<UString, Vec<u32>>, exceptions: SortedMapTable<UString, Vec<u32>>) -> Self {
        Self {
            patterns,
            exceptions,
        }
    }
}
