//! Face selection port of `precompute-fonts.js`: `faceCandidates`,
//! `findFace`, `faceCovers`, `selectFace`, `selectShapeFace`,
//! `renderFamiliesFor` (ADR 0050 parity oracle). Error strings match the JS
//! throws; the message builders live here so callers can embed them verbatim.

use std::sync::Arc;

use crate::font_record::FontRecord;
use crate::js_compat::js_number_string;
use crate::json::json_string;
use crate::shaping::FontEngine;
use crate::NamedError;

/// `NoSnapshotFontFace:families=...;weight=...;italic=...;text=...`
pub fn no_exact_font_face_message(
    families: &[String],
    weight: f64,
    italic: bool,
    text: &str,
) -> String {
    format!(
        "NoSnapshotFontFace:families={};weight={};italic={};text={}",
        families.join(","),
        js_number_string(weight),
        italic,
        json_string(text)
    )
}

/// `faceCovers`: every point inside the declared unicode-range and mapped by
/// the face's cmap. The coverage probe runs at the range's low weight, the
/// way `createFont(record, record.weightRange[0])` does. Fails with
/// `SfntDecode` when skrifa rejects the record bytes.
pub fn face_covers(record: &FontRecord, points: &[char]) -> Result<bool, NamedError> {
    if !points
        .iter()
        .all(|point| unicode_range_contains(&record.unicode_ranges, u32::from(*point)))
    {
        return Ok(false);
    }
    let engine = FontEngine::new(record, record.weight_range[0])?;
    Ok(points
        .iter()
        .all(|point| engine.nominal_glyph(*point).is_some()))
}

/// `faceCandidates`: first family with style and weight matches wins; the
/// matches keep session order.
pub fn face_candidates<'a>(
    records: &'a [Arc<FontRecord>],
    families: &[String],
    requested_weight: f64,
    italic: bool,
) -> Vec<&'a FontRecord> {
    let desired_style = if italic { "italic" } else { "normal" };
    for family in families {
        let family_matches: Vec<&FontRecord> = records
            .iter()
            .map(|record| record.as_ref())
            .filter(|record| {
                font_record_matches_family(record.family.as_str(), &record.local_names, family)
                    && record.style == desired_style
            })
            .collect();
        let matched = css_weight_matched(&family_matches, requested_weight, |record| {
            (record.weight_range[0], record.weight_range[1])
        });
        if !matched.is_empty() {
            return matched.into_iter().copied().collect();
        }
    }
    Vec::new()
}

/// `findFace`: like `faceCandidates`, but the candidate that covers all
/// points wins; composite faces prefer the later record. Fails with
/// `SfntDecode` when the coverage probe cannot decode a record.
pub fn find_face<'a>(
    records: &'a [Arc<FontRecord>],
    families: &[String],
    requested_weight: f64,
    italic: bool,
    text: &[char],
) -> Result<Option<&'a FontRecord>, String> {
    let desired_style = if italic { "italic" } else { "normal" };
    for family in families {
        let family_matches: Vec<&FontRecord> = records
            .iter()
            .map(|record| record.as_ref())
            .filter(|record| {
                font_record_matches_family(record.family.as_str(), &record.local_names, family)
                    && record.style == desired_style
            })
            .collect();
        let weight_matched = css_weight_matched(&family_matches, requested_weight, |record| {
            (record.weight_range[0], record.weight_range[1])
        });
        // CSS Fonts composite faces with the same descriptors use the later
        // @font-face rule for overlapping unicode-range coverage.
        for record in weight_matched.into_iter().rev() {
            if face_covers(record, text).map_err(|error| error.0)? {
                return Ok(Some(record));
            }
        }
    }
    Ok(None)
}

/// `selectFace`: `findFace` with the `NoSnapshotFontFace` throw on a miss.
pub fn select_face<'a>(
    records: &'a [Arc<FontRecord>],
    families: &[String],
    requested_weight: f64,
    italic: bool,
    text: &[char],
) -> Result<&'a FontRecord, String> {
    find_face(records, families, requested_weight, italic, text)?.ok_or_else(|| {
        let joined: String = text.iter().collect();
        no_exact_font_face_message(families, requested_weight, italic, &joined)
    })
}

/// `selectShapeFace` (ExactDisplaySubstitutionCoverage): try the display
/// text, then the source text with `displayCovered: false`, then fail
/// through `selectFace`'s error.
pub fn select_shape_face<'a>(
    records: &'a [Arc<FontRecord>],
    families: &[String],
    requested_weight: f64,
    italic: bool,
    display_text: &[char],
    source_text: &[char],
) -> Result<(&'a FontRecord, bool), String> {
    if let Some(record) = find_face(records, families, requested_weight, italic, display_text)? {
        return Ok((record, true));
    }
    if source_text != display_text {
        if let Some(record) = find_face(records, families, requested_weight, italic, source_text)? {
            return Ok((record, false));
        }
    }
    let display: String = display_text.iter().collect();
    Err(no_exact_font_face_message(
        families,
        requested_weight,
        italic,
        &display,
    ))
}

/// `renderFamiliesFor`: host families backed by the corpus, in the host's
/// order, deduped per canonical family.
pub fn render_families(
    records: &[Arc<FontRecord>],
    requested_families: &[String],
) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for requested_family in requested_families {
        for record in records {
            if !font_record_matches_family(
                record.family.as_str(),
                &record.local_names,
                requested_family,
            ) {
                continue;
            }
            let canonical = record.family.trim().to_lowercase();
            if !seen.insert(canonical) {
                continue;
            }
            result.push(record.family.clone());
        }
    }
    if result.is_empty() {
        return Err(format!(
            "NoSnapshotRenderFontFamily:families={}",
            requested_families.join(",")
        ));
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// CSS @font-face descriptor primitives moved from the deleted font_face.rs
// module. Face selection stays on the Rust shaping path; parse_unicode_range
// moved to the Node shared-gen layer. Equal-weight faces keep every best rank
// so composite faces survive.
// ---------------------------------------------------------------------------

/// CSS Fonts weight-matching rank for one face descriptor range. Lower rank
/// wins; equal ranks keep every face (composite faces). Mirrors
/// cssWeightPreference: (group, distance) with (inf, inf) for malformed
/// ranges.
fn css_weight_preference(range: (f64, f64), requested: f64) -> (f64, f64) {
    let (low, high) = range;
    if !low.is_finite() || !high.is_finite() || high < low {
        return (f64::INFINITY, f64::INFINITY);
    }
    if (low..=high).contains(&requested) {
        return (0.0, 0.0);
    }
    if (400.0..=500.0).contains(&requested) {
        if low > requested && low <= 500.0 {
            return (1.0, low - requested);
        }
        if high < requested {
            return (2.0, requested - high);
        }
        return (3.0, low - 500.0);
    }
    if requested < 400.0 {
        if high < requested {
            return (1.0, requested - high);
        }
        return (2.0, low - requested);
    }
    if low > requested {
        return (1.0, low - requested);
    }
    (2.0, requested - high)
}

/// Faces sharing the best rank, in declaration order (stable sort, mirroring
/// cssWeightMatched including the records.len() <= 1 short cut).
pub(crate) fn css_weight_matched<T>(
    records: &[T],
    requested: f64,
    range_of: impl Fn(&T) -> (f64, f64),
) -> Vec<&T> {
    if records.len() <= 1 {
        return records.iter().collect();
    }
    let mut ranked: Vec<(usize, (f64, f64))> = records
        .iter()
        .enumerate()
        .map(|(index, record)| (index, css_weight_preference(range_of(record), requested)))
        .collect();
    ranked.sort_by(|left, right| {
        left.1
             .0
            .total_cmp(&right.1 .0)
            .then_with(|| left.1 .1.total_cmp(&right.1 .1))
    });
    let best = ranked[0].1;
    ranked
        .into_iter()
        .filter(|(_, rank)| *rank == best)
        .map(|(index, _)| &records[index])
        .collect()
}

/// A face covers a code point when no unicode-range was declared or one
/// declared range contains it.
fn unicode_range_contains(ranges: &Option<Vec<(u32, u32)>>, code_point: u32) -> bool {
    match ranges {
        None => true,
        Some(ranges) => ranges
            .iter()
            .any(|&(start, end)| code_point >= start && code_point <= end),
    }
}

/// A face is matched by the same family name or an OpenType local name exposed
/// by the host. Comparison trims and lowercases, mirroring
/// fontRecordMatchesFamily.
fn font_record_matches_family(
    family: &str,
    local_names: &[String],
    requested_family: &str,
) -> bool {
    let requested = requested_family.trim().to_lowercase();
    if requested.is_empty() {
        return false;
    }
    if family.to_lowercase() == requested {
        return true;
    }
    local_names
        .iter()
        .any(|name| name.trim().to_lowercase() == requested)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_inside_range_ranks_first() {
        assert_eq!(css_weight_preference((400.0, 400.0), 400.0), (0.0, 0.0));
        assert_eq!(css_weight_preference((300.0, 700.0), 450.0), (0.0, 0.0));
    }

    #[test]
    fn weight_400_to_500_uses_css_special_ranks() {
        // Requesting 400: a 500 face beats a 300 face (CSS Fonts 400/500 rule).
        assert_eq!(css_weight_preference((500.0, 500.0), 400.0), (1.0, 100.0));
        assert_eq!(css_weight_preference((300.0, 300.0), 400.0), (2.0, 100.0));
        // Requesting 450: a higher low end up to 500 still ranks group 1.
        assert_eq!(css_weight_preference((460.0, 600.0), 450.0), (1.0, 10.0));
        // A face entirely above 500 falls to group 3.
        assert_eq!(css_weight_preference((550.0, 700.0), 450.0), (3.0, 50.0));
        assert_eq!(css_weight_preference((100.0, 440.0), 450.0), (2.0, 10.0));
    }

    #[test]
    fn weight_outside_400_to_500_prefers_nearest() {
        assert_eq!(css_weight_preference((600.0, 600.0), 300.0), (2.0, 300.0));
        assert_eq!(css_weight_preference((100.0, 200.0), 300.0), (1.0, 100.0));
        assert_eq!(css_weight_preference((800.0, 900.0), 700.0), (1.0, 100.0));
    }

    #[test]
    fn malformed_range_ranks_last() {
        assert_eq!(
            css_weight_preference((500.0, 400.0), 400.0),
            (f64::INFINITY, f64::INFINITY)
        );
    }

    #[test]
    fn matched_keeps_all_best_rank_faces_in_order() {
        let records = [
            ("a", (300.0, 300.0)),
            ("b", (500.0, 500.0)),
            ("c", (500.0, 700.0)),
            ("d", (600.0, 600.0)),
        ];
        let matched = css_weight_matched(&records, 400.0, |record| record.1);
        let names: Vec<&str> = matched.iter().map(|record| record.0).collect();
        assert_eq!(names, vec!["b", "c"]);
    }

    #[test]
    fn matched_short_cuts_single_record() {
        let records = [("only", (100.0, 100.0))];
        let matched = css_weight_matched(&records, 900.0, |record| record.1);
        assert_eq!(matched.len(), 1);
    }

    #[test]
    fn unicode_range_contains_matches_declared_ranges() {
        let ranges = Some(vec![(0x4E00, 0x9FFF)]);
        assert!(unicode_range_contains(&ranges, 0x4E00));
        assert!(unicode_range_contains(&ranges, 0x9FFF));
        assert!(!unicode_range_contains(&ranges, 0x9FFF + 1));
        assert!(unicode_range_contains(&None, 0x10FFFF));
    }

    #[test]
    fn family_matches_by_declared_or_local_name() {
        assert!(font_record_matches_family(
            "Source Han Sans CN",
            &[],
            "source han sans cn"
        ));
        assert!(font_record_matches_family(
            "Source Han Sans CN",
            &["思源黑体".to_string()],
            "思源黑体"
        ));
        // The requested family is trimmed; the record family is not.
        assert!(font_record_matches_family("Serif", &[], "serif "));
        assert!(!font_record_matches_family(" Serif", &[], "serif"));
        assert!(!font_record_matches_family("Serif", &[], ""));
    }
}
