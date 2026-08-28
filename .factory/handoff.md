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

The deployed commit, live verification, screenshots, and Lighthouse evidence
are appended after the deployment check.
