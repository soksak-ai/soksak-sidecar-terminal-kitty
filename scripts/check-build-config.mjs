#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (name) => fs.readFileSync(path.join(root, name), "utf8");
const manifest = JSON.parse(read("build-dependencies.json"));
const makefile = read("Makefile");
const workflow = read(".github/workflows/release.yml");
const build = read("build.rs");
const engine = read("src/engine.rs");
const prepare = read("scripts/prepare-kitty-sdk.sh");
const gate = read("scripts/gate.sh");
const cargo = read("Cargo.toml");
const dependency = manifest.dependencies?.[0];
for (const symbol of [
  "kitty_provider_pointer",
  "kitty_provider_selection_start",
  "kitty_provider_selection_update",
  "kitty_provider_selection_text",
  "kitty_provider_selection_range",
]) {
  if (!engine.includes(symbol)) throw new Error(`Kitty provider API is not consumed: ${symbol}`);
}
for (const fact of [
  "mouse_x10: false",
  "mouse_highlight: false",
  "modes.mouse_reporting()",
  "modes.reports_pointer(input.phase, input.button)",
]) {
  if (!engine.includes(fact)) throw new Error(`Kitty tracking contract is missing: ${fact}`);
}
if (engine.includes("..ModeSnap::default()")) {
  throw new Error("Kitty tracking facts must not fall back to TerminalModes defaults");
}
if (!cargo.includes('soksak-kit-sidecar-terminal = { git = "https://github.com/soksak-ai/soksak-kit-sidecar-terminal", rev = "20fb2d73d13e5bcde592380d3052c5d2204a592f"')) {
  throw new Error("terminal Kit must be pinned to the final 0.0.34 release revision");
}
const keys = (value) => Object.keys(value).sort().join("\n");
if (manifest.schema !== "soksak-build-dependencies-v1" || manifest.dependencies.length !== 1 ||
    keys(dependency) !== ["commit", "id", "repository", "targets", "tools"].join("\n")) {
  throw new Error("Kitty build dependency document is not flat and exact");
}
if (dependency.id !== "kitty-provider-sdk" || !/^[a-f0-9]{40}$/.test(dependency.commit) ||
    !/^https:\/\/[^/]+\/(?:[^/]+\/)+[^/]+[.]git$/.test(dependency.repository)) {
  throw new Error("Kitty build dependency identity is invalid");
}
if (keys(dependency.tools) !== "python" || !/^\d+[.]\d+[.]\d+$/.test(dependency.tools.python)) {
  throw new Error("Kitty Python build tool is not exact");
}
const targets = JSON.parse(read("release/targets.json")).map(({ target }) => target).sort();
if (JSON.stringify(targets) !== JSON.stringify(Object.keys(dependency.targets).sort())) throw new Error("Kitty target sets differ");
for (const target of targets) {
  const expected = [{ path: `targets/${target}/kitty-provider`, type: "tree" }];
  if (JSON.stringify(dependency.targets[target].outputs) !== JSON.stringify(expected)) throw new Error(`Kitty output differs for ${target}`);
}
for (const [name, source] of [["Makefile", makefile], ["workflow", workflow], ["build.rs", build], ["README", read("README.md")]]) {
  for (const duplicated of [dependency.repository, dependency.commit, dependency.tools.python]) {
    if (source.includes(duplicated)) throw new Error(`${name} duplicates build-dependencies.json metadata`);
  }
}
for (const target of ["preflight", "prepare", "build", "verify", "stage"]) {
  if (!new RegExp(`^${target}:`, "m").test(makefile)) throw new Error(`Makefile target is missing: ${target}`);
}
if (!makefile.includes("TARGET") || !makefile.includes("SOKSAK_BUILD_DEPENDENCY_ROOT=")) throw new Error("Makefile does not own the addressed build command");
if (!prepare.includes("soksak-validate build-receipt-create")) throw new Error("Kitty prepare does not use canonical receipt creation");
if (!prepare.includes('stage=$build_root/builds/$target/$commit') ||
    !prepare.includes('previous_target=$transaction/previous-target')) {
  throw new Error("Kitty prepare does not replace a changed declared SDK transactionally");
}
if (!gate.includes('previous=$test_sdk.previous.$$')) throw new Error("Kitty test SDK replacement is not transactional");
if (!prepare.includes('normalize-kitty-sdk.mjs" "$sdk"')) throw new Error("Kitty prepare does not normalize host-dependent SDK metadata");
if (!workflow.includes('make stage TARGET="${{ matrix.target }}" STAGE=dist')) throw new Error("workflow does not call the owner Make target");
if (build.includes("SOKSAK_KITTY_PROVIDER_SDK") || engine.includes("SOKSAK_KITTY_PROVIDER_SDK")) throw new Error("runtime retains the raw Kitty SDK input");
if (!build.includes("SOKSAK_BUILD_DEPENDENCY_ROOT")) throw new Error("build.rs does not consume the Make-owned root");
console.log("build configuration contract: passed");
