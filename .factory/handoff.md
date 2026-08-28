# Polish 2 handoff

## Result

PASS. Every finding in `.factory/review-1.md` and `.factory/review-2.md` is
closed. The production site is deployed at
<https://ci-provider-failover-drill.sociobot.in>.

The round-2 defect was an unbounded Team entitlement. Terms now say, “It covers
the browser tools shown on the Team page.” `@claim:paid-contract` asserts that
scope and rejects the removed “future v1 updates” promise. A cumulative audit
also finished F-1-12 by standardizing CLI help, reports, the recording, demo
docs, and claims on **job name**, **anonymous input**, and **GitHub-only action**.

## Exact verification evidence

- Repair commit deployed: `f721ea11335f4a6cc5435aa27b5903c9389482b7`.
- Azure Static Web Apps deployment:
  `45fb8fc1-6e03-40cf-a09f-9d28bb3dc669`.
- Clean clone: `/tmp/cifail-final-CX8AwL`, cloned from `origin/main` at the
  repair commit. `npm ci` reported 0 vulnerabilities.
- Every one of the 16 claim commands in `.factory/claims.json` ran separately
  and passed: `packet-generation`, `release-safety`, `secret-redaction`,
  `offline-generation`, `local-privacy`, `cli-demo-isolation`,
  `no-ci-mutation`, `demo-sandbox`, `privacy-local`, `paid-license`,
  `team-history`, `paid-contract`, `runner-contract`, `inspection-report`,
  `exit-codes`, and `license-verdict-cache`.
- `npm test`: 6 Rust tests and 24 Chromium tests passed on the final tree.
- `npm run lint`: TypeScript, rustfmt, and Clippy passed with warnings denied.
- `npm run build`: release CLI and `dist/site/` passed.
- `cargo package --locked`: 15 files, 53.1 KiB; package verification passed.
- Site budget: 6.98 KiB gzip JavaScript and 3.78 KiB gzip CSS.
- `/opt/fleet/lib/verify-url.sh`: HTTPS 200, title/lang/main/alt/control checks
  passed with no console errors. See [verify.json](qa-evidence/polish-2/verify.json).
- Cold production check: `/`, `/demo`, `/team`, `/privacy`, and `/terms`
  returned 200; `/missing-place` returned a styled 404. Raw title, canonical,
  Open Graph, Twitter, CSP, and nosniff checks passed for every route. SPA
  navigation and Back focused the new `h1`. See
  [live-route-check.json](qa-evidence/polish-2/live-route-check.json).
- The cold `?demo=1` path showed the sample banner, all sample facts, Reset,
  and View install command. Reset left no `demo:` keys, and every request stayed
  on the product origin. See
  [live-demo-mobile-390.png](qa-evidence/polish-2/live-demo-mobile-390.png).
- At 390 × 844, the action and all three facts fit before the fold with no
  horizontal overflow. See
  [live-landing-mobile-390.png](qa-evidence/polish-2/live-landing-mobile-390.png).
- Axe found zero serious or critical violations on all six checked routes.
  Lighthouse mobile scored 100 Performance, 100 Accessibility, 100 Best
  Practices, and 100 SEO; LCP was 1.5 s, CLS 0, and TBT 10 ms. See
  [lighthouse-mobile.json](qa-evidence/polish-2/lighthouse-mobile.json).

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
node scripts/verify-live.mjs \
  https://ci-provider-failover-drill.sociobot.in \
  .factory/qa-evidence/polish-2
```

## Known gaps and next steps

None for the reviewed scope. Registry publication remains a factory handoff
step; this work order correctly did not publish the Rust package.
