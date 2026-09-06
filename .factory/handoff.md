# CI Provider Failover Drill — repair 2 handoff

## Status

Implementation SHA: `1c48c1eb05f3c3335fc5abac6f8223fb603ac564`.

Verification documentation SHA: `03372b3182984df5ef4e9ce945ee39e823d40e21`.

The static deployment used the implementation SHA above. Deployment
`bccd359b-f9d3-4ffe-bb5a-3740114301f3` completed successfully on 6 September
2026 UTC.

## What changed

- The shell inspection now follows backtick and `$()` command substitutions.
  Release commands inside them are blocked by default and contribute their
  network hosts to JSON and Markdown reports.
- Packet exports anonymize a workflow environment key when it matches a GitHub
  secret identifier. Generated files now contain `DRILL_ENV_*` and
  `DRILL_SECRET_*` labels instead of that identifier.
- Each translated step runs in its own subshell from `/workspace`. Its working
  directory and step environment cannot affect the next step.
- `cifail demo` embeds the shipped sample at compile time. A copied installed
  binary still generated the sample after the build sample and Cargo source
  cache were moved away.
- The demo banner is sticky with phone safe-area padding. Reset and exit
  controls remain visible after scrolling.
- A 429 license verification preserves the stored token, honors `Retry-After`,
  and retries after the delay. Invalid or revoked results still remove a token.

## Review findings

F-1-1, F-1-2, F-5-3, F-5-4, F-5-5, F-5-7, and F-5-8 are repaired with
outcome-based regression coverage. The `runner-contract` claim now generates
and executes a no-secret packet, then requires an executed PASS report on a
Docker-capable runner.

The current worker installed Docker 29.1.3 and started a private temporary
daemon. Its kernel rejected Docker build isolation with
`unshare: operation not permitted`. This is a worker capability limit, not a
simulated pass: the Docker claim command was executed and failed at the real
build. Run the documented command below on a Docker-capable host before
declaring that one runtime claim independently passed.

## How verified

From a fresh clone at the implementation SHA:

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked --allow-dirty
```

Those commands completed successfully. `npm test` reports 26 passed and one
Docker-gated runtime assertion skipped; the 15 non-Docker declared claim
commands were also run separately. The exact Docker claim is:

```sh
CIFAIL_DOCKER_RUNTIME=1 npm run test:claims -- --grep @claim:runner-contract
```

It needs a usable Docker daemon. The explicit controlled-Docker failure path
still passes the documented exit-code test.

Consumer checks passed for both `cargo install --path .` and an exact Git
install at the implementation SHA. The Git-installed binary completed
`cifail demo --json` after its isolated Cargo source cache was moved away.

Live HTTPS checks passed for `/`, `/demo`, `/team`, `/privacy`, `/terms`, and a
designed HTTP 404. Fresh phone and desktop reads show the job, audience, and
**Try it with sample data** before scrolling. The demo was populated, kept its
sticky sample label and controls after scrolling, reset only `demo:` keys, and
made same-origin requests. Checkout returned its expected hosted redirect.

The live route report and screenshots are in
`.factory/qa-evidence/repair-1c48c1e/`. The factory URL verifier passed in
619 ms with no console error. Live light and dark axe scans found no serious or
critical violation. Mobile Lighthouse: Performance 99, Accessibility 100, Best
Practices 100, SEO 100; FCP 0.9 s, LCP 1.7 s, TBT 90 ms, CLS 0.

The license endpoint allowed 30 sequential verification requests and returned
429 with `Retry-After: 3` on request 31. The site test records 429 recovery
without spending or storing a real license.

## Remaining work

- Run the Docker runtime claim on a host whose kernel permits Docker container
  isolation. No product behavior is being claimed as passed from this worker
  for that final runtime execution.
