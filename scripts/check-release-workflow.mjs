#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const workflow = fs.readFileSync(path.join(ROOT, ".github/workflows/release.yml"), "utf8");
const manifest = JSON.parse(fs.readFileSync(path.join(ROOT, "sidecar.json"), "utf8"));
const ownerPath = `soksak-sidecars/${manifest.id}`;
const targets = JSON.parse(fs.readFileSync(path.join(ROOT, "release/targets.json"), "utf8"));
const requireText = (value, label) => { if (!workflow.includes(value)) throw new Error(`release workflow is missing ${label}: ${value}`); };
const cargo = fs.readFileSync(path.join(ROOT, "Cargo.toml"), "utf8");
if (!/^edition = "2024"$/m.test(cargo)) throw new Error("Rust packages must use edition 2024");
if (/\bpath\s*=\s*"\.\.\//.test(cargo)) throw new Error("Cargo dependencies must not require sibling checkouts");
if (!cargo.includes('rev = "2b7d7ee5855a2dbef4507da44c347ad4fd74e552"')) throw new Error("Cargo must pin the terminal sidecar kit commit");
if (!cargo.includes('rev = "cab0691a1a01fca7436ac29f6cc2850245788ea6"')) throw new Error("Cargo must pin the terminal contract commit");
requireText("ref: 4c83e41a0aa168bc4c2e11100aba242277c731b6", "platform spec commit");
requireText("package_json_file:", "validator-owned pnpm version");
requireText("node-version-file:", "validator-owned Node version");
if (/path:\s+soksak-(?:kits|contracts)\//.test(workflow)) throw new Error("Cargo dependencies must not be staged as sibling repositories");
if (/node-version:\s*["']?\d/.test(workflow)) throw new Error("release workflow must not hardcode Node");
if (/^\s+version:\s*["']?\d/m.test(workflow) || workflow.includes('with: { version: "')) throw new Error("release workflow must not hardcode pnpm");
requireText("ref: c222bdf6934b59f4eedea1254850479bd56cb62a", "Kitty SDK commit");
requireText(`path: ${ownerPath}`, "owner checkout path");
requireText(`working-directory: ${ownerPath}`, "owner working directory");
requireText(`${ownerPath}/\${{ steps.archive.outputs.asset }}`, "artifact upload path");
requireText(`working-directory: ${ownerPath}/.dependency/soksak-spec`, "validator build directory");
requireText("./scripts/package-release.sh", "reusable archive command");
requireText("python3 setup.py kitty-provider-sdk", "Kitty SDK build");
requireText("if brew tap | grep -Fxq aws/tap; then brew untap aws/tap; fi", "unrelated Homebrew tap removal");
requireText(`soksak-sidecar-terminal-kitty-$version-\${{ matrix.target }}`, "manifest-derived archive version");
requireText('--tag "v$version"', "manifest-derived release tag");
for (const obsolete of ["release/source-dependencies.json", "release/dependencies.json"]) {
  if (fs.existsSync(path.join(ROOT, obsolete))) throw new Error(`${obsolete} is obsolete`);
}
for (const { target, runner } of targets) { requireText(`target: ${target}`, "release target"); requireText(`runner: ${runner}`, "release runner"); }
requireText("release-template/sidecar/build-release.mjs", "canonical release builder");
requireText("release-template/sidecar/validate-with-spec.mjs", "canonical release validator");
requireText("release-template/publish-canonical-release.mjs", "canonical immutable publisher");
requireText("SOKSAK_RELEASE_TOKEN: ${{ steps.release-token.outputs.token }}", "canonical publisher token");
for (const duplicate of ["build-release.mjs", "release-contract.mjs", "validate-with-spec.mjs"]) if (fs.existsSync(path.join(ROOT, "scripts", duplicate))) throw new Error(`local spec copy is forbidden: scripts/${duplicate}`);
if (fs.existsSync(path.join(ROOT, "validation/spec-validator.json"))) throw new Error("local spec pin copy is forbidden");
if (workflow.includes("windows") || workflow.includes("pc-windows")) throw new Error("Kitty release must not declare Windows");
console.log("release workflow contract: passed");
