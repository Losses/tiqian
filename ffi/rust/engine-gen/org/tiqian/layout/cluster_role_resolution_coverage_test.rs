#![cfg(test)]

use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn cluster_role_ranges_modifier_base_with_variation_selector_and_modifier() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesModifierBaseWithVariationSelectorAndModifier", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesModifierBaseWithVariationSelectorAndModifier", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,77,111,100,105,102,105,101,114,66,97,115,101,87,105,116,104,86,97,114,105,97,116,105,111,110,83,101,108,101,99,116,111,114,65,110,100,77,111,100,105,102,105,101,114]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,69,109,111,106,105,32,97,99,116,117,97,108,61,69,109,111,106,105])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_ascii_point_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMark", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,65,115,99,105,105,80,111,105,110,116,77,97,114,107]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_ascii_point_mark_attached() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMarkAttached", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAsciiPointMarkAttached", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,65,115,99,105,105,80,111,105,110,116,77,97,114,107,65,116,116,97,99,104,101,100]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_attached_ascii_point_mark_at_start() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkAtStart", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkAtStart", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,65,116,83,116,97,114,116]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_attached_ascii_point_mark_followed_by_latin() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkFollowedByLatin", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkFollowedByLatin", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,70,111,108,108,111,119,101,100,66,121,76,97,116,105,110]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_attached_ascii_point_mark_not_adjacent() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkNotAdjacent", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithAttachedAsciiPointMarkNotAdjacent", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,65,116,116,97,99,104,101,100,65,115,99,105,105,80,111,105,110,116,77,97,114,107,78,111,116,65,100,106,97,99,101,110,116]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cjk_punctuation_and_coalesce() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationAndCoalesce", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationAndCoalesce", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,106,107,80,117,110,99,116,117,97,116,105,111,110,65,110,100,67,111,97,108,101,115,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cjk_punctuation_coalesce() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationCoalesce", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCjkPunctuationCoalesce", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,106,107,80,117,110,99,116,117,97,116,105,111,110,67,111,97,108,101,115,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_coalesce_repeatable_punctuation() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCoalesceRepeatablePunctuation", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCoalesceRepeatablePunctuation", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,111,97,108,101,115,99,101,82,101,112,101,97,116,97,98,108,101,80,117,110,99,116,117,97,116,105,111,110]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cr_at_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrAtEnd", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrAtEnd", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,114,65,116,69,110,100]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cr_not_followed_by_lf() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrNotFollowedByLf", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrNotFollowedByLf", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,114,78,111,116,70,111,108,108,111,119,101,100,66,121,76,102]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_cr_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrOnly", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,114,79,110,108,121]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_crlf_mandatory_break() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfMandatoryBreak", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfMandatoryBreak", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,114,108,102,77,97,110,100,97,116,111,114,121,66,114,101,97,107]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_crlf_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfOnly", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,114,108,102,79,110,108,121]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_crlf_pair_produces_single_cluster() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfPairProducesSingleCluster", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithCrlfPairProducesSingleCluster", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,67,114,108,102,80,97,105,114,80,114,111,100,117,99,101,115,83,105,110,103,108,101,67,108,117,115,116,101,114]));
        let _ = r.record(UStr::new(&[110,111,116,45,110,117,108,108,32,97,99,116,117,97,108,61,82,101,115,111,108,118,101,100,67,108,117,115,116,101,114,82,97,110,103,101,40,114,97,110,103,101,61,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,49,44,32,101,110,100,61,51,41,44,32,114,111,108,101,61,85,110,107,110,111,119,110,44,32,109,97,110,100,97,116,111,114,121,66,114,101,97,107,61,116,114,117,101,44,32,122,101,114,111,87,105,100,116,104,83,111,102,116,66,114,101,97,107,61,102,97,108,115,101,44,32,114,111,108,101,79,118,101,114,114,105,100,101,61,110,117,108,108,41])).unwrap();
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmoji", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmoji", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_modifier_base_combining_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierBaseCombiningMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierBaseCombiningMark", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,77,111,100,105,102,105,101,114,66,97,115,101,67,111,109,98,105,110,105,110,103,77,97,114,107]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_modifier_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierSequence", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiModifierSequence", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,77,111,100,105,102,105,101,114,83,101,113,117,101,110,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_role_promotion_null() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiRolePromotionNull", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiRolePromotionNull", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,82,111,108,101,80,114,111,109,111,116,105,111,110,78,117,108,108]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaries", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,83,104,97,112,105,110,103,66,111,117,110,100,97,114,105,101,115]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundary_at_grapheme_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryAtGraphemeEnd", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryAtGraphemeEnd", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,83,104,97,112,105,110,103,66,111,117,110,100,97,114,121,65,116,71,114,97,112,104,101,109,101,69,110,100]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundary_inside() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInside", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInside", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,83,104,97,112,105,110,103,66,111,117,110,100,97,114,121,73,110,115,105,100,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_shaping_boundary_inside_and_outside_range() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInsideAndOutsideRange", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiShapingBoundaryInsideAndOutsideRange", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,83,104,97,112,105,110,103,66,111,117,110,100,97,114,121,73,110,115,105,100,101,65,110,100,79,117,116,115,105,100,101,82,97,110,103,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_style_variation() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariation", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariation", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,83,116,121,108,101,86,97,114,105,97,116,105,111,110]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_style_variation_no_fe0_f() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariationNoFE0F", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiStyleVariationNoFE0F", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,83,116,121,108,101,86,97,114,105,97,116,105,111,110,78,111,70,69,48,70]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_emoji_variation_and_modifier() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiVariationAndModifier", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmojiVariationAndModifier", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,111,106,105,86,97,114,105,97,116,105,111,110,65,110,100,77,111,100,105,102,105,101,114]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,69,109,111,106,105,32,97,99,116,117,97,108,61,69,109,111,106,105])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_empty_text() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmptyText", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithEmptyText", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,69,109,112,116,121,84,101,120,116]));
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,48,32,97,99,116,117,97,108,61,48])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_grapheme_extend() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtend", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtend", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,71,114,97,112,104,101,109,101,69,120,116,101,110,100]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_grapheme_extend_after_emoji() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtendAfterEmoji", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithGraphemeExtendAfterEmoji", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,71,114,97,112,104,101,109,101,69,120,116,101,110,100,65,102,116,101,114,69,109,111,106,105]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_inline_object() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithInlineObject", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithInlineObject", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,73,110,108,105,110,101,79,98,106,101,99,116]));
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,49,32,97,99,116,117,97,108,61,49])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_keycap_base_and_keycap() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseAndKeycap", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseAndKeycap", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,75,101,121,99,97,112,66,97,115,101,65,110,100,75,101,121,99,97,112]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,69,109,111,106,105,32,97,99,116,117,97,108,61,69,109,111,106,105])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_keycap_base_no_keycap() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseNoKeycap", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapBaseNoKeycap", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,75,101,121,99,97,112,66,97,115,101,78,111,75,101,121,99,97,112]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_keycap_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapSequence", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithKeycapSequence", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,75,101,121,99,97,112,83,101,113,117,101,110,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lf_at_start() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfAtStart", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfAtStart", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,76,102,65,116,83,116,97,114,116]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lf_inside_crlf() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfInsideCrlf", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfInsideCrlf", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,76,102,73,110,115,105,100,101,67,114,108,102]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lf_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLfOnly", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,76,102,79,110,108,121]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lone_surrogate() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogate", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogate", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,76,111,110,101,83,117,114,114,111,103,97,116,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_lone_surrogate_high_only() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogateHighOnly", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithLoneSurrogateHighOnly", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,76,111,110,101,83,117,114,114,111,103,97,116,101,72,105,103,104,79,110,108,121]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_multiple_emoji_shaping_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleEmojiShapingBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleEmojiShapingBoundaries", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,77,117,108,116,105,112,108,101,69,109,111,106,105,83,104,97,112,105,110,103,66,111,117,110,100,97,114,105,101,115]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_multiple_span_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleSpanBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithMultipleSpanBoundaries", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,77,117,108,116,105,112,108,101,83,112,97,110,66,111,117,110,100,97,114,105,101,115]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_ascii_point_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonAsciiPointMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonAsciiPointMark", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,78,111,110,65,115,99,105,105,80,111,105,110,116,77,97,114,107]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_cjk_punctuation() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCjkPunctuation", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCjkPunctuation", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,78,111,110,67,106,107,80,117,110,99,116,117,97,116,105,111,110]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_combining_mark() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCombiningMark", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonCombiningMark", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,78,111,110,67,111,109,98,105,110,105,110,103,77,97,114,107]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_non_variation_selector() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonVariationSelector", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithNonVariationSelector", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,78,111,110,86,97,114,105,97,116,105,111,110,83,101,108,101,99,116,111,114]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_only_whitespace() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithOnlyWhitespace", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithOnlyWhitespace", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,79,110,108,121,87,104,105,116,101,115,112,97,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_role_override() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithRoleOverride", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithRoleOverride", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,82,111,108,101,79,118,101,114,114,105,100,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_simple_text() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSimpleText", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSimpleText", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,83,105,109,112,108,101,84,101,120,116]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_single_grapheme() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSingleGrapheme", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSingleGrapheme", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,83,105,110,103,108,101,71,114,97,112,104,101,109,101]));
        let _ = r.record(UStr::new(&[101,113,32,101,120,112,101,99,116,101,100,61,49,32,97,99,116,117,97,108,61,49])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_span_boundaries() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSpanBoundaries", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSpanBoundaries", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,83,112,97,110,66,111,117,110,100,97,114,105,101,115]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_supplementary_character() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSupplementaryCharacter", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSupplementaryCharacter", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,83,117,112,112,108,101,109,101,110,116,97,114,121,67,104,97,114,97,99,116,101,114]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_surrogate_pair_non_low() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSurrogatePairNonLow", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithSurrogatePairNonLow", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,83,117,114,114,111,103,97,116,101,80,97,105,114,78,111,110,76,111,119]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_variation_selector() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelector", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelector", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,86,97,114,105,97,116,105,111,110,83,101,108,101,99,116,111,114]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_variation_selector_after_emoji() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterEmoji", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterEmoji", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,86,97,114,105,97,116,105,111,110,83,101,108,101,99,116,111,114,65,102,116,101,114,69,109,111,106,105]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_variation_selector_after_latin() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterLatin", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithVariationSelectorAfterLatin", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,86,97,114,105,97,116,105,111,110,83,101,108,101,99,116,111,114,65,102,116,101,114,76,97,116,105,110]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_zwj_sequence() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZWJSequence", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZWJSequence", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,90,87,74,83,101,113,117,101,110,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn cluster_role_ranges_with_zero_width_space() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZeroWidthSpace", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.clusterRoleRangesWithZeroWidthSpace", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[99,108,117,115,116,101,114,82,111,108,101,82,97,110,103,101,115,87,105,116,104,90,101,114,111,87,105,100,116,104,83,112,97,99,101]));
        let _ = r.record(UStr::new(&[105,115,45,116,114,117,101,32,97,99,116,117,97,108,61,116,114,117,101])).unwrap();
    });
}

#[test]
fn require_covered_by_fails_when_cluster_crosses_decision_range() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClusterCrossesDecisionRange", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClusterCrossesDecisionRange", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,70,97,105,108,115,87,104,101,110,67,108,117,115,116,101,114,67,114,111,115,115,101,115,68,101,99,105,115,105,111,110,82,97,110,103,101]));
        let _ = r.record(UStr::new(&[114,97,105,115,101,115,32,101,120,99,101,112,116,105,111,110,61,73,108,108,101,103,97,108,65,114,103,117,109,101,110,116,69,120,99,101,112,116,105,111,110,32,116,104,114,111,119,110,61,39,84,101,120,116,83,104,97,112,101,114,32,114,101,116,117,114,110,101,100,32,99,108,117,115,116,101,114,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,51,41,32,99,114,111,115,115,105,110,103,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,50,41,39])).unwrap();
    });
}

#[test]
fn require_covered_by_fails_when_clusters_are_non_contiguous() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersAreNonContiguous", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersAreNonContiguous", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,70,97,105,108,115,87,104,101,110,67,108,117,115,116,101,114,115,65,114,101,78,111,110,67,111,110,116,105,103,117,111,117,115]));
        let _ = r.record(UStr::new(&[114,97,105,115,101,115,32,101,120,99,101,112,116,105,111,110,61,73,108,108,101,103,97,108,65,114,103,117,109,101,110,116,69,120,99,101,112,116,105,111,110,32,116,104,114,111,119,110,61,39,84,101,120,116,83,104,97,112,101,114,32,114,101,116,117,114,110,101,100,32,110,111,110,45,99,111,110,116,105,103,117,111,117,115,32,99,108,117,115,116,101,114,115,32,102,111,114,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,51,41,59,32,101,120,112,101,99,116,101,100,32,115,116,97,114,116,61,49,44,32,97,99,116,117,97,108,61,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,50,44,32,101,110,100,61,51,41,39])).unwrap();
    });
}

#[test]
fn require_covered_by_fails_when_clusters_do_not_cover_end() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersDoNotCoverEnd", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByFailsWhenClustersDoNotCoverEnd", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,70,97,105,108,115,87,104,101,110,67,108,117,115,116,101,114,115,68,111,78,111,116,67,111,118,101,114,69,110,100]));
        let _ = r.record(UStr::new(&[114,97,105,115,101,115,32,101,120,99,101,112,116,105,111,110,61,73,108,108,101,103,97,108,65,114,103,117,109,101,110,116,69,120,99,101,112,116,105,111,110,32,116,104,114,111,119,110,61,39,84,101,120,116,83,104,97,112,101,114,32,109,117,115,116,32,114,101,116,117,114,110,32,99,108,117,115,116,101,114,115,32,99,111,118,101,114,105,110,103,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,51,41,59,32,99,111,118,101,114,101,100,85,110,116,105,108,61,49,39])).unwrap();
    });
}

#[test]
fn require_covered_by_with_contiguous_clusters() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithContiguousClusters", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithContiguousClusters", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,87,105,116,104,67,111,110,116,105,103,117,111,117,115,67,108,117,115,116,101,114,115]));
        let _ = r.record(UStr::new(&[110,111,45,116,104,114,111,119])).unwrap();
    });
}

#[test]
fn require_covered_by_with_empty_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithEmptyDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithEmptyDecisions", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,87,105,116,104,69,109,112,116,121,68,101,99,105,115,105,111,110,115]));
        let _ = r.record(UStr::new(&[110,111,45,116,104,114,111,119])).unwrap();
    });
}

#[test]
fn require_covered_by_with_gap_between_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithGapBetweenDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithGapBetweenDecisions", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,87,105,116,104,71,97,112,66,101,116,119,101,101,110,68,101,99,105,115,105,111,110,115]));
        let _ = r.record(UStr::new(&[114,97,105,115,101,115,32,101,120,99,101,112,116,105,111,110,61,73,108,108,101,103,97,108,65,114,103,117,109,101,110,116,69,120,99,101,112,116,105,111,110,32,116,104,114,111,119,110,61,39,84,101,120,116,83,104,97,112,101,114,32,109,117,115,116,32,114,101,116,117,114,110,32,99,108,117,115,116,101,114,115,32,99,111,118,101,114,105,110,103,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,50,44,32,101,110,100,61,51,41,59,32,99,111,118,101,114,101,100,85,110,116,105,108,61,50,39])).unwrap();
    });
}

#[test]
fn require_covered_by_with_multiple_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithMultipleDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithMultipleDecisions", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,87,105,116,104,77,117,108,116,105,112,108,101,68,101,99,105,115,105,111,110,115]));
        let _ = r.record(UStr::new(&[110,111,45,116,104,114,111,119])).unwrap();
    });
}

#[test]
fn require_covered_by_with_overlapping_decisions() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithOverlappingDecisions", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithOverlappingDecisions", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,87,105,116,104,79,118,101,114,108,97,112,112,105,110,103,68,101,99,105,115,105,111,110,115]));
        let _ = r.record(UStr::new(&[114,97,105,115,101,115,32,101,120,99,101,112,116,105,111,110,61,73,108,108,101,103,97,108,65,114,103,117,109,101,110,116,69,120,99,101,112,116,105,111,110,32,116,104,114,111,119,110,61,39,84,101,120,116,83,104,97,112,101,114,32,114,101,116,117,114,110,101,100,32,99,108,117,115,116,101,114,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,50,44,32,101,110,100,61,52,41,32,99,114,111,115,115,105,110,103,32,84,101,120,116,82,97,110,103,101,40,115,116,97,114,116,61,48,44,32,101,110,100,61,51,41,39])).unwrap();
    });
}

#[test]
fn require_covered_by_with_single_cluster() {
    testlib::run("org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithSingleCluster", "org.tiqian.layout.ClusterRoleResolutionCoverageTest.requireCoveredByWithSingleCluster", || {
        let mut r = TestTraceRecorder::new(&(UStr::new(&[67,108,117,115,116,101,114,82,111,108,101,82,101,115,111,108,117,116,105,111,110,67,111,118,101,114,97,103,101,84,101,115,116])));
        r.section(UStr::new(&[114,101,113,117,105,114,101,67,111,118,101,114,101,100,66,121,87,105,116,104,83,105,110,103,108,101,67,108,117,115,116,101,114]));
        let _ = r.record(UStr::new(&[110,111,45,116,104,114,111,119])).unwrap();
    });
}
