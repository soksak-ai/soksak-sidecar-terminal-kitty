#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const workflow = fs.readFileSync(path.join(ROOT, ".github/workflows/release.yml"), "utf8");
const sources = JSON.parse(fs.readFileSync(path.join(ROOT, "release/source-dependencies.json"), "utf8"));
const targets = JSON.parse(fs.readFileSync(path.join(ROOT, "release/targets.json"), "utf8"));
const requireText = (value, label) => { if (!workflow.includes(value)) throw new Error(`release workflow is missing ${label}: ${value}`); };
requireText(`path: ${sources.ownerPath}`, "owner checkout path");
requireText(`working-directory: ${sources.ownerPath}`, "owner working directory");
requireText(`${sources.ownerPath}/\${{ steps.archive.outputs.asset }}`, "artifact upload path");
requireText(`working-directory: ${sources.ownerPath}/.dependency/soksak-spec`, "validator build directory");
requireText("./scripts/package-release.sh", "reusable archive command");
requireText("python3 setup.py kitty-provider-sdk", "Kitty SDK build");
for (const dependency of sources.dependencies) {
  if (!/^[0-9a-f]{40}$/.test(dependency.commit)) throw new Error(`dependency commit must be exact: ${dependency.repository}`);
  requireText(`repository: ${dependency.repository}`, "dependency repository"); requireText(`ref: ${dependency.commit}`, "dependency commit"); requireText(`path: ${dependency.path}`, "dependency checkout path");
}
for (const { target, runner } of targets) { requireText(`target: ${target}`, "release target"); requireText(`runner: ${runner}`, "release runner"); }
requireText("release-template/sidecar/build-release.mjs", "canonical release builder");
requireText("release-template/sidecar/validate-with-spec.mjs", "canonical release validator");
for (const duplicate of ["build-release.mjs", "release-contract.mjs", "validate-with-spec.mjs"]) if (fs.existsSync(path.join(ROOT, "scripts", duplicate))) throw new Error(`local spec copy is forbidden: scripts/${duplicate}`);
if (fs.existsSync(path.join(ROOT, "validation/spec-validator.json"))) throw new Error("local spec pin copy is forbidden");
if (workflow.includes("windows") || workflow.includes("pc-windows")) throw new Error("Kitty 0.0.1 release must not declare Windows");
console.log("release workflow contract: passed");
