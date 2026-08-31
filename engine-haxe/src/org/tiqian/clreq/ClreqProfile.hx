package org.tiqian.clreq;

import std.ReadOnlyArray;
import std.StringBuf;

class ClreqProfile {
    public final id:String;
    public final strictness:ClreqStrictness;
    public final region:ClreqRegion;
    public final punctuationGlyphPolicy:CjkPunctuationGlyphPolicy;
    public final coalesceRepeatablePunctuation:ReadOnlyArray<Int>;
    public final autoSpace:AutoSpacePolicy;
    public final gluePlacement:PunctuationGluePlacement;
    public final adjustment:AdjustmentStylePolicy;
    public final kinsokuMode:KinsokuMode;
    public final punctuationWidth:PunctuationWidthPolicy;

    public function new(
        id:String,
        strictness:ClreqStrictness,
        region:ClreqRegion,
        ?punctuationGlyphPolicy:Null<CjkPunctuationGlyphPolicy>,
        // Kotlin default reads the static field DefaultCoalesceRepeatablePunctuation
        // (boring gap 4); the parameter stays mandatory until that lowering lands.
        coalesceRepeatablePunctuation:Array<Int>,
        // Kotlin default reads the static field AutoSpacePolicy.Default (gap 4).
        autoSpace:AutoSpacePolicy,
        // Kotlin default calls PunctuationGluePlacement.forRegion(region), a call
        // on an earlier parameter (gap 4).
        gluePlacement:PunctuationGluePlacement,
        // Kotlin default constructs AdjustmentStylePolicy(); constructor calls
        // stay outside the sanctioned default grammar permanently.
        adjustment:AdjustmentStylePolicy,
        // Kotlin default constructs KinsokuMode.MeasureAdaptive(); permanent.
        kinsokuMode:KinsokuMode,
        // Kotlin default constructs PunctuationWidthPolicy(); permanent.
        punctuationWidth:PunctuationWidthPolicy
    ) {
        this.id = id;
        this.strictness = strictness;
        this.region = region;
        this.punctuationGlyphPolicy = punctuationGlyphPolicy == null
            ? CjkPunctuationGlyphPolicy.PreferClreqRecommendedCodepoints
            : punctuationGlyphPolicy;
        this.coalesceRepeatablePunctuation = coalesceRepeatablePunctuation;
        this.autoSpace = autoSpace;
        this.gluePlacement = gluePlacement;
        this.adjustment = adjustment;
        this.kinsokuMode = kinsokuMode;
        this.punctuationWidth = punctuationWidth;
    }

    public function toString():String {
        final coalesce = new StringBuf();
        coalesce.add("[");
        var index:Int = 0;
        while (index < coalesceRepeatablePunctuation.length) {
            if (index > 0) {
                coalesce.add(", ");
            }
            coalesce.add("" + coalesceRepeatablePunctuation[index]);
            index += 1;
        }
        coalesce.add("]");
        return "ClreqProfile(id=" + id
            + ", strictness=" + strictness
            + ", region=" + region
            + ", punctuationGlyphPolicy=" + punctuationGlyphPolicy
            + ", coalesceRepeatablePunctuation=" + coalesce.toString()
            + ", autoSpace=" + autoSpace.toString()
            + ", gluePlacement=" + gluePlacement
            + ", adjustment=" + adjustment.toString()
            + ", kinsokuMode=" + KinsokuModes.render(kinsokuMode)
            + ", punctuationWidth=" + punctuationWidth.toString()
            + ")";
    }

    public static function sameProfile(a:ClreqProfile, b:ClreqProfile):Bool {
        if (a.id != b.id
            || a.strictness != b.strictness
            || a.region != b.region
            || a.punctuationGlyphPolicy != b.punctuationGlyphPolicy
            || a.coalesceRepeatablePunctuation.length != b.coalesceRepeatablePunctuation.length
            || !AutoSpacePolicy.samePolicy(a.autoSpace, b.autoSpace)
            || a.gluePlacement != b.gluePlacement
            || !AdjustmentStylePolicy.samePolicy(a.adjustment, b.adjustment)
            || !KinsokuModes.sameMode(a.kinsokuMode, b.kinsokuMode)
            || !PunctuationWidthPolicy.samePolicy(a.punctuationWidth, b.punctuationWidth)) {
            return false;
        }
        var index:Int = 0;
        while (index < a.coalesceRepeatablePunctuation.length) {
            if (a.coalesceRepeatablePunctuation[index] != b.coalesceRepeatablePunctuation[index]) {
                return false;
            }
            index += 1;
        }
        return true;
    }

    /**
     * Codepoints that form a single semantic punctuation cluster when written
     * as consecutive repeats (CLREQ two-em dash and ellipsis). Listed in the
     * profile so region overrides do not require engine code changes.
     */
    public static final DefaultCoalesceRepeatablePunctuation:ReadOnlyArray<Int> = [0x2014, 0x2026, 0x22EF];

    public static final MainlandHorizontal:ClreqProfile = new ClreqProfile(
        "clreq-mainland-horizontal",
        ClreqStrictness.Normal,
        ClreqRegion.Mainland,
        null,
        [0x2014, 0x2026, 0x22EF],
        AutoSpacePolicy.Default,
        PunctuationGluePlacements.forRegion(ClreqRegion.Mainland),
        new AdjustmentStylePolicy(),
        KinsokuMode.MeasureAdaptive(14.0, 24.0, 32.0),
        new PunctuationWidthPolicy()
    );

    public static final TaiwanHorizontal:ClreqProfile = new ClreqProfile(
        "clreq-taiwan-horizontal",
        ClreqStrictness.Normal,
        ClreqRegion.Taiwan,
        null,
        [0x2014, 0x2026, 0x22EF],
        AutoSpacePolicy.Default,
        PunctuationGluePlacements.forRegion(ClreqRegion.Taiwan),
        new AdjustmentStylePolicy(),
        KinsokuMode.MeasureAdaptive(14.0, 24.0, 32.0),
        new PunctuationWidthPolicy()
    );

    public static final HongKongHorizontal:ClreqProfile = new ClreqProfile(
        "clreq-hongkong-horizontal",
        ClreqStrictness.Normal,
        ClreqRegion.HongKong,
        null,
        [0x2014, 0x2026, 0x22EF],
        AutoSpacePolicy.Default,
        PunctuationGluePlacements.forRegion(ClreqRegion.HongKong),
        new AdjustmentStylePolicy(),
        KinsokuMode.MeasureAdaptive(14.0, 24.0, 32.0),
        new PunctuationWidthPolicy()
    );
}
