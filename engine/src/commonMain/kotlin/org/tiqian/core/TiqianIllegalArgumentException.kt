package org.tiqian.core

@Deprecated("Legacy engine, migrate to engine-haxe")
sealed class TiqianIllegalArgumentException(override val message: String) : RuntimeException(message) {
    data object StartGreaterThanEnd : TiqianIllegalArgumentException("TextRange start must not be greater than end.")
    data object NegativeStart : TiqianIllegalArgumentException("TextRange start must be non-negative.")
    data class Message(val text: String) :
        TiqianIllegalArgumentException(text)
}
