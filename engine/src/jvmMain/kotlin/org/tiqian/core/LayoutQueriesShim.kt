package org.tiqian.core

/**
 * Transitional mount shim (Stage1-P3 Kotlin producer cutover).
 *
 * The generated [org.tiqian.layout.PlanLowering] calls the Haxe
 * LayoutQueries.positionedClustersForLine static; the hand-written engine
 * exposes the same computation as the top-level extension
 * [LayoutResult.positionedClusters] (LayoutQueries.kt:172). This object only
 * forwards to it so the mounted generated lowering compiles.
 *
 * Retire this file together with the hand-written LayoutQueries.kt when
 * org.tiqian.core.LayoutQueries is mounted from the kotlin bundle.
 */
object LayoutQueries {
    fun positionedClustersForLine(result: LayoutResult, line: LineBox): List<PositionedCluster> =
        result.positionedClusters(line)
}
