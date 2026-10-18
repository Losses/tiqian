import assert from "node:assert/strict";
import test from "node:test";

import {
  RELAYOUT_SYNC_STALL_MS,
  RELAYOUT_SYNC_UPPER_BOUND_MS,
  assertRelayoutSyncUpperBound,
  measureSyncDurationMs,
  RelayoutSyncTiming,
} from "../src/engine/relayout-sync-timing.js";

// SOP3 (E2b) timing gate: the assertion that replaces the browser end-to-end
// 283ms stall measurement. The pre-fix synchronous prepareParagraphLayout FFI
// call blocked the main thread for about 283ms; the worker-plan path removes
// that work from the main thread, so one synchronous relayout slice must stay
// under RELAYOUT_SYNC_UPPER_BOUND_MS.

test("the timing constants pin the stall reference and the 50ms bound", () => {
  assert.equal(RELAYOUT_SYNC_STALL_MS, 283);
  assert.equal(RELAYOUT_SYNC_UPPER_BOUND_MS, 50);
});

test("a healthy synchronous slice passes the 50ms upper bound", () => {
  const verdict = assertRelayoutSyncUpperBound(30);
  assert.deepEqual(verdict, { ok: true, maxSliceMs: 30, boundMs: 50 });
});

test("the pre-fix 283ms stall fails the 50ms upper bound (discriminative)", () => {
  // Discriminative validation: the assertion must reject the pre-fix form,
  // where one synchronous prepareParagraphLayout call blocked the main thread
  // for about 283ms. A bound that passed this value would not guard the
  // regression it is meant to catch.
  const verdict = assertRelayoutSyncUpperBound(RELAYOUT_SYNC_STALL_MS);
  assert.equal(verdict.ok, false);
  assert.equal(verdict.maxSliceMs, 283);
  assert.equal(verdict.boundMs, 50);
});

test("an explicit bound overrides the default 50ms", () => {
  assert.equal(assertRelayoutSyncUpperBound(283, 300).ok, true);
  assert.equal(assertRelayoutSyncUpperBound(60, 50).ok, false);
});

test("a non-finite maxSliceMs fails the assertion so a missing signal cannot pass", () => {
  assert.equal(assertRelayoutSyncUpperBound(NaN).ok, false);
  assert.equal(assertRelayoutSyncUpperBound(Infinity).ok, false);
});

test("the 50ms boundary is inclusive", () => {
  assert.equal(assertRelayoutSyncUpperBound(50).ok, true);
  assert.equal(assertRelayoutSyncUpperBound(50.0001).ok, false);
});

test("RelayoutSyncTiming accumulates the maximum recorded duration", () => {
  const timing = new RelayoutSyncTiming();
  assert.equal(timing.maxDurationMs, 0);
  assert.equal(timing.sampleCount, 0);
  timing.record(3);
  timing.record(10);
  timing.record(5);
  assert.equal(timing.maxDurationMs, 10);
  assert.equal(timing.sampleCount, 3);
});

test("RelayoutSyncTiming ignores non-finite and negative durations", () => {
  const timing = new RelayoutSyncTiming();
  timing.record(NaN);
  timing.record(Infinity);
  timing.record(-1);
  assert.equal(timing.sampleCount, 0);
  assert.equal(timing.maxDurationMs, 0);
});

test("RelayoutSyncTiming.assertUpperBound mirrors the free assertion", () => {
  const timing = new RelayoutSyncTiming();
  timing.record(12);
  assert.deepEqual(timing.assertUpperBound(), { ok: true, maxSliceMs: 12, boundMs: 50 });
  assert.deepEqual(timing.assertUpperBound(8), { ok: false, maxSliceMs: 12, boundMs: 8 });
});

test("measureSyncDurationMs returns a finite non-negative number", () => {
  const duration = measureSyncDurationMs(() => {});
  assert.equal(Number.isFinite(duration), true);
  assert.equal(duration >= 0, true);
});
