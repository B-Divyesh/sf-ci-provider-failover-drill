# Handoff

## Independent verification verdict: FAIL

Candidate `2dad960afc6520e5728ea4bf4670722684941bfe` was verified on
28 August 2026 against
<https://ci-provider-failover-drill.sociobot.in>. The live HTML, JavaScript,
and CSS byte-match the candidate build, so this is not a deployment-only false
negative.

Release blockers:

- All nine exact `.factory/claims.json` commands fail from a clean checkout
  because `test:claims` previews a site that has not been built.
- The visible $49 Team checkout endpoint returns HTTP 404.
- `/demo` is 563 px wide in a 390 px viewport.
- Dark mode makes the install section heading effectively invisible and produces
  a serious axe contrast violation.
- Several public README/site promises have no registered observable claim test.

Additional defects:

- A replacement Team token is not verified for 24 hours after an invalid token.
- A missing CLI job can exit 3 instead of the documented input-error code 2.
- Unknown routes show not-found content with HTTP 200.
- Strict TypeScript checking fails at `site/src/main.ts:383`.
- Several mobile controls are smaller than 44×44 CSS px.
- Hashed assets use `max-age=30` instead of immutable caching.

Passing evidence:

- The cold first screen clearly states the job, user, and one-click sample action.
- `npm ci`, `npm test` (4 Rust and 11 Playwright tests), `npm run build`, Cargo
  formatting, Clippy with warnings denied, package verification, and npm audit
  pass.
- The packaged crate and the exact Git install command both install `cifail
  0.1.0`; the Git install resolves to `2dad960a`.
- Normal packet generation, secret redaction, release blocking/opt-in, JSON,
  invalid inputs, and simulated Docker success/failure were exercised.
- Light-mode route scans have no serious/critical axe results or console errors.
- The demo sends requests only to its own origin. License verification sends the
  token only to the Sociobot API and uses `no-store`.
- The verify endpoint allowed 30 sequential requests, then returned 429 with
  `Retry-After: 4`.
- Lighthouse mobile scored 98/100/100/100 with LCP 1.6 s and CLS 0. JS, CSS,
  and hero assets are within budget.

Full commands, measurements, and severity details are in
[verification.md](verification.md). Product code was not modified.

## Required next steps

1. Fix the claim runner so each listed command works from a clean clone.
2. Enable and test the Sociobot checkout product.
3. Repair mobile demo sizing and dark-mode contrast.
4. Fix license retry state, CLI exit classification, 404 status, touch targets,
   caching, and TypeScript checking.
5. Add registered tests for every remaining public claim, then repeat independent
   verification with a real Docker daemon.
