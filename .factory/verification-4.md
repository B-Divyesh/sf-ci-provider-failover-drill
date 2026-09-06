# Prove one CI job runs elsewhere — verification 4

## Verdict: FAIL

Candidate implementation: `1c48c1eb05f3c3335fc5abac6f8223fb603ac564`.

Documentation reviewed: `a37dc2074ea782b5a396b0f79cb0e1606eb5a5fa`.

Live URL: https://ci-provider-failover-drill.sociobot.in

There is one finding and one untested public claim. The other 15 declared
claims passed from a fresh clone. The product must not be accepted until the
Docker runtime claim passes on a Docker-capable host.

## First screen

Before scrolling on fresh 1440 × 900 desktop and 390 × 844 phone sessions,
the page states the job: “Prove one CI job runs elsewhere.” It names GitHub
Actions maintainers as the audience and offers “Try it with sample data” as
the first action. The three facts are visible on both: free local drill, no
secrets stored, and release steps stay blocked.

## Finding

### F-4-1 — the Docker runner claim is untested

Severity: high. Claim: “The generated packet can run on a laptop or any runner
with Docker.”

I ran its declared command from a fresh clone:

```sh
CIFAIL_DOCKER_RUNTIME=1 npm run test:claims -- --grep @claim:runner-contract
```

The worker initially had no Docker binary. I installed the documented
prerequisite, started Docker 29.1.3 with a private temporary daemon, and ran
the command again with that daemon. It reached the real packet build and
failed with `unshare: operation not permitted`. The daemon reports the same
kernel isolation restriction. This is an environment capability limit, not a
simulated product pass. The command did not produce the required executed PASS
report, so the claim is untested.

Required follow-up on a Docker-capable host:

```sh
CIFAIL_DOCKER_RUNTIME=1 npm run test:claims -- --grep @claim:runner-contract
```

## Declared claims from a fresh clone

Fresh checkout: `/tmp/cifail-verify-4-Op3euY/repo` at
`a37dc2074ea782b5a396b0f79cb0e1606eb5a5fa`; `npm ci` completed with no
reported vulnerabilities.

| Claim | Result |
| --- | --- |
| packet-generation | PASS |
| release-safety | PASS |
| secret-redaction | PASS |
| offline-generation | PASS |
| local-privacy | PASS |
| cli-demo-isolation | PASS |
| no-ci-mutation | PASS |
| demo-sandbox | PASS |
| privacy-local | PASS |
| paid-license | PASS |
| team-history | PASS |
| paid-contract | PASS |
| inspection-report | PASS |
| exit-codes | PASS |
| license-verdict-cache | PASS |
| runner-contract | UNTESTED — Docker kernel isolation denied |

`npm test` also passed: 26 passed, 1 skipped (the Docker-gated contract).
`npm run lint`, `npm run build`, and `cargo package --locked --allow-dirty`
passed. The release build produced `dist/site/` with 7.32 KB gzip JavaScript
and 3.82 KB gzip CSS. A clean `cargo install --path . --root <temp>` consumer
install passed; its installed `cifail --help` and `cifail demo --json` worked
from a separate consumer directory.

## Live product checks

The live HTML references `index-Cs0Zb_CQ.js` and `index-DlGLPV0r.css`; their
SHA-256 values match the candidate build exactly. Fresh desktop and phone
sessions entered `/?demo=1` in one click. The populated sample shows three
shell steps, one blocked release step, one anonymous input, and the five packet
files. The persistent “Demo — sample data, nothing is saved” banner and both
controls remained visible after scrolling. Reset removed `demo:` keys while a
`real:sentinel` key stayed unchanged. The demo made no requests to another
origin.

`/`, `/demo`, `/team`, `/privacy`, and `/terms` returned 200. Each had its own
title, one `<h1>`, and one `<main>`. `/missing-place` returned its designed
page with HTTP 404; its browser 404 console message is expected and is not a
defect. The factory URL verifier passed in 584 ms with no console errors.
Live axe scans found no serious or critical issues on all six routes. The
mobile Lighthouse result was Performance 99, Accessibility 100, Best Practices
100, SEO 100; FCP 1.6 s, LCP 1.6 s, TBT 0 ms, CLS 0. Focus styling and reduced
motion rules are present, and the live sample controls remain keyboard- and
touch-reachable.

All internal routes, static metadata assets, `robots.txt`, `sitemap.xml`, and
the Sociobot terms link returned 200. Checkout returned its expected hosted
303 redirect. Thirty invalid-token verification requests returned 200; request
31 returned 429 with `Retry-After: 3`, as required. The product is a static
site and CLI, so backend tenant, restart, and health checks do not apply.

## Earlier findings

All earlier findings were rechecked. F-1-1, its F-3-1 and F-5-1 reopenings,
and F-1-2, F-3-2, and F-5-2 are covered by the passing `release-safety` and
`inspection-report` tests: substitutions and wrappers are blocked and their
hosts are reported. F-5-3 through F-5-5 pass through `secret-redaction`,
`cli-demo-isolation`, and the separate step-isolation test. F-5-7 and F-5-8
pass through the live sticky-banner check and the passing throttle-recovery
test.

F-1-3, F-1-4, F-1-15, F-1-16, F-3-3, and F-2-1 are covered by the passing
local-privacy, cli-demo-isolation, team-history, no-ci-mutation, paid-license,
license-verdict-cache, and paid-contract tests. F-1-5, F-1-8, F-1-13,
F-3-4 through F-3-6, and the earlier 404, dark-mode, touch-target, caching,
and URL-verifier findings pass through the live route, phone, keyboard/touch,
axe, factory-verifier, and Lighthouse checks above. F-1-6, F-1-7, F-1-9
through F-1-12, and F-1-14 have repaired wording in the current landing,
README, terms, and privacy copy; the current copy regression tests pass.

The older F-5-6 runner-contract coverage gap is improved because the test now
attempts a real execution. It is nevertheless F-4-1 until that attempt passes
on capable hardware.

## Evidence

Evidence is in `/work/.evidence/verification-4/`, including browser output,
desktop and phone screenshots, route and axe results, the factory URL report,
and the Lighthouse JSON. Individual claim logs are in
`/tmp/cifail-verify-4-Op3euY/claim-logs/`.
