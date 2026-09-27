#![cfg(test)]

use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver_test::ContextualDashEllipsisRoleResolverTestSupport;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn western_context_keeps_dash_and_ellipsis_on_latin_face_and_preserves_source_display() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.westernContextKeepsDashAndEllipsisOnLatinFaceAndPreservesSourceDisplay", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.westernContextKeepsDashAndEllipsisOnLatinFaceAndPreservesSourceDisplay", || {
        let s = UString::from("English — next; ellipsis… / slash. A——B; Wait……what?").to_ustring();
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(s.as_ustr(), None, None).unwrap();
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.role.to_ustring() != UString::from("LatinText") || (d.source_text).to_ustring() != (d.display_text).to_ustring() {
                let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[119,101,115,116,101,114,110,32,108,97,121,111,117,116])).unwrap();
            }
        }
    });
}

#[test]
fn cjk_context_keeps_clreq_display_substitution_independent_of_mark_count() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.cjkContextKeepsClreqDisplaySubstitutionIndependentOfMarkCount", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.cjkContextKeepsClreqDisplaySubstitutionIndependentOfMarkCount", || {
        let s = UString::from("中—文，等…真；中文——下句，省略号……。").to_ustring();
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(s.as_ustr(), None, None).unwrap();
        let mut n = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.role.to_ustring() == UString::from("CjkPunctuation") {
                n = u32::wrapping_add(n, 1);
            }
        }
        if n == 0 {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[99,106,107,32,100,101,99,105,115,105,111,110,115,32,109,105,115,115,105,110,103])).unwrap();
        }
    });
}

#[test]
fn parenthetical_pair_shares_one_face_and_substitution() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.parentheticalPairSharesOneFaceAndSubstitution", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.parentheticalPairSharesOneFaceAndSubstitution", || {
        let s = UString::from("他彻夜想Jessica——Jessica是他的前女友——睡不着觉").to_ustring();
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(s.as_ustr(), None, None).unwrap();
        let mut n = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().font_decisions[usize::try_from(i).unwrap_or(0)].clone().role.to_ustring() == UString::from("CjkPunctuation") {
                n = u32::wrapping_add(n, 1);
            }
        }
        if n == 0 {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[112,97,105,114,32,100,101,99,105,115,105,111,110,115,32,109,105,115,115,105,110,103])).unwrap();
        }
    });
}

#[test]
fn standalone_western_ellipsis_cannot_be_rewritten_by_the_substitutor() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.standaloneWesternEllipsisCannotBeRewrittenByTheSubstitutor", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.standaloneWesternEllipsisCannotBeRewrittenByTheSubstitutor", || {
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(UStr::new(&[8230]), Some(UString::from("en-US")), None).unwrap();
        let d = ((r.debug).clone().font_decisions[0usize]).clone();
        if d.role.to_ustring() != UString::from("LatinText") || (d.source_text).to_ustring() != UString::from("…") || (d.display_text).to_ustring() != UString::from("…") {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(UStr::new(&[115,116,97,110,100,97,108,111,110,101,32,101,108,108,105,112,115,105,115])).unwrap();
        }
    });
}
