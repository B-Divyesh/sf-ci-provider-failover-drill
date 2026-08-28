import { readFile, writeFile, mkdir } from "node:fs/promises";
import { resolve } from "node:path";

const site = resolve("dist/site");
const routes = {
  "demo": ["Demo — CI Provider Failover Drill", "See a sample release-check job become a failover packet."],
  "team": ["Team tools — CI Provider Failover Drill", "Restore a Team license and keep local drill history."],
  "privacy": ["Privacy — CI Provider Failover Drill", "How CI Provider Failover Drill handles workflows, reports, and licenses."],
  "terms": ["Terms — CI Provider Failover Drill", "Terms for the free CLI and one-time Team license."],
  "404": ["Page not found — CI Provider Failover Drill", "Return to CI Provider Failover Drill."]
};

function escapeHtml(value) {
  return value.replaceAll("&", "&amp;").replaceAll('"', "&quot;");
}

function pageHtml(template, route, [title, description]) {
  const canonical = `https://ci-provider-failover-drill.sociobot.in/${route === "404" ? "404" : route}`;
  return template
    .replace(/<title>[^<]*<\/title>/, `<title>${escapeHtml(title)}</title>`)
    .replace(/(<meta name="description" content=")[^"]*(" \/>)/, `$1${escapeHtml(description)}$2`)
    .replace(/(<link rel="canonical" href=")[^"]*(" \/>)/, `$1${canonical}$2`)
    .replace(/(<meta property="og:title" content=")[^"]*(" \/>)/, `$1${escapeHtml(title)}$2`)
    .replace(/(<meta property="og:description" content=")[^"]*(" \/>)/, `$1${escapeHtml(description)}$2`)
    .replace(/(<meta name="twitter:title" content=")[^"]*(" \/>)/, `$1${escapeHtml(title)}$2`)
    .replace(/(<meta name="twitter:description" content=")[^"]*(" \/>)/, `$1${escapeHtml(description)}$2`);
}

const template = await readFile(resolve(site, "index.html"), "utf8");
for (const [route, meta] of Object.entries(routes)) {
  const destination = route === "404" ? resolve(site, "404.html") : resolve(site, route, "index.html");
  const directory = route === "404" ? site : resolve(site, route);
  await mkdir(directory, { recursive: true });
  await writeFile(destination, pageHtml(template, route, meta));
}
