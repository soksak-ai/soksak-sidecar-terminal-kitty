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
if (/\bpath\s*=\s*"\.\.\//.test(cargo)) throw new Error("Cargo dependencies must not require sibling checkouts");
requireText("ref: 5407f266e65ddfc26e43c0ca6690e57ba0b6ff6a", "terminal sidecar kit commit");
requireText("ref: cab0691a1a01fca7436ac29f6cc2850245788ea6", "terminal contract commit");
requireText("ref: 418d6064fcdc5885be1ff73fd898fd7a0f778a0f", "platform spec commit");
requireText("ref: c222bdf6934b59f4eedea1254850479bd56cb62a", "Kitty SDK commit");
requireText(`path: ${ownerPath}`, "owner checkout path");
requireText(`working-directory: ${ownerPath}`, "owner working directory");
requireText(`${ownerPath}/\${{ steps.archive.outputs.asset }}`, "artifact upload path");
requireText(`working-directory: ${ownerPath}/.dependency/soksak-spec`, "validator build directory");
requireText("./scripts/package-release.sh", "reusable archive command");
requireText("python3 setup.py kitty-provider-sdk", "Kitty SDK build");
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
if (workflow.includes("windows") || workflow.includes("pc-windows")) throw new Error("Kitty 0.0.3 release must not declare Windows");
console.log("release workflow contract: passed");
