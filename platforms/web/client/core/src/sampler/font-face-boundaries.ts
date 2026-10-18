/**
 * Shared-layer font-face boundary generation (Stage2-shared).
 *
 * The parsing and face-selection logic is single-sourced in Haxe
 * (engine-haxe/src/org/tiqian/shared/fontface/FontFaceBoundaries.hx) and
 * generated into ./org/tiqian/shared/fontface/ by "boring gen shared-ts".
 *
 * This module is the thin public barrel: it keeps the historical loose
 * ("unknown") public types so existing call sites keep typechecking, coerces
 * them to the generated concrete types, and delegates to the generated
 * FontFaceBoundaries class. The generated module throws
 * FontFaceBoundaryException (extends Error) with the same messages the
 * previous hand-written module used.
 */
import { FontFaceBoundaries } from "./org/tiqian/shared/fontface/FontFaceBoundaries.js";
import type {
  FontFaceRecord as GeneratedFontFaceRecord,
  WorkerFontContractRequest as GeneratedWorkerFontContractRequest,
} from "./org/tiqian/shared/fontface/FontFaceBoundaries.js";

/** A face descriptor record read from the worker font contract. */
export interface FontFaceRecord {
  family: unknown;
  localNames?: Iterable<unknown>;
  style: string;
  weight: readonly unknown[];
  unicodeRange: unknown;
  publicUrl: unknown;
  faceIndex: unknown;
  sourceOrder: unknown;
}

/** The layout-worker font contract request this module consumes. */
export interface WorkerFontContractRequest {
  text: unknown;
  fontFamilies: unknown;
  fontSizePx: unknown;
  fontWeight: unknown;
  italic: unknown;
  textSpans: unknown;
}

function toGeneratedFace(face: FontFaceRecord): GeneratedFontFaceRecord {
  return {
    family: String(face.family ?? ""),
    localNames: face.localNames == null
      ? null
      : Array.from(face.localNames, (name) => String(name)),
    style: String(face.style),
    weight: Array.from(face.weight ?? [], (value) => Number(value)),
    unicodeRange: String(face.unicodeRange ?? ""),
    publicUrl: String(face.publicUrl ?? ""),
    faceIndex: Number(face.faceIndex),
    sourceOrder: Number(face.sourceOrder),
  };
}

export function workerSnapshotSubsetSourceBoundaries(
  faceMetadata: readonly FontFaceRecord[],
  request: WorkerFontContractRequest,
): number[] {
  const faces = Array.from(faceMetadata ?? [], toGeneratedFace);
  const concreteRequest: GeneratedWorkerFontContractRequest = {
    text: String(request.text),
    fontFamilies: String(request.fontFamilies ?? ""),
    fontSizePx: Number(request.fontSizePx),
    fontWeight: Number(request.fontWeight),
    italic: request.italic === true,
    textSpans: request.textSpans == null ? null : String(request.textSpans),
  };
  return FontFaceBoundaries.workerSnapshotSubsetSourceBoundaries(faces, concreteRequest);
}

export function mergeSerializedSourceBoundaries(
  serialized: unknown,
  additional: Iterable<unknown> | null | undefined,
): string {
  const existing = String(serialized ?? "")
    .split(",")
    .filter(Boolean)
    .map(Number);
  const additionalNumbers = Array.from(additional ?? [], (value) => Number(value));
  return FontFaceBoundaries.mergeSerializedSourceBoundaries(existing, additionalNumbers);
}
