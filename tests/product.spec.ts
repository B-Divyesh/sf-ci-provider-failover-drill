import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const binary = join(process.cwd(), "target/debug/cifail");
const sample = join(process.cwd(), "examples/sample-repo");
const image = "node:22-bookworm@sha256:8a34c4ab3ea2c5cd194f07e317b2a8f09461d3c8b05c4e34c8ccd56d56024c4d";

function runPacket(extra: string[] = []) {
  const output = mkdtempSync(join(tmpdir(), "cifail-claim-"));
  const args = ["drill", "--workflow", join(sample, ".github/workflows/release.yml"), "--job", "release-check", "--image", image, "--repo", sample, "--out", output, "--json", ...extra];
  const result = execFileSync(binary, args, { encoding: "utf8" });
  return { output, report: JSON.parse(result) };
}

test("@claim:packet-generation creates a portable five-file packet", () => {
  const { output, report } = runPacket();
  for (const file of ["Dockerfile", "run.sh", ".env.example", "drill.json", "report.md"]) {
    expect(existsSync(join(output, file))).toBe(true);
  }
  expect(report.ready).toBe(true);
  expect(readFileSync(join(output, "Dockerfile"), "utf8")).toContain(`FROM ${image}`);
});

test("@claim:release-safety blocks publish commands by default", () => {
  const { output, report } = runPacket();
  expect(report.commands_blocked).toBe(1);
  expect(readFileSync(join(output, "run.sh"), "utf8")).not.toContain("npm publish");
  expect(readFileSync(join(output, "report.md"), "utf8")).toContain("Publish package");
});

test("@claim:secret-redaction removes secret names and values", () => {
  const { output, report } = runPacket();
  const files = ["run.sh", ".env.example", "drill.json", "report.md"].map((file) => readFileSync(join(output, file), "utf8")).join("\n");
  expect(report.secret_inputs).toBe(1);
  expect(files).not.toContain("NPM_TOKEN");
  expect(files).toContain("DRILL_SECRET_1");
});

test("@claim:offline-generation analyzes local files without network", () => {
  const output = mkdtempSync(join(tmpdir(), "cifail-offline-"));
  const stdout = execFileSync(binary, ["drill", "--workflow", join(sample, ".github/workflows/release.yml"), "--job", "release-check", "--image", image, "--repo", sample, "--out", output, "--json"], {
    encoding: "utf8",
    env: { ...process.env, HTTP_PROXY: "http://127.0.0.1:1", HTTPS_PROXY: "http://127.0.0.1:1", NO_PROXY: "" }
  });
  expect(JSON.parse(stdout).ready).toBe(true);
});

test("@claim:demo-sandbox opens sample data and stores no demo records", async ({ page }) => {
  await page.goto("/demo");
  await expect(page.getByText("Demo — sample data, nothing is saved", { exact: true })).toBeVisible();
  await expect(page.getByText("3 included", { exact: true })).toBeVisible();
  await page.getByRole("button", { name: "Reset demo" }).click();
  const demoKeys = await page.evaluate(() => Object.keys(localStorage).filter((key) => key.startsWith("demo:")));
  expect(demoKeys).toEqual([]);
});

test("@claim:privacy-local demo flow makes only same-origin requests", async ({ page }) => {
  const origins = new Set<string>();
  page.on("request", (request) => origins.add(new URL(request.url()).origin));
  await page.goto("/demo");
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  expect([...origins]).toEqual(["http://127.0.0.1:4173"]);
});

test("@claim:paid-license verifies a returned Team token", async ({ page }) => {
  await page.route("https://api.sociobot.in/**", (route) => route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify({ valid: true, reason: "ok", expires_at: null }) }));
  await page.goto("/team?license=claim-test-token");
  await expect(page.getByRole("heading", { name: "Local organization log" })).toBeVisible();
  expect(page.url()).not.toContain("license=");
  expect(await page.evaluate(() => localStorage.getItem("sb_license:ci-provider-failover-drill"))).toBe("claim-test-token");
});

test("@claim:team-history saves reports locally and downloads a template", async ({ page }) => {
  await page.goto("/team");
  await page.evaluate(() => localStorage.setItem("sb_license_status:ci-provider-failover-drill", JSON.stringify({ valid: true, reason: "ok", checkedAt: Date.now() })));
  await page.reload();
  await page.getByRole("button", { name: "Save drill report" }).click();
  await expect(page.getByText("1 saved drill report")).toBeVisible();
  const downloadPromise = page.waitForEvent("download");
  await page.getByRole("button", { name: "Download report template" }).click();
  expect((await downloadPromise).suggestedFilename()).toBe("organization-failover-report.md");
});

test("@claim:paid-contract shows the price and free-core boundary", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("$49", { exact: true })).toBeVisible();
  await expect(page.locator(".price")).toContainText("one-time purchase");
  await expect(page.getByText("The free CLI keeps packet export and safety checks.")).toBeVisible();
  await expect(page.getByRole("link", { name: /Buy Team for \$49/ })).toHaveAttribute("href", "https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout");
});

test("landing and legal routes pass an accessibility smoke test", async ({ page }) => {
  for (const route of ["/", "/demo", "/team", "/privacy", "/terms", "/missing-place"]) {
    await page.goto(route);
    await expect(page.locator("main")).toHaveCount(1);
    await expect(page.locator("h1")).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact || ""))).toEqual([]);
  }
});

test("mobile first screen keeps its action and facts visible", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  await expect(page.getByRole("link", { name: "Try it with sample data" })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
});
