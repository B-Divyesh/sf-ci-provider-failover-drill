import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { chmodSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
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

test("@claim:runner-contract writes a Docker runner contract for the selected image", () => {
  const { output } = runPacket();
  expect(readFileSync(join(output, "Dockerfile"), "utf8")).toContain(`FROM ${image}`);
  const report = readFileSync(join(output, "report.md"), "utf8");
  expect(report).toContain("Any runner needs Docker");
  expect(report).toContain("checkout mounted at `/workspace`");
});

test("@claim:inspection-report records required files, network hosts, and provider actions", () => {
  const { output, report } = runPacket();
  expect(report.required_files).toEqual(expect.arrayContaining(["package.json", "package-lock.json"]));
  expect(report.network_hosts).toContain("registry.npmjs.org");
  expect(report.provider_assumptions.join("\n")).toContain("actions/checkout is replaced by the mounted repository");
  const markdown = readFileSync(join(output, "report.md"), "utf8");
  expect(markdown).toContain("### Required paths");
  expect(markdown).toContain("### Network hosts");
  expect(markdown).toContain("### Provider assumptions");
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

test("@claim:license-verdict-cache reuses only a matching verdict for one day", async ({ context, page }) => {
  let requests = 0;
  await context.addInitScript(() => {
    localStorage.setItem("sb_license:ci-provider-failover-drill", "cached-token");
    localStorage.setItem("sb_license_status:ci-provider-failover-drill", JSON.stringify({ valid: true, reason: "ok", checkedAt: Date.now() - 60_000, token: "cached-token" }));
  });
  await page.route("https://api.sociobot.in/**", async (route) => {
    requests += 1;
    await route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify({ valid: true, reason: "ok" }) });
  });
  await page.goto("/team");
  await expect(page.getByRole("heading", { name: "Local organization log" })).toBeVisible();
  expect(requests).toBe(0);
});

test("replacing an invalid license token always verifies the replacement", async ({ page }) => {
  const requests: string[] = [];
  await page.route("https://api.sociobot.in/**", async (route) => {
    const token = new URL(route.request().url()).searchParams.get("license") || "";
    requests.push(token);
    await route.fulfill({ status: 200, contentType: "application/json", body: JSON.stringify({ valid: token === "replacement-token", reason: token === "replacement-token" ? "ok" : "invalid" }) });
  });
  await page.goto("/team?license=bad-token");
  await expect(page.getByText("This license is no longer active. Check the token or buy Team.")).toBeVisible();
  await page.getByLabel("License token").fill("replacement-token");
  await page.getByRole("button", { name: "Verify license" }).click();
  await expect(page.getByRole("heading", { name: "Local organization log" })).toBeVisible();
  expect(requests).toEqual(["bad-token", "replacement-token"]);
});

test("@claim:team-history saves reports locally and downloads a template", async ({ page }) => {
  await page.goto("/team");
  await page.evaluate(() => {
    localStorage.setItem("sb_license:ci-provider-failover-drill", "history-token");
    localStorage.setItem("sb_license_status:ci-provider-failover-drill", JSON.stringify({ valid: true, reason: "ok", checkedAt: Date.now(), token: "history-token" }));
  });
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

test("@claim:exit-codes assigns input, safety, and execution failures documented codes", () => {
  const exitCode = (args: string[], env?: NodeJS.ProcessEnv) => {
    try {
      execFileSync(binary, args, { encoding: "utf8", env });
      return 0;
    } catch (error) {
      return (error as { status?: number }).status;
    }
  };
  expect(exitCode(["drill", "--workflow", join(sample, ".github/workflows/release.yml"), "--job", "not-a-job", "--image", image, "--repo", sample])).toBe(2);

  const root = mkdtempSync(join(tmpdir(), "cifail-exit-codes-"));
  const incomplete = join(root, "incomplete");
  cpSync(sample, incomplete, { recursive: true });
  rmSync(join(incomplete, "package.json"));
  expect(exitCode(["drill", "--workflow", join(incomplete, ".github/workflows/release.yml"), "--job", "release-check", "--image", image, "--repo", incomplete, "--out", join(root, "safety"), "--execute"])).toBe(3);

  const fakeBin = join(root, "bin");
  mkdirSync(fakeBin);
  const fakeDocker = join(fakeBin, "docker");
  writeFileSync(fakeDocker, "#!/bin/sh\n[ \"$1\" = \"--version\" ] && exit 0\nexit 1\n");
  chmodSync(fakeDocker, 0o755);
  expect(exitCode(["drill", "--workflow", join(sample, ".github/workflows/release.yml"), "--job", "release-check", "--image", image, "--repo", sample, "--out", join(root, "execution"), "--execute"], { ...process.env, PATH: `${fakeBin}:${process.env.PATH}` })).toBe(4);
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

test("dark mode has no serious or critical accessibility issues", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await page.goto("/");
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact || ""))).toEqual([]);
});

test("mobile first screen keeps its action and facts visible", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  await expect(page.getByRole("link", { name: "Try it with sample data" })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
});

test("mobile demo has no horizontal overflow and reachable touch targets", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/demo");
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  const undersized = await page.locator("a, button").evaluateAll((elements) => elements
    .filter((element) => {
      const box = element.getBoundingClientRect();
      return box.width < 44 || box.height < 44;
    })
    .map((element) => ({ label: (element.textContent || "").trim(), width: element.getBoundingClientRect().width, height: element.getBoundingClientRect().height })));
  expect(undersized).toEqual([]);
});

test("unknown routes return a real 404 response", async ({ page }) => {
  const response = await page.goto("/missing-place");
  expect(response?.status()).toBe(404);
  await expect(page.getByRole("heading", { name: "This route is not on the map." })).toBeVisible();
});

test("static deployment keeps routes real and hashed assets immutable", () => {
  const config = JSON.parse(readFileSync(join(process.cwd(), "site/public/staticwebapp.config.json"), "utf8"));
  expect(config.navigationFallback).toBeUndefined();
  expect(config.responseOverrides["404"].rewrite).toBe("/404.html");
  expect(config.routes).toContainEqual({ route: "/assets/*", headers: { "Cache-Control": "public, max-age=31536000, immutable" } });
  for (const route of ["demo", "team", "privacy", "terms"]) {
    expect(existsSync(join(process.cwd(), "dist/site", route, "index.html"))).toBe(true);
  }
});
