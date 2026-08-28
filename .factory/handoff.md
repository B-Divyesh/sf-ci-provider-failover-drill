# Review 2 handoff

This was a non-code adversarial review. It added
`.factory/review-2.md` and updated this handoff; product source was not
changed.

## Result

**FAIL:** one high-severity commercial-copy finding remains. `/terms` promises
that a $49 Team purchase covers “future v1 updates,” but the entitlement is not
listed or testable in `.factory/claims.json`. See `F-2-1` in
`.factory/review-2.md` for the exact quote and rewrite.

## Verification run

```sh
npm ci
npm test
npm run lint
npm run build
```

All commands passed. `npm test` passed 6 Rust and 23 Chromium tests; the build
produced `dist/site/`. Every one of the 16 commands listed in
`.factory/claims.json` was also run separately from the clean dependency state
and passed.

Live fresh-context checks covered 390 × 844 and 1440 × 900 first reads, the
one-click browser demo and Reset, CLI demo isolation from a temporary caller
directory, outgoing-request logs, raw route metadata, deep links, Back/focus,
headers, link crawl, checkout redirect, and the prior review’s 16 findings.
All of those checks passed. The full evidence and copy inventory are in
`.factory/review-2.md`.

## Next step

Remove the untestable future-update promise or define its exact scope and add a
matching claim/test. Re-run the review after that change.
