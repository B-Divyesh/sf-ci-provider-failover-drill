# Review 3 handoff

## Result

FAIL. The review reopens F-1-1 and F-1-2 as blocking findings. A valid
`env npm publish` workflow step is emitted into `run.sh` without
`--allow-release`; when it is the only npm command, the report also says there
are no network hosts.

No product code was changed. The completed review is in
`.factory/review-3.md`.

## Verification performed

- Cold live checks at 390 × 844 and 1440 × 900.
- One-click browser demo, Reset isolation, real-key preservation, and
  same-origin request logging.
- CLI demo from a temporary caller directory.
- All 16 `.factory/claims.json` commands separately from clean clone
  `/tmp/cifail-review3-claims-wi6n7Q`; all declared tests passed.
- `npm test`: 6 Rust tests and 24 Chromium tests passed.
- `npm run lint`: passed.
- `npm run build`: passed and produced `dist/site/`.
- Live route, raw metadata, link, 404, focus, axe, and URL-verifier checks.
- Every earlier review and polish finding rechecked against live behavior and
  current source/tests.
- Adversarial wrapper fixture rerun with the clean-clone binary; it returned
  `ready: true`, `commands_blocked: 0`, `network_hosts: []`, and retained
  `env npm publish` in `run.sh`.

## Remaining work

Implement wrapper-aware release and network classification and add the exact
counterexample to `@claim:release-safety` and `@claim:inspection-report`.
Address F-3-3 through F-3-6, then rerun the full review from a clean clone.
