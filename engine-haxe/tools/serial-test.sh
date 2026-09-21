#!/usr/bin/env bash
# Serialize test runs across parallel lanes (2026-08-31 ruling: tests never
# run in parallel; implementation and compilation may). Every test command a
# lane or the center runs goes through this script. It blocks on an exclusive
# flock so tests queue one at a time across worktrees, agents, and sessions,
# supplies the recorded test timeout budget, then runs the command and
# forwards its exit status.
#
# Usage: serial-test.sh <command> [args...]
#   TIQIAN_TEST_LOCK   lock path  (default /tmp/tiqian-serial-test.lock)
#   TIQIAN_TEST_LOG    log path   (default /tmp/tiqian-serial-test.log)
#   BORING_TEST_TIMEOUT_MS   per-test wall-clock budget in milliseconds for
#                    every target bundle (default 180000, set below)
#
# The recorded test timeout budget. boring reads BORING_TEST_TIMEOUT_MS on
# every run and falls back to 5000 when the variable is unset or not a
# positive integer (boring packages/compiler/reflaxe/kotlin/kotlincompiler/
# KotlinRuntime.hx, timeoutBudgetMs, f0774994:226-240; spec boring
# docs/specs/features/19-testing.md, "Stage 1, the timeout verdict",
# f0774994:425-434). In the worktree of tiqian 60a418ea five cases of the
# Kotlin bundle need more than 5000 ms on this machine:
# WidthIndependentAnnotationCacheTest
# .cachedAndUncachedEnginesProduceIdenticalLayoutResultsAcrossWidths measures
# 28-43 s, EnglishHyphenationTest.hyphenatesCommonWordsAtSyllablePoints
# 11-17 s, AsciiPointMarkKinsokuTest
# .reportedRealWorldParagraphNeverWrapsDirectlyBeforeAnAsciiComma 8.4-9.3 s,
# LineBreakCoverageTest.testBundledHyphenationResource 6.3-8.2 s, and
# RecordedEvidenceGoldenParityTest.recordedEvidenceLayoutMatchesGolden
# 4.7-7.0 s. The 5000 fallback therefore records those bodies as failures
# carrying "this test timed out after 5000ms", and the verdict of the
# borderline case depends on machine load. 180000 is the budget
# engine-haxe/tools/f64-trace-compare.sh:47 already records for the Kotlin
# f64 bundle, so the five targets and every lane measure against one value
# instead of drifting between rounds.
#
# The budget decides the timeout verdict alone. An assertion that fails
# raises inside the test body and is recorded with its own message, whatever
# the budget is. An explicit value from the caller wins, which keeps a budget
# sweep reproducible:  BORING_TEST_TIMEOUT_MS=5000 serial-test.sh java ...
#
# The lock and log live outside the worktrees so every lane serializes on the
# same queue. The log records acquire/release lines so a stuck queue is
# diagnosable: the last acquired line without a matching release names the
# holder.
set -u

# Recorded budget (see the header). An explicit value from the caller wins.
: "${BORING_TEST_TIMEOUT_MS:=180000}"
export BORING_TEST_TIMEOUT_MS

LOCK="${TIQIAN_TEST_LOCK:-/tmp/tiqian-serial-test.lock}"
LOG="${TIQIAN_TEST_LOG:-/tmp/tiqian-serial-test.log}"

if [ "$#" -eq 0 ]; then
  echo "usage: serial-test.sh <command> [args...]" >&2
  exit 2
fi

exec 9>"$LOCK"
flock 9

printf '[serial-test] %s acquired: %s\n' "$(date -Is)" "$*" >>"$LOG"
"$@"
status=$?
printf '[serial-test] %s released rc=%s: %s\n' "$(date -Is)" "$status" "$*" >>"$LOG"

exit "$status"
