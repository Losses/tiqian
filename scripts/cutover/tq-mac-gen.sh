#!/usr/bin/env bash
# tq-mac-gen.sh — run one whole generate+build round on Mac.
#
# Revision discipline (2026-09-27): the emitted code comes from the generator pin
# tiqian/.haxelib/boring/git, so the default boring checkout IS that pin. The
# sibling boring/ checkout on master is a different revision and must be passed
# explicitly if it is really what this round tests.
#
# Mac memory discipline: the Mac has 8 cores and 16 GB. Only one round may run at
# a time, which the flock below enforces; do not bypass this script to run a
# second cargo, and do not run cargo and swiftc at the same time.
#
# Disk discipline: every product lands under ~/tq-dc/<label>, which is a symlink
# to /Volumes/DATA CENTER/tq (863 GB free). Never write to the Mac internal disk
# (/System/Volumes/Data has ~26 GB free).
#
# usage: tq-mac-gen.sh <label> [boring-checkout] [tiqian-checkout]
#   env TQ_MAC_EXPECT_RS overrides the generated-.rs file-count gate (default 422,
#   the whole engine tree; a protocol-only chain has a different count).
set -u
WS=/home/losses/Development/tq-workspace
LABEL="${1:?usage: tq-mac-gen.sh <label> [boring-checkout] [tiqian-checkout]}"
BORING="${2:-tiqian/.haxelib/boring/git}"
TIQIAN="${3:-tiqian}"
EXPECT_RS="${TQ_MAC_EXPECT_RS:-422}"
B="/Users/losses/tq-rust/$LABEL-boring"
T="/Users/losses/tq-rust/$LABEL-tiqian"
OUT="/Users/losses/tq-dc/$LABEL"
HAXE_BIN=/nix/store/p21w77yvh8dgd01klw3xbg1sr9dwwlfl-haxe-4.3.7/bin
LOG="$WS/.tq-logs/$LABEL.macgen.log"

LOCK=/tmp/tq-mac-gen.lock
exec 9>"$LOCK"
command -v flock >/dev/null 2>&1 && flock 9
: > "$LOG"
exec >>"$LOG" 2>&1
set -x

PIN_DIRTY=$(git -C "$WS/tiqian/.haxelib/boring/git" status --porcelain | wc -l)
[ "$PIN_DIRTY" != "0" ] && { echo "### 终止：vendored pin 有 $PIN_DIRTY 处改动，所有以 pin 为输入的读数作废（PIT-63）。"; exit 6; }
DIRTY=$(cd "$WS/$TIQIAN" && git status --porcelain | grep -v '^??' | wc -l)
echo "### [$LABEL] boring=$BORING($(cd $WS/$BORING && git rev-parse --short HEAD)) tiqian=$TIQIAN($(cd $WS/$TIQIAN && git rev-parse --short HEAD)) 未提交改动=${DIRTY}处"
[ "$DIRTY" != "0" ] && { echo "### 终止：tiqian 工作树有未提交改动，读数会混入它们。"; exit 2; }

rsync -a --delete "$WS/$BORING/packages/" "mac:~/tq-rust/$LABEL-boring/packages/"
rsync -a --delete "$WS/$BORING/samples/"  "mac:~/tq-rust/$LABEL-boring/samples/"
rsync -a --delete --exclude out/ --exclude .git/ --exclude node_modules/ \
      "$WS/$TIQIAN/engine-haxe/" "mac:~/tq-rust/$LABEL-tiqian/engine-haxe/"

cat > "$WS/.tq-logs/$LABEL.mac.hxml" <<EOF
-lib reflaxe
engine-haxe/targets/classes.hxml
-cp engine-haxe/src
-cp engine-haxe/macros
-cp $B/packages/compiler
-cp $B/samples
-cp $B/packages/compiler/reflaxe/rust/std-shadow
-cp $B/packages/compiler/reflaxe/rust
--macro GoldenDataMacros.init()
--macro haxe.macro.Compiler.addGlobalMetadata('org.tiqian', '@:build(std.RecordMember.build())')
--macro Intercept.run(['engine-haxe/src', '$B/samples/std'])
--macro rustcompiler.Compiler.use()
-D runtime-import=crate::runtime
-D runtime-emit=runtime
-D package-name=tiqian-engine-gen
-D package-license=MPL-2.0
-D rust-output=$OUT/src
-D float-precision=f32
std.UStringException
EOF
rsync -a "$WS/.tq-logs/$LABEL.mac.hxml" "mac:~/tq-rust/$LABEL-tiqian/engine-haxe/targets/$LABEL.mac.hxml"

ssh -o BatchMode=yes mac "cd ~/tq-rust/$LABEL-tiqian && rm -rf '$OUT' && PATH=$HAXE_BIN:\$PATH HAXELIB_PATH=\$HOME/tq-warn/tiqian/.haxelib haxe engine-haxe/targets/$LABEL.mac.hxml > /tmp/$LABEL.gen.log 2>&1; echo gen_rc=\$?; tail -3 /tmp/$LABEL.gen.log"

FILES=$(ssh -o BatchMode=yes mac "find '$OUT' -name '*.rs' | wc -l" | tr -d ' ')
echo "### 生成 .rs 文件数=$FILES（期望 $EXPECT_RS）"
if [ "$FILES" != "$EXPECT_RS" ]; then
  echo "### 终止：文件数不是 $EXPECT_RS，生成树是半成品，错误计数作废（PIT-103）。"
  exit 5
fi

ssh -o BatchMode=yes mac "mkdir -p /Users/losses/tq-dc/shared-target && rm -rf '$OUT/src/target' && ln -s /Users/losses/tq-dc/shared-target '$OUT/src/target' && ls -ld '$OUT/src/target'"
ssh -o BatchMode=yes mac "cd '$OUT/src' && /Users/losses/.cargo/bin/cargo build --lib --tests --message-format=short > /tmp/$LABEL.build.log 2>&1; echo build_rc=\$?; echo -n 'error 条数: '; grep -c ': error' /tmp/$LABEL.build.log"
echo "### 错误构成（前 8）"
ssh -o BatchMode=yes mac "grep ': error' /tmp/$LABEL.build.log | sed -E 's/^[^:]+:[0-9]+:[0-9]+: error//' | cut -c1-70 | sort | uniq -c | sort -rn | head -8"
DU=$(ssh -o BatchMode=yes mac "du -sh '$OUT' 2>/dev/null | cut -f1")
echo "### Mac 产物占用: $DU"
echo "### 本机磁盘: $(df -h /home | tail -1)"
