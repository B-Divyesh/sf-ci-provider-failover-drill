import { cp, mkdir } from "node:fs/promises";
import { resolve } from "node:path";

const site = resolve("dist/site");
const routes = ["demo", "team", "privacy", "terms"];

await cp(resolve(site, "index.html"), resolve(site, "404.html"));
for (const route of routes) {
  const directory = resolve(site, route);
  await mkdir(directory, { recursive: true });
  await cp(resolve(site, "index.html"), resolve(directory, "index.html"));
}
