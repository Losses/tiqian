package org.tiqian.ffi.js

import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals

/**
 * Remaining coverage of the lowering-helper font-role exports whose Kotlin
 * surface this module hosts. The fontMetricsResolve / fontFallbackResolve
 * cases moved to `ffi/js/npm/font-facade.test.mjs` when the generated
 * TypeScript tree took over those exports (TCN-45 records the f64 semantics).
 */
class FontExportsTest {

    @Test
    fun loweringRoleBatchUsesCompleteParagraphContext() {
        val text = "A——B中文……下句"
        val starts = arrayOf(1, 2, 6, 7)
        val ends = arrayOf(2, 3, 7, 8)

        assertContentEquals(
            arrayOf("other", "other", "cjk-punctuation", "cjk-punctuation"),
            classifyFontRoles(text, starts, ends, "zh-Hans"),
        )
        assertEquals("other", classifyFontRole("A——B", 1, 2, "zh-Hans"))
    }

    @Test
    fun loweringRoleUsesStructuralQuotePairResolution() {
        val text = "word“中文”word"

        assertEquals("other", classifyFontRole(text, 4, 5, "zh-Hans"))
        assertEquals("other", classifyFontRole(text, 7, 8, "zh-Hans"))
        assertContentEquals(
            arrayOf("other", "cjk-text", "other"),
            classifyFontRoles(text, arrayOf(4, 5, 7), arrayOf(5, 6, 8), "zh-Hans"),
        )
    }
}
