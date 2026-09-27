// Type surface of the public entry (src/facade.mjs). All twelve exports are
// declared here beside their JavaScript implementations in the facade
// modules; every value is served by the single-source generated tree
// (./engine-gen). The export order matches src/facade.mjs and the
// compatibility contract in package.test.mts.

// The published request shape of the two precompute entries: the field set
// the retired Kotlin/JS @JsExport interface PrepareParagraphRequest carried
// (LayoutRequestDtos.kt), now validated by the generated protocol model
// (org/tiqian/protocol ParagraphRequestChecks).
export interface PrepareParagraphRequest {
  text: string;
  maxWidthPx: number;
  fontFamilies: string[];
  fontSizePx: number;
  lineHeightPx: number;
  locale: string;
  fontWeight: number;
  italic: boolean;
  firstLineIndentIc: number;
  lineLengthGridEnabled: boolean;
  sourceBoundaries: number[];
  textSpans: Array<{
    start: number;
    end: number;
    fontFamilies: string[];
    fontSize: number;
    fontWeight: number;
    italic: boolean;
    baselineShift: number;
  }>;
  inlineBoxes: Array<{
    start: number;
    end: number;
    inlineStart: number;
    inlineEnd: number;
    outerSpacing: string;
  }>;
  lineBreakSpans: Array<{
    start: number;
    end: number;
    policy: string;
  }>;
  inlineObjects: Array<{
    start: number;
    end: number;
    advance: number;
    ascent: number;
    descent: number;
  }>;
  decorations: Array<{
    start: number;
    end: number;
    kind: string;
  }>;
  emphasisDotGapEm: number | null;
  renderEvidenceOverride: boolean | null;
}

declare function liangHyphenate(
  word: string,
  patternsJson: string,
  exceptionsJson: string,
  leftMin?: number,
  rightMin?: number,
): string;
declare function unicodePunctuationLineBreakClassOf(codePoint: number): string;
declare function bopomofoParse(reading: string): string;
declare function numberSymbolCohesionUnbreakableRanges(text: string): string;
declare function fontMetricsResolve(requestJson: string): string;
declare function fontFallbackResolve(
  text: string,
  start: number,
  end: number,
  requestJson: string,
): string;
declare function classifyFontRole(text: string, start: number, end: number, locale: string): string;
declare function classifyFontRoles(
  text: string,
  starts: number[],
  ends: number[],
  locale: string,
): string[];
declare function unsupportedInlineShapingProperties(): string[];
declare function firstDivergentInlineShapingProperty(
  elementValues: string[],
  paragraphValues: string[],
): string | null;

declare function precomputeParagraphWithDiagnostics(
  request: PrepareParagraphRequest,
  zeroAdvanceEpsilonPx: number,
  shapeJson: (requestJson: string) => string,
  metricsJson: (requestJson: string) => string,
): string;
declare function precomputeParagraphWithBrowserMetrics(
  request: PrepareParagraphRequest,
  zeroAdvanceEpsilonPx: number,
  callbacks: { shapeJson: (requestJson: string) => string; metricsJson: (requestJson: string) => string },
): string;

export {
  bopomofoParse,
  numberSymbolCohesionUnbreakableRanges,
  fontMetricsResolve,
  fontFallbackResolve,
  liangHyphenate,
  unicodePunctuationLineBreakClassOf,
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
};

