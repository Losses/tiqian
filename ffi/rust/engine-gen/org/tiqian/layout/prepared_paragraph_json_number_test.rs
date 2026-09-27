#![cfg(test)]

use crate::org::tiqian::layout::prepared_paragraph_json_number_test_support::PreparedParagraphJsonNumberTestSupport;
use crate::org::tiqian::test::test_helpers::TestHelpers;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;


#[test]
fn zero_values_serialize_without_sign() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.zeroValuesSerializeWithoutSign", "org.tiqian.layout.PreparedParagraphJsonNumberTest.zeroValuesSerializeWithoutSign", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[122,101,114,111,86,97,108,117,101,115,83,101,114,105,97,108,105,122,101,87,105,116,104,111,117,116,83,105,103,110]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48]), TestHelpers::test_helpers_f32_literal(0.0f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48]), TestHelpers::test_helpers_f32_literal(-0.0f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[78,97,78]), f64::NAN).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[73,110,102,105,110,105,116,121]), f64::INFINITY).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[45,73,110,102,105,110,105,116,121]), f64::NEG_INFINITY).unwrap();
    });
}

#[test]
fn integer_forms_pad_to_decimal_exponent() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.integerFormsPadToDecimalExponent", "org.tiqian.layout.PreparedParagraphJsonNumberTest.integerFormsPadToDecimalExponent", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[105,110,116,101,103,101,114,70,111,114,109,115,80,97,100,84,111,68,101,99,105,109,97,108,69,120,112,111,110,101,110,116]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49]), TestHelpers::test_helpers_f32_literal(1 as f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[50,48,48]), TestHelpers::test_helpers_f32_literal(200 as f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[57,57,57,57,57,57,57,56,54,57,57,49,49,48,52]), TestHelpers::test_helpers_f32_literal(1.0e15f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,48,48,48,48,48,48,48,50,55,50,53,54,52,50,50,52]), TestHelpers::test_helpers_f32_literal(1.0e16f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,48,48,48,48,48,48,48,50,48,48,52,48,56,55,55,51,48,48,48,48]), TestHelpers::test_helpers_f32_literal(1.0e20f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[57,48,48,55,49,57,57,50,53,52,55,52,48,57,57,50]), TestHelpers::test_helpers_f32_literal(9007199254740992.0f64)).unwrap();
    });
}

#[test]
fn fraction_forms_insert_decimal_point() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.fractionFormsInsertDecimalPoint", "org.tiqian.layout.PreparedParagraphJsonNumberTest.fractionFormsInsertDecimalPoint", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[102,114,97,99,116,105,111,110,70,111,114,109,115,73,110,115,101,114,116,68,101,99,105,109,97,108,80,111,105,110,116]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,53]), TestHelpers::test_helpers_f32_literal(1.5f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,50,46,53]), TestHelpers::test_helpers_f32_literal(12.5f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,48,48,48,48,48,48,46,53]), TestHelpers::test_helpers_f32_literal(1000000.5f64)).unwrap();
    });
}

#[test]
fn small_fractions_use_leading_zeros() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.smallFractionsUseLeadingZeros", "org.tiqian.layout.PreparedParagraphJsonNumberTest.smallFractionsUseLeadingZeros", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[115,109,97,108,108,70,114,97,99,116,105,111,110,115,85,115,101,76,101,97,100,105,110,103,90,101,114,111,115]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48,46,49,48,48,48,48,48,48,48,49,52,57,48,49,49,54,49,50]), TestHelpers::test_helpers_f32_literal(0.1f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48,46,52,52,57,57,57,57,57,56,56,48,55,57,48,55,49,48,52]), TestHelpers::test_helpers_f32_literal(0.45f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48,46,48,53,48,48,48,48,48,48,48,55,52,53,48,53,56,48,54]), TestHelpers::test_helpers_f32_literal(0.05f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48,46,48,48,57,57,57,57,57,57,57,55,55,54,52,56,50,53,56,50]), TestHelpers::test_helpers_f32_literal(0.01f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48,46,48,48,48,48,57,57,57,57,57,57,57,55,52,55,51,55,56,55,53,50]), TestHelpers::test_helpers_f32_literal(0.0001f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[48,46,48,48,48,51,52,57,57,57,57,57,56,55,53,50,48,50,55,55,53]), TestHelpers::test_helpers_f32_literal(0.00035f64)).unwrap();
    });
}

#[test]
fn exponent_forms_carry_explicit_sign() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.exponentFormsCarryExplicitSign", "org.tiqian.layout.PreparedParagraphJsonNumberTest.exponentFormsCarryExplicitSign", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[101,120,112,111,110,101,110,116,70,111,114,109,115,67,97,114,114,121,69,120,112,108,105,99,105,116,83,105,103,110]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,48,48,48,48,48,48,48,50,48,48,52,48,56,55,55,51,101,43,50,49]), TestHelpers::test_helpers_f32_literal(1e21f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[57,46,57,57,57,57,57,57,55,55,56,49,57,54,51,48,56,101,43,50,49]), TestHelpers::test_helpers_f32_literal(1e22f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,52,57,57,57,57,57,57,54,54,55,50,57,52,52,54,51,101,43,50,50]), TestHelpers::test_helpers_f32_literal(1.5e22f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[50,46,52,57,57,57,57,57,57,52,52,53,52,57,48,55,55,101,43,50,50]), TestHelpers::test_helpers_f32_literal(2.5e22f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,53,48,48,48,48,48,48,50,48,55,55,50,54,52,49,56,101,43,50,52]), TestHelpers::test_helpers_f32_literal(1.5e24f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,48,48,48,48,48,48,48,49,49,54,56,54,48,57,55,52,101,45,55]), TestHelpers::test_helpers_f32_literal(1e-7f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,53,48,48,48,48,48,48,53,51,48,53,54,50,56,51,101,45,55]), TestHelpers::test_helpers_f32_literal(1.5e-7f64)).unwrap();
    });
}

#[test]
fn negative_values_keep_only_magnitude_sign() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.negativeValuesKeepOnlyMagnitudeSign", "org.tiqian.layout.PreparedParagraphJsonNumberTest.negativeValuesKeepOnlyMagnitudeSign", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[110,101,103,97,116,105,118,101,86,97,108,117,101,115,75,101,101,112,79,110,108,121,77,97,103,110,105,116,117,100,101,83,105,103,110]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[45,49,46,53]), TestHelpers::test_helpers_f32_literal(-1.5f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[45,50,46,52,57,57,57,57,57,57,57,51,54,56,56,49,48,55,101,45,55]), TestHelpers::test_helpers_f32_literal(-2.5e-7f64)).unwrap();
    });
}

#[test]
fn exact_ties_round_to_even_digit() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.exactTiesRoundToEvenDigit", "org.tiqian.layout.PreparedParagraphJsonNumberTest.exactTiesRoundToEvenDigit", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[101,120,97,99,116,84,105,101,115,82,111,117,110,100,84,111,69,118,101,110,68,105,103,105,116]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[53,46,57,54,48,52,54,52,52,55,55,53,51,57,48,54,50,101,45,56]), TestHelpers::test_helpers_f32_literal(5.960464477539063e-8f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[50,46,57,56,48,50,51,50,50,51,56,55,54,57,53,51,49,50,101,45,56]), TestHelpers::test_helpers_f32_literal(2.9802322387695312e-8f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,55,52,51,50,54,52,49,57,56,51,48,51,50,50,50,55]), TestHelpers::test_helpers_f32_literal(1.7432641983032227f64)).unwrap();
    });
}

#[test]
fn exact_expansion_rounds_platform_digits() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.exactExpansionRoundsPlatformDigits", "org.tiqian.layout.PreparedParagraphJsonNumberTest.exactExpansionRoundsPlatformDigits", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[101,120,97,99,116,69,120,112,97,110,115,105,111,110,82,111,117,110,100,115,80,108,97,116,102,111,114,109,68,105,103,105,116,115]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,49,53,50,57,50,49,53,48,52,54,48,54,56,52,55,48,48,48]), TestHelpers::test_helpers_f32_literal(1152921504606846976.0f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[53,46,54,56,52,51,52,49,56,56,54,48,56,48,56,48,49,101,45,49,52]), TestHelpers::test_helpers_f32_literal(5.684341886080802e-14f64)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[53,46,51,49,54,57,49,49,57,56,51,49,51,57,54,54,51,101,43,51,54]), TestHelpers::test_helpers_f32_literal(5.316911983139664e+36f64)).unwrap();
    });
}

#[test]
fn boundary_midpoints_accept_only_at_even_mantissa() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.boundaryMidpointsAcceptOnlyAtEvenMantissa", "org.tiqian.layout.PreparedParagraphJsonNumberTest.boundaryMidpointsAcceptOnlyAtEvenMantissa", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[98,111,117,110,100,97,114,121,77,105,100,112,111,105,110,116,115,65,99,99,101,112,116,79,110,108,121,65,116,69,118,101,110,77,97,110,116,105,115,115,97]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[51,51,52,55,52,55,54,50,53,48,52,49,52,50,56,53,48]), TestHelpers::test_helpers_f32_bits(1525537341)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,48,51,53,55,49,57,50,53,49,54,50,50,54,50,53,51,48]), TestHelpers::test_helpers_f32_bits(1538784015)).unwrap();
    });
}

#[test]
fn decimal_aligned_mantissa_skips_zero_chunk() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.decimalAlignedMantissaSkipsZeroChunk", "org.tiqian.layout.PreparedParagraphJsonNumberTest.decimalAlignedMantissaSkipsZeroChunk", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[100,101,99,105,109,97,108,65,108,105,103,110,101,100,77,97,110,116,105,115,115,97,83,107,105,112,115,90,101,114,111,67,104,117,110,107]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,50,53,48,48,48,48,48]), TestHelpers::test_helpers_f32_literal(12500000 as f64)).unwrap();
    });
}

#[test]
fn subnormal_expansions_serialize() {
    testlib::run("org.tiqian.layout.PreparedParagraphJsonNumberTest.subnormalExpansionsSerialize", "org.tiqian.layout.PreparedParagraphJsonNumberTest.subnormalExpansionsSerialize", || {
        PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_rec(UStr::new(&[115,117,98,110,111,114,109,97,108,69,120,112,97,110,115,105,111,110,115,83,101,114,105,97,108,105,122,101]));
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[49,46,52,48,49,50,57,56,52,54,52,51,50,52,56,49,55,101,45,52,53]), TestHelpers::test_helpers_f32_bits(1)).unwrap();
        let _ = PreparedParagraphJsonNumberTestSupport::prepared_paragraph_json_number_test_support_eq(UStr::new(&[52,46,50,48,51,56,57,53,51,57,50,57,55,52,52,53,49,101,45,52,53]), TestHelpers::test_helpers_f32_bits(3)).unwrap();
    });
}
