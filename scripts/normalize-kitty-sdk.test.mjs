import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

import { normalizeKittySdk } from "./normalize-kitty-sdk.mjs";

function archiveMember(name, bytes) {
  const field = (value, width) => String(value).padEnd(width, " ");
  const header = `${field(`${name}/`, 16)}${field(1720000000, 12)}${field(501, 6)}${field(20, 6)}${field(100644, 8)}${field(bytes.length, 10)}` + "`\n";
  return Buffer.concat([Buffer.from(header), bytes, ...(bytes.length % 2 === 0 ? [] : [Buffer.from("\n")])]);
}

test("normalizes host paths and static archive ownership deterministically", () => {
  const root = fs.mkdtempSync(path.join(fs.realpathSync(os.tmpdir()), "kitty-sdk-normalize-"));
  try {
    fs.mkdirSync(path.join(root, "lib"), { recursive: true });
    fs.writeFileSync(path.join(root, "python-config.json"), JSON.stringify({
      executable: "/host/python3.14",
      library: "python3.14",
      library_dir: "/random/build/runtime/lib",
    }));
    const archive = path.join(root, "lib/libkitty_provider.a");
    fs.writeFileSync(archive, Buffer.concat([Buffer.from("!<arch>\n"), archiveMember("provider.o", Buffer.from("object"))]));

    normalizeKittySdk(root);
    const once = fs.readFileSync(archive);
    normalizeKittySdk(root);
    assert.deepEqual(fs.readFileSync(archive), once);
    assert.equal(fs.readFileSync(path.join(root, "python-config.json"), "utf8"), '{"executable":"python3.14","library":"python3.14","library_dir":"runtime/lib"}\n');
    assert.equal(once.subarray(8 + 16, 8 + 28).toString("ascii").trim(), "0");
    assert.equal(once.subarray(8 + 28, 8 + 34).toString("ascii").trim(), "0");
    assert.equal(once.subarray(8 + 34, 8 + 40).toString("ascii").trim(), "0");
  } finally {
    fs.rmSync(root, { recursive: true, force: true });
  }
});
