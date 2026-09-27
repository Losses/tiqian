#![cfg(test)]

use crate::org::tiqian::protocol::plan_schema::PlanSchema;
use crate::org::tiqian::protocol::revision::Revision;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault {
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsAssertEqualsStringFaultFault(value) => write!(formatter, "{}", value),
            PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsAssertEqualsFaultFault(value) => write!(formatter, "{}", value),
            PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault) -> Self {
        match value {
            PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault {
    fn from(value: PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault) -> Self {
        match value {
            PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsAssertEqualsFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault) -> Self {
        match value {
            PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault> for PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFault) -> Self {
        PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsAssertEqualsFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PlanSchemaTestPlanSchemaConstantsAlignWithRevisionFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn plan_schema_constants_align_with_revision() {
    testlib::run("org.tiqian.protocol.PlanSchemaTest.planSchemaConstantsAlignWithRevision", "org.tiqian.protocol.PlanSchemaTest.planSchemaConstantsAlignWithRevision", || {
        let _ = TracedAssertions::traced_assertions_assert_equals_string(Revision::REVISION_LAYOUT_REVISION.to_ustring().as_ustr(), PlanSchema::PLAN_SCHEMA_PLAN_LAYOUT_REVISION.to_ustring().as_ustr(), Some(UString::from("PLAN_LAYOUT_REVISION vs Revision.LAYOUT_REVISION"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(PlanSchema::PLAN_SCHEMA_PLAN_SCHEMA, Revision::REVISION_SNAPSHOT_SCHEMA, Some(UString::from("PLAN_SCHEMA vs SNAPSHOT_SCHEMA"))).unwrap();
    });
}
