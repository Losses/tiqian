use crate::org::tiqian::clreq::clreq_region::ClreqRegion;
use crate::org::tiqian::clreq::glue_side::GlueSide;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;


#[derive(Clone, Copy)]
pub struct PunctuationGluePlacements;

impl PunctuationGluePlacements {
    pub fn punctuation_glue_placements_for_region(region: ClreqRegion) -> PunctuationGluePlacement {
        return match region {
            ClreqRegion::Mainland => PunctuationGluePlacement::MainlandSimplified,
            ClreqRegion::Taiwan => PunctuationGluePlacement::Traditional,
            ClreqRegion::HongKong => PunctuationGluePlacement::Traditional,
            ClreqRegion::Custom => PunctuationGluePlacement::MainlandSimplified,
        };
    }

    pub fn punctuation_glue_placements_glue_side_for(placement: PunctuationGluePlacement, punctuation_class: PunctuationClass) -> GlueSide {
        return match placement {
            PunctuationGluePlacement::MainlandSimplified => PunctuationGluePlacements::punctuation_glue_placements_mainland_glue_side(punctuation_class),
            PunctuationGluePlacement::Traditional => GlueSide::BothSides,
        };
    }

    pub(crate) fn punctuation_glue_placements_mainland_glue_side(punctuation_class: PunctuationClass) -> GlueSide {
        return match punctuation_class {
            PunctuationClass::Opening => GlueSide::LeadingOnly,
            PunctuationClass::Closing => GlueSide::TrailingOnly,
            PunctuationClass::PauseOrStop => GlueSide::TrailingOnly,
            PunctuationClass::MiddleDot => GlueSide::BothSides,
            PunctuationClass::Interpunct => GlueSide::BothSides,
            PunctuationClass::Connector => GlueSide::BothSides,
            PunctuationClass::Solidus => GlueSide::BothSides,
            PunctuationClass::Ellipsis => GlueSide::BothSides,
            PunctuationClass::Dash => GlueSide::BothSides,
            PunctuationClass::Other => GlueSide::BothSides,
        };
    }
}
