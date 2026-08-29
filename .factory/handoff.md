# CI Provider Failover Drill — review 4 handoff

## Result

Review 4 is **PASS** with zero findings. No product code changed. This review
and this handoff are the only repository changes.

## What was verified

- Cold live first read at 390 × 844 and 1440 × 900.
- Browser demo entry, reset isolation, same-origin request log, and real-data
  local-storage sentinel preservation.
- `cargo run -- demo --json` from a temporary caller directory; it created the
  documented temporary sample packet and left the caller unchanged.
- All 16 registered claim tests from clean clone
  `/tmp/cifail-review4-clean-psaXDX` at `f8edf19`.
- `npm test` (5 Rust unit, 2 CLI, 25 Chromium), `npm run lint`, and
  `npm run build` from that clone.
- Metadata, links, history/back focus, designed 404, mobile layout,
  accessibility smoke checks, copy audit, and every earlier finding.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
```

Run the commands in `.factory/claims.json` for individual claim checks. Open
`/?demo=1` for the browser sample, or run `cargo run -- demo` in a temporary
directory for the CLI sample.

## Known gaps and next steps

None in the reviewed scope. Preserve claim coverage and the local-first demo
boundary when command parsing, data storage, or paid features change.
