#![cfg(test)]

use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct LayoutDumpGoldens;

impl LayoutDumpGoldens {
    pub fn layout_dump_goldens_by_id() -> SortedMapTable<UString, UString> {
        let mut b: SortedMapTableBuilder<UString, UString> = SortedTable::sorted_table_map_builder::<UString, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_ustr(), b.as_ustr())));
        b.put(&(UString::from("adaptive-short-line-indent")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_0.txt"))).to_ustring());
        b.put(&(UString::from("adjacent-curly-quote-list-context")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_1.txt"))).to_ustring());
        b.put(&(UString::from("adjacent-punctuation-spacing")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_2.txt"))).to_ustring());
        b.put(&(UString::from("ascii-brackets-in-cjk")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_3.txt"))).to_ustring());
        b.put(&(UString::from("ascii-point-mark-impossible-measure")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_4.txt"))).to_ustring());
        b.put(&(UString::from("ascii-point-mark-in-cjk")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_5.txt"))).to_ustring());
        b.put(&(UString::from("basic-pause-stop")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_6.txt"))).to_ustring());
        b.put(&(UString::from("bibliographic-numeric-locator-break")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_7.txt"))).to_ustring());
        b.put(&(UString::from("bopomofo-tone-em-box")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_8.txt"))).to_ustring());
        b.put(&(UString::from("contextual-curly-quotes")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_9.txt"))).to_ustring());
        b.put(&(UString::from("contextual-dash-ellipsis")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_10.txt"))).to_ustring());
        b.put(&(UString::from("ellipsis-and-dash")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_11.txt"))).to_ustring());
        b.put(&(UString::from("emphasis-marks")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_12.txt"))).to_ustring());
        b.put(&(UString::from("fallback-roles")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_13.txt"))).to_ustring());
        b.put(&(UString::from("first-line-indent")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_14.txt"))).to_ustring());
        b.put(&(UString::from("greedy-multi-line")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_15.txt"))).to_ustring());
        b.put(&(UString::from("indent-opening-quote")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_16.txt"))).to_ustring());
        b.put(&(UString::from("interlinear-lines")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_17.txt"))).to_ustring());
        b.put(&(UString::from("justify-cjk-paragraph")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_18.txt"))).to_ustring());
        b.put(&(UString::from("justify-mixed-paragraph")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_19.txt"))).to_ustring());
        b.put(&(UString::from("justify-unbreakable-number-symbol")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_20.txt"))).to_ustring());
        b.put(&(UString::from("kinsoku-carry-previous")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_21.txt"))).to_ustring());
        b.put(&(UString::from("kinsoku-push-in")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_22.txt"))).to_ustring());
        b.put(&(UString::from("latin-camelcase")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_23.txt"))).to_ustring());
        b.put(&(UString::from("latin-existing-hyphen")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_24.txt"))).to_ustring());
        b.put(&(UString::from("latin-hard-break")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_25.txt"))).to_ustring());
        b.put(&(UString::from("latin-opaque-url-token")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_26.txt"))).to_ustring());
        b.put(&(UString::from("latin-word-wrap")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_27.txt"))).to_ustring());
        b.put(&(UString::from("line-end-kinsoku")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_28.txt"))).to_ustring());
        b.put(&(UString::from("lookahead-avoids-repair")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_29.txt"))).to_ustring());
        b.put(&(UString::from("lookahead-future-push-in")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_30.txt"))).to_ustring());
        b.put(&(UString::from("mandatory-blank-lines")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_31.txt"))).to_ustring());
        b.put(&(UString::from("mandatory-crlf")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_32.txt"))).to_ustring());
        b.put(&(UString::from("mandatory-leading-trailing-newline")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_33.txt"))).to_ustring());
        b.put(&(UString::from("mandatory-single-newline")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_34.txt"))).to_ustring());
        b.put(&(UString::from("mandatory-wraps-long-line")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_35.txt"))).to_ustring());
        b.put(&(UString::from("mi10s-adjacent-curly-quote-wrap")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_36.txt"))).to_ustring());
        b.put(&(UString::from("mi10s-western-bracket-citation-wrap")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_37.txt"))).to_ustring());
        b.put(&(UString::from("mixed-script-quote-paragraph-language")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_38.txt"))).to_ustring());
        b.put(&(UString::from("mourning-frame")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_39.txt"))).to_ustring());
        b.put(&(UString::from("nested-quotes")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_40.txt"))).to_ustring());
        b.put(&(UString::from("parenthetical-dash-pairs")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_41.txt"))).to_ustring());
        b.put(&(UString::from("progressive-technical-alpha-numeric")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_42.txt"))).to_ustring());
        b.put(&(UString::from("progressive-technical-current-line-emergency")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_43.txt"))).to_ustring());
        b.put(&(UString::from("progressive-technical-hash-fill")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_44.txt"))).to_ustring());
        b.put(&(UString::from("progressive-technical-inline")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_45.txt"))).to_ustring());
        b.put(&(UString::from("quote-digit-boundaries")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_46.txt"))).to_ustring());
        b.put(&(UString::from("real-paragraph-1")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_47.txt"))).to_ustring());
        b.put(&(UString::from("ruby-line-height")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_48.txt"))).to_ustring());
        b.put(&(UString::from("unmatched-curly-quotes")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_49.txt"))).to_ustring());
        b.put(&(UString::from("western-hyphenation")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_50.txt"))).to_ustring());
        b.put(&(UString::from("zero-width-space-soft-break")).to_ustring().to_ustring(), &(UString::from(include_str!("LayoutDumpGoldens__asset_51.txt"))).to_ustring());
        return b.clone().build();
    }
}
