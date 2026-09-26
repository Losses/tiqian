package org.tiqian.shaping.android.nativefont

import android.content.Context
import android.os.Build
import androidx.test.core.app.ApplicationProvider
import org.junit.Assume.assumeTrue
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class AndroidSystemStylesTest {
    @Test
    fun discoveredWeightsStayInsideTheSelectedPhysicalFaceRange() {
        assumeTrue(Build.VERSION.SDK_INT >= 31)
        val context = ApplicationProvider.getApplicationContext<Context>()
        val before = nativeFontResourceStats()
        val catalog = assertNotNull(AndroidPlatformFontOracle.styleCatalogOrNull())
        assertTrue(catalog.faceSpecs.isNotEmpty())
        assertEquals(before, nativeFontResourceStats(), "Discovery must release temporary native handles")
        for (spec in catalog.faceSpecs) {
            val source = when (val prepared = spec.source.prepare(context)) {
                is PreparedAndroidFontSource.FileMapping -> NativeFontBridge.nativeRegisterFileSource(prepared.path)
                is PreparedAndroidFontSource.DirectBuffer -> NativeFontBridge.nativeRegisterBufferSource(prepared.buffer, prepared.sizeBytes)
                is PreparedAndroidFontSource.DescriptorRegion -> error("System fonts must be files or buffers")
            }
            check(source != 0L)
            try {
                val face = NativeFontBridge.nativeCreateFace(source, spec.collectionIndex, intArrayOf(), floatArrayOf())
                check(face != 0L)
                try {
                    val range = NativeFontFace(face, NativeFontBridge.nativeUnitsPerEm(face)).axisRange("wght")
                    if (range != null) {
                        assertTrue(spec.weight.toFloat() in range, "${spec.source.label}: ${spec.weight} outside $range")
                        assertEquals(spec.weight.toFloat(), spec.variationAxes["wght"])
                    } else {
                        assertTrue("wght" !in spec.variationAxes)
                    }
                } finally {
                    NativeFontBridge.nativeReleaseFace(face)
                }
            } finally {
                NativeFontBridge.nativeReleaseSource(source)
            }
        }
        assertEquals(before, nativeFontResourceStats())
    }
}
