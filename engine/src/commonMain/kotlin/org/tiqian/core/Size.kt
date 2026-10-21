package org.tiqian.core

@Deprecated("Legacy engine, migrate to engine-haxe")
data class Size(val width: Float, val height: Float) {

    override fun toString(): String {
        return "Size(width=" + this.width + ", height=" + this.height + ")"
    }
}
