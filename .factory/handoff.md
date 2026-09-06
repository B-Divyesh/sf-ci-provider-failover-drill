# CI Provider Failover Drill — review 5 handoff

## Result

Review 5 is **FAIL** with 8 findings: 2 blocking, 4 high, and 2 medium. One
public runtime claim remains untested. No product code changed.

The report is `.factory/review-5.md`. The implementation candidate is
`efe9fd0fa693980394cbd4d650c02d24f690a944`; the pre-review documentation SHA
is `f51f7d59eeab5469b7c8cffd3395385626327a14`.

## What was verified

- Fresh live phone and desktop first reads, populated demo, reset isolation,
  same-origin traffic, route metadata, links, legal pages, and designed 404.
- Keyboard focus, reduced motion, light/dark axe checks, invalid input recovery,
  security headers, rate limiting, factory URL verification, and mobile
  Lighthouse.
- All 16 exact claim commands from a no-local clean clone.
- `npm test`, `npm run lint`, `npm run build`, and
  `cargo package --locked --allow-dirty`.
- The exact Git install and a packaged install in isolated consumer roots.
- Normal, invalid, boundary, safety, and controlled Docker failure paths.
- Every finding from reviews 1–4 and verifications 1–3.
- Live assets hash-match the implementation candidate build.

## Findings to fix

1. Block backtick command substitutions that can execute release commands.
2. Report network hosts inside those substitutions.
3. Do not emit a GitHub secret identifier when it matches an environment key.
4. Embed the CLI demo sample in the binary instead of reading the build source.
5. Restore working directory and step environment between translated steps.
6. Make `runner-contract` execute a packet on real Docker.
7. Keep the demo label and reset/exit controls visible while scrolling.
8. Preserve a license on 429 and honor `Retry-After`.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked --allow-dirty
```

Run every command in `.factory/claims.json` separately. Test the installed
binary after removing its source/cache, and run a no-secret generated packet to
completion on a Docker-capable host.

## Environment limit

Docker 29.1.3 was installed. The pinned image exists, but this worker kernel
rejected layer registration with `unshare: operation not permitted`. This did
not hide a claim failure: the current `runner-contract` test only checks text
and still needs a real runtime assertion.
