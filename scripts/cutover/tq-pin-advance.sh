#!/usr/bin/env bash
# tq-pin-advance.sh - advance the boring pin to an immutable, verified commit.
#
# Why this exists. The pin used to be advanced by hand to a short sha of
# whatever branch head was current. That produced two failure modes.
#
#   Orphaned sha. 455ad5849 sat in flake.nix and locked fine, but it was a
#   commit on the lineage that boring's history rewrite replaced. It is on no
#   branch and no tag, so GitHub may collect it at any time.
#
#   Stale sha. ed59d31ba was on master when it was pinned, and master had
#   already moved to 6b04d384 by the time the pin landed. The generated Kotlin
#   then misses the two Kotlin fixes that landed in between.
#
# The pin form this script enforces: github:Losses/boring/<full 40-char sha>
# where the sha is reachable from master. A release tag is accepted only when
# the tag is also reachable from master. boring's tags predate the history
# rewrite, so every tag in that checkout is refused by that check today.
#
# usage:
#   tq-pin-advance.sh --rev <sha>       pin to an explicit commit
#   tq-pin-advance.sh --from-ci         pin to the last successful ci.yml run on master
#   tq-pin-advance.sh --tag <name>      pin to a release tag (must be on master)
#   tq-pin-advance.sh --check           report the current pin, change nothing
#
# options:
#   --boring <path>     boring checkout (default: TQ_BORING, else ../boring)
#   --repo <path>       tiqian checkout (default: the script's repo root)
#   --fetch             run git fetch origin master first
#   --dry-run           print the plan and write nothing
#   --allow-stale-ci    accept a ci.yml run older than 7 days
set -euo pipefail

SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$SELF_DIR/../.." && pwd)"
BORING="${TQ_BORING:-$(cd "$REPO/../boring" 2>/dev/null && pwd || true)}"
MODE=""
ARG=""
FETCH=0
DRY_RUN=0
ALLOW_STALE_CI=0
CI_MAX_AGE_DAYS=7
RECORD="flake.boring-pin.json"

die() { echo "tq-pin-advance: $*" >&2; exit 1; }
note() { echo "tq-pin-advance: $*"; }

while [ $# -gt 0 ]; do
  case "$1" in
    --rev) MODE=rev; ARG="${2:-}"; shift 2 ;;
    --tag) MODE=tag; ARG="${2:-}"; shift 2 ;;
    --from-ci) MODE=ci; shift ;;
    --check) MODE=check; shift ;;
    --boring) BORING="${2:-}"; shift 2 ;;
    --repo) REPO="${2:-}"; shift 2 ;;
    --fetch) FETCH=1; shift ;;
    --dry-run) DRY_RUN=1; shift ;;
    --allow-stale-ci) ALLOW_STALE_CI=1; shift ;;
    -h|--help) sed -n '2,40p' "$0"; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

[ -n "$MODE" ] || die "one of --rev, --tag, --from-ci, --check is required"
[ -d "$BORING/.git" ] || die "not a git checkout: $BORING (pass --boring)"
[ -f "$REPO/flake.nix" ] || die "no flake.nix in $REPO (pass --repo)"

if [ "$FETCH" = 1 ]; then
  git -C "$BORING" fetch --quiet origin master
fi

MASTER=$(git -C "$BORING" rev-parse master)
MASTER_SHORT=$(git -C "$BORING" rev-parse --short master)
CURRENT_PIN=$(sed -n 's|.*github:Losses/boring/\([^"]*\)".*|\1|p' "$REPO/flake.nix" | head -1)
[ -n "$CURRENT_PIN" ] || die "no boring pin found in $REPO/flake.nix"

is_on_master() { git -C "$BORING" merge-base --is-ancestor "$1" master 2>/dev/null; }

resolve_rev() {
  local sha
  sha=$(git -C "$BORING" rev-parse --verify --quiet "$1^{commit}") || die "unknown commit: $1"
  is_on_master "$sha" || die "commit $sha is not reachable from master ($MASTER_SHORT); refusing an orphaned lineage"
  printf '%s' "$sha"
}

resolve_tag() {
  local name="$1" sha
  sha=$(git -C "$BORING" rev-parse --verify --quiet "refs/tags/$name^{commit}") || die "unknown tag: $name"
  if ! is_on_master "$sha"; then
    die "tag $name ($sha) is not reachable from master ($MASTER_SHORT); boring's tags predate the history rewrite, pin the master commit instead"
  fi
  printf '%s' "$sha"
}

resolve_ci() {
  local json sha stamp age_days
  json=$(curl -fsS "https://api.github.com/repos/Losses/boring/actions/workflows/ci.yml/runs?branch=master&status=success&per_page=1")
  sha=$(printf '%s' "$json" | sed -n 's/.*"head_sha": *"\([0-9a-f]*\)".*/\1/p' | head -1)
  [ -n "$sha" ] || die "no successful ci.yml run on master"
  stamp=$(printf '%s' "$json" | sed -n 's/.*"updated_at": *"\([0-9T:-]*\)Z".*/\1/p' | head -1)
  if [ -n "$stamp" ]; then
    age_days=$(( ( $(date -u +%s) - $(date -u -d "$stamp" +%s) ) / 86400 ))
    if [ "$age_days" -gt "$CI_MAX_AGE_DAYS" ] && [ "$ALLOW_STALE_CI" = 0 ]; then
      die "the last successful ci.yml run on master is $age_days days old ($sha); pass --allow-stale-ci to accept it"
    fi
  fi
  is_on_master "$sha" || die "ci.yml head $sha is not reachable from master ($MASTER_SHORT)"
  printf '%s' "$sha"
}

case "$MODE" in
  check)
    lag=$(git -C "$BORING" log --format=%h "$CURRENT_PIN..master" 2>/dev/null | wc -l | tr -d ' ')
    if is_on_master "$CURRENT_PIN"; then
      note "pin $CURRENT_PIN is on master; master is $MASTER_SHORT ($lag commits ahead)"
    else
      note "pin $CURRENT_PIN is NOT reachable from master ($MASTER_SHORT); it is on an orphaned lineage"
    fi
    exit 0
    ;;
  rev) NEW=$(resolve_rev "$ARG") ;;
  tag) NEW=$(resolve_tag "$ARG") ;;
  ci) NEW=$(resolve_ci) ;;
esac

NEW_SHORT=$(git -C "$BORING" rev-parse --short "$NEW")
note "pin $CURRENT_PIN -> $NEW ($NEW_SHORT)"
note "master at advance: $MASTER_SHORT ($MASTER)"

if [ "$DRY_RUN" = 1 ]; then
  note "dry run: flake.nix and $RECORD left untouched"
  exit 0
fi

python3 - "$REPO/flake.nix" "$NEW" <<'PY'
import io, sys
path, new = sys.argv[1], sys.argv[2]
s = io.open(path, encoding="utf-8").read()
import re
pat = re.compile(r'(github:Losses/boring/)[^"]+(")')
assert len(pat.findall(s)) == 1, "expected exactly one boring pin"
io.open(path, "w", encoding="utf-8").write(pat.sub(lambda m: m.group(1) + new + m.group(2), s))
PY

( cd "$REPO" && nix flake lock >/dev/null )

python3 - "$REPO/$RECORD" "$NEW" "$MASTER" "$MODE" <<'PY'
import io, json, sys, datetime
path, pin, master, mode = sys.argv[1:5]
record = {
    "pin": pin,
    "masterAtPin": master,
    "resolvedFrom": mode,
    "resolvedAt": datetime.datetime.now(datetime.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
}
io.open(path, "w", encoding="utf-8").write(json.dumps(record, indent=2) + "\n")
PY

note "wrote $RECORD (pin $NEW_SHORT, masterAtPin $MASTER_SHORT)"
