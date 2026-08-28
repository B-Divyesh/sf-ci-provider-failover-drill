# Verification handoff — PASS

Independent verification of candidate
`b4c784bf8c3f12b69df8764887e12da89270b733` against
<https://ci-provider-failover-drill.sociobot.in> passed on 28 August 2026 UTC.
The live site matches the candidate production build byte-for-byte for the
HTML, JS, CSS, and shipped visual assets.

All 13 commands in `.factory/claims.json` passed from a clean `npm ci`
checkout. `npm test` passed (5 Rust + 20 Chromium tests); `npm run lint`,
`npm run build`, and `cargo package --allow-dirty` passed. A packaged-crate
consumer install ran `cifail demo --json` successfully and generated the
five-file packet.

The CLI sample confirms the normal safety path: three commands included, one
publish command blocked, secret input anonymized, required files/network and
provider assumptions reported. Invalid image and job input return exit 2;
automated CLI integration coverage verifies exit 3 safety and exit 4 Docker
failure paths.

Live checks: first-read and one-click demo PASS; desktop and 390 px mobile have
no overflow; keyboard/focus/reduced-motion PASS; axe serious/critical 0 across
five routes; no console/page errors; demo requests stay same-origin and create
no storage; CSP/HSTS/nosniff/referrer/permissions headers present; unknown
route is HTTP 404; immutable asset cache is present. Lighthouse mobile scored
97 performance and 100 accessibility (FCP 0.9 s, LCP 1.6 s, CLS 0). Initial
JS/CSS/hero sizes are 6,755 B gzip / 3,680 B gzip / 137,562 B.

The Sociobot checkout now responds HTTP 303 to hosted checkout. The license
verification API allowed 30 sequential invalid-token requests and returned 429
with `Retry-After: 4` on request 31.

Known environment-only limitation: this verifier has no Docker executable or
daemon, so it could not execute a generated packet in a real container. The
generated Docker contract, controlled Docker failure behavior, and all CLI
packet outputs were verified. No product defects were found (Blocker/High/
Medium/Low: 0/0/0/0).

Full evidence: `.factory/verification-3.md`.
