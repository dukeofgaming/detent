import { readFileSync, writeFileSync, mkdirSync, readdirSync, statSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { spawnSync } from "node:child_process";

const args = process.argv.slice(2);
let input = "target/cucumber-report";
let htmlPath = "target/cucumber-report/index.html";
let runTests = true;

if (args.includes("--no-test")) {
  runTests = false;
}

const positional = args.filter((a) => a !== "--no-test");
if (positional.length === 2) {
  [input, htmlPath] = positional;
  runTests = false;
} else if (positional.length !== 0) {
  console.error("Usage: node scripts/cucumber-report.mjs [--no-test] [<input-dir-or-file> <output.html>]");
  process.exit(1);
}

if (runTests) {
  const testRun = spawnSync("cargo", ["test", "--test", "feature_slices"], {
    stdio: "inherit",
  });
  if (testRun.status !== 0) {
    process.exit(testRun.status ?? 1);
  }
}

const absInput = resolve(input);
const features = [];
if (statSync(absInput).isDirectory()) {
  const files = readdirSync(absInput).filter((f) => f.endsWith(".json"));
  for (const file of files) {
    const data = JSON.parse(readFileSync(join(absInput, file), "utf-8"));
    features.push(...data);
  }
  console.log(`Merged ${files.length} JSON files`);
} else {
  features.push(...JSON.parse(readFileSync(absInput, "utf-8")));
}
const total = { passed: 0, failed: 0, skipped: 0, undefined: 0 };
let rows = "";

for (const feature of features) {
  for (const scenario of feature.elements || []) {
    for (const step of scenario.steps || []) {
      const status = step.result?.status || "undefined";
      total[status] = (total[status] || 0) + 1;
    }
  }

  const scenarioRows = (feature.elements || [])
    .map((s) => {
      const stepRows = (s.steps || [])
        .map(
          (st) =>
            `<tr><td class="status-${st.result?.status ?? "undefined"}">${st.result?.status ?? "undefined"}</td>` +
            `<td>${st.name}</td>` +
            `<td>${st.result?.duration ? `${(st.result.duration / 1_000_000).toFixed(2)}ms` : "-"}</td></tr>`
        )
        .join("\n");

      const scenarioStatus = (s.steps || []).some((st) => st.result?.status === "failed")
        ? "failed"
        : (s.steps || []).every((st) => st.result?.status === "passed")
          ? "passed"
          : "skipped";

      return `<details class="scenario-${scenarioStatus}">` +
        `<summary><span class="badge badge-${scenarioStatus}">${scenarioStatus}</span> ${s.name}</summary>` +
        `<table><thead><tr><th>Status</th><th>Step</th><th>Duration</th></tr></thead><tbody>${stepRows}</tbody></table></details>`;
    })
    .join("\n");

  rows += `<section><h2>${feature.name}</h2>${feature.description ? `<p>${feature.description}</p>` : ""}${scenarioRows}</section>`;
}

const totalAll = total.passed + total.failed + total.skipped + total.undefined;

const html = `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Cucumber Report</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0d1117; color: #c9d1d9; padding: 20px; }
  h1 { font-size: 1.5rem; margin-bottom: 16px; }
  h2 { font-size: 1.2rem; margin: 16px 0 8px; }
  .summary { display: flex; gap: 12px; margin-bottom: 20px; flex-wrap: wrap; }
  .summary-item { padding: 8px 16px; border-radius: 6px; font-size: 0.9rem; font-weight: 600; }
  .bg-passed { background: #238636; color: #fff; }
  .bg-failed { background: #da3633; color: #fff; }
  .bg-skipped { background: #1f6feb; color: #fff; }
  .bg-undefined { background: #7a8288; color: #fff; }
  details { border: 1px solid #30363d; border-radius: 6px; padding: 8px 12px; margin: 8px 0; }
  summary { cursor: pointer; font-weight: 600; }
  summary .badge { display: inline-block; padding: 2px 8px; border-radius: 10px; font-size: 0.75rem; font-weight: 700; text-transform: uppercase; margin-right: 6px; }
  .badge-passed { background: #238636; color: #fff; }
  .badge-failed { background: #da3633; color: #fff; }
  .badge-skipped { background: #1f6feb; color: #fff; }
  .scenario-failed { border-color: #da3633; }
  table { width: 100%; border-collapse: collapse; margin-top: 8px; font-size: 0.85rem; }
  th, td { text-align: left; padding: 4px 8px; border-bottom: 1px solid #21262d; }
  th { color: #8b949e; font-weight: 600; }
  .status-passed { color: #3fb950; }
  .status-failed { color: #f85149; }
  .status-skipped { color: #58a6ff; }
  .status-undefined { color: #8b949e; }
  section { margin-bottom: 20px; }
</style>
</head>
<body>
<h1>Cucumber Report</h1>
<div class="summary">
  <div class="summary-item bg-passed">Passed: ${total.passed}</div>
  <div class="summary-item bg-failed">Failed: ${total.failed}</div>
  <div class="summary-item bg-skipped">Skipped: ${total.skipped}</div>
  <div class="summary-item bg-undefined">Undefined: ${total.undefined}</div>
  <div class="summary-item" style="background:#30363d;color:#c9d1d9;">Total: ${totalAll}</div>
</div>
${rows}
</body>
</html>`;

mkdirSync(dirname(resolve(htmlPath)), { recursive: true });
writeFileSync(resolve(htmlPath), html);
console.log(`Report written to ${htmlPath}`);
