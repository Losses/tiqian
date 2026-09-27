// Mapping adapter from the test fixture shape (BinaryTableInput) to the
// generated single-source encoder. It contains no byte-level code: the
// TIQTBL03 contract lives in protocol-gen, produced from Haxe
// (org.tiqian.protocol.SnapshotTableBinary); this file only converts JSON
// values to the pre-serialized text form the generated encoder takes, per
// the Stage1-P2 boundary ruling.
import { SnapshotTableBinary } from "./protocol-gen/org/tiqian/protocol/SnapshotTableBinary.js";
import { TableInput } from "./protocol-gen/org/tiqian/protocol/TableInput.js";
import { TableMetricRow } from "./protocol-gen/org/tiqian/protocol/TableMetricRow.js";
import { TableProbe } from "./protocol-gen/org/tiqian/protocol/TableProbe.js";

export interface BinaryTableInput {
  replayStrings?: readonly string[];
  metrics?: readonly {
    serializedFamilies: string;
    fontWeight: number;
    italic: boolean;
    role: string;
    faceSelectionText: string;
    valuesEm: readonly (number | null)[];
  }[];
  probes?: readonly {
    text: string;
    advancePx?: number;
    fontSizePx?: number;
    fontWeight?: number;
    italic?: boolean;
    script?: string;
    language?: string;
    features?: readonly string[];
  }[];
  faces?: readonly unknown[];
  typographies?: readonly unknown[];
  valueStyles?: readonly string[];
  fontPreloads?: readonly string[];
  revisions?: unknown;
}

export function writeBinaryTable(table: BinaryTableInput): Uint8Array {
  const metrics = (table.metrics ?? []).map(
    (row) =>
      new TableMetricRow(
        row.serializedFamilies,
        row.fontWeight,
        row.italic,
        row.role,
        row.faceSelectionText,
        [...row.valuesEm],
      ),
  );
  const probes = (table.probes ?? []).map(
    (probe) =>
      new TableProbe(
        probe.text,
        probe.advancePx ?? 0,
        probe.fontSizePx ?? 0,
        probe.fontWeight ?? 400,
        probe.italic ?? false,
        probe.script ?? "",
        probe.language ?? "",
        [...(probe.features ?? [])],
      ),
  );
  const texts = (rows: readonly unknown[] | undefined): string[] | undefined =>
    rows === undefined ? undefined : rows.map((row) => JSON.stringify(row));
  return SnapshotTableBinary.encode(
    new TableInput(
      [...(table.replayStrings ?? [])],
      metrics,
      probes,
      texts(table.faces) ?? [],
      texts(table.typographies) ?? [],
      table.valueStyles === undefined ? [] : [...table.valueStyles],
      texts(table.fontPreloads) ?? [],
      JSON.stringify(table.revisions ?? {}),
    ),
  );
}
