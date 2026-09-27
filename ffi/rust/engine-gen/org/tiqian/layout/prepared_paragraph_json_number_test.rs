#![cfg(test)]

use crate::org::tiqian::layout::prepared_paragraph_json_number_test_support::PreparedParagraphJsonNumberTestSupport;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::runtime::test as testlib;


#[test]
fn zero_values_serialize_without_sign() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.zeroValuesSerializeWithoutSign", "org.tiqian.layout.PreparedParagraphJsonNumberTest.zeroValuesSerializeWithoutSign", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"zeroValuesSerializeWithoutSign");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0", TestHelpers::test_helpers_f32_literal(0.0f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0", TestHelpers::test_helpers_f32_literal(-0.0f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"NaN", f64::NAN).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"Infinity", f64::INFINITY).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"-Infinity", f64::NEG_INFINITY).unwrap();
    });
}

#[test]
fn integer_forms_pad_to_decimal_exponent() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.integerFormsPadToDecimalExponent", "org.tiqian.layout.PreparedParagraphJsonNumberTest.integerFormsPadToDecimalExponent", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"integerFormsPadToDecimalExponent");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1", TestHelpers::test_helpers_f32_literal(1 as f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"200", TestHelpers::test_helpers_f32_literal(200 as f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"999999986991104", TestHelpers::test_helpers_f32_literal(1.0e15f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"10000000272564224", TestHelpers::test_helpers_f32_literal(1.0e16f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"100000002004087730000", TestHelpers::test_helpers_f32_literal(1.0e20f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"9007199254740992", TestHelpers::test_helpers_f32_literal(9007199254740992.0f64)).unwrap();
    });
}

#[test]
fn fraction_forms_insert_decimal_point() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.fractionFormsInsertDecimalPoint", "org.tiqian.layout.PreparedParagraphJsonNumberTest.fractionFormsInsertDecimalPoint", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"fractionFormsInsertDecimalPoint");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.5", TestHelpers::test_helpers_f32_literal(1.5f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"12.5", TestHelpers::test_helpers_f32_literal(12.5f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1000000.5", TestHelpers::test_helpers_f32_literal(1000000.5f64)).unwrap();
    });
}

#[test]
fn small_fractions_use_leading_zeros() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.smallFractionsUseLeadingZeros", "org.tiqian.layout.PreparedParagraphJsonNumberTest.smallFractionsUseLeadingZeros", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"smallFractionsUseLeadingZeros");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0.10000000149011612", TestHelpers::test_helpers_f32_literal(0.1f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0.44999998807907104", TestHelpers::test_helpers_f32_literal(0.45f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0.05000000074505806", TestHelpers::test_helpers_f32_literal(0.05f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0.009999999776482582", TestHelpers::test_helpers_f32_literal(0.01f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0.00009999999747378752", TestHelpers::test_helpers_f32_literal(0.0001f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"0.0003499999875202775", TestHelpers::test_helpers_f32_literal(0.00035f64)).unwrap();
    });
}

#[test]
fn exponent_forms_carry_explicit_sign() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.exponentFormsCarryExplicitSign", "org.tiqian.layout.PreparedParagraphJsonNumberTest.exponentFormsCarryExplicitSign", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"exponentFormsCarryExplicitSign");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.0000000200408773e+21", TestHelpers::test_helpers_f32_literal(1e21f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"9.999999778196308e+21", TestHelpers::test_helpers_f32_literal(1e22f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.4999999667294463e+22", TestHelpers::test_helpers_f32_literal(1.5e22f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"2.499999944549077e+22", TestHelpers::test_helpers_f32_literal(2.5e22f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.5000000207726418e+24", TestHelpers::test_helpers_f32_literal(1.5e24f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.0000000116860974e-7", TestHelpers::test_helpers_f32_literal(1e-7f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.500000053056283e-7", TestHelpers::test_helpers_f32_literal(1.5e-7f64)).unwrap();
    });
}

#[test]
fn negative_values_keep_only_magnitude_sign() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.negativeValuesKeepOnlyMagnitudeSign", "org.tiqian.layout.PreparedParagraphJsonNumberTest.negativeValuesKeepOnlyMagnitudeSign", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"negativeValuesKeepOnlyMagnitudeSign");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"-1.5", TestHelpers::test_helpers_f32_literal(-1.5f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"-2.499999993688107e-7", TestHelpers::test_helpers_f32_literal(-2.5e-7f64)).unwrap();
    });
}

#[test]
fn exact_ties_round_to_even_digit() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.exactTiesRoundToEvenDigit", "org.tiqian.layout.PreparedParagraphJsonNumberTest.exactTiesRoundToEvenDigit", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"exactTiesRoundToEvenDigit");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"5.960464477539062e-8", TestHelpers::test_helpers_f32_literal(5.960464477539063e-8f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"2.9802322387695312e-8", TestHelpers::test_helpers_f32_literal(2.9802322387695312e-8f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.7432641983032227", TestHelpers::test_helpers_f32_literal(1.7432641983032227f64)).unwrap();
    });
}

#[test]
fn exact_expansion_rounds_platform_digits() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.exactExpansionRoundsPlatformDigits", "org.tiqian.layout.PreparedParagraphJsonNumberTest.exactExpansionRoundsPlatformDigits", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"exactExpansionRoundsPlatformDigits");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1152921504606847000", TestHelpers::test_helpers_f32_literal(1152921504606846976.0f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"5.684341886080801e-14", TestHelpers::test_helpers_f32_literal(5.684341886080802e-14f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"5.316911983139663e+36", TestHelpers::test_helpers_f32_literal(5.316911983139664e+36f64)).unwrap();
    });
}

#[test]
fn boundary_midpoints_accept_only_at_even_mantissa() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.boundaryMidpointsAcceptOnlyAtEvenMantissa", "org.tiqian.layout.PreparedParagraphJsonNumberTest.boundaryMidpointsAcceptOnlyAtEvenMantissa", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"boundaryMidpointsAcceptOnlyAtEvenMantissa");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"33474762504142850", TestHelpers::test_helpers_f32_bits(1525537341)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"103571925162262530", TestHelpers::test_helpers_f32_bits(1538784015)).unwrap();
    });
}

#[test]
fn decimal_aligned_mantissa_skips_zero_chunk() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.decimalAlignedMantissaSkipsZeroChunk", "org.tiqian.layout.PreparedParagraphJsonNumberTest.decimalAlignedMantissaSkipsZeroChunk", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"decimalAlignedMantissaSkipsZeroChunk");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"12500000", TestHelpers::test_helpers_f32_literal(12500000 as f64)).unwrap();
    });
}

#[test]
fn subnormal_expansions_serialize() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.subnormalExpansionsSerialize", "org.tiqian.layout.PreparedParagraphJsonNumberTest.subnormalExpansionsSerialize", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(&"subnormalExpansionsSerialize");
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"1.401298464324817e-45", TestHelpers::test_helpers_f32_bits(1)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(&"4.203895392974451e-45", TestHelpers::test_helpers_f32_bits(3)).unwrap();
    });
}
