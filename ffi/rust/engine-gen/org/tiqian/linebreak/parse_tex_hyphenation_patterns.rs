use crate::org::tiqian::linebreak::parsed_tex_hyphenation::ParsedTexHyphenation;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ParseTexHyphenationPatterns;

impl ParseTexHyphenationPatterns {
    pub(crate) fn parse_tex_hyphenation_patterns_block(text: &str, name: &str) -> String {
        let start = u_string::find_from(&text, name, 0);
        if start < (0) {
            return String::new();
        }
        let open = u_string::find_from(&text, "{", i32::from_ne_bytes((start).to_ne_bytes()));
        if open < (0) {
            return String::new();
        }
        let close = u_string::find_from(&text, "}", i32::from_ne_bytes((i32::wrapping_add(open, 1)).to_ne_bytes()));
        if close < (0) {
            return String::new();
        }
        return u_string::substring(&text, i32::from_ne_bytes((i32::wrapping_add(open, 1)).to_ne_bytes()), i32::from_ne_bytes((close).to_ne_bytes()));
    }

    pub(crate) fn parse_tex_hyphenation_patterns_tokens(text: &str) -> Vec<String> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut out: Vec<String> = vec![];
        let mut token = String::new();
        let mut i = 0u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count).to_ne_bytes())) {
            let c = u_string::char_at_from(&__units1, i);
            let sep = c == " " || c == "\t" || c == concat!("\n",
"") || c == "\r";
            if sep {
                if i32::from_ne_bytes((u_string::unit_count(&(token))).to_ne_bytes()) > (0) {
                    out.push(token.clone());
                    token = "".to_string();
                }
            } else {
                token += &(c);
            }
            i = u32::wrapping_add(i, 1);
        }
        if i32::from_ne_bytes((u_string::unit_count(&(token))).to_ne_bytes()) > (0) {
            out.push(token.clone());
        }
        return out;
    }

    pub fn parse_tex_hyphenation_patterns_parse(tex: &str) -> ParsedTexHyphenation {
        let mut no_comments = String::new();
        let source_lines = u_string::split(&tex, &concat!("\n",
""));
        let mut li = 0u32;
        while (i32::from_ne_bytes((li).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((source_lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let line = (source_lines[usize::try_from(li).unwrap_or(0)]).clone();
            let p = u_string::find_from(&line, "%", 0);
            no_comments += &(format!("{}{}",
            (if p < (0) { line.to_string() } else { u_string::substring(&line, 0i32, i32::from_ne_bytes((p).to_ne_bytes())).to_string() }),
            concat!("\n",
"")
        ));
            li = u32::wrapping_add(li, 1);
        }
        let mut pb = SortedTable::sorted_table_map_builder::<String, Vec<u32>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let mut eb = SortedTable::sorted_table_map_builder::<String, Vec<u32>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str())));
        let pattern_tokens = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_tokens(ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_block(no_comments.as_str(), &"\\patterns").as_str());
        let mut ti = 0u32;
        while (i32::from_ne_bytes((ti).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pattern_tokens.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let token = (pattern_tokens[usize::try_from(ti).unwrap_or(0)]).clone();
            let mut key_b = String::new();
            let mut levels = vec![0];
            let mut i = 0u32;
            let __units2 = u_string::units(&token);
            let __count2 = u_string::unit_count(&token);
            while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count2).to_ne_bytes())) {
                let code = u_string::unit_at_from(&__units2, i).unwrap_or(0);
                if i32::from_ne_bytes((code).to_ne_bytes()) >= 48 && (i32::from_ne_bytes((code).to_ne_bytes())) <= 57 {
                    { let __grow_idx = usize::try_from(u32::wrapping_sub(u32::try_from((levels.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0); while levels.len() <= __grow_idx { levels.push(0); } levels[__grow_idx] = u32::wrapping_sub(code, 48); };
                } else {
                    {
                        let x = u_string::char_at_from(&__units2, i);
                        key_b += &(x.to_string());
                    }
                    levels.push(0);
                }
                i = u32::wrapping_add(i, 1);
            }
            pb.put(&(key_b).to_string(), &(levels));
            ti = u32::wrapping_add(ti, 1);
        }
        let hyphenation_tokens = ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_tokens(ParseTexHyphenationPatterns::parse_tex_hyphenation_patterns_block(no_comments.as_str(), &"\\hyphenation").as_str());
        let mut ei = 0u32;
        while (i32::from_ne_bytes((ei).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((hyphenation_tokens.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let token = (hyphenation_tokens[usize::try_from(ei).unwrap_or(0)]).clone();
            let mut key_b = String::new();
            let mut offsets: Vec<u32> = vec![];
            let mut pos = 0u32;
            let mut i = 0u32;
            let __units3 = u_string::units(&token);
            let __count3 = u_string::unit_count(&token);
            while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((__count3).to_ne_bytes())) {
                if u_string::char_at_from(&__units3, i) == "-" {
                    offsets.push(pos);
                } else {
                    {
                        let x = u_string::char_at_from(&__units3, i);
                        key_b += &(x.to_string());
                    }
                    pos = u32::wrapping_add(pos, 1);
                }
                i = u32::wrapping_add(i, 1);
            }
            eb.put(&key_b.to_lowercase(), &(offsets));
            ei = u32::wrapping_add(ei, 1);
        }
        return ParsedTexHyphenation::new(pb.clone().build(), eb.clone().build());
    }
}
