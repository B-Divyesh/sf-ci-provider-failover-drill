# Handoff\n\n(written by the worker at the end of each work order)
# CI Provider Failover Drill v0.1 handoff

## What was built

- Rust `cifail` single binary with `drill`, `demo`, `--json`, useful help, and
  documented exit behavior.
- Selection and validation of one GitHub Actions job.
- A pinned `Dockerfile`, translated `run.sh`, anonymous `.env.example`,
  machine-readable `drill.json`, and human `report.md`.
- Required-file checks, network-host inventory, provider-action assumptions,
  job-context warnings, and optional Docker execution.
- Default blocking for publish, push, release, deploy, and apply commands.
- Secret-expression replacement with anonymous inputs. Generated packet files
  do not include the workflow's secret names or values.
- A bundled Node release-check repository and isolated `cifail demo` flow.
- A Vite site with `/`, `/demo`, `/team`, `/privacy`, `/terms`, and styled 404
  routes. History navigation, route focus, and live announcements are wired.
- One-time $49 Team checkout and restore flow through Sociobot. License results
  cache for one day. Team report history stays in local storage.
- Original topographic hero art, optimized to a 135 KB WebP. Full generation
  provenance is in `.factory/assets/topographic-route.json`.
- Claim registry, copy audit, demo contract, responsive design, security
  headers, sitemap, robots file, social card, and favicons.

## Run and verify

```sh
npm install
npm test
npm run build
cargo run -- demo
cargo package --allow-dirty
```

`npm run build` creates the release binary under `target/release/` and the
deployable site at `dist/site/`. The static root contains `index.html`.

Verification completed on 28 August 2026:

- `cargo test --locked`: 3 passed.
- Playwright claim, route, mobile, and accessibility suite: 11 passed.
- Axe serious and critical findings: 0 across all six route states.
- Factory `verify-url.sh`: HTTP 200, one h1, `lang=en`, main landmark, complete
  image alt text, no browser console errors. Local measured load: 550 ms.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100. LCP 2.0 s, CLS 0, total blocking time 0 ms.
- Initial JavaScript: 19.99 KB raw / 6.68 KB gzip.
- CSS: 11.99 KB raw / 3.60 KB gzip.
- Hero: 135 KB WebP. Full deploy output: 336 KB.
- `npm audit`: 0 vulnerabilities after updating Vite to 7.3.6.
- Desktop and 390×844 mobile screenshots were inspected.

## Known gaps

- The CLI translates shell steps. It reports provider actions as assumptions
  instead of trying to reproduce arbitrary marketplace action code.
- Docker is absent in the factory container, so `--execute` error handling is
  implemented but the container run was not exercised here.
- Expression contexts outside `secrets.*` become anonymous context variables.
  An operator must fill them before execution.
- Scheduled drills are not included in v1. Team provides local history and an
  organization report template; scheduling still needs an organization runner.
- Release execution is intentionally possible only with `--allow-release`.
  Operators should use a disposable registry or test target.

## Next steps

1. Register the product and return URL with the Sociobot billing factory.
2. Publish release binaries after factory signing and registry approval.
3. Pilot against varied release jobs and add translators only for repeated
   provider-action patterns.
4. Exercise `--execute` in CI with Docker and a disposable package registry.
