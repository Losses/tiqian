#![cfg(test)]

use crate::org::tiqian::layout::quote_pair_analyzer::QuotePair;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::quote_pair_analyzer::QuoteType;
use crate::org::tiqian::layout::quote_pair_analyzer_surrogate_adjacency_test_support::QuotePairAnalyzerSurrogateAdjacencyTestSupport;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertTrueFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestPlainAndBoundaryNeighboursWalkTheNonSurrogateArmsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertEqualsQuotePairArrayFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuotePairArrayFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TracedAssertionsAssertEqualsQuotePairArrayFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuotePairArrayFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TracedAssertionsAssertEqualsQuotePairArrayFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuotePairArrayFault> for QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsQuotePairArrayFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TracedAssertionsAssertEqualsQuotePairArrayFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestLowQuoteCodePointsTakeTheSwitchDefaultWithoutPairingFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheBeforeASurrogateWalksBothLowCheckArmsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsAssertFalseFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsAssertFalseFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsAssertFalseFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault) -> Self {
        match value {
            QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFalseFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsAssertFalseFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        QuotePairAnalyzerSurrogateAdjacencyTestApostropheAfterASurrogatePairWalksTheCombineArmBeforeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn low_quote_code_points_take_the_switch_default_without_pairing() {
    testlib::run("org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing", "org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.lowQuoteCodePointsTakeTheSwitchDefaultWithoutPairing", || {
        QuotePairAnalyzerSurrogateAdjacencyTestSupport::quote_pair_analyzer_surrogate_adjacency_test_support_rec(UStr::new(&[108,111,119,81,117,111,116,101,67,111,100,101,80,111,105,110,116,115,84,97,107,101,84,104,101,83,119,105,116,99,104,68,101,102,97,117,108,116,87,105,116,104,111,117,116,80,97,105,114,105,110,103]));
        let _ = TracedAssertions::traced_assertions_assert_equals_quote_pair_array(&vec![(QuotePair::new(2u32, 3u32, QuoteType::Double)).clone()], &QuotePairAnalyzer::new().analyze(UStr::new(&[8218,8219,8220,8221])).unwrap(), None).unwrap();
    });
}

#[test]
fn apostrophe_after_a_surrogate_pair_walks_the_combine_arm_before() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.apostropheAfterASurrogatePairWalksTheCombineArmBefore", "org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.apostropheAfterASurrogatePairWalksTheCombineArmBefore");
}

#[test]
fn apostrophe_before_a_surrogate_walks_both_low_check_arms() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.apostropheBeforeASurrogateWalksBothLowCheckArms", "org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.apostropheBeforeASurrogateWalksBothLowCheckArms");
}

#[test]
fn plain_and_boundary_neighbours_walk_the_non_surrogate_arms() {
    testlib::record_not_applicable("org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.plainAndBoundaryNeighboursWalkTheNonSurrogateArms", "org.tiqian.layout.QuotePairAnalyzerSurrogateAdjacencyTest.plainAndBoundaryNeighboursWalkTheNonSurrogateArms");
}
