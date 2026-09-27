#![cfg(test)]

use crate::org::tiqian::layout::contextual_dash_ellipsis_role_resolver_test::ContextualDashEllipsisRoleResolverTestSupport;
use crate::runtime::test as testlib;


#[test]
fn western_context_keeps_dash_and_ellipsis_on_latin_face_and_preserves_source_display() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.westernContextKeepsDashAndEllipsisOnLatinFaceAndPreservesSourceDisplay", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.westernContextKeepsDashAndEllipsisOnLatinFaceAndPreservesSourceDisplay", || {
        let s = "English — next; ellipsis… / slash. A——B; Wait……what?".to_string();
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(s.as_str(), None, None).unwrap();
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.role.to_string() != "LatinText" || (d.source_text).to_string() != (d.display_text).to_string() {
                let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"western layout").unwrap();
            }
        }
    });
}

#[test]
fn cjk_context_keeps_clreq_display_substitution_independent_of_mark_count() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.cjkContextKeepsClreqDisplaySubstitutionIndependentOfMarkCount", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.cjkContextKeepsClreqDisplaySubstitutionIndependentOfMarkCount", || {
        let s = "中—文，等…真；中文——下句，省略号……。".to_string();
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(s.as_str(), None, None).unwrap();
        let mut n = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((r.debug).clone().font_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.role.to_string() == "CjkPunctuation" {
                n = u32::wrapping_add(n, 1);
            }
        }
        if n == 0 {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"cjk decisions missing").unwrap();
        }
    });
}

#[test]
fn parenthetical_pair_shares_one_face_and_substitution() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.parentheticalPairSharesOneFaceAndSubstitution", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.parentheticalPairSharesOneFaceAndSubstitution", || {
        let s = "他彻夜想Jessica——Jessica是他的前女友——睡不着觉".to_string();
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(s.as_str(), None, None).unwrap();
        let mut n = 0u32;
        for i in 0..match u32::try_from((r.debug).clone().font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if r.debug.clone().font_decisions[usize::try_from(i).unwrap_or(0)].clone().role.to_string() == "CjkPunctuation" {
                n = u32::wrapping_add(n, 1);
            }
        }
        if n == 0 {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"pair decisions missing").unwrap();
        }
    });
}

#[test]
fn standalone_western_ellipsis_cannot_be_rewritten_by_the_substitutor() {
    testlib::run("org.tiqian.layout.ContextualDashEllipsisLayoutTest.standaloneWesternEllipsisCannotBeRewrittenByTheSubstitutor", "org.tiqian.layout.ContextualDashEllipsisLayoutTest.standaloneWesternEllipsisCannotBeRewrittenByTheSubstitutor", || {
        let r = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_layout(&"…", Some("en-US".to_string()), None).unwrap();
        let d = ((r.debug).clone().font_decisions[0usize]).clone();
        if d.role.to_string() != "LatinText" || (d.source_text).to_string() != "…" || (d.display_text).to_string() != "…" {
            let _ = ContextualDashEllipsisRoleResolverTestSupport::contextual_dash_ellipsis_role_resolver_test_support_fail(&"standalone ellipsis").unwrap();
        }
    });
}
