package org.tiqian.core;

@:dataClass
class RichTextBackgroundPaint {
    public final horizontalPadding:Float;
    public final verticalPadding:Float;
    public final cornerRadius:Float;
    public final continuationCornerRadius:Float;
    public final metricPolicy:RichTextBackgroundMetricPolicy;
    public final drawStyle:RichTextBackgroundDrawStyle;

    public function new(
        ?horizontalPadding:Null<Float>,
        ?verticalPadding:Null<Float>,
        ?cornerRadius:Null<Float>,
        // Kotlin declares continuationCornerRadius = cornerRadius, a
        // parameter-reading default (boring gap 4), and drawStyle =
        // RichTextBackgroundDrawStyle.Fill (a static field, also gap 4).
        // Both parameters stay mandatory until that lowering lands.
        continuationCornerRadius:Float,
        ?metricPolicy:Null<RichTextBackgroundMetricPolicy>,
        drawStyle:RichTextBackgroundDrawStyle
    ) {
        this.horizontalPadding = horizontalPadding == null ? 0.0 : horizontalPadding;
        this.verticalPadding = verticalPadding == null ? 0.0 : verticalPadding;
        this.cornerRadius = cornerRadius == null ? 0.0 : cornerRadius;
        this.continuationCornerRadius = continuationCornerRadius;
        this.metricPolicy = metricPolicy == null ? RichTextBackgroundMetricPolicy.MarkedFaces : metricPolicy;
        this.drawStyle = drawStyle;
        if (!isFinite(this.horizontalPadding) || this.horizontalPadding < 0.0
            || !isFinite(this.verticalPadding) || this.verticalPadding < 0.0
            || !isFinite(this.cornerRadius) || this.cornerRadius < 0.0
            || !isFinite(this.continuationCornerRadius) || this.continuationCornerRadius < 0.0) {
            throw new TiqianIllegalArgumentException(Message("Failed requirement."));
        }
    }

    public static function withHorizontalPadding(value:Float):RichTextBackgroundPaint {
        return new RichTextBackgroundPaint(value, 0.0, 0.0, 0.0, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill);
    }

    public static function withMetricPolicy(value:RichTextBackgroundMetricPolicy):RichTextBackgroundPaint {
        return new RichTextBackgroundPaint(0.0, 0.0, 0.0, 0.0, value, RichTextBackgroundDrawStyle.Fill);
    }

    public static function withCornerRadius(corner:Float, continuation:Null<Float>):RichTextBackgroundPaint {
        return new RichTextBackgroundPaint(0.0, 0.0, corner, continuation, RichTextBackgroundMetricPolicy.MarkedFaces, RichTextBackgroundDrawStyle.Fill);
    }



    @:allow(org.tiqian.core.RichTextPaint)
    private static function sameValues(a:RichTextBackgroundPaint, b:RichTextBackgroundPaint):Bool {
        return a.horizontalPadding == b.horizontalPadding
            && a.verticalPadding == b.verticalPadding
            && a.cornerRadius == b.cornerRadius
            && a.continuationCornerRadius == b.continuationCornerRadius
            && a.metricPolicy == b.metricPolicy
            && RichTextBackgroundDrawStyle.sameValues(a.drawStyle, b.drawStyle);
    }

    private static function isFinite(value:Float):Bool {
        return value == value && value != Math.POSITIVE_INFINITY && value != Math.NEGATIVE_INFINITY;
    }
}
