package std

@Deprecated("Legacy engine, migrate to engine-haxe")
sealed class UStringException(override val message: String) : RuntimeException(message) {
    data class InvalidCodePoint(val code: Int) :
        UStringException("invalid code point: " + code)
    data class UnpairedSurrogate(val unit: Int) :
        UStringException("unpaired surrogate: " + unit)
}
