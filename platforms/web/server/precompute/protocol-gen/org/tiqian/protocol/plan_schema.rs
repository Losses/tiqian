#[derive(Clone, Copy)]
pub struct PlanSchema;

impl PlanSchema {
    pub const PLAN_SCHEMA_PLAN_SCHEMA: u32 = 1;
    pub const PLAN_SCHEMA_PLAN_LAYOUT_REVISION: &str = "tiqian-layout-v2";
}
