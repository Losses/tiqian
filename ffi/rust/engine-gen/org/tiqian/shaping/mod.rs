#![allow(ambiguous_glob_reexports)]

#[cfg(test)]
pub mod explainable_stub_text_shaper_test;
pub mod explainable_stub_text_shaper_test_support;
pub mod replayable_font_backend;
#[cfg(test)]
pub mod replayable_font_backend_coverage_test;
pub mod replayable_font_backend_coverage_test_support;
pub mod text_shaper;
#[cfg(test)]
pub mod text_shaper_coverage_test;
pub mod text_shaper_coverage_test_support;

#[cfg(test)]
pub use explainable_stub_text_shaper_test::*;
pub use explainable_stub_text_shaper_test_support::*;
pub use replayable_font_backend::*;
#[cfg(test)]
pub use replayable_font_backend_coverage_test::*;
pub use replayable_font_backend_coverage_test_support::*;
pub use text_shaper::*;
#[cfg(test)]
pub use text_shaper_coverage_test::*;
pub use text_shaper_coverage_test_support::*;
