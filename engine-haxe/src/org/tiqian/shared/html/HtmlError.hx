package org.tiqian.shared.html;

/**
 * The issue variants of the shared HTML scan and injection lane. The two
 * names are the published issue names the Rust addon lane produces
 * (platforms/web/server/precompute/engine/src/precompute_html.rs:
 * to_offset at L210-212 emits HtmlOffsetConversion, the injection gate at
 * L219-258 emits InvalidHtmlAttributeInsertionOffset); the generated lane
 * reuses the same names so the two error surfaces stay byte-identical.
 * Declaration order is the check order of the Rust injection gate.
 */
enum HtmlError {
    HtmlOffsetConversion;
    InvalidHtmlAttributeInsertionOffset;
}
