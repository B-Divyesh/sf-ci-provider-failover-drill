# Polish 1 handoff

Repairs the release candidate using every finding in review 1. The CLI now
recognizes release commands independent of option order or documented aliases,
uses that result for network reporting, and keeps all release forms out of a
packet unless `--allow-release` is explicit. The site has direct first-screen
wording, isolated `?demo=1` entry, route-specific static metadata, a real 404,
mobile first-screen facts, and local Team history JSON export/import.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked --allow-dirty
```

`npm test` passed with 6 Rust tests and 23 Playwright tests. `npm run lint`,
`npm run build`, and `cargo package --locked --allow-dirty` passed. All 16
declared claim commands were run individually from a clean dependency install;
each rebuilds its CLI and static site before its tagged test. The local
connection-recorder claim observed no connection attempts from `cifail drill`
or `cifail demo`.

The static deployment directory is `dist/site`. Deploy with:

```sh
/opt/fleet/lib/deploy-static.sh ci-provider-failover-drill dist/site
```

## Deployment and live verification

Commit `5603cd5fa5887b241f7a4483fdcb5d369399a0df` was deployed through the
static work-order configuration as deployment
`29ca4110-a525-4131-9a24-3ba8f1fbdeb7`.

- Cold live checks returned 200 for `/`, `/demo`, `/team`, `/privacy`, and
  `/terms`; `/missing-place` returned 404.
- `/opt/fleet/lib/verify-url.sh` recorded no console errors, title/lang/main,
  one h1, and complete image alt text at
  `.factory/qa-evidence/polish-1/verify.json`.
- Playwright axe checks on all routes in mobile light mode and landing dark mode
  found zero serious or critical findings. No checked route overflowed 390 px.
  See `.factory/qa-evidence/polish-1/live-route-check.json`.
- Cold `?demo=1` had the persistent sample banner, Reset demo, and View install
  command. The three landing facts were all inside the 390×844 first viewport.
  See `.factory/qa-evidence/polish-1/live-demo-query.json` and
  `.factory/qa-evidence/polish-1/live-first-screen.json`.
- The locally run axe CLI could not locate a Selenium Chrome binary, but the
  repository’s Playwright axe suite ran against the installed Playwright
  Chromium and passed locally and live.

No known product gaps remain.
