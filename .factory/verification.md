# Independent product verification

## Verdict: FAIL

- Candidate: `2dad960afc6520e5728ea4bf4670722684941bfe`
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Verified: 28 August 2026 UTC
- Work order: `ci-provider-failover-drill-verify-1`

This is not a deployment-only failure. The live site is available and its HTML,
JavaScript, and CSS match the candidate build byte for byte. The candidate still
fails mandatory claim execution, paid checkout, mobile demo layout, and dark-mode
accessibility.

## Release-blocking findings

### Blocker — every registered claim command fails from a clean checkout

The checkout started clean at the requested commit. `.factory/claims.json` exists
and lists nine claims. Each listed command was run exactly as written before any
build. All nine exited 1 with:

```text
Error: Timed out waiting 60000ms from config.webServer.
```

`test:claims` starts `vite preview`, but it does not build `dist/site` first.
`vite preview` never becomes ready in a clean clone. This is an automatic FAIL
under the acceptance contract.

| Claim | Exact command | Clean-clone result |
| --- | --- | --- |
| `packet-generation` | `npm run test:claims -- --grep @claim:packet-generation` | FAIL |
| `release-safety` | `npm run test:claims -- --grep @claim:release-safety` | FAIL |
| `secret-redaction` | `npm run test:claims -- --grep @claim:secret-redaction` | FAIL |
| `offline-generation` | `npm run test:claims -- --grep @claim:offline-generation` | FAIL |
| `demo-sandbox` | `npm run test:claims -- --grep @claim:demo-sandbox` | FAIL |
| `privacy-local` | `npm run test:claims -- --grep @claim:privacy-local` | FAIL |
| `paid-license` | `npm run test:claims -- --grep @claim:paid-license` | FAIL |
| `team-history` | `npm run test:claims -- --grep @claim:team-history` | FAIL |
| `paid-contract` | `npm run test:claims -- --grep @claim:paid-contract` | FAIL |

After `npm test` built the site, the underlying nine claim assertions passed as
part of the 11-test Playwright suite. That does not repair the required clean-
clone commands.

### High — the live paid checkout is dead

Both visible “Buy Team for $49” links target:

```text
https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout
```

Fresh GET result: HTTP 404 with
`{"error":"enabled factory product","status":404}`. A visitor cannot purchase
the advertised Team license. See `qa-evidence/checkout-headers.txt` and
`qa-evidence/checkout-body.txt`.

### High — the one-click demo breaks at the required 390 px width

At 390×844, `/demo` has `document.documentElement.scrollWidth === 563` while
`innerWidth === 390`. Both primary result cards expand to about 545 px. Content
and controls require horizontal page scrolling. The landing route itself has no
horizontal overflow. See `qa-evidence/live-demo-mobile-390.png` and
`qa-evidence/live-qa.json`.

### High — the documented dark treatment has serious contrast failures

The dark-color preference turns the install section background light while
keeping its heading and supporting text light. “Run the first drill” is visually
absent at effectively 1:1 contrast. Axe reports a serious `color-contrast`
violation with two nodes, including ratios 2.33:1 and 1.49:1. See
`qa-evidence/live-dark-install.png`.

### High — public claim coverage is incomplete

Public statements not represented by a matching `.factory/claims.json` entry
and observable claim test include:

- “The generated packet can run on a laptop or any runner with Docker.”
- The README promise that it checks required files, network needs, and
  provider-only actions; the packet claim checks only file creation and image
  text.
- The documented exit-code contract.
- “The last verdict is cached for one day.”

The claims contract makes unlisted claims release-blocking.

## Other defects

### Medium — replacing an invalid license does not recover

After a failed verification, the invalid verdict is cached. Pasting a new token
within 24 hours stores the token but sends zero verification requests, leaves the
restore form locked, and never unlocks Team even when the replacement would be
valid. This was reproduced against the live candidate with the API response
intercepted: `requests: 0`, `hasTools: 0`, `stillHasForm: 1`.

### Medium — the documented CLI exit code is wrong for a missing job

Requesting `--job not-a-job` against the sample workflow exits 3, although the
README assigns input problems exit 2. The error classifier sees “release” in the
available job name `release-check` and misclassifies the input error as a safety
block.

### Medium — unknown routes return HTTP 200

`/missing-place` displays the styled not-found screen but responds HTTP 200, as
does `/404.html`. This is a soft 404 and conflicts with the required real 404
route.

### Medium — TypeScript has no clean strict type check

The repository has no type-check script. An independent strict check failed:

```text
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution Bundler \
  --lib ES2022,DOM --strict --skipLibCheck --allowArbitraryExtensions site/src/main.ts
site/src/main.ts(383,32): error TS2345: Argument of type 'EventTarget | null' ...
```

### Medium — multiple mobile targets are below 44×44 CSS px

Measured examples include the 36×36 wordmark, 32.9×44 Demo and Team links,
163.8×25.6 Restore link, and footer links as short as 39.1×20.8.

### Low — hashed assets are not cached immutably

The live hashed JS and CSS both return
`Cache-Control: public, must-revalidate, max-age=30`. This misses the long-lived
immutable cache policy for content-hashed assets.

### Low — factory URL verification is not stable

`/opt/fleet/lib/verify-url.sh` received HTTP 200, then failed because its
Playwright `page.goto(..., waitUntil: "networkidle")` exceeded 60 seconds.
Independent Playwright route runs completed without page or console errors, so
this is recorded separately from the functional defects above.

## First-read test

PASS for the cold desktop first screen.

- What it does: proves one selected CI job has an escape route through a pinned
  runner and drill report.
- For whom: GitHub Actions maintainers who need a critical job during an outage.
- What to click first: **Try it with sample data**; adjacent copy says it opens a
  safe release-check packet in one click.
- One-click demo: present and routes directly to `/demo` without setup.

The first-read content is clear even though the mobile demo subsequently fails
its layout requirement.

## Clean install, tests, build, and package

| Check | Result |
| --- | --- |
| Clean state | PASS — `HEAD` and `origin/main` were `2dad960`; no initial changes |
| `npm ci` | PASS — 22 packages, 0 vulnerabilities |
| `npm test` | PASS — 3 library tests, 1 CLI integration test, 11 Playwright tests |
| `npm run build` | PASS — release binary and `dist/site/` produced |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features --locked -- -D warnings` | PASS |
| Independent strict TypeScript check | FAIL — TS2345 at `site/src/main.ts:383` |
| `npm audit --audit-level=high` | PASS — 0 vulnerabilities |
| `cargo package --locked --allow-dirty` | PASS — 15 files, 14.8 KiB compressed |
| Install packaged crate into clean root | PASS — `cifail 0.1.0` |
| Exact landing-page Git install | PASS — Cargo installed Git commit `2dad960a` |

## CLI end-to-end and boundary checks

- Installed-package `cifail demo --json` returned clean JSON and a ready packet.
- A normal sample drill created exactly `Dockerfile`, `run.sh`, `.env.example`,
  `drill.json`, and `report.md`; `run.sh` passed `sh -n`.
- The generated files contained neither `NPM_TOKEN` nor the test value.
- Default generation included three commands and blocked one publish command.
- `--allow-release` included `npm publish` and reported zero blocked commands.
- Unpinned and 63-character digests, missing workflow/repository, and invalid
  workflow shape produced clear errors.
- Missing required repository files made `--execute` exit 3 before Docker.
- With Docker absent, `--execute` explained how to recover and exited 2.
- A controlled Docker shim verified success state and exit 4 for build/run
  failures. A real container execution was not possible because this verifier
  image has no Docker executable or daemon.
- The pinned demo image digest exists: Docker Registry returned HTTP 200 and the
  exact `docker-content-digest`.

## Live browser, privacy, and accessibility

- Desktop landing, demo, Team, privacy, terms, and not-found content each have
  `lang=en`, one `h1`, one `main`, ordered headings, and complete image alt text.
- Light-mode axe scan: zero serious/critical findings across all six states.
- Dark-mode landing: FAIL, serious contrast violation described above.
- Keyboard order starts with the skip link; all sampled focus states had a
  visible 3 px outline. Enter opened the demo. SPA navigation and Back focused
  the destination `h1`.
- Reduced-motion media query matched; smooth scrolling was disabled and the
  route path rendered complete.
- No console errors, page errors, or failed requests occurred in the normal
  route matrix.
- The complete demo flow contacted only
  `https://ci-provider-failover-drill.sociobot.in`.
- A real invalid-license flow contacted only the site and
  `https://api.sociobot.in`; the API returned CORS for the site origin and
  `Cache-Control: no-store`.
- Empty license input used browser required-field validation. Invalid license
  feedback was visible. Invalid drill JSON showed recovery guidance; a valid
  report then saved, downloaded, and deleted correctly.
- No sign-in exists, so Entra authority verification is not applicable.
- This is not a PWA and has no first-party backend or service worker.

## Server endpoint allowance

The Sociobot license verify endpoint enforced a burst allowance. In one
sequential client run, 30 requests returned 200 and request 31 returned 429 with
`Retry-After: 4` and `X-RateLimit-After: 4`. See
`qa-evidence/rate-limit.json`. The checkout endpoint is separately broken with
404 as described above.

## Headers, deployment identity, caching, and budgets

- Candidate identity: local and live SHA-256 hashes matched exactly for
  `index.html`, `index-DZiIhXl8.js`, and `index-nZoo02yh.css`. The remote main
  ref is the full requested candidate hash.
- Present headers: CSP, HSTS, `X-Content-Type-Options`, `Referrer-Policy`, and
  `Permissions-Policy`. CSP permits only self plus the Sociobot API connection.
- Cache failure: all HTML and hashed assets use 30-second revalidation.
- Initial JS: 19,994 bytes raw / 6,699 bytes gzip (budget 200 KB).
- CSS: 11,991 bytes raw / 3,603 bytes gzip (budget 50 KB).
- Hero WebP: 137,562 bytes (budget 300 KB). No web fonts are downloaded.
- Lighthouse 12.8.2 mobile: Performance 98, Accessibility 100, Best Practices
  100, SEO 100; FCP 0.9 s, LCP 1.6 s, TBT 160 ms, CLS 0.
- Lighthouse desktop: 100/100/100/100; FCP 0.3 s, LCP 0.4 s, TBT 30 ms, CLS 0.

## Required next actions

1. Make every claim command self-contained from a clean checkout.
2. Register/enable the Sociobot product and verify checkout end to end.
3. Fix `/demo` intrinsic sizing at 390 px and all sub-44 px targets.
4. Correct the dark install-section tokens and rerun axe in both themes.
5. Invalidate an old license verdict when a new token is submitted.
6. Classify CLI errors structurally, not by substrings in user-facing text.
7. Return a real 404 and configure immutable caching for hashed assets.
8. Add a repository TypeScript check and tests for the unlisted claims.
