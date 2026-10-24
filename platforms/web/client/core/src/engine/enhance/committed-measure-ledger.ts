// CommittedMeasureLedger — the committed paragraph measure signature per
// enhanced root. The dedup baseline was two per-instance fields on the
// ResponsiveManager (lastParagraphMeasures for the normal pass and
// lastCommittedParagraphMeasures for the forced pass). Both were written by
// several paths, and both died when refreshRuntimeFromSource re-created the
// manager. This record lives in this module closure behind a Symbol.for
// registry key so the dedup baseline survives that re-creation, and module
// copies in one document share one record.
interface CommittedMeasureLedger {
  byRoot: WeakMap<HTMLElement, string>;
}

const COMMITTED_MEASURE_LEDGER_KEY: unique symbol = Symbol.for("@tiqian/core.committed-measure-ledger.v1");

type CommittedMeasureLedgerRegistry = Record<symbol, CommittedMeasureLedger | undefined>;

const getLedger = (): CommittedMeasureLedger => {
  const registry = globalThis as CommittedMeasureLedgerRegistry;
  return registry[COMMITTED_MEASURE_LEDGER_KEY] ??= { byRoot: new WeakMap() };
};

/** The committed measure signature for a root, or "" when none is recorded. */
export function committedMeasureSignatureFor(root: HTMLElement): string {
  return getLedger().byRoot.get(root) ?? "";
}

/** Records (or clears, when signature is "") the committed measure signature. */
export function recordCommittedMeasureSignature(root: HTMLElement, signature: string): void {
  const ledger = getLedger();
  if (signature === "") ledger.byRoot.delete(root);
  else ledger.byRoot.set(root, signature);
}
