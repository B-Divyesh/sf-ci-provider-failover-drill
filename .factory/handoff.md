# CI Provider Failover Drill — polish 3 handoff

## Result

PASS. Every finding in `.factory/review-1.md`, `.factory/review-2.md`, and
`.factory/review-3.md` is closed. The product remains a Rust CLI with a static
Vite landing/docs site and its original topographic cartography identity.

The round-3 repair adds fail-closed wrapper-aware release classification and
network inference, adversarial claim fixtures, direct first-screen/demo copy,
observable checkout privacy wording, and cumulative regression coverage.
`/?demo=1` opens the isolated sample in one click with its banner and reset.

## Verification

From clean clone `/tmp/cifail-polish3-clean-jPKHe4` at repair commit
`efe9fd0`:

- All 16 commands declared in `.factory/claims.json` passed separately.
- `npm test` passed 5 Rust unit, 2 CLI integration, and 25 Chromium tests.
- `npm run lint` passed TypeScript, rustfmt, and clippy with warnings denied.
- `npm run build` produced `dist/site/` and the release CLI.
- `cargo package --locked` packaged and verified 15 files.
- Initial JavaScript is 21.68 KiB raw / 7.01 KiB gzip. CSS is 12.99 KiB raw /
  3.78 KiB gzip.

The browser suite covers every claim plus raw route metadata, History API
focus, a real HTTP 404, mobile 390 × 844 layout, 44 px touch targets, light and
dark axe checks, reduced motion, browser/CLI privacy, offline generation, and
demo/real-data isolation.

## Live evidence

Production: <https://ci-provider-failover-drill.sociobot.in>

Static deployment `c8c116bd-74e3-4566-aab5-7e917841cedb` published pushed
commit `a7e78341485d8e8892ffd65f2ed2caee0d66c534`. The final cold audit completed
at 29 August 2026 02:50 UTC. Local and live HTML, JavaScript, and CSS hashes
match byte for byte.

- Cold route and finding checks:
  `.factory/qa-evidence/polish-3/live-route-check.json`
- Landing and query-demo mobile screenshots:
  `.factory/qa-evidence/polish-3/live-landing-mobile-390.png` and
  `.factory/qa-evidence/polish-3/live-demo-mobile-390.png`
- Privacy and Terms screenshots:
  `.factory/qa-evidence/polish-3/live-privacy-desktop.png` and
  `.factory/qa-evidence/polish-3/live-terms-desktop.png`
- Factory URL check: no console errors or baseline accessibility defects;
  `.factory/qa-evidence/polish-3/verify.json`
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1.5 s, CLS 0, total blocking time 60 ms;
  `.factory/qa-evidence/polish-3/lighthouse-mobile.json`

The live route report verifies route-specific raw and rendered metadata,
status codes, one `h1` and `main`, same-origin demo traffic, preserved real
storage, route focus, checkout redirect, and the absence of every superseded
claim. The complete finding-to-change-to-evidence map is in
`.factory/polish-3.md`.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

Run each command in `.factory/claims.json` separately for the claims gate.
Deploy `dist/site/` with the static work-order configuration.

## Known gaps and next steps

None for the reviewed scope. Registry publishing remains a factory handoff
step; no package was published and no infrastructure or billing setting was
changed from this repository.
