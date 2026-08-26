#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

function normalizeArchiveHeader(archive) {
  const bytes = fs.readFileSync(archive);
  if (bytes.subarray(0, 8).toString("ascii") !== "!<arch>\n") throw new Error("Kitty provider archive magic is invalid");
  let offset = 8;
  while (offset < bytes.length) {
    if (offset + 60 > bytes.length || bytes.subarray(offset + 58, offset + 60).toString("ascii") !== "`\n") throw new Error("Kitty provider archive header is invalid");
    const size = Number.parseInt(bytes.subarray(offset + 48, offset + 58).toString("ascii").trim(), 10);
    if (!Number.isSafeInteger(size) || size < 0 || offset + 60 + size > bytes.length) throw new Error("Kitty provider archive member size is invalid");
    for (const [start, width] of [[16, 12], [28, 6], [34, 6]]) bytes.write("0".padEnd(width, " "), offset + start, width, "ascii");
    offset += 60 + size + (size % 2);
  }
  if (offset !== bytes.length) throw new Error("Kitty provider archive padding is invalid");
  fs.writeFileSync(archive, bytes);
}

export function normalizeKittySdk(root) {
  const configPath = path.join(root, "python-config.json");
  const config = JSON.parse(fs.readFileSync(configPath, "utf8"));
  for (const name of ["executable", "library", "library_dir"]) {
    if (typeof config[name] !== "string" || config[name] === "") throw new Error(`Kitty Python config ${name} is invalid`);
  }
  fs.writeFileSync(configPath, `${JSON.stringify({ executable: path.basename(config.executable), library: config.library, library_dir: "runtime/lib" })}\n`);
  normalizeArchiveHeader(path.join(root, "lib/libkitty_provider.a"));
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const [root] = process.argv.slice(2);
  if (!root || process.argv.length !== 3 || !path.isAbsolute(root)) throw new Error("usage: normalize-kitty-sdk.mjs <absolute-sdk-root>");
  normalizeKittySdk(root);
  process.stdout.write(`KITTY_SDK_NORMALIZED root=${root}\n`);
}
