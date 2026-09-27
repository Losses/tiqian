import { Canonical } from "../engine-haxe/out/protocol-ts/gen/org/tiqian/protocol/Canonical.ts";
const r = Canonical.encode({ kind: "WObj", fields: [
  { name: "key", value: { kind: "WStr", value: "p-1" } },
  { name: "text", value: { kind: "WStr", value: "中文" } },
  { name: "maxWidthPx", value: { kind: "WNum", value: 144 } },
] }, 0);
if (r.kind === "COk") {
  console.log(Array.from(r.bytes).map((b) => b.toString(16).padStart(2, "0")).join(""));
} else console.log("ERR " + r.issue);
