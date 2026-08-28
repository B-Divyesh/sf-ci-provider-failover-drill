# Independent verification 3 — CI Provider Failover Drill

## Verdict: PASS

- Candidate: `b4c784bf8c3f12b69df8764887e12da89270b733`
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Verified: 28 August 2026 UTC
- Work order: `ci-provider-failover-drill-verify-3`

The prior deployment-only concern is not reproduced. The live production HTML,
JavaScript, CSS, hero image, terminal illustration, and social card hash-match
the production build from this candidate. No release-blocking defects were
found.

## First-read test

**PASS.** A cold 390×844 visit says “Prove your CI escape route.” It says this
is for “GitHub Actions maintainers who need one critical job to run during an
outage.” The first primary action is **Try it with sample data**, with adjacent
copy explaining that it opens a safe release-check packet in one click. Clicking
it opens `/demo` without setup.

## Required clean-clone claim execution

Started from the supplied clean checkout, ran `npm ci`, then every exact command
listed in `.factory/claims.json`. All 13 passed through the product demo entry
point:

| Claim id | Result |
| --- | --- |
| `packet-generation` | PASS |
| `release-safety` | PASS |
| `secret-redaction` | PASS |
| `offline-generation` | PASS |
| `demo-sandbox` | PASS |
| `privacy-local` | PASS |
| `paid-license` | PASS |
| `team-history` | PASS |
| `paid-contract` | PASS |
| `runner-contract` | PASS |
| `inspection-report` | PASS |
| `exit-codes` | PASS |
| `license-verdict-cache` | PASS |

`npm test` then passed all 5 Rust tests and all 20 Chromium checks, including
all claim tags. The self-contained claim pre-step built the CLI and static demo
before Playwright, so this is a clean-clone result rather than a warmed-build
false positive.

## Local CLI and package verification

| Check | Result |
| --- | --- |
| `npm ci` | PASS — 22 packages, 0 reported audit vulnerabilities |
| `npm test` | PASS — 5 Rust tests, 20 Chromium tests |
| `npm run lint` | PASS — TypeScript, Rust fmt, and Clippy with warnings denied |
| `npm run build` | PASS — release binary and `dist/site/` created |
| `cargo package --allow-dirty` | PASS |
| Clean consumer install of `target/package/cifail-0.1.0` | PASS — installed `cifail 0.1.0` and ran `demo --json` |

Normal end-to-end `cifail demo --json --out <temp>` generated a ready
`release-check` packet with exactly `Dockerfile`, `run.sh`, `.env.example`,
`drill.json`, and `report.md`; it reported 3 included commands, 1 blocked
publish command, one anonymous input, required files, `registry.npmjs.org`, and
provider-action assumptions. `run.sh` contains `DRILL_SECRET_1`, not the secret
name/value, and does not contain `npm publish` by default.

Boundary/recovery checks passed: an unpinned image and unknown job each returned
a clear diagnostic and exit 2. The automated integration test also covers exit
3 before execution for missing required files and exit 4 for a controlled Docker
failure. The verifier container has no Docker executable or daemon, so a real
container run could not be performed; this is an environment limitation, not a
product failure. The packet's Docker and generic-runner contract are covered by
the claim test and inspected output.

## Live product, accessibility, privacy, and resilience

- `/`, `/demo`, `/team`, `/privacy`, and `/terms` each returned HTTP 200 with
  one `h1`, one `main`, `lang="en"`, route-specific title, and no console or page
  errors.
- Playwright axe scans of all five routes found **zero serious or critical**
  findings. Lighthouse mobile recorded Performance **97** and Accessibility
  **100** (FCP 0.9 s, LCP 1.6 s, CLS 0). The Chromium tab exited during
  Lighthouse's final screenshot collection after audit results were generated;
  the complete score JSON is at `/tmp/ci-failover-lighthouse.json` in this
  verifier environment.
- Desktop (1440 px) and mobile (390 px) had no horizontal overflow. The demo
  banner persisted, exposed Reset demo and Start for real, and reset correctly.
  No demo storage keys were created.
- Keyboard testing began at the visible skip link. Enter navigated to `#main`;
  the next Tab reached the primary sample-data action. The primary action had a
  designed 3 px teal focus outline and a 350×53.6 px mobile target. Reduced
  motion changed animation and transition duration to 0.00001 s and scrolling
  to `auto`.
- The complete live demo request log contained only the product origin (HTML,
  JS, CSS, and self-hosted assets). No analytics, third-party fonts, or scripts
  were requested. The license API is contacted only after an explicit license
  verification, as stated in Privacy.
- Live headers include CSP with `frame-ancestors 'none'`, HSTS,
  `X-Content-Type-Options: nosniff`, strict-origin referrer policy, and a
  restrictive permissions policy. Hashed JS is cached
  `public, max-age=31536000, immutable`; HTML uses a 30-second revalidation
  policy. A nonexistent route returns HTTP 404.
- The Sociobot checkout link returned HTTP 303 to the hosted Dodo checkout. No
  purchase was attempted.
- The only service endpoint used by the product, Sociobot license verification,
  enforced a single-client allowance of **30** requests: the 31st sequential
  invalid-token request returned HTTP 429 with `Retry-After: 4` and
  `X-RateLimit-After: 4`.

## Deployment identity and budgets

SHA-256 values from the local candidate build exactly matched the live response
for `index.html`, `assets/index-C3PTOHVk.js`,
`assets/index-LFAgROc7.css`, `topographic-route.webp`,
`terminal-demo.svg`, and `social-card.jpg`.

- Initial JavaScript: 20,100 bytes raw / 6,755 bytes gzip (under 200 KB).
- CSS: 12,450 bytes raw / 3,680 bytes gzip (under 50 KB).
- Hero WebP: 137,562 bytes (under 300 KB).
- No downloaded web fonts or third-party scripts.

## Defects by severity

None found: Blocker 0, High 0, Medium 0, Low 0.

## Non-applicable checks

This is a CLI with a static documentation/demo site, not a PWA and not a
first-party backend. It has no sign-in, service worker, or persistence service;
Microsoft Entra and PWA update/offline checks are therefore not applicable.
