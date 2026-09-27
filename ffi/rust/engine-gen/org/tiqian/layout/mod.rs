#![allow(ambiguous_glob_reexports)]

pub mod annotation_geometry_stage;
#[cfg(test)]
pub mod annotation_geometry_stage_coverage_test;
pub mod annotation_geometry_stage_coverage_test_support;
#[cfg(test)]
pub mod ascii_point_mark_kinsoku_test;
pub mod ascii_point_mark_kinsoku_test_support;
#[cfg(test)]
pub mod attached_inline_virtual_adjacency_test;
pub mod attached_inline_virtual_adjacency_test_support;
#[cfg(test)]
pub mod auto_space_single_gap_test;
pub mod auto_space_single_gap_test_support;
#[cfg(test)]
pub mod baseline_alignment_test;
#[cfg(test)]
pub mod bilingual_emphasis_test;
pub mod bilingual_emphasis_test_support;
#[cfg(test)]
pub mod bopomofo_layout_test;
pub mod bopomofo_layout_test_support;
pub mod cluster_role_resolution;
#[cfg(test)]
pub mod cluster_role_resolution_coverage_test;
#[cfg(test)]
pub mod cluster_role_resolution_surrogate_and_extender_edge_test;
#[cfg(test)]
pub mod contextual_dash_ellipsis_cluster_coverage_test;
#[cfg(test)]
pub mod contextual_dash_ellipsis_layout_test;
pub mod contextual_dash_ellipsis_role_resolver;
#[cfg(test)]
pub mod contextual_dash_ellipsis_role_resolver_coverage_test;
#[cfg(test)]
pub mod contextual_dash_ellipsis_role_resolver_test;
pub mod contextual_punctuation_display_substitution;
pub mod contextual_quote_role_resolver;
#[cfg(test)]
pub mod contextual_quote_role_resolver_coverage_test;
#[cfg(test)]
pub mod contextual_quote_role_resolver_nested_and_surrogate_test;
#[cfg(test)]
pub mod contextual_role_extension_coverage_test;
#[cfg(test)]
pub mod decide_hyphen_break_test;
pub mod decide_hyphen_break_test_support;
pub mod default_hyphenator;
#[cfg(test)]
pub mod display_glyph_substitution_engine_test;
pub mod display_glyph_substitution_engine_test_support;
#[cfg(test)]
pub mod emergency_grapheme_tracking_test;
pub mod emergency_grapheme_tracking_test_support;
#[cfg(test)]
pub mod explainable_stub_paragraph_layout_engine_test;
pub mod explainable_stub_paragraph_layout_engine_test_support;
#[cfg(test)]
pub mod font_instance_metrics_request_test;
pub mod font_instance_metrics_request_test_support;
#[cfg(test)]
pub mod greedy_line_breaker_test;
#[cfg(test)]
pub mod hyphenation_layout_test;
pub mod hyphenation_layout_test_support;
#[cfg(test)]
pub mod inline_box_layout_test;
pub mod inline_box_layout_test_support;
#[cfg(test)]
pub mod inline_object_layout_test;
pub mod inline_object_layout_test_support;
#[cfg(test)]
pub mod interpunct_shrink_opportunity_test;
pub mod interpunct_shrink_opportunity_test_support;
pub mod justifier;
#[cfg(test)]
pub mod justifier_compression_test;
#[cfg(test)]
pub mod justifier_coverage_test;
#[cfg(test)]
pub mod justifier_engine_test;
pub mod justifier_engine_test_support;
#[cfg(test)]
pub mod justifier_jf_test;
#[cfg(test)]
pub mod justifier_test;
#[cfg(test)]
pub mod kinsoku_and_cohesion_repair_engine_test;
pub mod kinsoku_rule;
pub mod layout_debug_assembly;
pub mod layout_dump_format;
#[cfg(test)]
pub mod layout_dump_golden_parity_test;
pub mod layout_dump_goldens;
#[cfg(test)]
pub mod line_adjustment_push_in_test;
pub mod line_adjustment_push_in_test_support;
pub mod line_adjustment_stage;
#[cfg(test)]
pub mod line_adjustment_stage_coverage_test;
pub mod line_adjustment_stage_coverage_test_support;
#[cfg(test)]
pub mod line_adjustment_stage_jf_test;
pub mod line_adjustment_stage_jf_test_support;
pub mod line_break_planning_stage;
#[cfg(test)]
pub mod line_break_planning_stage_coverage2_test;
pub mod line_break_planning_stage_coverage2_test_support;
#[cfg(test)]
pub mod line_break_planning_stage_coverage_test;
pub mod line_break_planning_stage_coverage_test_support;
#[cfg(test)]
pub mod line_break_repair_engine_test;
pub mod line_break_repair_engine_test_support;
pub mod line_breaker;
#[cfg(test)]
pub mod line_breaker_coverage2_test;
#[cfg(test)]
pub mod line_candidate_validation_test;
pub mod line_candidate_validation_test_support;
#[cfg(test)]
pub mod line_geometry_direct_tail_test;
pub mod line_geometry_stage;
pub mod line_optimization;
#[cfg(test)]
pub mod line_optimization_coverage_test;
pub mod line_repair;
#[cfg(test)]
pub mod line_repair_coverage_test;
#[cfg(test)]
pub mod line_repair_tail_coverage_test;
#[cfg(test)]
pub mod lookahead_line_breaker_test;
#[cfg(test)]
pub mod opening_bracket_line_start_test;
pub mod paragraph_dp_line_breaker;
#[cfg(test)]
pub mod paragraph_dp_line_breaker_coverage2_test;
pub mod paragraph_dp_line_breaker_coverage2_test_support;
#[cfg(test)]
pub mod paragraph_dp_line_breaker_coverage_test;
pub mod paragraph_dp_line_breaker_coverage_test_support;
#[cfg(test)]
pub mod paragraph_dp_line_breaker_test;
pub mod paragraph_dp_line_breaker_test_support;
#[cfg(test)]
pub mod paragraph_dp_tier_promotion_pool_test;
pub mod paragraph_dp_tier_promotion_pool_test_support;
pub mod paragraph_layout_engine;
pub mod paragraph_layout_engine_validation_coverage_support;
#[cfg(test)]
pub mod paragraph_layout_engine_validation_coverage_test;
pub mod paragraph_shaping_stage;
#[cfg(test)]
pub mod paragraph_shaping_stage_coverage_test;
pub mod paragraph_shaping_stage_coverage_test_support;
pub mod prepared_paragraph;
#[cfg(test)]
pub mod prepared_paragraph_inline_edges_test;
#[cfg(test)]
pub mod prepared_paragraph_jf_test;
pub mod prepared_paragraph_jf_test_support;
#[cfg(test)]
pub mod prepared_paragraph_json_number_test;
pub mod prepared_paragraph_json_number_test_support;
#[cfg(test)]
pub mod prepared_paragraph_plan_construction_test;
pub mod prepared_paragraph_plan_construction_test_support;
#[cfg(test)]
pub mod prepared_paragraph_render_evidence_test;
pub mod prepared_paragraph_render_evidence_test_support;
pub mod progressive_break_decisions;
#[cfg(test)]
pub mod progressive_break_decisions_coverage_test;
pub mod progressive_break_decisions_coverage_test_support;
#[cfg(test)]
pub mod progressive_break_decisions_tail_test;
pub mod progressive_break_decisions_tail_test_support;
pub mod progressive_break_tier_priority;
#[cfg(test)]
pub mod progressive_technical_break_test;
pub mod progressive_technical_break_test_support;
#[cfg(test)]
pub mod punctuation_atom_builder_halt_test;
#[cfg(test)]
pub mod punctuation_body_floor_invariant_test;
#[cfg(test)]
pub mod punctuation_geometry_branch_arms_coverage_test;
#[cfg(test)]
pub mod punctuation_geometry_engine_test;
pub mod punctuation_geometry_engine_test_support;
pub mod punctuation_geometry_ledger;
#[cfg(test)]
pub mod punctuation_geometry_ledger_coverage_test;
pub mod punctuation_geometry_stage;
#[cfg(test)]
pub mod punctuation_geometry_stage_coverage_test;
pub mod punctuation_model;
#[cfg(test)]
pub mod punctuation_model_coverage_test;
#[cfg(test)]
pub mod punctuation_spacing_rule_test;
pub mod punctuation_spacing_rule_test_support;
#[cfg(test)]
pub mod push_in_line_wide_capacity_test;
pub mod push_in_line_wide_capacity_test_support;
#[cfg(test)]
pub mod quote_classification_engine_test;
pub mod quote_classification_engine_test_support;
pub mod quote_pair_analyzer;
#[cfg(test)]
pub mod quote_pair_analyzer_coverage_test;
pub mod quote_pair_analyzer_coverage_test_support;
#[cfg(test)]
pub mod quote_pair_analyzer_surrogate_adjacency_test;
pub mod quote_pair_analyzer_surrogate_adjacency_test_support;
#[cfg(test)]
pub mod quote_pair_analyzer_test;
pub mod quote_pair_analyzer_test_support;
#[cfg(test)]
pub mod r3_geometry_tail_coverage_test;
pub mod r3_geometry_tail_coverage_test_support;
#[cfg(test)]
pub mod recorded_evidence_golden_parity_test;
pub mod recorded_layout_dump_goldens;
pub mod recorded_shaping_evidence_data;
#[cfg(test)]
pub mod ruby_layout_test;
pub mod ruby_layout_test_support;
#[cfg(test)]
pub mod spacing_and_line_geometry_engine_test;
pub mod spacing_and_line_geometry_engine_test_support;
#[cfg(test)]
pub mod unicode_emoji17_rgi_role_audit_test;
pub mod unicode_emoji17_rgi_role_audit_test_support;
pub mod unicode_punctuation_boundary_resolver;
#[cfg(test)]
pub mod unicode_punctuation_boundary_resolver_coverage_test;
#[cfg(test)]
pub mod unicode_punctuation_boundary_test;
pub mod unicode_punctuation_boundary_test_support;
#[cfg(test)]
pub mod verbatim_range_auto_space_test;
pub mod verbatim_range_auto_space_test_support;
pub mod width_independent_annotation_cache;
#[cfg(test)]
pub mod width_independent_annotation_cache_coverage_test;
pub mod width_independent_annotation_cache_coverage_test_support;
#[cfg(test)]
pub mod width_independent_annotation_cache_test;
pub mod width_independent_annotation_cache_test_support;
#[cfg(test)]
pub mod zero_width_break_control_layout_test;
pub mod zero_width_break_control_layout_test_support;

pub use annotation_geometry_stage::*;
#[cfg(test)]
pub use annotation_geometry_stage_coverage_test::*;
pub use annotation_geometry_stage_coverage_test_support::*;
#[cfg(test)]
pub use ascii_point_mark_kinsoku_test::*;
pub use ascii_point_mark_kinsoku_test_support::*;
#[cfg(test)]
pub use attached_inline_virtual_adjacency_test::*;
pub use attached_inline_virtual_adjacency_test_support::*;
#[cfg(test)]
pub use auto_space_single_gap_test::*;
pub use auto_space_single_gap_test_support::*;
#[cfg(test)]
pub use baseline_alignment_test::*;
#[cfg(test)]
pub use bilingual_emphasis_test::*;
pub use bilingual_emphasis_test_support::*;
#[cfg(test)]
pub use bopomofo_layout_test::*;
pub use bopomofo_layout_test_support::*;
pub use cluster_role_resolution::*;
#[cfg(test)]
pub use cluster_role_resolution_coverage_test::*;
#[cfg(test)]
pub use cluster_role_resolution_surrogate_and_extender_edge_test::*;
#[cfg(test)]
pub use contextual_dash_ellipsis_cluster_coverage_test::*;
#[cfg(test)]
pub use contextual_dash_ellipsis_layout_test::*;
pub use contextual_dash_ellipsis_role_resolver::*;
#[cfg(test)]
pub use contextual_dash_ellipsis_role_resolver_coverage_test::*;
#[cfg(test)]
pub use contextual_dash_ellipsis_role_resolver_test::*;
pub use contextual_punctuation_display_substitution::*;
pub use contextual_quote_role_resolver::*;
#[cfg(test)]
pub use contextual_quote_role_resolver_coverage_test::*;
#[cfg(test)]
pub use contextual_quote_role_resolver_nested_and_surrogate_test::*;
#[cfg(test)]
pub use contextual_role_extension_coverage_test::*;
#[cfg(test)]
pub use decide_hyphen_break_test::*;
pub use decide_hyphen_break_test_support::*;
pub use default_hyphenator::*;
#[cfg(test)]
pub use display_glyph_substitution_engine_test::*;
pub use display_glyph_substitution_engine_test_support::*;
#[cfg(test)]
pub use emergency_grapheme_tracking_test::*;
pub use emergency_grapheme_tracking_test_support::*;
#[cfg(test)]
pub use explainable_stub_paragraph_layout_engine_test::*;
pub use explainable_stub_paragraph_layout_engine_test_support::*;
#[cfg(test)]
pub use font_instance_metrics_request_test::*;
pub use font_instance_metrics_request_test_support::*;
#[cfg(test)]
pub use greedy_line_breaker_test::*;
#[cfg(test)]
pub use hyphenation_layout_test::*;
pub use hyphenation_layout_test_support::*;
#[cfg(test)]
pub use inline_box_layout_test::*;
pub use inline_box_layout_test_support::*;
#[cfg(test)]
pub use inline_object_layout_test::*;
pub use inline_object_layout_test_support::*;
#[cfg(test)]
pub use interpunct_shrink_opportunity_test::*;
pub use interpunct_shrink_opportunity_test_support::*;
pub use justifier::*;
#[cfg(test)]
pub use justifier_compression_test::*;
#[cfg(test)]
pub use justifier_coverage_test::*;
#[cfg(test)]
pub use justifier_engine_test::*;
pub use justifier_engine_test_support::*;
#[cfg(test)]
pub use justifier_jf_test::*;
#[cfg(test)]
pub use justifier_test::*;
#[cfg(test)]
pub use kinsoku_and_cohesion_repair_engine_test::*;
pub use kinsoku_rule::*;
pub use layout_debug_assembly::*;
pub use layout_dump_format::*;
#[cfg(test)]
pub use layout_dump_golden_parity_test::*;
pub use layout_dump_goldens::*;
#[cfg(test)]
pub use line_adjustment_push_in_test::*;
pub use line_adjustment_push_in_test_support::*;
pub use line_adjustment_stage::*;
#[cfg(test)]
pub use line_adjustment_stage_coverage_test::*;
pub use line_adjustment_stage_coverage_test_support::*;
#[cfg(test)]
pub use line_adjustment_stage_jf_test::*;
pub use line_adjustment_stage_jf_test_support::*;
pub use line_break_planning_stage::*;
#[cfg(test)]
pub use line_break_planning_stage_coverage2_test::*;
pub use line_break_planning_stage_coverage2_test_support::*;
#[cfg(test)]
pub use line_break_planning_stage_coverage_test::*;
pub use line_break_planning_stage_coverage_test_support::*;
#[cfg(test)]
pub use line_break_repair_engine_test::*;
pub use line_break_repair_engine_test_support::*;
pub use line_breaker::*;
#[cfg(test)]
pub use line_breaker_coverage2_test::*;
#[cfg(test)]
pub use line_candidate_validation_test::*;
pub use line_candidate_validation_test_support::*;
#[cfg(test)]
pub use line_geometry_direct_tail_test::*;
pub use line_geometry_stage::*;
pub use line_optimization::*;
#[cfg(test)]
pub use line_optimization_coverage_test::*;
pub use line_repair::*;
#[cfg(test)]
pub use line_repair_coverage_test::*;
#[cfg(test)]
pub use line_repair_tail_coverage_test::*;
#[cfg(test)]
pub use lookahead_line_breaker_test::*;
#[cfg(test)]
pub use opening_bracket_line_start_test::*;
pub use paragraph_dp_line_breaker::*;
#[cfg(test)]
pub use paragraph_dp_line_breaker_coverage2_test::*;
pub use paragraph_dp_line_breaker_coverage2_test_support::*;
#[cfg(test)]
pub use paragraph_dp_line_breaker_coverage_test::*;
pub use paragraph_dp_line_breaker_coverage_test_support::*;
#[cfg(test)]
pub use paragraph_dp_line_breaker_test::*;
pub use paragraph_dp_line_breaker_test_support::*;
#[cfg(test)]
pub use paragraph_dp_tier_promotion_pool_test::*;
pub use paragraph_dp_tier_promotion_pool_test_support::*;
pub use paragraph_layout_engine::*;
pub use paragraph_layout_engine_validation_coverage_support::*;
#[cfg(test)]
pub use paragraph_layout_engine_validation_coverage_test::*;
pub use paragraph_shaping_stage::*;
#[cfg(test)]
pub use paragraph_shaping_stage_coverage_test::*;
pub use paragraph_shaping_stage_coverage_test_support::*;
pub use prepared_paragraph::*;
#[cfg(test)]
pub use prepared_paragraph_inline_edges_test::*;
#[cfg(test)]
pub use prepared_paragraph_jf_test::*;
pub use prepared_paragraph_jf_test_support::*;
#[cfg(test)]
pub use prepared_paragraph_json_number_test::*;
pub use prepared_paragraph_json_number_test_support::*;
#[cfg(test)]
pub use prepared_paragraph_plan_construction_test::*;
pub use prepared_paragraph_plan_construction_test_support::*;
#[cfg(test)]
pub use prepared_paragraph_render_evidence_test::*;
pub use prepared_paragraph_render_evidence_test_support::*;
pub use progressive_break_decisions::*;
#[cfg(test)]
pub use progressive_break_decisions_coverage_test::*;
pub use progressive_break_decisions_coverage_test_support::*;
#[cfg(test)]
pub use progressive_break_decisions_tail_test::*;
pub use progressive_break_decisions_tail_test_support::*;
pub use progressive_break_tier_priority::*;
#[cfg(test)]
pub use progressive_technical_break_test::*;
pub use progressive_technical_break_test_support::*;
#[cfg(test)]
pub use punctuation_atom_builder_halt_test::*;
#[cfg(test)]
pub use punctuation_body_floor_invariant_test::*;
#[cfg(test)]
pub use punctuation_geometry_branch_arms_coverage_test::*;
#[cfg(test)]
pub use punctuation_geometry_engine_test::*;
pub use punctuation_geometry_engine_test_support::*;
pub use punctuation_geometry_ledger::*;
#[cfg(test)]
pub use punctuation_geometry_ledger_coverage_test::*;
pub use punctuation_geometry_stage::*;
#[cfg(test)]
pub use punctuation_geometry_stage_coverage_test::*;
pub use punctuation_model::*;
#[cfg(test)]
pub use punctuation_model_coverage_test::*;
#[cfg(test)]
pub use punctuation_spacing_rule_test::*;
pub use punctuation_spacing_rule_test_support::*;
#[cfg(test)]
pub use push_in_line_wide_capacity_test::*;
pub use push_in_line_wide_capacity_test_support::*;
#[cfg(test)]
pub use quote_classification_engine_test::*;
pub use quote_classification_engine_test_support::*;
pub use quote_pair_analyzer::*;
#[cfg(test)]
pub use quote_pair_analyzer_coverage_test::*;
pub use quote_pair_analyzer_coverage_test_support::*;
#[cfg(test)]
pub use quote_pair_analyzer_surrogate_adjacency_test::*;
pub use quote_pair_analyzer_surrogate_adjacency_test_support::*;
#[cfg(test)]
pub use quote_pair_analyzer_test::*;
pub use quote_pair_analyzer_test_support::*;
#[cfg(test)]
pub use r3_geometry_tail_coverage_test::*;
pub use r3_geometry_tail_coverage_test_support::*;
#[cfg(test)]
pub use recorded_evidence_golden_parity_test::*;
pub use recorded_layout_dump_goldens::*;
pub use recorded_shaping_evidence_data::*;
#[cfg(test)]
pub use ruby_layout_test::*;
pub use ruby_layout_test_support::*;
#[cfg(test)]
pub use spacing_and_line_geometry_engine_test::*;
pub use spacing_and_line_geometry_engine_test_support::*;
#[cfg(test)]
pub use unicode_emoji17_rgi_role_audit_test::*;
pub use unicode_emoji17_rgi_role_audit_test_support::*;
pub use unicode_punctuation_boundary_resolver::*;
#[cfg(test)]
pub use unicode_punctuation_boundary_resolver_coverage_test::*;
#[cfg(test)]
pub use unicode_punctuation_boundary_test::*;
pub use unicode_punctuation_boundary_test_support::*;
#[cfg(test)]
pub use verbatim_range_auto_space_test::*;
pub use verbatim_range_auto_space_test_support::*;
pub use width_independent_annotation_cache::*;
#[cfg(test)]
pub use width_independent_annotation_cache_coverage_test::*;
pub use width_independent_annotation_cache_coverage_test_support::*;
#[cfg(test)]
pub use width_independent_annotation_cache_test::*;
pub use width_independent_annotation_cache_test_support::*;
#[cfg(test)]
pub use zero_width_break_control_layout_test::*;
pub use zero_width_break_control_layout_test_support::*;
