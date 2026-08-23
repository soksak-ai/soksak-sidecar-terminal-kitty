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
requireText("https://github.com/soksak-ai/soksak-spec/releases/download/v0.0.19/soksak-ai-plugin-spec-0.0.19.tgz", "immutable spec package");
requireText("928c0e6cc12d5500bedd386c1807003bf7c3dd34eed836b4a978d8b16d9e5b7b", "spec package digest");
requireText('node-version: "26.7.0"', "exact Node version");
requireText("--spec-package .dependency/spec-package", "package validator input");
if (/path:\s+soksak-(?:kits|contracts)\//.test(workflow)) throw new Error("Cargo dependencies must not be staged as sibling repositories");
if (workflow.includes("repository: soksak-ai/soksak-spec")) throw new Error("release workflow must not checkout the spec source");
if (workflow.includes("pnpm/action-setup")) throw new Error("release workflow must not rebuild the spec package");
requireText("ref: c222bdf6934b59f4eedea1254850479bd56cb62a", "Kitty SDK commit");
requireText(`path: ${ownerPath}`, "owner checkout path");
requireText(`working-directory: ${ownerPath}`, "owner working directory");
requireText(`${ownerPath}/\${{ steps.archive.outputs.asset }}`, "artifact upload path");
requireText(".dependency/spec-package/release-template/", "immutable package tools");
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
