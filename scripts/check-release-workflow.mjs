#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (name) => fs.readFileSync(path.join(root, name), "utf8");
const workflow = read(".github/workflows/release.yml");
const manifest = JSON.parse(read("sidecar.json"));
if (manifest.processRole !== "sidecar-terminal-kitty") throw new Error("Sidecar manifest must declare its project-independent processRole");
const dependency = JSON.parse(read("build-dependencies.json")).dependencies[0];
const targets = JSON.parse(read("release/targets.json"));
const makefile = read("Makefile");
const gate = read("scripts/gate.sh");
const stage = read("scripts/stage-built.sh");
const ownerPath = `soksak-sidecars/${manifest.id}`;
const requireText = (value, label) => { if (!workflow.includes(value)) throw new Error(`release workflow is missing ${label}: ${value}`); };
if (!/^lock: preflight$/m.test(makefile) || !makefile.includes("cargo metadata --format-version 1")) throw new Error("Makefile must own Cargo lock regeneration");
if (!read("README.md").includes("make lock TARGET=")) throw new Error("README must document the owner lock target");
for (const target of ["require-tooling", "require-out", "release", "attest"]) if (!new RegExp(`^${target}:`, "m").test(makefile)) throw new Error(`Makefile target is missing: ${target}`);
if (!/^STAGE \?= dist$/m.test(makefile) || /^OUT \?= dist$/m.test(makefile)) throw new Error("Makefile must separate STAGE from release OUT");
for (const value of ["command -v soksak-sdk", "SDK_VERSION", "soksak-sdk pack-target", "soksak-sdk package", "soksak-sdk attest"]) if (!makefile.includes(value)) throw new Error(`Makefile release boundary is missing: ${value}`);
if (!makefile.includes('chmod -R u+w "$$work"')) throw new Error("release cleanup must reclaim its read-only SDK copy");
if (!read("README.md").includes("make attest TARGET=") || !read("README.md").includes("OUT=/absolute/")) throw new Error("README must document owner attestation");
for (const value of [dependency.repository, dependency.commit, dependency.tools.python]) {
  if (workflow.includes(value)) throw new Error("workflow duplicates build-dependencies.json metadata");
}
for (const value of ["sdk_archive_url:", "sdk_archive_sha256:", "sdk_release_url:", "sdk_release_sha256:", "${{ inputs.sdk_archive_url }}", "${{ inputs.sdk_release_url }}", "$RUNNER_TEMP/soksak-sdk", "soksak-sdk prepare", ".dependencies/soksak-spec/release-template"]) requireText(value, "release-train input");
requireText("actions/setup-python@a309ff8b426b58ec0e2a45f0f869d46889d02405", "pinned Python installer");
requireText("python-version: ${{ steps.build-dependency.outputs.python }}", "manifest-owned Python version");
requireText('make verify TARGET="${{ matrix.target }}"', "owner Make verification");
requireText('make stage TARGET="${{ matrix.target }}" STAGE=dist', "owner Make staging");
requireText("build-dependency-receipt.json", "SDK receipt in the archive");
requireText("release-template/sidecar/pack-target.mjs", "canonical target packer");
requireText(`path: ${ownerPath}`, "owner checkout path");
requireText(`working-directory: ${ownerPath}`, "owner working directory");
for (const { target, runner } of targets) { requireText(`target: ${target}`, "release target"); requireText(`runner: ${runner}`, "release runner"); }
for (const match of workflow.matchAll(/^\s*-?\s*uses:\s*([^\s#]+)/gm)) {
  if (!/^[^@\s]+@[a-f0-9]{40}$/.test(match[1])) throw new Error(`workflow action is not commit-pinned: ${match[1]}`);
}
for (const obsolete of ["repository: min-median-max", "stage.sh", "SOKSAK_KITTY_PROVIDER_SDK", "scripts/package-release.sh", "brew untap", "spec_url:", "spec_sha256:", ".dependency/spec-package"]) {
  if (workflow.includes(obsolete)) throw new Error(`workflow retains obsolete behavior: ${obsolete}`);
}
if (workflow.includes("windows") || workflow.includes("pc-windows")) throw new Error("Kitty release must not declare Windows");
if (!stage.includes("absolute candidate output")) throw new Error("stage-built does not permit isolated absolute output");
if (!/^benchmark:/m.test(makefile) || /--test bench/.test(gate)) throw new Error("benchmark ownership is not separated from verification");
console.log("release workflow contract: passed");
