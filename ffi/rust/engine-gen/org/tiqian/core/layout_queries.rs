use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::cluster_geometry_decision_info::ClusterGeometryDecisionInfo;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::metric_decision_info::MetricDecisionInfo;
use crate::org::tiqian::core::positioned_cluster::PositionedCluster;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::rich_text_background_metric_policy::RichTextBackgroundMetricPolicy;
use crate::org::tiqian::core::rich_text_corner_radii::RichTextCornerRadii;
use crate::org::tiqian::core::rich_text_line_segment::RichTextLineSegment;
use crate::org::tiqian::core::rich_text_role::RichTextRole;
use crate::org::tiqian::core::rich_text_span::RichTextSpan;
use crate::org::tiqian::core::ruby_decision_info::RubyDecisionInfo;
use crate::org::tiqian::core::source_boundary_bias::SourceBoundaryBias;
use crate::org::tiqian::core::source_interaction_boundaries::SourceInteractionBoundaries;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError;
use crate::org::tiqian::core::unicode_word_character_data::UnicodeWordCharacterData;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutQueriesGetSelectionOffsetForPositionFault {
    NoSuchElementErrorFault(crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}
impl std::fmt::Display for LayoutQueriesGetSelectionOffsetForPositionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutQueriesGetSelectionOffsetForPositionFault::NoSuchElementErrorFault(value) => write!(formatter, "{}", value),
            LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<LayoutQueriesGetSelectionOffsetForPositionFault> for crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError {
    fn from(value: LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        match value {
            LayoutQueriesGetSelectionOffsetForPositionFault::NoSuchElementErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutQueriesGetSelectionOffsetForPositionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutQueriesGetSelectionOffsetForPositionFault) -> Self {
        match value {
            LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError> for LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: crate::org::tiqian::core::tiqian_no_such_element_exception::NoSuchElementError) -> Self {
        LayoutQueriesGetSelectionOffsetForPositionFault::NoSuchElementErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutQueriesGetSelectionOffsetForPositionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(value)
    }
}

#[derive(Clone, PartialEq)]
pub struct SelectionBounds {
    pub left: f64,
    pub right: f64,
}

impl SelectionBounds {
    pub fn new(left: f64, right: f64) -> Self {
        Self {
            left,
            right,
        }
    }
}

#[derive(Clone, Copy)]
pub struct LayoutQueries;

impl LayoutQueries {
    const LAYOUT_QUERIES_INTERLINEAR_UNDERLINE_OFFSET_EM: f64 = 0.18f64;
    const LAYOUT_QUERIES_IDEOGRAPHIC_EM_BOX_NAME: &UStr = unsafe { &*(&[0x0049u16, 0x0064u16, 0x0065u16, 0x006Fu16, 0x0067u16, 0x0072u16, 0x0061u16, 0x0070u16, 0x0068u16, 0x0069u16, 0x0063u16, 0x0045u16, 0x006Du16, 0x0042u16, 0x006Fu16, 0x0078u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    const LAYOUT_QUERIES_BACKGROUND_FALLBACK_ASCENT_EM: f64 = 0.88f64;
    const LAYOUT_QUERIES_BACKGROUND_FALLBACK_DESCENT_EM: f64 = 0.12f64;
    const LAYOUT_QUERIES_CR: u32 = 13;
    const LAYOUT_QUERIES_LF: u32 = 10;
    const LAYOUT_QUERIES_NEL: u32 = 133;
    const LAYOUT_QUERIES_LINE_SEPARATOR: u32 = 8232;
    const LAYOUT_QUERIES_PARAGRAPH_SEPARATOR: u32 = 8233;

    pub fn layout_queries_resolved_background_corner_radii(segment: RichTextLineSegment, inset: f64) -> Result<RichTextCornerRadii, TextRangeError> {
        let _ = LayoutQueries::layout_queries_require_finite_non_negative(inset, UStr::new(&[70,97,105,108,101,100,32,114,101,113,117,105,114,101,109,101,110,116,46]))?;
        let box_width = LayoutQueries::layout_queries_max_float(segment.get_width() - inset * 2.0f64, 0.0f64);
        let box_height = LayoutQueries::layout_queries_max_float(segment.get_height() - inset * 2.0f64, 0.0f64);
        let maximum = LayoutQueries::layout_queries_min_float(box_width / 2.0f64, box_height / 2.0f64);
        let paint = (((segment.span).clone().paint).clone().background).clone();
        let left_radius = LayoutQueries::layout_queries_resolve_radius(if segment.get_continues_from_previous_line() { paint.continuation_corner_radius } else { paint.corner_radius }, inset, maximum);
        let right_radius = LayoutQueries::layout_queries_resolve_radius(if segment.get_continues_on_next_line() { paint.continuation_corner_radius } else { paint.corner_radius }, inset, maximum);
        return Ok(RichTextCornerRadii::new(left_radius, right_radius, right_radius, left_radius));
    }

    pub fn layout_queries_get_text_for_copy(result: LayoutResult, range: TextRange) -> Result<UString, UStringFault> {
        let source = (((result.input).clone().content).clone().text).to_ustring();
        let start = LayoutQueries::layout_queries_clamp_int(range.start, 0, u_string::unit_count(&(source)));
        let end = LayoutQueries::layout_queries_clamp_int(range.end, start, u_string::unit_count(&(source)));
        if start == end {
            return Ok(UString::new());
        }
        let mut annotations: Vec<CopyAnnotation> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().ruby_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            LayoutQueries::layout_queries_add_copy_annotation(&mut annotations, (decision.base_range).clone(), (decision.text).to_ustring().as_ustr(), start, end);
            index = u32::wrapping_add(index, 1);
        }
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().bopomofo_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            LayoutQueries::layout_queries_add_copy_annotation(&mut annotations, (decision.base_range).clone(), (decision.text).to_ustring().as_ustr(), start, end);
            index = u32::wrapping_add(index, 1);
        }
        LayoutQueries::layout_queries_insertion_sort_annotations(&mut annotations);
        let mut output = Vec::<u16>::new();
        let mut cursor = start;
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((annotations.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let annotation = (annotations[usize::try_from(index).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((annotation.end) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((annotation.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((end) as i32).to_ne_bytes()) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !u_string::substring(&source, i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()), i32::from_ne_bytes(((annotation.end) as i32).to_ne_bytes())).is_empty() {
                        if !u_string::substring(&source, i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()), i32::from_ne_bytes(((annotation.end) as i32).to_ne_bytes())).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend(u_string::substring(&source, i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()), i32::from_ne_bytes(((annotation.end) as i32).to_ne_bytes())).encode_utf16());
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("（").is_empty() {
                        if !UString::from("（").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend(UString::from("（").encode_utf16());
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !(annotation.text).to_ustring().is_empty() {
                        if !(annotation.text).to_ustring().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend((annotation.text).to_ustring().encode_utf16());
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("）").is_empty() {
                        if !UString::from("）").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend(UString::from("）").encode_utf16());
                cursor = annotation.end;
            }
            index = u32::wrapping_add(index, 1);
        }
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !u_string::substring(&source, i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()), i32::from_ne_bytes(((end) as i32).to_ne_bytes())).is_empty() {
                if !u_string::substring(&source, i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()), i32::from_ne_bytes(((end) as i32).to_ne_bytes())).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        output.extend(u_string::substring(&source, i32::from_ne_bytes(((cursor) as i32).to_ne_bytes()), i32::from_ne_bytes(((end) as i32).to_ne_bytes())).encode_utf16());
        return Ok(UString::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub fn layout_queries_positioned_clusters(result: LayoutResult) -> Vec<PositionedCluster> {
        let mut output: Vec<PositionedCluster> = vec![];
        let mut line_index = 0u32;
        while (i32::from_ne_bytes(((line_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let line_positions = LayoutQueries::layout_queries_positioned_clusters_at((result).clone(), line_index, (result.lines[usize::try_from(line_index).unwrap_or(0)]).clone());
            let mut index = 0u32;
            while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((line_positions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                output.push((line_positions[usize::try_from(index).unwrap_or(0)]).clone());
                index = u32::wrapping_add(index, 1);
            }
            line_index = u32::wrapping_add(line_index, 1);
        }
        return output;
    }

    pub fn layout_queries_positioned_clusters_for_line(result: LayoutResult, line: LineBox) -> Result<Vec<PositionedCluster>, TextRangeError> {
        let mut line_index = 0u32;
        while (i32::from_ne_bytes(((line_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && (result.lines[usize::try_from(line_index).unwrap_or(0)]).clone() != line {
            line_index = u32::wrapping_add(line_index, 1);
        }
        if i32::from_ne_bytes(((line_index) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) {
            return Err(TextRangeError::Message { text: UString::from("line must belong to this LayoutResult.") });
        }
        return Ok(LayoutQueries::layout_queries_positioned_clusters_at((result).clone(), line_index, (line).clone()));
    }

    pub fn layout_queries_glyph_ink_bounds(result: LayoutResult) -> Option<Rect> {
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let mut left = f64::INFINITY;
        let mut top = f64::INFINITY;
        let mut right = f64::NEG_INFINITY;
        let mut bottom = f64::NEG_INFINITY;
        let mut run_index = 0u32;
        while (i32::from_ne_bytes(((run_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.glyph_runs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let run = (result.glyph_runs[usize::try_from(run_index).unwrap_or(0)]).clone();
            let mut glyph_index = 0u32;
            while (i32::from_ne_bytes(((glyph_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((run.glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                let glyph = (run.glyphs[usize::try_from(glyph_index).unwrap_or(0)]).clone();
                match &(glyph.bounds) {
                    Some(__option) => {
                        let cluster = LayoutQueries::layout_queries_find_positioned_by_range(&positioned, (glyph.cluster_range).clone());
                        match &(cluster) {
                            Some(__option1) => {
                                let bounds = (*__option).clone();
                                left = LayoutQueries::layout_queries_min_float(left, __option1.draw_x + glyph.x + bounds.left);
                                top = LayoutQueries::layout_queries_min_float(top, __option1.baseline + glyph.y + bounds.top);
                                right = LayoutQueries::layout_queries_max_float(right, __option1.draw_x + glyph.x + bounds.right);
                                bottom = LayoutQueries::layout_queries_max_float(bottom, __option1.baseline + glyph.y + bounds.bottom);
                            }
                            None => {
                            }
                        }
                    }
                    None => {
                    }
                }
                glyph_index = u32::wrapping_add(glyph_index, 1);
            }
            run_index = u32::wrapping_add(run_index, 1);
        }
        if !LayoutQueries::layout_queries_is_finite(left) || !LayoutQueries::layout_queries_is_finite(top) || !LayoutQueries::layout_queries_is_finite(right) || !LayoutQueries::layout_queries_is_finite(bottom) {
            return None;
        }
        return Some(Rect::new(left, top, right, bottom));
    }

    pub fn layout_queries_get_line_for_offset(result: LayoutResult, offset: u32) -> u32 {
        if u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return 4294967295u32;
        }
        let clamped = LayoutQueries::layout_queries_clamp_int(offset, 0, u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())));
        if clamped == u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())) {
            return u32::wrapping_sub(u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let line = (result.lines[usize::try_from(index).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((line.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((clamped) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((line.range).clone().end) as i32).to_ne_bytes())) {
                return index;
            }
            index = u32::wrapping_add(index, 1);
        }
        return LayoutQueries::layout_queries_nearest_line_for_offset((result).clone(), clamped);
    }

    pub fn layout_queries_get_bounding_box(result: LayoutResult, offset: u32) -> Result<Rect, NoSuchElementError> {
        if u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(Rect::new(0.0f64, 0.0f64, 0.0f64, 0.0f64));
        }
        let clamped = LayoutQueries::layout_queries_clamp_int(offset, 0, u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())));
        if clamped == u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())) {
            return Ok(LayoutQueries::layout_queries_get_cursor_rect((result).clone(), clamped)?);
        }
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let mut pipeline_result: Option<PositionedCluster> = None;
        for cluster in &positioned {
            if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((clamped) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) {
                pipeline_result = Some(cluster.clone());
                break;
            }
        }
        let r#match = (pipeline_result).clone();
        return Ok(match &(r#match) { None => LayoutQueries::layout_queries_get_cursor_rect((result).clone(), clamped)?, Some(__option2) => __option2.get_rect() });
    }

    pub fn layout_queries_get_bounding_boxes(result: LayoutResult, range: TextRange) -> Vec<Rect> {
        if range.get_is_empty() || u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let start = LayoutQueries::layout_queries_clamp_int(range.start, 0, u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())));
        let end = LayoutQueries::layout_queries_clamp_int(range.end, start, u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())));
        if start == end {
            return vec![];
        }
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let mut pipeline_result: Vec<Rect> = Vec::new();
        for cluster in &positioned {
            {
                let pipeline_item = LayoutQueries::layout_queries_slice_rect_if_covered((cluster).clone(), start, end);
                match &(pipeline_item) {
                    Some(__option3) => {
                        pipeline_result.push((__option3).clone());
                    }
                    None => {
                    }
                }
            }
        }
        return pipeline_result;
    }

    pub fn layout_queries_get_bounding_boxes_int(result: LayoutResult, start: u32, end: u32) -> Result<Vec<Rect>, TextRangeError> {
        return Ok(LayoutQueries::layout_queries_get_bounding_boxes((result).clone(), TextRange::new(start, end)?));
    }

    pub fn layout_queries_positioned_rich_text_segments(result: LayoutResult, spans: &Vec<RichTextSpan>) -> Result<Vec<RichTextLineSegment>, TextRangeError> {
        if u32::try_from((spans.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(vec![]);
        }
        let clusters = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let text_length = u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring()));
        let mut output: Vec<RichTextLineSegment> = vec![];
        let mut span_index = 0u32;
        while (i32::from_ne_bytes(((span_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((spans.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let span = (spans[usize::try_from(span_index).unwrap_or(0)]).clone();
            let start = LayoutQueries::layout_queries_clamp_int((span.range).clone().start, 0, text_length);
            let end = LayoutQueries::layout_queries_clamp_int((span.range).clone().end, start, text_length);
            if start != end {
                let normalized = RichTextSpan::new(TextRange::new(start, end)?, span.role.clone(), (span.paint).clone());
                let mut pending: Option<RichTextLineSegment> = None;
                let mut cluster_index = 0u32;
                while (i32::from_ne_bytes(((cluster_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let cluster = (clusters[usize::try_from(cluster_index).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((end) as i32).to_ne_bytes()) {
                        break;
                    }
                    if i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((start) as i32).to_ne_bytes())) {
                        let slice_start = LayoutQueries::layout_queries_max_int(start, (cluster.range).clone().start);
                        let slice_end = LayoutQueries::layout_queries_min_int(end, (cluster.range).clone().end);
                        if i32::from_ne_bytes(((slice_start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((slice_end) as i32).to_ne_bytes())) {
                            let rect = LayoutQueries::layout_queries_slice_rect((cluster).clone(), slice_start, slice_end);
                            let next = RichTextLineSegment::new((normalized).clone(), cluster.line_index, TextRange::new(slice_start, slice_end)?, rect.left, rect.top, rect.right, rect.bottom, cluster.baseline);
                            if match &(pending) { Some(__option4) => __option4.line_index == next.line_index && ((__option4.span).clone()).range == ((next.span).clone()).range && ((__option4.span).clone()).role.__haxe_type_name() == ((next.span).clone()).role.__haxe_type_name() && (((__option4.span).clone()).paint).argb == (((next.span).clone()).paint).argb && (((__option4.span).clone()).paint).line_pattern.__haxe_type_name() == (((next.span).clone()).paint).line_pattern.__haxe_type_name() && ((((__option4.span).clone()).paint).background).horizontal_padding == ((((next.span).clone()).paint).background).horizontal_padding && ((((__option4.span).clone()).paint).background).vertical_padding == ((((next.span).clone()).paint).background).vertical_padding && ((((__option4.span).clone()).paint).background).corner_radius == ((((next.span).clone()).paint).background).corner_radius && ((((__option4.span).clone()).paint).background).continuation_corner_radius == ((((next.span).clone()).paint).background).continuation_corner_radius && ((((__option4.span).clone()).paint).background).metric_policy == ((((next.span).clone()).paint).background).metric_policy && ((((__option4.span).clone()).paint).background).draw_style.__haxe_type_name() == ((((next.span).clone()).paint).background).draw_style.__haxe_type_name() && (((__option4.span).clone()).paint).adjacent_same_style_clearance == (((next.span).clone()).paint).adjacent_same_style_clearance && __option4.range.end == (next.range).clone().start, None => false } {
                                pending = Some(RichTextLineSegment::new(((pending).as_ref().unwrap().span).clone(), (pending).as_ref().unwrap().line_index, TextRange::new(((pending).as_ref().unwrap().range).clone().start, (next.range).clone().end)?, (pending).as_ref().unwrap().left, LayoutQueries::layout_queries_min_float((pending).as_ref().unwrap().top, next.top), next.right, LayoutQueries::layout_queries_max_float((pending).as_ref().unwrap().bottom, next.bottom), (pending).as_ref().unwrap().baseline).clone());
                            } else {
                                match &(pending) {
                                    Some(__option5) => {
                                        output.push((__option5).clone());
                                    }
                                    None => {
                                    }
                                }
                                pending = Some(next.clone());
                            }
                        }
                    }
                    cluster_index = u32::wrapping_add(cluster_index, 1);
                }
                match &(pending) {
                    Some(__option6) => {
                        output.push((__option6).clone());
                    }
                    None => {
                    }
                }
            }
            span_index = u32::wrapping_add(span_index, 1);
        }
        return Ok(output);
    }

    pub fn layout_queries_trimmed_rich_text_decoration_segments(result: LayoutResult, occupied_segments: &Vec<RichTextLineSegment>) -> Vec<RichTextLineSegment> {
        if u32::try_from((occupied_segments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let mut pipeline_result: Vec<RichTextLineSegment> = Vec::new();
        for v in occupied_segments {
            if LayoutQueries::layout_queries_is_decoration_role(((v.span).clone().role).clone()) {
                pipeline_result.push(v.clone());
            }
        }
        let decorations = (pipeline_result).clone();
        if u32::try_from((decorations.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        return LayoutQueries::layout_queries_with_adjacent_same_style_clearance((result).clone(), &LayoutQueries::layout_queries_trim_outer_punctuation_glue((result).clone(), &decorations));
    }

    pub fn layout_queries_rich_text_background_segments(result: LayoutResult, occupied_segments: &Vec<RichTextLineSegment>) -> Vec<RichTextLineSegment> {
        if u32::try_from((occupied_segments.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let mut pipeline_result: Vec<RichTextLineSegment> = Vec::new();
        for v in occupied_segments {
            if LayoutQueries::layout_queries_is_background_role(((v.span).clone().role).clone()) {
                pipeline_result.push(v.clone());
            }
        }
        let backgrounds = (pipeline_result).clone();
        if u32::try_from((backgrounds.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return vec![];
        }
        let positioned = LayoutQueries::layout_queries_positioned_clusters((result).clone());
        let trimmed = LayoutQueries::layout_queries_trim_outer_punctuation_glue((result).clone(), &backgrounds);
        let mut output: Vec<RichTextLineSegment> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((trimmed.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let segment = (trimmed[usize::try_from(index).unwrap_or(0)]).clone();
            let covered = LayoutQueries::layout_queries_clusters_on_segment_line(&positioned, (segment).clone());
            if u32::try_from((covered.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
                output.push(segment.clone());
                index = u32::wrapping_add(index, 1);
                continue;
            }
            let first = (covered[0usize]).clone();
            let last = (covered[usize::try_from(u32::wrapping_sub(u32::try_from((covered.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
            let horizontal_padding = (((segment.span).clone().paint).clone().background).clone().horizontal_padding;
            let leading_padding = if segment.range.clone().start == ((segment.span).clone().range).clone().start { horizontal_padding } else { 0.0f64 };
            let trailing_padding = if segment.range.clone().end == ((segment.span).clone().range).clone().end { horizontal_padding } else { 0.0f64 };
            let left = LayoutQueries::layout_queries_min_float(segment.right, LayoutQueries::layout_queries_max_float(segment.left, first.draw_x - leading_padding));
            let natural_last_right = LayoutQueries::layout_queries_natural_last_right((result).clone(), (last).clone());
            let right = LayoutQueries::layout_queries_max_float(left, LayoutQueries::layout_queries_min_float(segment.right, natural_last_right + trailing_padding));
            let face_top: f64;
            let face_bottom: f64;
            {
                let _g = (((segment.span).clone().paint).clone().background).clone().metric_policy;
                let _ = match _g {
    RichTextBackgroundMetricPolicy::MarkedFaces => {
    let faces = LayoutQueries::layout_queries_marked_face_vertical_bounds((result).clone(), &covered);
    face_top = faces[0usize];
    face_bottom = faces[1usize]
},
    RichTextBackgroundMetricPolicy::UniformTextStyle => {
    let uniform = LayoutQueries::layout_queries_uniform_text_style_vertical_bounds((result).clone(), (segment).clone(), LayoutQueries::layout_queries_resolved_text_style_at((result).clone(), (segment.range).clone().start));
    face_top = uniform[0usize];
    face_bottom = uniform[1usize]
},
    RichTextBackgroundMetricPolicy::UniformParagraphStyle => {
    let paragraph = LayoutQueries::layout_queries_uniform_text_style_vertical_bounds((result).clone(), (segment).clone(), ((result.input).clone().text_style).clone());
    face_top = paragraph[0usize];
    face_bottom = paragraph[1usize]
},
};
            }
            let vertical_padding = (((segment.span).clone().paint).clone().background).clone().vertical_padding;
            output.push(RichTextLineSegment::new((segment.span).clone(), segment.line_index, (segment.range).clone(), left, LayoutQueries::layout_queries_max_float(face_top - vertical_padding, segment.top), right, LayoutQueries::layout_queries_min_float(face_bottom + vertical_padding, segment.bottom), segment.baseline));
            index = u32::wrapping_add(index, 1);
        }
        return LayoutQueries::layout_queries_with_adjacent_same_style_clearance((result).clone(), &output);
    }

    pub fn layout_queries_rich_text_decoration_line_y(result: LayoutResult, segment: RichTextLineSegment, stroke_width: f64) -> Result<f64, TextRangeError> {
        let _ = LayoutQueries::layout_queries_require_finite_non_negative(stroke_width, UStr::new(&[115,116,114,111,107,101,87,105,100,116,104,32,109,117,115,116,32,98,101,32,102,105,110,105,116,101,32,97,110,100,32,110,111,110,45,110,101,103,97,116,105,118,101]))?;
        if !LayoutQueries::layout_queries_is_decoration_role((segment.span).clone().role.clone()) {
            return Err(TextRangeError::Message { text: UString::from("richTextDecorationLineY only supports underline and line-through segments") });
        }
        let style = LayoutQueries::layout_queries_resolved_text_style_at((result).clone(), (segment.range).clone().start);
        let raw_line_y: f64;
        if segment.span.clone().role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Underline" {
            raw_line_y = segment.baseline + style.font_size * LayoutQueries::LAYOUT_QUERIES_INTERLINEAR_UNDERLINE_OFFSET_EM;
        } else {
            if segment.span.clone().role.__haxe_type_name() == "org.tiqian.core.RichTextRole.LineThrough" {
                let face = LayoutQueries::layout_queries_uniform_text_style_vertical_bounds((result).clone(), (segment).clone(), (style).clone());
                raw_line_y = (face[0usize] + face[1usize]) / 2.0f64;
            } else {
                raw_line_y = segment.baseline;
            }
        }
        return Ok(LayoutQueries::layout_queries_clamp_float(raw_line_y, segment.top + stroke_width / 2.0f64, segment.bottom - stroke_width / 2.0f64));
    }

    pub fn layout_queries_get_cursor_rect(result: LayoutResult, offset: u32) -> Result<Rect, NoSuchElementError> {
        if u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(Rect::new(0.0f64, 0.0f64, 0.0f64, 0.0f64));
        }
        let clamped = LayoutQueries::layout_queries_clamp_int(offset, 0, u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())));
        let line_index = LayoutQueries::layout_queries_max_int(LayoutQueries::layout_queries_get_line_for_offset((result).clone(), clamped), 0);
        let line = (result.lines[usize::try_from(line_index).unwrap_or(0)]).clone();
        let positioned = LayoutQueries::layout_queries_positioned_clusters_at((result).clone(), line_index, (line).clone());
        let mut x = line.indent;
        if i32::from_ne_bytes(((u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((((positioned[0usize]).clone().range).clone().start) as i32).to_ne_bytes()) {
                x = positioned[0usize].left;
            } else {
                if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((((positioned[usize::try_from(u32::wrapping_sub(u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end) as i32).to_ne_bytes()) {
                    x = positioned[usize::try_from(u32::wrapping_sub(u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].right;
                } else {
                    let mut index = 0u32;
                    let mut found = false;
                    while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                        let cluster = (positioned[usize::try_from(index).unwrap_or(0)]).clone();
                        if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((clamped) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes()) {
                            x = LayoutQueries::layout_queries_x_for_offset((cluster).clone(), clamped);
                            found = true;
                            break;
                        }
                        index = u32::wrapping_add(index, 1);
                    }
                    if !found {
                        return Err(NoSuchElementError::Message { text: UString::from("Collection contains no element matching the predicate.") });
                    }
                }
            }
        }
        return Ok(Rect::new(x, line.top, x + 1.0f64, line.bottom));
    }

    pub fn layout_queries_get_offset_for_position(result: LayoutResult, x: f64, y: f64) -> u32 {
        if u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return 0;
        }
        let line_index = LayoutQueries::layout_queries_nearest_line_for_y((result).clone(), y);
        let positioned = LayoutQueries::layout_queries_positioned_clusters_at((result).clone(), line_index, (result.lines[usize::try_from(line_index).unwrap_or(0)]).clone());
        if u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return ((result.lines[usize::try_from(line_index).unwrap_or(0)]).clone().range).clone().start;
        }
        if x <= positioned[0usize].left {
            return ((positioned[0usize]).clone().range).clone().start;
        }
        if x >= positioned[usize::try_from(u32::wrapping_sub(u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].right {
            return ((positioned[usize::try_from(u32::wrapping_sub(u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end;
        }
        let cluster = LayoutQueries::layout_queries_nearest_cluster(&positioned, x);
        return LayoutQueries::layout_queries_offset_for_x((cluster).clone(), x);
    }

    pub fn layout_queries_get_selection_offset_for_position(result: LayoutResult, x: f64, y: f64) -> Result<u32, LayoutQueriesGetSelectionOffsetForPositionFault> {
        if u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(0);
        }
        let line_index = LayoutQueries::layout_queries_nearest_line_for_y((result).clone(), y);
        let positioned = LayoutQueries::layout_queries_positioned_clusters_at((result).clone(), line_index, (result.lines[usize::try_from(line_index).unwrap_or(0)]).clone());
        if u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), ((result.lines[usize::try_from(line_index).unwrap_or(0)]).clone().range).clone().start, SourceBoundaryBias::Nearest).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(e))?);
        }
        if x <= positioned[0usize].left {
            return Ok(LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), ((positioned[0usize]).clone().range).clone().start, SourceBoundaryBias::Nearest).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(e))?);
        }
        if x >= positioned[usize::try_from(u32::wrapping_sub(u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)].right {
            return Ok(LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), ((positioned[usize::try_from(u32::wrapping_sub(u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().range).clone().end, SourceBoundaryBias::Nearest).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(e))?);
        }
        let mut pipeline_result: Option<PositionedCluster> = None;
        for cluster in &positioned {
            if x >= cluster.left && (x) <= cluster.right {
                pipeline_result = Some(cluster.clone());
                break;
            }
        }
        let candidate = (pipeline_result).clone();
        let cluster = match &(candidate) { Some(__option7) => (*__option7).clone(), None => LayoutQueries::layout_queries_nearest_cluster(&positioned, x) };
        let raw_offset = LayoutQueries::layout_queries_offset_for_x((cluster).clone(), x);
        let backward = LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), raw_offset, SourceBoundaryBias::Backward).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(e))?;
        let forward = LayoutQueries::layout_queries_coerce_selection_offset((result).clone(), raw_offset, SourceBoundaryBias::Forward).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::TextRangeErrorFault(e))?;
        if backward == forward {
            return Ok(backward);
        }
        let backward_distance = (LayoutQueries::layout_queries_get_cursor_rect((result).clone(), backward).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::NoSuchElementErrorFault(e))?.left - x).abs();
        let forward_distance = (LayoutQueries::layout_queries_get_cursor_rect((result).clone(), forward).map_err(|e| LayoutQueriesGetSelectionOffsetForPositionFault::NoSuchElementErrorFault(e))?.left - x).abs();
        return Ok(if backward_distance < (forward_distance) { backward } else { forward });
    }

    pub fn layout_queries_coerce_selection_offset(result: LayoutResult, offset: u32, bias: SourceBoundaryBias) -> Result<u32, TextRangeError> {
        let text = (((result.input).clone().content).clone().text).to_ustring();
        let clamped = LayoutQueries::layout_queries_clamp_int(offset, 0, u_string::unit_count(&(text)));
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.input).clone().inline_objects.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let inline_object = ((result.input).clone().inline_objects[usize::try_from(index).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) > (i32::from_ne_bytes((((inline_object.range).clone().start) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((clamped) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((inline_object.range).clone().end) as i32).to_ne_bytes())) {
                let snapped: u32;
                let _ = match bias {
    SourceBoundaryBias::Backward => snapped = (inline_object.range).clone().start,
    SourceBoundaryBias::Forward => snapped = (inline_object.range).clone().end,
    SourceBoundaryBias::Nearest => snapped = if i32::from_ne_bytes(((u32::wrapping_sub(clamped, (inline_object.range).clone().start)) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::wrapping_sub((inline_object.range).clone().end, clamped)) as i32).to_ne_bytes())) { (inline_object.range).clone().start } else { (inline_object.range).clone().end },
};
                return Ok(snapped);
            }
            index = u32::wrapping_add(index, 1);
        }
        return Ok(SourceInteractionBoundaries::source_interaction_boundaries_coerce_to_interaction_boundary(text.as_ustr(), clamped, TextRange::new(0u32, u_string::unit_count(&(text)))?, bias));
    }

    pub fn layout_queries_get_selection_word_boundary(result: LayoutResult, offset: u32) -> Result<TextRange, TextRangeError> {
        let text = (((result.input).clone().content).clone().text).to_ustring();
        if u_string::unit_count(&(text)) == 0 {
            return Ok(TextRange::new(0u32, 0u32)?);
        }
        let clamped = LayoutQueries::layout_queries_clamp_int(offset, 0, u_string::unit_count(&(text)));
        let mut object_index = 0u32;
        while (i32::from_ne_bytes(((object_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.input).clone().inline_objects.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let inline_object = ((result.input).clone().inline_objects[usize::try_from(object_index).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((clamped) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((inline_object.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((clamped) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((inline_object.range).clone().end) as i32).to_ne_bytes())) {
                return Ok(((inline_object.range).clone()).clone());
            }
            object_index = u32::wrapping_add(object_index, 1);
        }
        let boundaries = SourceInteractionBoundaries::source_interaction_boundaries_interaction_boundaries(text.as_ustr(), TextRange::new(0u32, u_string::unit_count(&(text)))?);
        let mut unit_index: u32;
        if clamped == u_string::unit_count(&(text)) {
            unit_index = u32::wrapping_sub(u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), 2);
        } else {
            unit_index = LayoutQueries::layout_queries_boundary_index(&boundaries, clamped);
        }
        if unit_index > 2147483647 {
            unit_index = 0u32;
        }
        let kind = LayoutQueries::layout_queries_selection_word_kind(text.as_ustr(), boundaries[usize::try_from(unit_index).unwrap_or(0)], boundaries[usize::try_from(u32::wrapping_add(unit_index, 1)).unwrap_or(0)]);
        if kind == SelectionWordKind::Single {
            return Ok(TextRange::new(boundaries[usize::try_from(unit_index).unwrap_or(0)], boundaries[usize::try_from(u32::wrapping_add(unit_index, 1)).unwrap_or(0)])?);
        }
        let mut first = unit_index;
        let mut last = unit_index;
        while (i32::from_ne_bytes(((first) as i32).to_ne_bytes())) > (0) && LayoutQueries::layout_queries_selection_word_kind(text.as_ustr(), boundaries[usize::try_from(u32::wrapping_sub(first, 1)).unwrap_or(0)], boundaries[usize::try_from(first).unwrap_or(0)]) == kind {
            first = u32::wrapping_sub(first, 1);
        }
        while (i32::from_ne_bytes(((u32::wrapping_add(last, 2)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) && LayoutQueries::layout_queries_selection_word_kind(text.as_ustr(), boundaries[usize::try_from(u32::wrapping_add(last, 1)).unwrap_or(0)], boundaries[usize::try_from(u32::wrapping_add(last, 2)).unwrap_or(0)]) == kind {
            last = u32::wrapping_add(last, 1);
        }
        return Ok(TextRange::new(boundaries[usize::try_from(first).unwrap_or(0)], boundaries[usize::try_from(u32::wrapping_add(last, 1)).unwrap_or(0)])?);
    }

    pub fn layout_queries_get_selection_word_boundary_for_position(result: LayoutResult, x: f64, y: f64) -> Result<Option<TextRange>, TextRangeError> {
        if u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 || u_string::unit_count(&((((result.input).clone().content).clone().text).to_ustring())) == 0 {
            return Ok(None);
        }
        let line_index = LayoutQueries::layout_queries_nearest_line_for_y((result).clone(), y);
        let positioned = LayoutQueries::layout_queries_positioned_clusters_at((result).clone(), line_index, (result.lines[usize::try_from(line_index).unwrap_or(0)]).clone());
        if u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return Ok(None);
        }
        let mut pipeline_result: Option<PositionedCluster> = None;
        for cluster in &positioned {
            if x >= cluster.left && (x) <= cluster.right {
                pipeline_result = Some(cluster.clone());
                break;
            }
        }
        let candidate = (pipeline_result).clone();
        let cluster = match &(candidate) { Some(__option8) => (*__option8).clone(), None => LayoutQueries::layout_queries_nearest_cluster(&positioned, x) };
        if cluster.range.clone().get_is_empty() {
            return Ok(None);
        }
        let source_unit_offset = LayoutQueries::layout_queries_clamp_int(LayoutQueries::layout_queries_offset_for_x((cluster).clone(), x), (cluster.range).clone().start, u32::wrapping_sub((cluster.range).clone().end, 1));
        return Ok(Some(LayoutQueries::layout_queries_get_selection_word_boundary((result).clone(), source_unit_offset)?));
    }

    pub(crate) fn layout_queries_positioned_clusters_at(result: LayoutResult, line_index: u32, line: LineBox) -> Vec<PositionedCluster> {
        let mut leading_consumed: Vec<FloatRangeValue> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().geometry_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().geometry_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if decision.leading_glue_consumed > (0.0f64) {
                LayoutQueries::layout_queries_set_float_by_range(&mut leading_consumed, (decision.range).clone(), decision.leading_glue_consumed);
            }
            index = u32::wrapping_add(index, 1);
        }
        let mut leading_auto_space_gaps: Vec<FloatRangeValue> = vec![];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().auto_space_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().auto_space_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if decision.side.to_ustring() == UString::from("leading") {
                LayoutQueries::layout_queries_set_float_by_range(&mut leading_auto_space_gaps, (decision.cluster_range).clone(), -decision.total_reduction);
            }
            index = u32::wrapping_add(index, 1);
        }
        let mut positioned: Vec<PositionedCluster> = vec![];
        let mut x = line.indent;
        let mut cluster_index = line.cluster_range.start;
        let mut index_in_line = 0u32;
        while !line.cluster_range.get_is_empty() && (i32::from_ne_bytes(((cluster_index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((line.cluster_range.end) as i32).to_ne_bytes()) {
            if cluster_index <= 2147483647 && (i32::from_ne_bytes(((cluster_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                let cluster = (result.clusters[usize::try_from(cluster_index).unwrap_or(0)]).clone();
                let leading_gap = if index_in_line == 0 { 0.0f64 } else { LayoutQueries::layout_queries_float_by_range(&leading_auto_space_gaps, (cluster.range).clone()) };
                let draw_x = x + cluster.leading_layout_advance + cluster.glyph_inline_shift + leading_gap - LayoutQueries::layout_queries_float_by_range(&leading_consumed, (cluster.range).clone());
                let right = x + cluster.advance;
                let glyphs = LayoutQueries::layout_queries_glyphs_for_cluster((result).clone(), (cluster.range).clone());
                let mut source_stops: Option<Vec<f64>> = None;
                if i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes()) > (1) && u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) == (cluster.range).clone().get_length() {
                    let mut stops = vec![x];
                    let mut glyph_index = 1u32;
                    while (i32::from_ne_bytes(((glyph_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes())) {
                        stops.push(LayoutQueries::layout_queries_clamp_float(draw_x + glyphs[usize::try_from(glyph_index).unwrap_or(0)].x, x, right));
                        glyph_index = u32::wrapping_add(glyph_index, 1);
                    }
                    stops.push(right);
                    source_stops = Some(stops.clone());
                }
                positioned.push(PositionedCluster::new(line_index, cluster_index, (cluster.range).clone(), x, line.top, right, line.bottom, line.baseline + cluster.baseline_shift, draw_x, (source_stops).clone()));
                x += cluster.advance;
                index_in_line = u32::wrapping_add(index_in_line, 1);
            }
            cluster_index = u32::wrapping_add(cluster_index, 1);
        }
        return LayoutQueries::layout_queries_with_ruby_selection_geometry((result).clone(), &positioned, line_index);
    }

    pub(crate) fn layout_queries_with_ruby_selection_geometry(result: LayoutResult, positioned: &Vec<PositionedCluster>, line_index: u32) -> Vec<PositionedCluster> {
        let mut rubies: Vec<RubyDecisionInfo> = vec![];
        let mut ruby_index = 0u32;
        while (i32::from_ne_bytes(((ruby_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let ruby = ((result.debug).clone().ruby_decisions[usize::try_from(ruby_index).unwrap_or(0)]).clone();
            if ruby.line_index == line_index && (ruby.width) > (0.0f64) {
                rubies.push(ruby.clone());
            }
            ruby_index = u32::wrapping_add(ruby_index, 1);
        }
        if u32::try_from((rubies.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return (*positioned).clone();
        }
        let capacity = positioned.len();
        let mut pipeline_result = Vec::with_capacity(capacity);
        for pipeline_index in 0..match u32::try_from(positioned.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let v = (positioned[usize::try_from(pipeline_index).unwrap_or(0)]).clone();
            let spread = LayoutQueries::layout_queries_float_by_range_from_geometry((result).clone(), (v.range).clone());
            pipeline_result.push(SelectionBounds::new(v.left, LayoutQueries::layout_queries_max_float(v.right - spread, v.left)));
        }
        let mut bounds = (pipeline_result).clone();
        let mut ruby_index = 0u32;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((ruby_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((rubies.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let ruby = (rubies[usize::try_from(ruby_index).unwrap_or(0)]).clone();
            let mut _g: Vec<u32> = vec![];
            for index in 0..match u32::try_from(positioned.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if i32::from_ne_bytes(((((positioned[usize::try_from(index).unwrap_or(0)]).clone().range).clone().start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((ruby.base_range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((((positioned[usize::try_from(index).unwrap_or(0)]).clone().range).clone().end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((ruby.base_range).clone().end) as i32).to_ne_bytes()) {
                    _g.push(index);
                }
            }
            let base_indices = (_g).clone();
            if i32::from_ne_bytes(((u32::try_from((base_indices.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                let capacity = base_indices.len();
                let mut pipeline_result1 = Vec::with_capacity(capacity);
                for pipeline_index1 in 0..match u32::try_from(base_indices.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                    let v = base_indices[usize::try_from(pipeline_index1).unwrap_or(0)];
                    pipeline_result1.push(LayoutQueries::layout_queries_center_of_cluster((result).clone(), (positioned[usize::try_from(v).unwrap_or(0)]).clone()));
                }
                let centers = (pipeline_result1).clone();
                let ruby_left = ruby.center_x - ruby.width / 2.0f64;
                let ruby_right = ruby.center_x + ruby.width / 2.0f64;
                index = 0u32;
                while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((base_indices.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let idx = base_indices[usize::try_from(index).unwrap_or(0)];
                    let bound = (bounds[usize::try_from(idx).unwrap_or(0)]).clone();
                    bounds[usize::try_from(idx).unwrap_or(0)] = SelectionBounds::new(LayoutQueries::layout_queries_min_float(bound.left, if index == 0 { ruby_left } else { LayoutQueries::layout_queries_max_float(ruby_left, (centers[usize::try_from(u32::wrapping_sub(index, 1)).unwrap_or(0)] + centers[usize::try_from(index).unwrap_or(0)]) / 2.0f64) }), LayoutQueries::layout_queries_max_float(bound.right, if index == u32::wrapping_sub(u32::try_from((base_indices.len()) & 0xFFFF_FFFF).unwrap_or(0), 1) { ruby_right } else { LayoutQueries::layout_queries_min_float(ruby_right, (centers[usize::try_from(index).unwrap_or(0)] + centers[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]) / 2.0f64) }));
                    index = u32::wrapping_add(index, 1);
                }
            }
            ruby_index = u32::wrapping_add(ruby_index, 1);
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes(((u32::wrapping_add(index, 1)) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((bounds.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let left = (bounds[usize::try_from(index).unwrap_or(0)]).clone();
            let right = (bounds[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]).clone();
            if left.right > (right.left) {
                let center = LayoutQueries::layout_queries_clamp_float((LayoutQueries::layout_queries_center_of_cluster((result).clone(), (positioned[usize::try_from(index).unwrap_or(0)]).clone()) + LayoutQueries::layout_queries_center_of_cluster((result).clone(), (positioned[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)]).clone())) / 2.0f64, LayoutQueries::layout_queries_min_float(left.left, right.left), LayoutQueries::layout_queries_max_float(left.right, right.right));
                bounds[usize::try_from(index).unwrap_or(0)] = SelectionBounds::new(left.left, LayoutQueries::layout_queries_max_float(LayoutQueries::layout_queries_min_float(left.right, center), left.left));
                bounds[usize::try_from(u32::wrapping_add(index, 1)).unwrap_or(0)] = SelectionBounds::new(LayoutQueries::layout_queries_min_float(LayoutQueries::layout_queries_max_float(right.left, center), right.right), right.right);
            }
            index = u32::wrapping_add(index, 1);
        }
        let mut output: Vec<PositionedCluster> = vec![];
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let cluster = (positioned[usize::try_from(index).unwrap_or(0)]).clone();
            let bound = (bounds[usize::try_from(index).unwrap_or(0)]).clone();
            output.push(PositionedCluster::new(cluster.line_index, cluster.cluster_index, (cluster.range).clone(), bound.left, cluster.top, bound.right, cluster.bottom, cluster.baseline, cluster.draw_x, None));
            index = u32::wrapping_add(index, 1);
        }
        return output;
    }

    pub(crate) fn layout_queries_trim_outer_punctuation_glue(result: LayoutResult, segments: &Vec<RichTextLineSegment>) -> Vec<RichTextLineSegment> {
        let mut output: Vec<RichTextLineSegment> = vec![];
        let mut segment_index = 0u32;
        while (i32::from_ne_bytes(((segment_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let segment = (segments[usize::try_from(segment_index).unwrap_or(0)]).clone();
            if segment.line_index > 2147483647 || (i32::from_ne_bytes(((segment.line_index) as i32).to_ne_bytes())) >= i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) {
                output.push(segment.clone());
                segment_index = u32::wrapping_add(segment_index, 1);
                continue;
            }
            let line = (result.lines[usize::try_from(segment.line_index).unwrap_or(0)]).clone();
            let mut first: Option<Cluster> = None;
            let mut last: Option<Cluster> = None;
            let mut cluster_index = line.cluster_range.start;
            while !line.cluster_range.get_is_empty() && (i32::from_ne_bytes(((cluster_index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((line.cluster_range.end) as i32).to_ne_bytes()) {
                if cluster_index <= 2147483647 && (i32::from_ne_bytes(((cluster_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let cluster = (result.clusters[usize::try_from(cluster_index).unwrap_or(0)]).clone();
                    if i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes()) > (i32::from_ne_bytes((((segment.range).clone().start) as i32).to_ne_bytes())) && first.is_none() {
                        first = Some(cluster.clone());
                    }
                    if i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes()) < (i32::from_ne_bytes((((segment.range).clone().end) as i32).to_ne_bytes())) {
                        last = Some(cluster.clone());
                    }
                }
                cluster_index = u32::wrapping_add(cluster_index, 1);
            }
            let mut leading_glue = 0.0f64;
            let mut trailing_glue = 0.0f64;
            match &(first) {
                Some(__option9) => {
                    if segment.range.clone().start == __option9.range.start {
                    let decision = LayoutQueries::layout_queries_geometry_decision_for_range((result).clone(), (__option9.range).clone());
                    match &(decision) {
                        Some(__option10) => {
                            leading_glue = LayoutQueries::layout_queries_max_float(__option10.leading_glue_natural - __option10.leading_glue_consumed, 0.0f64);
                        }
                        None => {
                        }
                    }
                    }
                }
                None => {
                }
            }
            match &(last) {
                Some(__option11) => {
                    if segment.range.clone().end == __option11.range.end {
                    let decision = LayoutQueries::layout_queries_geometry_decision_for_range((result).clone(), (__option11.range).clone());
                    match &(decision) {
                        Some(__option12) => {
                            trailing_glue = LayoutQueries::layout_queries_max_float(__option12.trailing_glue_natural - __option12.trailing_glue_consumed, 0.0f64);
                        }
                        None => {
                        }
                    }
                    }
                }
                None => {
                }
            }
            let left = LayoutQueries::layout_queries_min_float(segment.right, segment.left + leading_glue);
            output.push(RichTextLineSegment::new((segment.span).clone(), segment.line_index, (segment.range).clone(), left, segment.top, LayoutQueries::layout_queries_max_float(left, segment.right - trailing_glue), segment.bottom, segment.baseline));
            segment_index = u32::wrapping_add(segment_index, 1);
        }
        return output;
    }

    pub(crate) fn layout_queries_with_adjacent_same_style_clearance(_result: LayoutResult, segments: &Vec<RichTextLineSegment>) -> Vec<RichTextLineSegment> {
        if i32::from_ne_bytes(((u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (2) {
            return (*segments).clone();
        }
        let mut output: Vec<RichTextLineSegment> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let segment = (segments[usize::try_from(index).unwrap_or(0)]).clone();
            let mut leading_neighbour: Option<RichTextLineSegment> = None;
            let mut trailing_neighbour: Option<RichTextLineSegment> = None;
            let mut other_index = 0u32;
            while (i32::from_ne_bytes(((other_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((segments.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                let other = (segments[usize::try_from(other_index).unwrap_or(0)]).clone();
                if other.line_index == segment.line_index && LayoutQueries::layout_queries_same_visible_style((segment).clone(), (other).clone()) {
                    if other.range.clone().end == (segment.range).clone().start {
                        leading_neighbour = Some(other.clone());
                    }
                    if other.range.clone().start == (segment.range).clone().end {
                        trailing_neighbour = Some(other.clone());
                    }
                }
                other_index = u32::wrapping_add(other_index, 1);
            }
            let leading_clearance = LayoutQueries::layout_queries_shared_clearance((segment).clone(), (leading_neighbour).clone());
            let trailing_clearance = LayoutQueries::layout_queries_shared_clearance((segment).clone(), (trailing_neighbour).clone());
            let left = LayoutQueries::layout_queries_min_float(segment.right, segment.left + leading_clearance / 2.0f64);
            output.push(RichTextLineSegment::new((segment.span).clone(), segment.line_index, (segment.range).clone(), left, segment.top, LayoutQueries::layout_queries_max_float(left, segment.right - trailing_clearance / 2.0f64), segment.bottom, segment.baseline));
            index = u32::wrapping_add(index, 1);
        }
        return output;
    }

    pub(crate) fn layout_queries_marked_face_vertical_bounds(result: LayoutResult, covered: &Vec<PositionedCluster>) -> Vec<f64> {
        let mut top = f64::INFINITY;
        let mut bottom = f64::NEG_INFINITY;
        for cluster in covered {
            let metric = LayoutQueries::layout_queries_last_metric_containing((result).clone(), (cluster.range).clone());
            match &(metric) {
                Some(__option13) => {
                    top = LayoutQueries::layout_queries_min_float(top, cluster.baseline - __option13.layout_ascent);
                    bottom = LayoutQueries::layout_queries_max_float(bottom, cluster.baseline + __option13.layout_descent);
                }
                None => {
                    let style = LayoutQueries::layout_queries_resolved_text_style_at((result).clone(), (cluster.range).clone().start);
                    top = LayoutQueries::layout_queries_min_float(top, cluster.baseline - style.font_size * LayoutQueries::LAYOUT_QUERIES_BACKGROUND_FALLBACK_ASCENT_EM);
                    bottom = LayoutQueries::layout_queries_max_float(bottom, cluster.baseline + style.font_size * LayoutQueries::LAYOUT_QUERIES_BACKGROUND_FALLBACK_DESCENT_EM);
                }
            }
        }
        return vec![top, bottom];
    }

    pub(crate) fn layout_queries_uniform_text_style_vertical_bounds(result: LayoutResult, segment: RichTextLineSegment, style: TextStyle) -> Vec<f64> {
        let mut reference: Option<MetricDecisionInfo> = None;
        let mut first_match: Option<MetricDecisionInfo> = None;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().metric_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().metric_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if LayoutQueries::layout_queries_same_font_metric_style(LayoutQueries::layout_queries_resolved_text_style_at((result).clone(), (decision.range).clone().start), (style).clone()) {
                if first_match.is_none() {
                    first_match = Some(decision.clone());
                }
                if decision.metric_box.to_ustring() == LayoutQueries::LAYOUT_QUERIES_IDEOGRAPHIC_EM_BOX_NAME.to_ustring() {
                    reference = Some(decision.clone());
                }
            }
            index = u32::wrapping_add(index, 1);
        }
        if reference.is_none() {
            reference = first_match;
        }
        let ascent = match &(reference) { None => style.font_size * LayoutQueries::LAYOUT_QUERIES_BACKGROUND_FALLBACK_ASCENT_EM, Some(__option14) => __option14.layout_ascent };
        let descent = match &(reference) { None => style.font_size * LayoutQueries::LAYOUT_QUERIES_BACKGROUND_FALLBACK_DESCENT_EM, Some(__option15) => __option15.layout_descent };
        return vec![segment.baseline - ascent, segment.baseline + descent];
    }

    pub(crate) fn layout_queries_resolved_text_style_at(result: LayoutResult, offset: u32) -> TextStyle {
        let mut index = u32::try_from((((result.input).clone().content).clone().spans.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (index) > (0) {
            let span = (((result.input).clone().content).clone().spans[usize::try_from(u32::wrapping_sub(index, 1)).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((offset) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((span.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((offset) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((span.range).clone().end) as i32).to_ne_bytes())) {
                return ((span.style).clone()).clone();
            }
            index = u32::wrapping_sub(index, 1);
        }
        return (((result.input).clone().text_style).clone()).clone();
    }

    pub(crate) fn layout_queries_same_font_metric_style(first: TextStyle, second: TextStyle) -> bool {
        return LayoutQueries::layout_queries_same_string_array(&first.font_families, &second.font_families) && first.font_size == second.font_size && (first.locale).to_ustring() == (second.locale).to_ustring() && first.font_weight == second.font_weight && first.italic == second.italic && first.baseline_shift == second.baseline_shift;
    }

    pub(crate) fn layout_queries_nearest_line_for_offset(result: LayoutResult, offset: u32) -> u32 {
        let mut best_index = 0u32;
        let mut best_distance = 2147483647u32;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let line = (result.lines[usize::try_from(index).unwrap_or(0)]).clone();
            let distance = if i32::from_ne_bytes(((offset) as i32).to_ne_bytes()) < (i32::from_ne_bytes((((line.range).clone().start) as i32).to_ne_bytes())) { u32::wrapping_sub((line.range).clone().start, offset) } else { if i32::from_ne_bytes(((offset) as i32).to_ne_bytes()) > (i32::from_ne_bytes((((line.range).clone().end) as i32).to_ne_bytes())) { u32::wrapping_sub(offset, (line.range).clone().end) } else { 0 } };
            if ({ let v: u32 = distance; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((best_distance) as i32).to_ne_bytes())) {
                best_distance = distance;
                best_index = index;
            }
            index = u32::wrapping_add(index, 1);
        }
        return best_index;
    }

    pub(crate) fn layout_queries_nearest_line_for_y(result: LayoutResult, y: f64) -> u32 {
        let mut best_index = 0u32;
        let mut best_distance = f64::INFINITY;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let line = (result.lines[usize::try_from(index).unwrap_or(0)]).clone();
            let distance = if y < (line.top) { line.top - y } else { if y > (line.bottom) { y - line.bottom } else { 0.0f64 } };
            if distance < (best_distance) {
                best_distance = distance;
                best_index = index;
            }
            index = u32::wrapping_add(index, 1);
        }
        return best_index;
    }

    pub(crate) fn layout_queries_nearest_cluster(clusters: &Vec<PositionedCluster>, x: f64) -> PositionedCluster {
        let mut best = (clusters[0usize]).clone();
        let mut best_distance = LayoutQueries::layout_queries_distance_to_cluster((best).clone(), x);
        let mut index = 1u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((clusters.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let candidate = (clusters[usize::try_from(index).unwrap_or(0)]).clone();
            let distance = LayoutQueries::layout_queries_distance_to_cluster((candidate).clone(), x);
            if distance < (best_distance) {
                best = candidate;
                best_distance = distance;
            }
            index = u32::wrapping_add(index, 1);
        }
        return best;
    }

    pub(crate) fn layout_queries_distance_to_cluster(cluster: PositionedCluster, x: f64) -> f64 {
        if x < (cluster.left) {
            return cluster.left - x;
        }
        if x > (cluster.right) {
            return x - cluster.right;
        }
        return 0.0f64;
    }

    pub(crate) fn layout_queries_x_for_offset(cluster: PositionedCluster, offset: u32) -> f64 {
        if i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes()) <= 0 {
            return cluster.left;
        }
        let index = LayoutQueries::layout_queries_clamp_int(u32::wrapping_sub(offset, (cluster.range).clone().start), 0, (cluster.range).clone().get_length());
        match &(cluster.source_stops) {
            Some(__option16) => {
                return ((*__option16).clone())[usize::try_from(index).unwrap_or(0)];
            }
            None => {
            }
        }
        return cluster.left + cluster.get_width() * (format!("{}", (i32::from_ne_bytes(((index) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) * 1.0f64 / format!("{}", (i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0));
    }

    pub(crate) fn layout_queries_offset_for_x(cluster: PositionedCluster, x: f64) -> u32 {
        if i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes()) <= 0 {
            return (cluster.range).clone().start;
        }
        match &(cluster.source_stops) {
            Some(__option17) => {
                let mut best_index = 0u32;
                let mut best_distance = f64::INFINITY;
                let mut index = 0u32;
                while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((*__option17).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let distance = (x - ((*__option17).clone())[usize::try_from(index).unwrap_or(0)]).abs();
                    if distance < (best_distance) {
                        best_distance = distance;
                        best_index = index;
                    }
                    index = u32::wrapping_add(index, 1);
                }
                return LayoutQueries::layout_queries_clamp_int(u32::wrapping_add((cluster.range).clone().start, best_index), (cluster.range).clone().start, (cluster.range).clone().end);
            }
            None => {
            }
        }
        if cluster.get_width() <= 0.0f64 {
            return (cluster.range).clone().start;
        }
        let ratio = LayoutQueries::layout_queries_clamp_float((x - cluster.left) / cluster.get_width(), 0.0f64, 1.0f64);
        return LayoutQueries::layout_queries_clamp_int(u32::wrapping_add((cluster.range).clone().start, u32::from_ne_bytes(((match f64::from(f64::round(ratio * format!("{}", (i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0))) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes())), (cluster.range).clone().start, (cluster.range).clone().end);
    }

    pub(crate) fn layout_queries_slice_rect(cluster: PositionedCluster, start: u32, end: u32) -> Rect {
        if i32::from_ne_bytes((((cluster.range).clone().get_length()) as i32).to_ne_bytes()) <= 0 || (cluster.get_width()) <= 0.0f64 {
            return cluster.get_rect();
        }
        return Rect::new(LayoutQueries::layout_queries_x_for_offset((cluster).clone(), start), cluster.top, LayoutQueries::layout_queries_x_for_offset((cluster).clone(), end), cluster.bottom);
    }

    pub(crate) fn layout_queries_slice_rect_if_covered(cluster: PositionedCluster, start: u32, end: u32) -> Option<Rect> {
        let slice_start = LayoutQueries::layout_queries_max_int(start, (cluster.range).clone().start);
        let slice_end = LayoutQueries::layout_queries_min_int(end, (cluster.range).clone().end);
        return if i32::from_ne_bytes(((slice_start) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((slice_end) as i32).to_ne_bytes())) { Some(LayoutQueries::layout_queries_slice_rect((cluster).clone(), slice_start, slice_end)) } else { None };
    }

    pub(crate) fn layout_queries_glyphs_for_cluster(result: LayoutResult, range: TextRange) -> Vec<Glyph> {
        let mut output: Vec<Glyph> = vec![];
        let mut run_index = 0u32;
        while (i32::from_ne_bytes(((run_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((result.glyph_runs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let glyphs = ((result.glyph_runs[usize::try_from(run_index).unwrap_or(0)]).clone().glyphs).clone();
            let mut glyph_index = 0u32;
            while (i32::from_ne_bytes(((glyph_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                if LayoutQueries::layout_queries_same_range(((glyphs[usize::try_from(glyph_index).unwrap_or(0)]).clone().cluster_range).clone(), (range).clone()) {
                    output.push((glyphs[usize::try_from(glyph_index).unwrap_or(0)]).clone());
                }
                glyph_index = u32::wrapping_add(glyph_index, 1);
            }
            run_index = u32::wrapping_add(run_index, 1);
        }
        return output;
    }

    pub(crate) fn layout_queries_natural_last_right(result: LayoutResult, cluster: PositionedCluster) -> f64 {
        let glyphs = LayoutQueries::layout_queries_glyphs_for_cluster((result).clone(), (cluster.range).clone());
        if u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return cluster.right;
        }
        let mut right = f64::NEG_INFINITY;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            right = LayoutQueries::layout_queries_max_float(right, cluster.draw_x + glyphs[usize::try_from(index).unwrap_or(0)].x + glyphs[usize::try_from(index).unwrap_or(0)].advance);
            index = u32::wrapping_add(index, 1);
        }
        return right;
    }

    pub(crate) fn layout_queries_center_of_cluster(result: LayoutResult, cluster: PositionedCluster) -> f64 {
        let glyphs = LayoutQueries::layout_queries_glyphs_for_cluster((result).clone(), (cluster.range).clone());
        let mut natural = 0.0f64;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            natural += glyphs[usize::try_from(index).unwrap_or(0)].advance;
            index = u32::wrapping_add(index, 1);
        }
        if u32::try_from((glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            natural = cluster.get_width() - LayoutQueries::layout_queries_float_by_range_from_geometry((result).clone(), (cluster.range).clone());
        }
        return cluster.draw_x + LayoutQueries::layout_queries_max_float(natural, 0.0f64) / 2.0f64;
    }

    pub(crate) fn layout_queries_float_by_range_from_geometry(result: LayoutResult, range: TextRange) -> f64 {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().geometry_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().geometry_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if LayoutQueries::layout_queries_same_range((decision.range).clone(), (range).clone()) && decision.ruby_spread != 0.0f64 {
                return decision.ruby_spread;
            }
            index = u32::wrapping_add(index, 1);
        }
        return 0.0f64;
    }

    pub(crate) fn layout_queries_clusters_on_segment_line(positioned: &Vec<PositionedCluster>, segment: RichTextLineSegment) -> Vec<PositionedCluster> {
        let mut output: Vec<PositionedCluster> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let cluster = (positioned[usize::try_from(index).unwrap_or(0)]).clone();
            if cluster.line_index == segment.line_index && (i32::from_ne_bytes((((cluster.range).clone().end) as i32).to_ne_bytes())) > (i32::from_ne_bytes((((segment.range).clone().start) as i32).to_ne_bytes())) && (i32::from_ne_bytes((((cluster.range).clone().start) as i32).to_ne_bytes())) < (i32::from_ne_bytes((((segment.range).clone().end) as i32).to_ne_bytes())) {
                output.push(cluster.clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return output;
    }

    pub(crate) fn layout_queries_find_positioned_by_range(positioned: &Vec<PositionedCluster>, range: TextRange) -> Option<PositionedCluster> {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((positioned.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if LayoutQueries::layout_queries_same_range(((positioned[usize::try_from(index).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                return Some((positioned[usize::try_from(index).unwrap_or(0)]).clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return None;
    }

    pub(crate) fn layout_queries_geometry_decision_for_range(result: LayoutResult, range: TextRange) -> Option<ClusterGeometryDecisionInfo> {
        let mut found: Option<ClusterGeometryDecisionInfo> = None;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().geometry_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().geometry_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if LayoutQueries::layout_queries_same_range((decision.range).clone(), (range).clone()) {
                found = Some(decision.clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return found;
    }

    pub(crate) fn layout_queries_last_metric_containing(result: LayoutResult, range: TextRange) -> Option<MetricDecisionInfo> {
        let mut found: Option<MetricDecisionInfo> = None;
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((result.debug).clone().metric_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let decision = ((result.debug).clone().metric_decisions[usize::try_from(index).unwrap_or(0)]).clone();
            if i32::from_ne_bytes(((range.start) as i32).to_ne_bytes()) >= i32::from_ne_bytes((((decision.range).clone().start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((range.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes((((decision.range).clone().end) as i32).to_ne_bytes()) {
                found = Some(decision.clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return found;
    }

    pub(crate) fn layout_queries_set_float_by_range(values: &mut Vec<FloatRangeValue>, range: TextRange, value: f64) {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if LayoutQueries::layout_queries_same_range(((values[usize::try_from(index).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                values[usize::try_from(index).unwrap_or(0)].value = value;
                return;
            }
            index = u32::wrapping_add(index, 1);
        }
        values.push(FloatRangeValue { range: range, value: value });
    }

    pub(crate) fn layout_queries_float_by_range(values: &Vec<FloatRangeValue>, range: TextRange) -> f64 {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if LayoutQueries::layout_queries_same_range(((values[usize::try_from(index).unwrap_or(0)]).clone().range).clone(), (range).clone()) {
                return values[usize::try_from(index).unwrap_or(0)].value;
            }
            index = u32::wrapping_add(index, 1);
        }
        return 0.0f64;
    }

    pub(crate) fn layout_queries_selection_word_kind(text: &UStr, start: u32, end: u32) -> SelectionWordKind {
        let code_point = SourceInteractionBoundaries::source_interaction_boundaries_code_point_at_compat(text, start, end);
        if code_point == LayoutQueries::LAYOUT_QUERIES_CR || code_point == LayoutQueries::LAYOUT_QUERIES_LF || code_point == LayoutQueries::LAYOUT_QUERIES_NEL || code_point == LayoutQueries::LAYOUT_QUERIES_LINE_SEPARATOR || code_point == LayoutQueries::LAYOUT_QUERIES_PARAGRAPH_SEPARATOR {
            return SelectionWordKind::Single;
        }
        if LayoutQueries::layout_queries_is_whitespace(code_point) {
            return SelectionWordKind::Whitespace;
        }
        if LayoutQueries::layout_queries_is_han_ideograph(code_point) {
            return SelectionWordKind::Single;
        }
        if LayoutQueries::layout_queries_is_letter_or_digit(code_point) || code_point == 95 || code_point == 39 || code_point == 8217 {
            return SelectionWordKind::Word;
        }
        return SelectionWordKind::Single;
    }

    pub(crate) fn layout_queries_is_whitespace(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 9 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 13 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 28 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 32 || code_point == 160 || code_point == 5760 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 8192 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 8202 || code_point == 8232 || code_point == 8233 || code_point == 8239 || code_point == 8287 || code_point == 12288;
    }

    pub(crate) fn layout_queries_is_letter_or_digit(code_point: u32) -> bool {
        if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 57343 {
            return false;
        }
        if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) >= 65 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 90 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 97 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 122 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 48 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 57 {
            return true;
        }
        if code_point > 2147483647 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) > (1114111) {
            return false;
        }
        return UnicodeWordCharacterData::unicode_word_character_data_contains(u32::from_ne_bytes(((code_point) as u32).to_ne_bytes()));
    }

    pub(crate) fn layout_queries_is_han_ideograph(code_point: u32) -> bool {
        return (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 13312 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 19903 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 19968 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 40959 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 63744 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 64255 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) >= 131072 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 205743;
    }

    pub(crate) fn layout_queries_boundary_index(boundaries: &Vec<u32>, offset: u32) -> u32 {
        let mut low = 0u32;
        let mut high = u32::wrapping_sub(u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
        while (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((high) as i32).to_ne_bytes()) {
            let middle = u32::wrapping_add(low, high) >> 1;
            let value = boundaries[usize::try_from(middle).unwrap_or(0)];
            if value == offset {
                return if middle == u32::wrapping_sub(u32::try_from((boundaries.len()) & 0xFFFF_FFFF).unwrap_or(0), 1) { u32::wrapping_sub(middle, 1) } else { middle };
            }
            if ({ let v: u32 = value; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((offset) as i32).to_ne_bytes())) {
                low = u32::wrapping_add(middle, 1);
            } else {
                high = u32::wrapping_sub(middle, 1);
            }
        }
        return LayoutQueries::layout_queries_max_int(0, high);
    }

    pub(crate) fn layout_queries_add_copy_annotation(values: &mut Vec<CopyAnnotation>, base_range: TextRange, text: &UStr, start: u32, end: u32) {
        if i32::from_ne_bytes(((base_range.start) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((start) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((base_range.end) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((end) as i32).to_ne_bytes()) {
            values.push(CopyAnnotation { end: base_range.end, text: text.to_ustring() });
        }
    }

    pub(crate) fn layout_queries_insertion_sort_annotations(values: &mut Vec<CopyAnnotation>) {
        let mut index = 1u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let value = (values[usize::try_from(index).unwrap_or(0)]).clone();
            let mut cursor = index;
            while (cursor) > (0) && (i32::from_ne_bytes(((values[usize::try_from(u32::wrapping_sub(cursor, 1)).unwrap_or(0)].end) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((value.end) as i32).to_ne_bytes())) {
                values[usize::try_from(cursor).unwrap_or(0)] = (values[usize::try_from(u32::wrapping_sub(cursor, 1)).unwrap_or(0)]).clone();
                cursor = u32::wrapping_sub(cursor, 1);
            }
            values[usize::try_from(cursor).unwrap_or(0)] = value;
            index = u32::wrapping_add(index, 1);
        }
    }

    pub(crate) fn layout_queries_same_range(first: TextRange, second: TextRange) -> bool {
        return first.start == second.start && first.end == second.end;
    }

    pub(crate) fn layout_queries_same_string_array(first: &[UString], second: &[UString]) -> bool {
        if u32::try_from((first.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((second.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((first.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if first[usize::try_from(index).unwrap_or(0)].clone() != (second[usize::try_from(index).unwrap_or(0)]).clone() {
                return false;
            }
            index = u32::wrapping_add(index, 1);
        }
        return true;
    }

    pub(crate) fn layout_queries_same_visible_style(first: RichTextLineSegment, second: RichTextLineSegment) -> bool {
        return RichTextSpan::rich_text_span_same_role((first.span).clone().role.clone(), (second.span).clone().role.clone()) && ((first.span).clone().paint).clone().same_visible_style(((second.span).clone().paint).clone());
    }

    pub(crate) fn layout_queries_shared_clearance(segment: RichTextLineSegment, neighbour: Option<RichTextLineSegment>) -> f64 {
        if match &(neighbour) { None => true, Some(__option18) => !LayoutQueries::layout_queries_same_visible_style((segment).clone(), ((*__option18).clone()).clone()) } {
            return 0.0f64;
        }
        return LayoutQueries::layout_queries_min_float(((segment.span).clone().paint).clone().adjacent_same_style_clearance, ((neighbour.as_ref().unwrap().span).clone().paint).clone().adjacent_same_style_clearance);
    }

    pub(crate) fn layout_queries_is_decoration_role(role: Box<dyn RichTextRole>) -> bool {
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Underline" || role.__haxe_type_name() == "org.tiqian.core.RichTextRole.LineThrough" {
            return true;
        }
        return false;
    }

    pub(crate) fn layout_queries_is_background_role(role: Box<dyn RichTextRole>) -> bool {
        if role.__haxe_type_name() == "org.tiqian.core.RichTextRole.Background" || role.__haxe_type_name() == "org.tiqian.core.RichTextRole.InlineCode" {
            return true;
        }
        return false;
    }

    pub(crate) fn layout_queries_resolve_radius(radius: f64, inset: f64, maximum: f64) -> f64 {
        return LayoutQueries::layout_queries_clamp_float(radius - inset, 0.0f64, maximum);
    }

    pub(crate) fn layout_queries_require_finite_non_negative(value: f64, message: &UStr) -> Result<(), TextRangeError> {
        if !LayoutQueries::layout_queries_is_finite(value) || (value) < (0.0f64) {
            return Err(TextRangeError::Message { text: message.to_ustring() });
        }
        Ok(())
    }

    pub(crate) fn layout_queries_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }

    pub(crate) fn layout_queries_clamp_int(value: u32, low: u32, high: u32) -> u32 {
        if i32::from_ne_bytes(((value) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((low) as i32).to_ne_bytes())) {
            return low;
        }
        if i32::from_ne_bytes(((value) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((high) as i32).to_ne_bytes())) {
            return high;
        }
        return value;
    }

    pub(crate) fn layout_queries_max_int(first: u32, second: u32) -> u32 {
        return if i32::from_ne_bytes(((first) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((second) as i32).to_ne_bytes())) { first } else { second };
    }

    pub(crate) fn layout_queries_min_int(first: u32, second: u32) -> u32 {
        return if i32::from_ne_bytes(((first) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((second) as i32).to_ne_bytes())) { first } else { second };
    }

    pub(crate) fn layout_queries_clamp_float(value: f64, low: f64, high: f64) -> f64 {
        if value < (low) {
            return low;
        }
        if value > (high) {
            return high;
        }
        return value;
    }

    pub(crate) fn layout_queries_max_float(first: f64, second: f64) -> f64 {
        return if first > (second) { first } else { second };
    }

    pub(crate) fn layout_queries_min_float(first: f64, second: f64) -> f64 {
        return if first < (second) { first } else { second };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionWordKind {
    Word,
    Whitespace,
    Single,
}

pub fn compare_selection_word_kind(a: &SelectionWordKind, b: &SelectionWordKind) -> i32 {
    if a == b { return 0; }
    fn rank(v: &SelectionWordKind) -> i32 {
        match v {
            SelectionWordKind::Word => 0,
            SelectionWordKind::Whitespace => 1,
            SelectionWordKind::Single => 2,
        }
    }
    rank(a) - rank(b)
}

impl SelectionWordKind {
    pub fn to_string(&self) -> String {
        match self {
            SelectionWordKind::Word => "Word".to_string(),
            SelectionWordKind::Whitespace => "Whitespace".to_string(),
            SelectionWordKind::Single => "Single".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyAnnotation {
    pub end: u32,
    pub text: UString,
}

pub fn compare_copy_annotation(a: &CopyAnnotation, b: &CopyAnnotation) -> i32 {
    let cmp_end = if a.end < b.end { -1 } else if a.end > b.end { 1 } else { 0 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_text = SortedTable::sorted_table_compare_strings(a.text.as_ustr(), b.text.as_ustr());
    if cmp_text != 0 { return cmp_text; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatRangeValue {
    pub range: TextRange,
    pub value: f64,
}
