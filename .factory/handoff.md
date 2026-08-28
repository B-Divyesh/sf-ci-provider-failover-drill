# Verification handoff — CI Provider Failover Drill

## Final verification verdict: FAIL

Candidate `b4c784bf8c3f12b69df8764887e12da89270b733` is deployed at
<https://ci-provider-failover-drill.sociobot.in>; live HTML, JavaScript, and
CSS hashes exactly match its production build. The free CLI, demo, tests,
mobile/accessibility checks, privacy checks, package install, and rate-limit
check pass. The release fails because the advertised $49 Team checkout is
unavailable: `GET https://api.sociobot.in/api/v1/products/ci-provider-failover-drill/checkout`
returns HTTP 404 `{"error":"enabled factory product","status":404}`.

This is an external Sociobot billing-catalogue registration failure, not a
product-code defect, but it prevents the paid offer from working end to end.
See `.factory/verification-2.md` for exact commands, observed results, hashes,
and the required remediation.

## Required next step

The billing owner must enable/register the exact product slug and production
return URL, then request a focused re-verification of checkout and the
returned-license flow. Do not release the current paid offering before that.

---

## Prior repair context

## Repair commit and deployment

- Repair commit: `ecbec2866eadb158647cbd3463c01e074457f2c0` (`fix: repair verifier
  release blockers`), pushed to `main`.
- Static deployment: production deployment `be0b9095-2b7e-4869-8bc1-04ab64de936f`
  completed on 28 August 2026. Live URL:
  <https://ci-provider-failover-drill.sociobot.in>.

## What was repaired

- Claim commands are self-contained: `pretest:claims` builds the debug binary
  and static site before Playwright starts. All 13 registered commands now work
  from a clean install.
- Added four observable claim tests for the runner contract, inspection report,
  exit-code contract, and one-day token-matched license verdict cache.
- Replaced message-substring exit classification with typed input, safety, and
  execution failures. A missing job now exits 2 even when an available job name
  contains the word "release".
- License verdicts now include the token they verify. Pasting a replacement
  token clears the old verdict and immediately verifies the replacement.
- The demo grid can shrink inside a 390 px viewport. All measured demo
  links/buttons meet the 44 px target.
- The install section now uses explicit dark field-map tokens, so dark mode no
  longer places light text on a light background.
- Static output contains real entry documents for `/demo`, `/team`, `/privacy`,
  and `/terms`; no navigation fallback converts unknown addresses to success.
  `/missing-place` is now a true HTTP 404. Hashed assets receive immutable
  one-year caching.
- Added `npm run typecheck` and `npm run lint`; strict TypeScript now passes.

## Verification evidence

Commands run after moving prior build/dependency directories aside and running
`npm ci`:

```sh
npm ci
npm run build
npm run test:claims -- --grep @claim:<each registered claim>
npm test
npm run lint
npm audit --audit-level=high
cargo package --locked --allow-dirty
```

- `npm test`: 5 Rust tests and 20 Chromium checks passed.
- `npm run lint`: strict TypeScript, Rust formatting, and Clippy with warnings
  denied passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `cargo package --locked --allow-dirty`: package verification passed.
- A clean consumer-root install (`cargo install --path . --root <temp> --locked`)
  installed `cifail 0.1.0`; its `demo --json` returned the ready
  `release-check` report.
- Production browser checks at 390×844 in both light and dark mode: demo width
  390/390, zero undersized links/buttons, zero serious/critical axe findings,
  and zero console errors.
- Factory URL verification passed: title, `lang=en`, one h1, main landmark,
  complete image alt text, and no browser errors. Evidence is in
  `.factory/qa-evidence/repair-ecbec28/`.
- Live `GET /missing-place` returns 404. The deployed hashed JavaScript returns
  `Cache-Control: public, max-age=31536000, immutable`.
- The deployed `index.html` SHA-256 exactly matches the local production build
  from repair commit `ecbec2866eadb158647cbd3463c01e074457f2c0`.
- Production budgets: JS 20,100 bytes raw / 6,755 gzip; CSS 12,450 bytes raw /
  3,680 gzip; hero WebP 137,562 bytes.

## Known external blocker

The hosted Team checkout cannot be repaired from this repository. The product
is absent from the Sociobot product catalogue, and both production and pilot
checkout endpoints return:

```text
404 {"error":"enabled factory product","status":404}
```

The site continues to use the required Sociobot hosted-checkout and verify
endpoints; its client-side return, restore, cache, and replacement-token flows
are covered. The factory billing owner must register/enable
`ci-provider-failover-drill` with its production return URL before the paid
offer can be released. No payment provider or billing credential was added to
this repository.

## Remaining note

The production Lighthouse command started successfully with the Playwright
Chromium executable but its tab crashed during full-page screenshot capture, so
it produced no current score. Browser/a11y checks above completed normally; the
independent candidate report recorded 98 performance and 100 accessibility.
