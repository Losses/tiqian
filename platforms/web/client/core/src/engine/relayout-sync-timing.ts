// Relayout synchronous-slice timing: the assertion harness that replaces the
// browser end-to-end 283ms stall measurement.
//
// Background. The 283ms stall was one synchronous prepareParagraphLayout FFI
// call blocking the main thread during a width-change relayout. E2a made the
// relayout dispatch asynchronous, and E2b routes the heavy layout off the main
// thread (layoutWorker.take -> commitWorkerPreparedParagraph), so the
// synchronous work that remains on the main thread is the light commit pass.
// A browser performance capture cannot run in the isolated copy, so this
// module asserts the synchronous-slice duration upper bound instead of
// reproducing the 283ms end-to-end number.
//
// Signal source. The synchronous-slice duration is already measured by the
// layout job pool. layout-job-pool.ts:259-260 records maxSliceDuration as the
// largest runSliceInternal wall-clock slice (a Date.now() delta around the
// synchronous processItem loop), and finishJob at layout-job-pool.ts:184
// reports it as report.maxSliceMs. progressive-drivers.ts:258-299 dispatches
// that report as the tiqian:relayout-ready event detail, and
// mount.ts:144-145 records it as the tiqianRelayoutMaxSliceMs diagnosis.
// This module supplies the measurement primitive and the upper-bound assertion
// a test applies to that signal.
//
// Coordinated-only caveat. A slice is bounded by the coordination budget only
// when the root is attached to a coordinator (layout-job-pool.ts:318-322). An
// unattached job runs to completion in one slice (layout-job-pool.ts:323-330),
// so its maxSliceMs is the whole-job duration, not a single-item stall. The
// bound below applies to a coordinated relayout, where one slice contains at
// most the budget plus one item's work, so a single 283ms prepareParagraphLayout
// makes the slice about 283ms and maxSliceMs records it.
//
// Embedding constraint: this file may be wrapped in a Kotlin raw string by the
// generator, so it contains no dollar sign and no triple double-quote sequence.
// Use string concatenation, never template literals.

// The pre-fix stall reference, in milliseconds: one synchronous
// prepareParagraphLayout FFI call blocked the main thread for about 283ms.
// This is the value the worker-plan path removes from the main thread. It is a
// reference for deriving the bound, not a runtime signal.
export const RELAYOUT_SYNC_STALL_MS: number = 283;

// The proposed upper bound for one synchronous relayout slice, in
// milliseconds. The coordination slicing budget is capped at 6ms
// (coordination-service.ts:349), so a healthy slice that commits one or more
// worker-prepared paragraphs stays in single-digit milliseconds. A 50ms bound
// leaves headroom for slower hosts while still rejecting the 283ms stall. The
// exact value is a proposal for the tech lead to adjudicate.
export const RELAYOUT_SYNC_UPPER_BOUND_MS: number = 50;

export interface RelayoutSyncBoundVerdict {
  ok: boolean;
  maxSliceMs: number;
  boundMs: number;
}

// Measure the synchronous wall-clock duration of fn in milliseconds, using
// performance.now for sub-millisecond resolution. fn runs synchronously; the
// returned value is the main-thread blocking time of that body.
export function measureSyncDurationMs(fn: () => void): number {
  const startedAt = performance.now();
  fn();
  const finishedAt = performance.now();
  const duration = finishedAt - startedAt;
  return duration < 0 ? 0 : duration;
}

// Accumulate synchronous durations recorded during one relayout pass. The
// maximum recorded value is the synchronous-slice upper bound the assertion
// checks. Use measureSyncDurationMs to feed it per-item or per-slice durations
// when a direct measurement is available instead of the report.maxSliceMs
// signal.
export class RelayoutSyncTiming {
  #maxDurationMs: number = 0;
  #sampleCount: number = 0;

  record(durationMs: number): void {
    if (!Number.isFinite(durationMs) || durationMs < 0) return;
    this.#sampleCount += 1;
    if (durationMs > this.#maxDurationMs) this.#maxDurationMs = durationMs;
  }

  get maxDurationMs(): number {
    return this.#maxDurationMs;
  }

  get sampleCount(): number {
    return this.#sampleCount;
  }

  assertUpperBound(boundMs: number = RELAYOUT_SYNC_UPPER_BOUND_MS): RelayoutSyncBoundVerdict {
    return {
      ok: this.#maxDurationMs <= boundMs,
      maxSliceMs: this.#maxDurationMs,
      boundMs: boundMs,
    };
  }
}

// Assert that a reported maxSliceMs value stays within the bound. A test reads
// maxSliceMs from the tiqian:relayout-ready event detail or the
// tiqianRelayoutMaxSliceMs diagnosis and passes it here. A non-finite value is
// treated as a failure so a missing signal cannot pass the assertion.
export function assertRelayoutSyncUpperBound(
  maxSliceMs: number,
  boundMs: number = RELAYOUT_SYNC_UPPER_BOUND_MS,
): RelayoutSyncBoundVerdict {
  return {
    ok: Number.isFinite(maxSliceMs) && maxSliceMs <= boundMs,
    maxSliceMs: maxSliceMs,
    boundMs: boundMs,
  };
}
