use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationSpan {
    pub range: TextRange,
    pub kind: DecorationKind,
}

impl DecorationSpan {
    pub fn new(range: TextRange, kind: DecorationKind) -> Self {
        Self {
            range,
            kind,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "DecorationSpan(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "kind=",
            self.kind.name(),
            ")"
        );
    }
}

fn decoration_span_kind_order(v: &DecorationKind) -> i32 {
    match v {
        DecorationKind::ProperNoun => 2,
        DecorationKind::Mourning => 1,
        DecorationKind::Emphasis => 0,
        DecorationKind::BookTitle => 3,
    }
}
pub fn compare_decoration_span(a: &DecorationSpan, b: &DecorationSpan) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_kind = match decoration_span_kind_order(&a.kind).cmp(&decoration_span_kind_order(&b.kind)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_kind != 0 { return cmp_kind; }
    0
}
