use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;


#[derive(Clone, Copy)]
pub struct ProgressiveBreakTierPriority;

impl ProgressiveBreakTierPriority {
    pub fn progressive_break_tier_priority_priority(tier: ProgressiveBreakTier) -> u32 {
        return match tier {
            ProgressiveBreakTier::Whitespace => 0,
            ProgressiveBreakTier::Structural => 1,
            ProgressiveBreakTier::Syllable => 2,
            ProgressiveBreakTier::WholeToken => 3,
            ProgressiveBreakTier::Emergency => 4,
        };
    }

    pub fn progressive_break_tier_priority_from_priority(value: u32) -> ProgressiveBreakTier {
        if value == 0 {
            return ProgressiveBreakTier::Whitespace;
        }
        if value == 1 {
            return ProgressiveBreakTier::Structural;
        }
        if value == 2 {
            return ProgressiveBreakTier::Syllable;
        }
        if value == 3 {
            return ProgressiveBreakTier::WholeToken;
        }
        return ProgressiveBreakTier::Emergency;
    }
}
