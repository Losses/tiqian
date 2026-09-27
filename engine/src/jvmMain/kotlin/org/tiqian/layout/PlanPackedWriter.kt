package org.tiqian.layout

import org.tiqian.core.LayoutResult

/**
 * Packed plan writer entry (corrective-2, ADR 0050/0053).
 *
 * The engine is the single writer for the packed plan (tiqian_plan_abi.h).
 * Writing is generated: [PreparedParagraphFns.toPackedPlanBytes] lowers the
 * result through [PlanLowering] and serializes with [org.tiqian.protocol.PlanPacked].
 * The former hand-written column-major packer (nativeMain) is retired; this
 * file only keeps the public entry point. It lives in jvmMain because the
 * generated writer goes through the boring runtime shims, which target the
 * JVM at pin ede5476f.
 */
fun LayoutResult.toPackedPlanBytes(): ByteArray =
    PreparedParagraphFns.toPackedPlanBytes(this)
