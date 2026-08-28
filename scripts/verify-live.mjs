import { chromium, request } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const base = (process.argv[2] || "https://ci-provider-failover-drill.sociobot.in").replace(/\/$/, "");
const evidence = resolve(process.argv[3] || ".factory/qa-evidence/polish-2");
await mkdir(evidence, { recursive: true });

function check(condition, message) {
  if (!condition) throw new Error(message);
}

const expected = {
  "/": "CI Provider Failover Drill — test one job",
  "/demo": "Demo — CI Provider Failover Drill",
  "/team": "Team tools — CI Provider Failover Drill",
  "/privacy": "Privacy — CI Provider Failover Drill",
  "/terms": "Terms — CI Provider Failover Drill",
  "/missing-place": "Page not found — CI Provider Failover Drill"
};
const report = { base, checkedAt: new Date().toISOString(), routes: {}, demo: {}, mobile: {}, navigation: {}, links: {}, errors: [] };
const browser = await chromium.launch();

for (const [route, title] of Object.entries(expected)) {
  const context = await browser.newContext();
  const page = await context.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  const response = await page.goto(`${base}${route}?cold=${Date.now()}`, { waitUntil: "networkidle" });
  const unexpectedErrors = route === "/missing-place"
    ? errors.filter((message) => !message.includes("server responded with a status of 404"))
    : errors;
  const axe = await new AxeBuilder({ page }).analyze();
  const serious = axe.violations.filter((item) => ["serious", "critical"].includes(item.impact || ""));
  const facts = await page.evaluate(() => ({
    title: document.title,
    lang: document.documentElement.lang,
    main: document.querySelectorAll("main").length,
    h1: document.querySelectorAll("h1").length,
    canonical: document.querySelector('link[rel="canonical"]')?.getAttribute("href"),
    description: document.querySelector('meta[name="description"]')?.getAttribute("content"),
    overflow: document.documentElement.scrollWidth > window.innerWidth
  }));
  check(response?.status() === (route === "/missing-place" ? 404 : 200), `${route} returned ${response?.status()}`);
  check(facts.title === title, `${route} title mismatch: ${facts.title}`);
  check(facts.lang === "en" && facts.main === 1 && facts.h1 === 1, `${route} semantic structure failed`);
  check(Boolean(facts.canonical && facts.description), `${route} metadata missing`);
  check(!facts.overflow && unexpectedErrors.length === 0 && serious.length === 0, `${route} browser or accessibility errors: ${JSON.stringify({ overflow: facts.overflow, errors: unexpectedErrors, serious: serious.map((item) => item.id) })}`);
  report.routes[route] = { status: response?.status(), ...facts, consoleErrors: unexpectedErrors, expected404ConsoleSignal: route === "/missing-place" && errors.length > 0, seriousAxeViolations: serious.length };
  await context.close();
}

{
  const api = await request.newContext();
  report.rawMetadata = {};
  for (const [route, title] of Object.entries(expected)) {
    const response = await api.get(`${base}${route}`);
    const html = await response.text();
    const canonicalPath = route === "/missing-place" ? "/404" : route;
    const titleEscaped = title.replaceAll("&", "&amp;");
    const checks = {
      title: html.includes(`<title>${titleEscaped}</title>`),
      canonical: html.includes(`<link rel="canonical" href="${base}${canonicalPath}"`),
      openGraph: html.includes(`<meta property="og:title" content="${titleEscaped}"`),
      twitter: html.includes(`<meta name="twitter:title" content="${titleEscaped}"`),
      cspHeader: Boolean(response.headers()["content-security-policy"]),
      noSniff: response.headers()["x-content-type-options"] === "nosniff"
    };
    check(Object.values(checks).every(Boolean), `${route} raw metadata or security headers failed: ${JSON.stringify(checks)}`);
    report.rawMetadata[route] = checks;
  }
  await api.dispose();
}

{
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const origins = new Set();
  page.on("request", (req) => origins.add(new URL(req.url()).origin));
  await page.goto(`${base}/?demo=1`, { waitUntil: "networkidle" });
  check(await page.getByText("Demo — sample data, nothing is saved", { exact: true }).isVisible(), "query demo banner missing");
  check(await page.getByText("3 included", { exact: true }).isVisible(), "query demo sample missing");
  await page.getByRole("button", { name: "Reset demo" }).click();
  const demoKeys = await page.evaluate(() => Object.keys(localStorage).filter((key) => key.startsWith("demo:")));
  const controls = await page.locator(".demo-banner button").evaluateAll((nodes) => nodes.map((node) => {
    const box = node.getBoundingClientRect();
    return { label: node.textContent?.trim(), width: box.width, height: box.height };
  }));
  check(demoKeys.length === 0, "query demo wrote demo storage");
  check(controls.every((item) => item.width >= 44 && item.height >= 44), "query demo has a small touch target");
  check([...origins].every((origin) => origin === new URL(base).origin), "query demo contacted another origin");
  await page.screenshot({ path: resolve(evidence, "live-demo-mobile-390.png"), fullPage: true });
  report.demo = { banner: true, sample: "3 included / 1 blocked / 1 anonymous", demoKeys, controls, requestOrigins: [...origins] };
  await context.close();
}

{
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  await page.goto(`${base}/?cold=${Date.now()}`, { waitUntil: "networkidle" });
  const firstScreen = {};
  for (const label of ["Try it with sample data", "Free local drill", "No secrets stored", "Release steps stay blocked"]) {
    const box = await page.getByText(label, { exact: true }).boundingBox();
    check(Boolean(box && box.y >= 0 && box.y + box.height <= 844), `${label} is outside the mobile first screen`);
    firstScreen[label] = box;
  }
  await page.screenshot({ path: resolve(evidence, "live-landing-mobile-390.png"), fullPage: false });
  report.mobile = { viewport: "390x844", firstScreen, overflow: await page.evaluate(() => document.documentElement.scrollWidth > innerWidth) };
  check(!report.mobile.overflow, "mobile landing overflows horizontally");
  await context.close();
}

{
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(base, { waitUntil: "networkidle" });
  await page.getByRole("link", { name: "Demo", exact: true }).click();
  const demoFocused = await page.evaluate(() => document.activeElement === document.querySelector("h1"));
  await page.goBack();
  const homeFocused = await page.evaluate(() => document.activeElement === document.querySelector("h1"));
  check(demoFocused && homeFocused, "route focus was not restored to h1");
  report.navigation = { demoFocused, backFocused: homeFocused, backPath: new URL(page.url()).pathname };
  await context.close();
}

{
  const api = await request.newContext({ maxRedirects: 0 });
  const terms = await api.get("https://sociobot.in/terms");
  const checkout = await api.get("https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout");
  check(terms.status() === 200, `Sociobot terms returned ${terms.status()}`);
  check([301, 302, 303, 307, 308].includes(checkout.status()), `checkout returned ${checkout.status()}`);
  report.links = { sociobotTerms: terms.status(), checkout: checkout.status() };
  await api.dispose();
}

{
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(`${base}/terms?cold=${Date.now()}`, { waitUntil: "networkidle" });
  const text = await page.locator("main").innerText();
  check(text.includes("It covers the browser tools shown on the Team page."), "replacement Team scope is absent");
  check(!text.toLowerCase().includes("future v1 updates"), "future-update promise remains live");
  await page.screenshot({ path: resolve(evidence, "live-terms-desktop.png"), fullPage: true });
  report.terms = { replacementScope: true, futurePromiseAbsent: true };
  await context.close();
}

await browser.close();
await writeFile(resolve(evidence, "live-route-check.json"), `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify(report, null, 2));
