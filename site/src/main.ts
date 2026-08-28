import "./style.css";

const PRODUCT = "ci-provider-failover-drill";
const API = "https://api.sociobot.in/api/v1";
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `sb_license_status:${PRODUCT}`;
const TEAM_DATA_KEY = "team:drills";

type Page = { title: string; description: string; body: string; demo?: boolean };
type Verdict = { valid: boolean; reason: string; checkedAt: number };

const routeMeta: Record<string, [string, string]> = {
  "/": ["CI Failover Drill — prove one job runs elsewhere", "Turn one GitHub Actions job into a safe, provider-neutral container drill."],
  "/demo": ["Demo — CI Provider Failover Drill", "See a sample release-check job become a safe failover packet."],
  "/team": ["Team tools — CI Provider Failover Drill", "Restore a Team license and keep local drill history."],
  "/privacy": ["Privacy — CI Provider Failover Drill", "How CI Provider Failover Drill handles workflows, reports, and licenses."],
  "/terms": ["Terms — CI Provider Failover Drill", "Terms for the free CLI and one-time Team license."],
  "/404": ["Page not found — CI Provider Failover Drill", "Return to the CI Provider Failover Drill map."]
};

function header(): string {
  return `
    <header class="site-header">
      <a class="wordmark" href="/" data-link aria-label="CI Provider Failover Drill home">
        <svg aria-hidden="true" viewBox="0 0 44 44"><path d="M5 31c6-16 11 2 18-11s10 7 16-6M5 37c8-12 13 2 21-10s8 3 13-3"/><circle cx="23" cy="20" r="3"/></svg>
        <span>CI / FAILOVER</span>
      </a>
      <nav aria-label="Main navigation">
        <a href="/demo" data-link>Demo</a>
        <a href="/#install" data-link>Install</a>
        <a href="/team" data-link>Team</a>
        <a href="/privacy" data-link>Privacy</a>
      </nav>
    </header>`;
}

function footer(): string {
  return `
    <footer class="site-footer">
      <p>Prove one GitHub Actions job can run elsewhere.</p>
      <div class="footer-links">
        <a href="/privacy" data-link>Privacy</a>
        <a href="/terms" data-link>Terms</a>
        <span>Built by Param Factory</span>
        <span>v0.1.0 · build 2026.08.28</span>
      </div>
    </footer>`;
}

function routeLine(): string {
  return `<svg class="route-line" viewBox="0 0 620 100" aria-hidden="true"><path d="M8 68C90 18 138 84 212 47S344 12 406 58s112 33 205-38"/><circle cx="212" cy="47" r="7"/><circle cx="406" cy="58" r="7"/></svg>`;
}

function landing(): Page {
  return {
    title: routeMeta["/"][0],
    description: routeMeta["/"][1],
    body: `
      <main id="main">
        <section class="hero survey-section" aria-labelledby="landing-title">
          <div class="coordinate" aria-hidden="true">N 37°46′ · W 122°25′</div>
          <div class="hero-copy">
            <p class="eyebrow">Outage route / field check 01</p>
            <h1 id="landing-title" tabindex="-1">Prove your CI escape route.</h1>
            <p class="lede">For GitHub Actions maintainers who need one critical job to run during an outage.</p>
            <div class="hero-action">
              <a class="button primary" href="/demo" data-link>Try it with sample data</a>
              <span>See a safe release-check packet in one click.</span>
            </div>
            <ul class="plain-facts" aria-label="Product facts">
              <li>Free local drill</li>
              <li>No secrets stored</li>
              <li>Release steps stay blocked</li>
            </ul>
          </div>
          <figure class="map-plate">
            <img src="/topographic-route.webp" width="1200" height="800" fetchpriority="high" alt="A mapped route crosses between two isolated CI provider regions." />
            <figcaption>Selected job → pinned runner → drill report</figcaption>
            ${routeLine()}
          </figure>
        </section>

        <section class="preview survey-section" aria-labelledby="preview-title">
          <div class="section-mark"><span>PLATE 02</span><span>KNOWN ROUTE</span></div>
          <div class="section-intro">
            <p class="eyebrow">The product</p>
            <h2 id="preview-title">See what the drill catches</h2>
            <p>One command reads one job. It produces a packet for Docker on another runner.</p>
          </div>
          <div class="terminal-wrap">
            <img src="/terminal-demo.svg" width="1080" height="580" alt="Terminal demo showing a ready packet and one blocked release step." />
            <div class="terminal-key" aria-label="Terminal demo summary">
              <span><b>3</b> shell steps included</span>
              <span><b>1</b> publish step blocked</span>
              <span><b>1</b> secret input anonymized</span>
            </div>
          </div>
        </section>

        <section class="steps survey-section" aria-labelledby="steps-title">
          <div class="section-mark"><span>ROUTE 03</span><span>THREE LEGS</span></div>
          <div class="section-intro">
            <p class="eyebrow">How it works</p>
            <h2 id="steps-title">Make the escape route repeatable</h2>
          </div>
          <ol class="route-steps">
            <li><span class="step-number">01</span><div><h3>Select one job</h3><p>Point the CLI at a workflow and job key.</p></div></li>
            <li><span class="step-number">02</span><div><h3>Inspect the packet</h3><p>Review files, network hosts, anonymous inputs, and skipped actions.</p></div></li>
            <li><span class="step-number">03</span><div><h3>Run it elsewhere</h3><p>Use the pinned container on a laptop or independent Docker runner.</p></div></li>
          </ol>
        </section>

        <section id="install" class="install survey-section" aria-labelledby="install-title">
          <div class="section-mark"><span>FIELD KIT</span><span>RUST 1.85+</span></div>
          <div class="section-intro">
            <p class="eyebrow">Install</p>
            <h2 id="install-title">Run the first drill</h2>
            <p>Build the single binary from this repository.</p>
          </div>
          <div class="command-block">
            <code tabindex="0">cargo install --git https://github.com/B-Divyesh/sf-ci-provider-failover-drill cifail</code>
            <button class="copy-button" data-copy="cargo install --git https://github.com/B-Divyesh/sf-ci-provider-failover-drill cifail">Copy install command</button>
          </div>
          <pre tabindex="0" aria-label="Example command"><code>cifail drill \\
  --workflow .github/workflows/release.yml \\
  --job release-check \\
  --image 'node:22-bookworm@sha256:&lt;digest&gt;'</code></pre>
        </section>

        <section class="limits survey-section" aria-labelledby="limits-title">
          <div class="section-intro">
            <p class="eyebrow">Boundaries</p>
            <h2 id="limits-title">Know what it does not do</h2>
          </div>
          <ul class="boundary-list">
            <li><strong>No automatic cutover.</strong> You decide where and when to run.</li>
            <li><strong>No secret storage.</strong> Reports use anonymous input labels.</li>
            <li><strong>No universal conversion.</strong> Provider actions become named assumptions.</li>
            <li><strong>No surprise releases.</strong> Publish commands need an explicit flag.</li>
          </ul>
        </section>

        <section class="pricing survey-section" aria-labelledby="pricing-title">
          <div class="price-copy">
            <p class="eyebrow">Team license</p>
            <h2 id="pricing-title">Keep a shared drill record</h2>
            <p>Team adds browser-only drill history and organization report templates.</p>
            <p class="price"><span>$49</span> one-time purchase</p>
            <p class="legal-note">Sociobot is the merchant of record. Refunds are handled there.</p>
          </div>
          <div class="price-actions">
            <a class="button primary" href="https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout">Buy Team for $49 <span class="sr-only">(opens hosted checkout)</span></a>
            <a class="text-link" href="/team" data-link>Restore a license</a>
            <p>The free CLI keeps packet export and safety checks.</p>
          </div>
        </section>
      </main>`
  };
}

function demo(): Page {
  return {
    title: routeMeta["/demo"][0],
    description: routeMeta["/demo"][1],
    demo: true,
    body: `
      <main id="main" class="inner-main">
        <section class="page-heading">
          <p class="eyebrow">Demo / sample repository</p>
          <h1 tabindex="-1">Inspect a safe failover packet.</h1>
          <p>The sample selects a realistic Node release-check job.</p>
        </section>
        <section class="demo-grid" aria-labelledby="demo-result">
          <div class="workflow-card">
            <div class="section-mark"><span>INPUT</span><span>release.yml</span></div>
            <pre tabindex="0"><code>release-check:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-node@v4
    - run: npm ci
    - run: npm test
    - env:
        NODE_AUTH_TOKEN: \${{ secrets.NPM_TOKEN }}
      run: npm whoami
    - run: npm publish</code></pre>
          </div>
          <div class="result-card">
            <div class="section-mark"><span>OUTPUT</span><span>READY</span></div>
            <h2 id="demo-result">The route is ready to inspect</h2>
            <dl class="result-list">
              <div><dt>Shell steps</dt><dd>3 included</dd></div>
              <div><dt>Release steps</dt><dd>1 blocked</dd></div>
              <div><dt>Secret inputs</dt><dd>1 anonymous</dd></div>
              <div><dt>Network</dt><dd>registry.npmjs.org</dd></div>
            </dl>
            <h3>Packet files</h3>
            <ul class="file-list"><li>Dockerfile</li><li>run.sh</li><li>.env.example</li><li>drill.json</li><li>report.md</li></ul>
          </div>
        </section>
        <section class="demo-command" aria-labelledby="demo-command-title">
          <h2 id="demo-command-title">Run the same sandbox locally</h2>
          <div class="command-block"><code tabindex="0">cargo run -- demo</code><button class="copy-button" data-copy="cargo run -- demo">Copy demo command</button></div>
          <p>The CLI writes the sample to a new temporary directory. It prints the packet path when done.</p>
        </section>
      </main>`
  };
}

function privacy(): Page {
  return {
    title: routeMeta["/privacy"][0], description: routeMeta["/privacy"][1], body: `
      <main id="main" class="prose-main">
        <p class="eyebrow">Policy / updated 28 August 2026</p>
        <h1 tabindex="-1">Your workflow stays on your machine.</h1>
        <p>The free CLI reads local workflow files and writes local packet files. It has no telemetry.</p>
        <h2>Demo data</h2>
        <p>The website demo uses a bundled sample. It does not upload or save your repository data.</p>
        <h2>Team data</h2>
        <p>Team drill history stays in this browser. You can delete it from the Team page.</p>
        <h2>License checks</h2>
        <p>A Team token is stored in this browser. It is sent only to the Sociobot license API for verification.</p>
        <p>The last verdict is cached for one day. The checkout site handles payment details.</p>
        <h2>Contact</h2>
        <p>Email <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a> for a privacy request.</p>
      </main>`
  };
}

function terms(): Page {
  return {
    title: routeMeta["/terms"][0], description: routeMeta["/terms"][1], body: `
      <main id="main" class="prose-main">
        <p class="eyebrow">Terms / updated 28 August 2026</p>
        <h1 tabindex="-1">Use the drill before an outage.</h1>
        <p>The CLI is provided under the MIT License. You remain responsible for commands you execute.</p>
        <h2>Release safety</h2>
        <p>Release commands stay blocked unless you pass the explicit release flag. Inspect every generated packet before execution.</p>
        <h2>Team purchase</h2>
        <p>Team costs $49 as a one-time purchase. It covers the current browser tools and future v1 updates.</p>
        <p>Sociobot is the merchant of record. Its checkout handles payment, receipts, and refunds.</p>
        <h2>Service limits</h2>
        <p>The drill reports assumptions but cannot guarantee another provider will stay available. Keep an independent runner and test it regularly.</p>
        <h2>Contact</h2>
        <p>Email <a href="mailto:support@sociobot.in">support@sociobot.in</a> for purchase help.</p>
      </main>`
  };
}

function team(): Page {
  const unlocked = getCachedVerdict()?.valid === true;
  return {
    title: routeMeta["/team"][0], description: routeMeta["/team"][1], body: `
      <main id="main" class="inner-main">
        <section class="page-heading">
          <p class="eyebrow">Team / local organization record</p>
          <h1 tabindex="-1">Keep drill results in one field log.</h1>
          <p>Team stores imported drill reports in this browser.</p>
        </section>
        <div id="license-notice" class="notice" aria-live="polite"></div>
        ${unlocked ? teamTools() : licensePanel()}
      </main>`
  };
}

function licensePanel(): string {
  return `<section class="license-panel" aria-labelledby="restore-title">
    <div><p class="eyebrow">One-time license</p><h2 id="restore-title">Restore Team on this device</h2><p>Paste the token from your receipt. Verification uses the Sociobot license API.</p></div>
    <form id="license-form">
      <label for="license-token">License token</label>
      <input id="license-token" name="license" autocomplete="off" required />
      <button class="button primary" type="submit">Verify license</button>
    </form>
    <a class="text-link" href="https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout">Buy Team for $49 <span class="sr-only">(opens hosted checkout)</span></a>
  </section>`;
}

function teamTools(): string {
  const count = readHistory().length;
  return `<section class="team-tools" aria-labelledby="team-tools-title">
    <div class="section-mark"><span>LICENSE</span><span>ACTIVE</span></div>
    <h2 id="team-tools-title">Local organization log</h2>
    <p id="history-count">${count} saved drill ${count === 1 ? "report" : "reports"}</p>
    <form id="report-form">
      <label for="report-json">Paste drill.json</label>
      <textarea id="report-json" rows="9" required>{"job":"release-check","ready":true,"commands_blocked":1}</textarea>
      <button class="button primary" type="submit">Save drill report</button>
    </form>
    <div class="team-actions"><button id="download-template" class="button secondary">Download report template</button><button id="clear-history" class="text-button">Delete local history</button></div>
    <ul id="history-list" class="history-list" aria-label="Saved drill reports"></ul>
  </section>`;
}

function notFound(): Page {
  return { title: routeMeta["/404"][0], description: routeMeta["/404"][1], body: `
    <main id="main" class="not-found">
      <div class="contour-404" aria-hidden="true">404</div>
      <p class="eyebrow">Outside mapped terrain</p>
      <h1 tabindex="-1">This route is not on the map.</h1>
      <p>The address may have moved. Return to the drill start.</p>
      <a class="button primary" href="/" data-link>Return to the start</a>
    </main>` };
}

function demoBanner(): string {
  return `<aside class="demo-banner" aria-label="Demo mode"><strong>Demo — sample data, nothing is saved</strong><div><button id="reset-demo">Reset demo</button><button id="start-real">Start for real</button></div></aside>`;
}

function normalizePath(): string {
  const path = window.location.pathname.replace(/\/+$/, "") || "/";
  return routeMeta[path] ? path : "/404";
}

function render(focus = false): void {
  const path = normalizePath();
  const page = path === "/" ? landing() : path === "/demo" ? demo() : path === "/team" ? team() : path === "/privacy" ? privacy() : path === "/terms" ? terms() : notFound();
  document.title = page.title;
  document.querySelector('meta[name="description"]')?.setAttribute("content", page.description);
  document.querySelector('link[rel="canonical"]')?.setAttribute("href", `https://ci-provider-failover-drill.sociobot.in${path === "/404" ? window.location.pathname : path}`);
  const app = document.querySelector<HTMLDivElement>("#app");
  if (!app) return;
  app.innerHTML = `${page.demo ? demoBanner() : ""}${header()}${page.body}${footer()}`;
  bindEvents();
  if (path === "/team") bindTeamEvents();
  if (focus) {
    window.scrollTo({ top: 0, behavior: "auto" });
    const heading = document.querySelector<HTMLHeadingElement>("h1");
    heading?.focus();
    const status = document.querySelector("#route-status");
    if (status) status.textContent = page.title;
  }
}

function navigate(href: string): void {
  const url = new URL(href, window.location.origin);
  history.pushState({}, "", url.pathname + url.search + url.hash);
  render(true);
  if (url.hash) document.querySelector(url.hash)?.scrollIntoView();
}

function bindEvents(): void {
  document.querySelectorAll<HTMLAnchorElement>("a[data-link]").forEach((link) => link.addEventListener("click", (event) => {
    if (event.ctrlKey || event.metaKey || event.shiftKey || link.target === "_blank") return;
    event.preventDefault();
    navigate(link.href);
  }));
  document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => button.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(button.dataset.copy || "");
      button.textContent = "Command copied";
    } catch {
      button.textContent = "Copy failed — select the command";
    }
  }));
  document.querySelector("#reset-demo")?.addEventListener("click", () => {
    Object.keys(localStorage).filter((key) => key.startsWith("demo:")).forEach((key) => localStorage.removeItem(key));
    const button = document.querySelector<HTMLButtonElement>("#reset-demo");
    if (button) button.textContent = "Demo reset";
  });
  document.querySelector("#start-real")?.addEventListener("click", () => navigate("/#install"));
}

function readHistory(): Array<Record<string, unknown>> {
  try { return JSON.parse(localStorage.getItem(TEAM_DATA_KEY) || "[]"); } catch { return []; }
}

function renderHistory(): void {
  const history = readHistory();
  const list = document.querySelector<HTMLUListElement>("#history-list");
  const count = document.querySelector("#history-count");
  if (count) count.textContent = `${history.length} saved drill ${history.length === 1 ? "report" : "reports"}`;
  if (!list) return;
  list.replaceChildren(...history.map((entry) => {
    const item = document.createElement("li");
    item.textContent = `${String(entry.job || "Unnamed job")} — ${entry.ready === true ? "ready" : "needs work"}`;
    return item;
  }));
}

function bindTeamEvents(): void {
  document.querySelector<HTMLFormElement>("#license-form")?.addEventListener("submit", async (event) => {
    event.preventDefault();
    const token = new FormData(event.currentTarget).get("license")?.toString().trim();
    if (!token) return;
    localStorage.setItem(LICENSE_KEY, token);
    await verifyLicense(token, true);
  });
  document.querySelector<HTMLFormElement>("#report-form")?.addEventListener("submit", (event) => {
    event.preventDefault();
    const field = document.querySelector<HTMLTextAreaElement>("#report-json");
    const notice = document.querySelector("#license-notice");
    try {
      const report = JSON.parse(field?.value || "");
      if (typeof report.job !== "string") throw new Error("missing job");
      const history = readHistory();
      history.push(report);
      localStorage.setItem(TEAM_DATA_KEY, JSON.stringify(history));
      if (notice) notice.textContent = "Drill report saved in this browser.";
      renderHistory();
    } catch {
      if (notice) notice.textContent = "The report is not valid drill JSON. Paste the contents of drill.json.";
    }
  });
  document.querySelector("#clear-history")?.addEventListener("click", () => {
    localStorage.removeItem(TEAM_DATA_KEY);
    renderHistory();
    const notice = document.querySelector("#license-notice");
    if (notice) notice.textContent = "Local drill history deleted.";
  });
  document.querySelector("#download-template")?.addEventListener("click", () => {
    const template = "# Organization failover report\n\n## Selected job\n\n## Last drill result\n\n## Provider assumptions\n\n## Next drill date\n";
    const link = document.createElement("a");
    link.href = URL.createObjectURL(new Blob([template], { type: "text/markdown" }));
    link.download = "organization-failover-report.md";
    link.click();
    URL.revokeObjectURL(link.href);
  });
  renderHistory();
}

function getCachedVerdict(): Verdict | null {
  try {
    const verdict = JSON.parse(localStorage.getItem(VERDICT_KEY) || "null") as Verdict | null;
    return verdict && Date.now() - verdict.checkedAt < 86_400_000 ? verdict : null;
  } catch { return null; }
}

async function verifyLicense(token: string, rerender = false): Promise<void> {
  const cached = getCachedVerdict();
  if (cached) {
    if (rerender) render();
    return;
  }
  try {
    const response = await fetch(`${API}/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`);
    const data = await response.json() as { valid: boolean; reason: string };
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ valid: data.valid, reason: data.reason, checkedAt: Date.now() }));
    if (!data.valid) localStorage.removeItem(LICENSE_KEY);
    if (rerender) {
      render();
      const notice = document.querySelector("#license-notice");
      if (notice && !data.valid) notice.textContent = "This license is no longer active. Check the token or buy Team.";
    }
  } catch {
    const notice = document.querySelector("#license-notice");
    if (notice) notice.textContent = "The license check could not connect. Try again when you are online.";
  }
}

function captureLicense(): void {
  const url = new URL(window.location.href);
  const token = url.searchParams.get("license");
  if (!token) return;
  localStorage.setItem(LICENSE_KEY, token);
  localStorage.removeItem(VERDICT_KEY);
  url.searchParams.delete("license");
  history.replaceState({}, "", url.pathname + url.search + url.hash);
}

captureLicense();
render();
const existingToken = localStorage.getItem(LICENSE_KEY);
if (existingToken) void verifyLicense(existingToken, true);
window.addEventListener("popstate", () => render(true));
