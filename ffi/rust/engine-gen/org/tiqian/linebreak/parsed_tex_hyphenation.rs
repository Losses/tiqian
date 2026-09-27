use crate::runtime::sorted_table::SortedMapTable;


#[derive(Clone, PartialEq)]
pub struct ParsedTexHyphenation {
    pub patterns: SortedMapTable<String, Vec<u32>>,
    pub exceptions: SortedMapTable<String, Vec<u32>>,
}

impl ParsedTexHyphenation {
    pub fn new(patterns: SortedMapTable<String, Vec<u32>>, exceptions: SortedMapTable<String, Vec<u32>>) -> Self {
        Self {
            patterns,
            exceptions,
        }
    }
}
