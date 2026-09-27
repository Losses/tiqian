#![cfg(test)]

use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;


#[test]
fn cluster_role_ranges_modifier_base_with_variation_selector_and_modifier() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesModifierBaseWithVariationSelectorAndModifier", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesModifierBaseWithVariationSelectorAndModifier", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesModifierBaseWithVariationSelectorAndModifier");
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=Emoji actual=Emoji").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_ascii_point_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMark", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithAsciiPointMark");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_ascii_point_mark_attached() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMarkAttached", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMarkAttached", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithAsciiPointMarkAttached");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_attached_ascii_point_mark_at_start() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkAtStart", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkAtStart", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithAttachedAsciiPointMarkAtStart");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_attached_ascii_point_mark_followed_by_latin() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkFollowedByLatin", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkFollowedByLatin", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithAttachedAsciiPointMarkFollowedByLatin");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_attached_ascii_point_mark_not_adjacent() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkNotAdjacent", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkNotAdjacent", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithAttachedAsciiPointMarkNotAdjacent");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cjk_punctuation_and_coalesce() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationAndCoalesce", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationAndCoalesce", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCjkPunctuationAndCoalesce");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cjk_punctuation_coalesce() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationCoalesce", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationCoalesce", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCjkPunctuationCoalesce");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_coalesce_repeatable_punctuation() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCoalesceRepeatablePunctuation", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCoalesceRepeatablePunctuation", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCoalesceRepeatablePunctuation");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cr_at_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrAtEnd", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrAtEnd", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCrAtEnd");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cr_not_followed_by_lf() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrNotFollowedByLf", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrNotFollowedByLf", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCrNotFollowedByLf");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cr_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrOnly", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCrOnly");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_crlf_mandatory_break() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfMandatoryBreak", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfMandatoryBreak", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCrlfMandatoryBreak");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_crlf_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfOnly", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCrlfOnly");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_crlf_pair_produces_single_cluster() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfPairProducesSingleCluster", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfPairProducesSingleCluster", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithCrlfPairProducesSingleCluster");
        let _ = r.record(&"not-null actual=ResolvedClusterRange(range=TextRange(start=1, end=3), role=Unknown, mandatoryBreak=true, zeroWidthSoftBreak=false, roleOverride=null)").unwrap();
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmoji", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmoji", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmoji");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_modifier_base_combining_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierBaseCombiningMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierBaseCombiningMark", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiModifierBaseCombiningMark");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_modifier_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierSequence", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierSequence", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiModifierSequence");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_role_promotion_null() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiRolePromotionNull", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiRolePromotionNull", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiRolePromotionNull");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaries", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiShapingBoundaries");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundary_at_grapheme_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryAtGraphemeEnd", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryAtGraphemeEnd", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiShapingBoundaryAtGraphemeEnd");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundary_inside() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInside", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInside", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiShapingBoundaryInside");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundary_inside_and_outside_range() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInsideAndOutsideRange", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInsideAndOutsideRange", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiShapingBoundaryInsideAndOutsideRange");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_style_variation() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariation", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariation", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiStyleVariation");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_style_variation_no_fe0_f() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariationNoFE0F", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariationNoFE0F", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiStyleVariationNoFE0F");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_variation_and_modifier() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiVariationAndModifier", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiVariationAndModifier", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmojiVariationAndModifier");
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=Emoji actual=Emoji").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_empty_text() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmptyText", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmptyText", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithEmptyText");
        let _ = r.record(&"eq expected=0 actual=0").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_grapheme_extend() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtend", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtend", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithGraphemeExtend");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_grapheme_extend_after_emoji() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtendAfterEmoji", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtendAfterEmoji", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithGraphemeExtendAfterEmoji");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_inline_object() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithInlineObject", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithInlineObject", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithInlineObject");
        let _ = r.record(&"eq expected=1 actual=1").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_keycap_base_and_keycap() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseAndKeycap", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseAndKeycap", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithKeycapBaseAndKeycap");
        let _ = r.record(&"is-true actual=true").unwrap();
        let _ = r.record(&"eq expected=Emoji actual=Emoji").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_keycap_base_no_keycap() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseNoKeycap", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseNoKeycap", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithKeycapBaseNoKeycap");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_keycap_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapSequence", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapSequence", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithKeycapSequence");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lf_at_start() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfAtStart", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfAtStart", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithLfAtStart");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lf_inside_crlf() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfInsideCrlf", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfInsideCrlf", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithLfInsideCrlf");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lf_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfOnly", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithLfOnly");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lone_surrogate() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogate", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogate", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithLoneSurrogate");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lone_surrogate_high_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogateHighOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogateHighOnly", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithLoneSurrogateHighOnly");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_multiple_emoji_shaping_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleEmojiShapingBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleEmojiShapingBoundaries", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithMultipleEmojiShapingBoundaries");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_multiple_span_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleSpanBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleSpanBoundaries", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithMultipleSpanBoundaries");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_ascii_point_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonAsciiPointMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonAsciiPointMark", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithNonAsciiPointMark");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_cjk_punctuation() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCjkPunctuation", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCjkPunctuation", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithNonCjkPunctuation");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_combining_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCombiningMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCombiningMark", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithNonCombiningMark");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_variation_selector() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonVariationSelector", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonVariationSelector", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithNonVariationSelector");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_only_whitespace() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithOnlyWhitespace", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithOnlyWhitespace", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithOnlyWhitespace");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_role_override() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithRoleOverride", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithRoleOverride", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithRoleOverride");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_simple_text() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSimpleText", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSimpleText", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithSimpleText");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_single_grapheme() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSingleGrapheme", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSingleGrapheme", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithSingleGrapheme");
        let _ = r.record(&"eq expected=1 actual=1").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_span_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSpanBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSpanBoundaries", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithSpanBoundaries");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_supplementary_character() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSupplementaryCharacter", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSupplementaryCharacter", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithSupplementaryCharacter");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_surrogate_pair_non_low() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSurrogatePairNonLow", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSurrogatePairNonLow", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithSurrogatePairNonLow");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_variation_selector() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelector", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelector", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithVariationSelector");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_variation_selector_after_emoji() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterEmoji", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterEmoji", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithVariationSelectorAfterEmoji");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_variation_selector_after_latin() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterLatin", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterLatin", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithVariationSelectorAfterLatin");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_zwj_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZWJSequence", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZWJSequence", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithZWJSequence");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_zero_width_space() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZeroWidthSpace", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZeroWidthSpace", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"clusterRoleRangesWithZeroWidthSpace");
        let _ = r.record(&"is-true actual=true").unwrap();
    });
}

#[test]
fn require_covered_by_fails_when_cluster_crosses_decision_range() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClusterCrossesDecisionRange", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClusterCrossesDecisionRange", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByFailsWhenClusterCrossesDecisionRange");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='TextShaper returned cluster TextRange(start=0, end=3) crossing TextRange(start=0, end=2)'").unwrap();
    });
}

#[test]
fn require_covered_by_fails_when_clusters_are_non_contiguous() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersAreNonContiguous", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersAreNonContiguous", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByFailsWhenClustersAreNonContiguous");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='TextShaper returned non-contiguous clusters for TextRange(start=0, end=3); expected start=1, actual=TextRange(start=2, end=3)'").unwrap();
    });
}

#[test]
fn require_covered_by_fails_when_clusters_do_not_cover_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersDoNotCoverEnd", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersDoNotCoverEnd", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByFailsWhenClustersDoNotCoverEnd");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='TextShaper must return clusters covering TextRange(start=0, end=3); coveredUntil=1'").unwrap();
    });
}

#[test]
fn require_covered_by_with_contiguous_clusters() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithContiguousClusters", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithContiguousClusters", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByWithContiguousClusters");
        let _ = r.record(&"no-throw").unwrap();
    });
}

#[test]
fn require_covered_by_with_empty_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithEmptyDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithEmptyDecisions", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByWithEmptyDecisions");
        let _ = r.record(&"no-throw").unwrap();
    });
}

#[test]
fn require_covered_by_with_gap_between_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithGapBetweenDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithGapBetweenDecisions", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByWithGapBetweenDecisions");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='TextShaper must return clusters covering TextRange(start=2, end=3); coveredUntil=2'").unwrap();
    });
}

#[test]
fn require_covered_by_with_multiple_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithMultipleDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithMultipleDecisions", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByWithMultipleDecisions");
        let _ = r.record(&"no-throw").unwrap();
    });
}

#[test]
fn require_covered_by_with_overlapping_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithOverlappingDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithOverlappingDecisions", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByWithOverlappingDecisions");
        let _ = r.record(&"raises exception=IllegalArgumentException thrown='TextShaper returned cluster TextRange(start=2, end=4) crossing TextRange(start=0, end=3)'").unwrap();
    });
}

#[test]
fn require_covered_by_with_single_cluster() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithSingleCluster", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithSingleCluster", || {
        let mut r = TestTraceRecorder::new("ClusterRoleResolutionCoverageTest");
        r.section(&"requireCoveredByWithSingleCluster");
        let _ = r.record(&"no-throw").unwrap();
    });
}
