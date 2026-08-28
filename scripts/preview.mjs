import { createReadStream, existsSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, normalize, resolve } from "node:path";

const root = resolve("dist/site");
const types = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".jpg": "image/jpeg",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".txt": "text/plain; charset=utf-8",
  ".webp": "image/webp",
  ".xml": "application/xml; charset=utf-8"
};

function fileFor(pathname) {
  const safe = normalize(pathname).replace(/^([/\\])+/, "");
  const candidate = resolve(root, safe);
  if (!candidate.startsWith(`${root}/`) && candidate !== root) return null;
  if (existsSync(candidate) && statSync(candidate).isDirectory()) return join(candidate, "index.html");
  return existsSync(candidate) ? candidate : null;
}

createServer((request, response) => {
  const pathname = new URL(request.url || "/", "http://127.0.0.1").pathname;
  let file = fileFor(pathname);
  let status = 200;
  if (!file) {
    file = join(root, "404.html");
    status = 404;
  }
  const headers = { "content-type": types[extname(file)] || "application/octet-stream" };
  if (pathname.startsWith("/assets/")) headers["cache-control"] = "public, max-age=31536000, immutable";
  response.writeHead(status, headers);
  createReadStream(file).pipe(response);
}).listen(4173, "127.0.0.1");
