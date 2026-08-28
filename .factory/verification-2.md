# Independent product verification 2

## Verdict: FAIL

- Candidate: `b4c784bf8c3f12b69df8764887e12da89270b733`
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Verified: 28 August 2026 UTC
- Work order: `ci-provider-failover-drill-verify-2`

The candidate's free CLI and static site are working and the live assets are
byte-identical to this checkout. It cannot be released while it advertises a
$49 Team purchase whose required Sociobot checkout endpoint returns HTTP 404.
That external billing registration failure blocks the paid feature end to end.

## Release-blocking finding

### High — advertised Team purchase cannot be completed

The visible **Buy Team for $49** control links to the required hosted checkout:

```text
https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout
```

A fresh unauthenticated `GET` on 28 August returned:

```text
HTTP/2 404
content-type: application/json

{"error":"enabled factory product","status":404}
```

The landing page, README, privacy/terms copy, and Team route all advertise the
one-time $49 Team unlock. The site correctly uses the Sociobot billing URL, but
the product is not enabled there, so a buyer cannot obtain a license. This is
outside repository code, but it is a release-blocking acceptance failure until
the billing owner registers/enables this exact slug and return URL, then the
checkout and returned-license flow are retested.

No other release-blocking defect was reproduced.

## First-read test — PASS

Cold desktop landing at `/` answered all mandatory questions in plain words:

- **What it does:** “Prove your CI escape route.” It shows the selected job,
  pinned runner, and drill report outcome.
- **For whom:** “For GitHub Actions maintainers who need one critical job to
  run during an outage.”
- **What to do first:** visible **Try it with sample data** link, with adjacent
  text saying it opens a safe release-check packet in one click.

The action opens `/demo` in one click. Its persistent banner says “Demo —
sample data, nothing is saved” and exposes **Reset demo** and **Start for
real**.

## Required claims — PASS

From this clean checkout, after `npm ci`, every command listed in
`.factory/claims.json` was executed exactly as declared. The self-contained
`pretest:claims` hook built the Rust binary and static site before Playwright.
All 13 commands passed:

| Claim | Result |
| --- | --- |
| `packet-generation` | PASS |
| `release-safety` | PASS |
| `secret-redaction` | PASS |
| `offline-generation` | PASS |
| `demo-sandbox` | PASS |
| `privacy-local` | PASS |
| `paid-license` | PASS (recorded verification response) |
| `team-history` | PASS (recorded valid cached verdict) |
| `paid-contract` | PASS |
| `runner-contract` | PASS |
| `inspection-report` | PASS |
| `exit-codes` | PASS |
| `license-verdict-cache` | PASS |

The full `npm test` run also passed all 5 Rust unit/integration tests and all
20 Chromium tests, including the 13 claim tests.

## Local build, lint, package, and CLI — PASS

| Check | Evidence |
| --- | --- |
| Clean install | `npm ci`: 22 packages installed; 0 vulnerabilities reported |
| Test suite | `npm test`: 5 Rust tests + 20 Chromium tests passed |
| Type/lint | `npm run lint`: strict `tsc`, `cargo fmt --check`, and Clippy with `-D warnings` passed |
| Production build | `npm run build` completed; `dist/site/` produced and release binary built |
| Publishable crate | `cargo package --locked --allow-dirty` verified `cifail 0.1.0` (15 files, 15.6 KiB compressed) |
| Clean consumer | `cargo install --path . --root <temp> --locked` installed `cifail`; `--help` and `demo --json` worked |

Installed-consumer normal drill against the bundled workflow created exactly
`Dockerfile`, `run.sh`, `.env.example`, `drill.json`, and `report.md`.
The original GitHub secret identifier `NPM_TOKEN` and its value were absent.
By default `npm publish` was absent; adding `--allow-release` included it.
A missing job exited `2` with a clear recovery message. Real Docker execution
was not attempted because this verifier image has no Docker executable/daemon.

## Functional, privacy, and browser checks — PASS

- Normal routes `/`, `/demo`, `/team`, `/privacy`, and `/terms` returned 200,
  with route-specific titles, one `h1`, and one `main`. `/missing-place`
  returned a true HTTP 404 with the styled not-found page.
- The demo selected the realistic bundled release-check sample and exposed
  three included shell steps, one blocked publish step, one anonymous secret
  input, the registry host, and all five packet files. Demo localStorage was
  empty; its full outgoing request log contained only
  `https://ci-provider-failover-drill.sociobot.in`.
- Desktop and 390×844 mobile checks in both light and dark schemes had
  `scrollWidth === innerWidth`, no sampled visible interactive control below
  44×44 CSS px, no console/page errors, and zero axe serious/critical (or other)
  violations across the five routes.
- Keyboard starts at the visible 3 px skip-link focus ring; activating it
  reaches `#main`. Reduced-motion CSS disables smooth scroll and reduces all
  animation/transition durations.
- The response has HSTS, CSP with `frame-ancestors 'none'`, nosniff,
  strict-origin referrer policy, and permissions policy. The CSP limits network
  connections to self plus the documented Sociobot API. Hashed JS is immutable
  cached for one year.
- Initial product JS is 20,100 bytes raw / 6,730 bytes gzip and CSS is 12,450
  bytes raw / 3,670 bytes gzip, within the static-product budgets. No external
  fonts or scripts loaded.
- This is not a PWA, has no service worker, no sign-in, and no product backend;
  therefore service-worker, Entra tenant, concurrency, and persistence checks
  do not apply.

## Server-side request allowance — PASS

The only server-side product-unlock call is the Sociobot license verification
endpoint. From one client, 30 sequential invalid-token requests returned 200;
request 31 returned `429` with `Retry-After: 2`. The observed allowance is 30
requests per burst/window.

## Deployment identity — PASS

The live deployment matches the candidate production build exactly:

| File | SHA-256 |
| --- | --- |
| `index.html` | `67be3ef13fbf32a20ab2a341090f9bdcca299961dcf72c73f12f794e9e260739` |
| `assets/index-C3PTOHVk.js` | `b0e7444db6484e6e76ecdc86c4fc9c70592e7635c5bf5eeb11950d523e398362` |
| `assets/index-LFAgROc7.css` | `5a7400383490e41d5f154b36bcbddeb8594ef39cb92dd311db677a3c8f1f1bf0` |

## Required next action

Enable/register `ci-provider-failover-drill` in the production Sociobot billing
catalogue with the deployed return URL. Re-run the checkout and returned-license
flow after it returns a hosted checkout instead of 404. No product-code change
is indicated by this verification.
