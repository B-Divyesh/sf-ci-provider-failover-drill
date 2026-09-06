# Prove one CI job runs elsewhere — independent review 5

## Verdict: FAIL

- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Implementation candidate: `efe9fd0fa693980394cbd4d650c02d24f690a944`
- Documentation reviewed: `f51f7d59eeab5469b7c8cffd3395385626327a14`
- Reviewed: 6 September 2026 UTC
- Findings: 2 blocking, 4 high, 2 medium, 0 low
- Finding count: 8
- Untested claim count: 1

The product is not ready. The live site and all declared commands work on their
registered samples, but adversarial checks disprove three safety/privacy
claims. The installed binary also loses its demo when the Cargo source cache is
removed. One core Docker runtime claim remains incompletely tested.

## Job, audience, and first action

Before scrolling at 390 × 844 and 1440 × 900:

- Job: prove one GitHub Actions job can run on another runner.
- Audience: maintainers who need a critical job during a GitHub outage.
- First action: **Try it with sample data**.

The page states all three directly. The action note defines the output as a
five-file drill packet and says the sample has one blocked publish step. The
three facts fit inside the phone viewport at y=746–796. Neither viewport had
horizontal overflow or a console error.

## Findings

### Blocking

#### F-1-1 — reopened: command substitution bypasses release blocking

The registered `release-safety` command passes, but its coverage is incomplete.
A valid workflow step was tested without `--allow-release`:

```yaml
- name: Publish package
  run: echo `npm publish`
```

`cifail` returned exit 0 with `ready: true`, `commands_included: 4`, and
`commands_blocked: 0`. The generated executable file contains:

```sh
echo `npm publish`
```

The shell runs `npm publish` during command substitution. The implementation
does not parse backtick substitutions inside arguments, so its advertised
default can publish.

Required fix: parse or conservatively block all command substitutions. Add
backticks and indirect command launchers to the claim fixture, and assert that
the generated script cannot execute them without the release flag.

#### F-1-2 — reopened: the same command produces a false network report

The same isolated workflow replaced every other npm command with local `echo`
commands. Its JSON result contained `network_hosts: []`, and `report.md` said
“Network hosts — None.” The generated script still executes `npm publish` and
therefore needs npm registry access.

Required fix: use the same complete command model for release classification
and network inference. The adversarial fixture must require
`registry.npmjs.org` in both JSON and Markdown.

### High

#### F-5-3 — a common environment mapping leaks a secret name

The `secret-redaction` command passes only because the sample maps secret
`NPM_TOKEN` to a differently named variable, `NODE_AUTH_TOKEN`. A valid and
common mapping was tested:

```yaml
env:
  NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
```

The generated `.env.example` used `DRILL_SECRET_1`, but `run.sh` contained:

```sh
export NPM_TOKEN="$DRILL_SECRET_1"
```

This contradicts the registered claim that secret names do not appear in any
generated packet file. The test scans only the favorable sample.

Required fix: separate anonymous packet inputs from workflow environment
names without printing the GitHub secret identifier, and add fixtures where
the environment key equals the secret identifier.

#### F-5-4 — the installed binary does not contain its claimed bundled demo

The exact documented Git install succeeded at `f51f7d5`, and `cifail demo
--json` initially worked. The binary resolves the sample through the absolute
compile-time `CARGO_MANIFEST_DIR`. After the isolated Cargo source cache was
moved away, the same installed binary exited 2:

```text
cifail: No such file or directory (os error 2)
```

The same result occurred after installing the packaged crate and moving its
source directory. The claim test runs `cargo run` inside the source checkout,
so it cannot detect this installed-artifact failure.

Required fix: embed the sample at compile time or generate it from embedded
constants. Test a copied installed binary after removing or moving every source
and Cargo cache directory.

#### F-5-5 — working-directory state leaks into later workflow steps

A valid job with a first step using `working-directory: src` and a second step
expecting the default workspace was reported as ready. The generated script is:

```sh
cd /workspace
cd 'src'
pwd
test "$PWD" = /workspace
```

GitHub Actions starts each step in its configured or default working directory.
The packet runs all steps in one shell and never restores `/workspace`, so the
second step fails even though the source job is valid. Step-level environment
exports likewise remain active for later steps.

Required fix: execute each translated step in an isolated subshell or reset its
directory and step environment explicitly. Add a multi-step claim fixture that
observes both directory and environment isolation.

#### F-5-6 — `runner-contract` does not test that a packet runs

The public claim says the packet can run on a laptop or any runner with Docker.
Its only test checks that the Dockerfile contains a `FROM` line and that the
report contains two strings. It never builds or runs the packet.

Docker 29.1.3 was installed for this review. The pinned image exists and began
downloading, but this worker kernel rejected layer registration with `unshare:
operation not permitted`. That environment limit is not a product defect, but
it means the real runtime outcome was not independently established. The claim
test remains incomplete under the claims contract.

Required fix: run a no-secret sample packet in a Docker-capable clean runner and
assert its observable exit and output. Keep a controlled Docker-failure test as
a separate recovery check.

### Medium

#### F-5-7 — the demo label and controls are not persistent

The phone demo starts with “Demo — sample data, nothing is saved,” **Reset
demo**, and **View install command**. After scrolling, the banner box moved from
`y=0` to `y=-486` and was outside the viewport. Its CSS position is `relative`.

The demo contract requires a persistent sample label with reset and exit
controls. Make the banner sticky or fixed without covering content, including
safe-area handling on phones.

#### F-5-8 — a rate limit is reported as an invalid license

The product-specific verification endpoint allowed 30 sequential requests and
returned 429 on request 31 with `Retry-After: 3`, which is correct server
behavior. When the page received a recorded 429 response during license
restore, it removed the stored token and said:

```text
This license is no longer active. Check the token or buy Team.
```

A temporary throttle is not an invalid or revoked license. The page ignores
the HTTP status and `Retry-After`, then treats missing `valid` data as false.

Required fix: only remove a license after a successful verification response
with `valid: false`. Keep the token on 429, show when to retry, and test recovery
after the delay.

## Demo and real-data isolation

The one-click landing action opened `/?demo=1`. The first populated screen
showed the `release-check` workflow, 3 included shell steps, 1 blocked release
step, 1 anonymous input, `registry.npmjs.org`, and all five packet files.

The banner text was present before scrolling. Reset removed a seeded `demo:`
key and preserved a real-data sentinel and stored license. Exit cleared demo
data and opened the install section. All browser demo requests stayed on the
product origin. The banner persistence failure is F-5-7.

From a temporary caller directory, the source-run CLI demo created a new
`/tmp/cifail-demo-*` packet and left the caller sentinel unchanged. The
installed-artifact failure is separately recorded as F-5-4.

## Declared claims

A no-local clone at documentation SHA `f51f7d5` received `npm ci`. Every exact
command in `.factory/claims.json` then ran separately. All 16 commands exited
0, and each claim ID has exactly one tagged test.

| Claim | Declared command | Review result |
| --- | --- | --- |
| `packet-generation` | PASS | Observable five-file sample passed. |
| `release-safety` | PASS | Incomplete and disproved by F-1-1. |
| `secret-redaction` | PASS | Incomplete and disproved by F-5-3. |
| `offline-generation` | PASS | Proxy-denied local generation passed. |
| `local-privacy` | PASS | Connection recorder saw no CLI attempt. |
| `cli-demo-isolation` | PASS | Source run passed; installed binary fails in F-5-4. |
| `no-ci-mutation` | PASS | Sample repository snapshot stayed unchanged. |
| `demo-sandbox` | PASS | Demo storage isolation passed. |
| `privacy-local` | PASS | Demo traffic stayed same-origin. |
| `paid-license` | PASS | Recorded valid response stored and verified a token. |
| `team-history` | PASS | Export, delete, import, and restore passed. |
| `paid-contract` | PASS | Price, current scope, checkout, and free core passed. |
| `runner-contract` | PASS | String inspection only; runtime remains untested in F-5-6. |
| `inspection-report` | PASS | Incomplete and disproved by F-1-2. |
| `exit-codes` | PASS | Input, safety, and Docker failure codes passed. |
| `license-verdict-cache` | PASS | Matching, stale, and replaced token cases passed. |

There is one untested public outcome: successful execution of the generated
packet on real Docker. Three other registered claims have tests but are false
outside their current fixtures.

## Clean build and installed CLI

- `npm ci`: PASS, 22 packages and no reported vulnerabilities.
- `npm test`: PASS, 5 Rust unit tests, 2 CLI tests, and 25 Chromium tests.
- `npm run lint`: PASS, TypeScript, formatting, and Clippy with warnings denied.
- `npm run build`: PASS; release binary and `dist/site/` produced.
- `cargo package --locked --allow-dirty`: PASS, 15 files and 19.3 KiB compressed.
- Exact Git install: PASS at `f51f7d5`; `--help`, `--version`, and the initial
  demo worked.
- Installed binary after source/cache removal: FAIL as F-5-4.
- Normal packet generation: PASS on the bundled sample.
- Invalid job, unpinned image, missing path, malformed input, safety stop, and
  controlled Docker failure: clear errors and documented exit codes passed.
- Real Docker run: not completed because this worker forbids container layer
  `unshare`; this is reflected in F-5-6 rather than blamed on the product.

## Live routes, accessibility, privacy, and performance

- `/`, `/demo`, `/team`, `/privacy`, and `/terms` returned 200.
- `/missing-place` deliberately returned HTTP 404 with the designed page and a
  way home. The expected 404 is not a defect.
- Each route had its own title, description, canonical, Open Graph and Twitter
  data, one `h1`, one `main`, and `lang="en"`.
- Internal routes, robots, sitemap, favicon, touch icon, social card, hero, and
  terminal art returned 200. Sociobot terms returned 200. Checkout returned
  the expected 303 hosted-checkout redirect.
- Fresh live axe scans found no serious or critical issue on any route. The
  local suite also passed dark mode. Focus rings, skip navigation, history/back
  focus, 44 px phone controls, native required-field validation, and reduced
  motion passed manual or automated checks.
- The factory URL verifier passed in 743 ms with no console, title, language,
  landmark, alt-text, or button-label defect.
- Invalid report JSON and invalid history JSON produced specific recovery
  instructions; a later valid report saved successfully.
- The privacy route provides `privacy@sociobot.in`. The site loaded no external
  script or font. Demo traffic was same-origin. License requests went only to
  the documented Sociobot endpoint.
- The product is not a PWA and promises no website offline/update behavior.
  CLI offline generation passed. There is no first-party backend, tenant,
  server-side product state, or restart persistence to test.
- Security headers include CSP with header-only `frame-ancestors`, HSTS,
  nosniff, strict-origin referrer policy, and a restrictive permissions policy.
- Mobile Lighthouse: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; FCP 0.87 s, LCP 1.59 s, TBT 61 ms, CLS 0.
- Initial JavaScript is 21,676 bytes raw / 7.01 KiB gzip. CSS is 12,985 bytes
  raw / 3.78 KiB gzip. Total Lighthouse transfer was 152,083 bytes.

## Live candidate identity

The live hashed JavaScript, CSS, topographic image, terminal SVG, social card,
favicon, and touch icon all SHA-256 matched the build from the clean checkout.
The last product implementation change is `efe9fd0`; later commits changed
tests or reports. The live runtime therefore matches the implementation
candidate. The documentation SHA reviewed before this report is `f51f7d5`.

## Earlier finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| F-1-1 | **Reopened.** Backtick command substitution bypasses the release block. |
| F-1-2 | **Reopened.** The same command produces a false empty network list. |
| F-1-3 | Fixed. The syscall connection-recorder claim passed. |
| F-1-4 | Fixed for source-run isolation; F-5-4 covers the newly tested installed artifact. |
| F-1-5 | Fixed. Raw and rendered route metadata is specific. |
| F-1-6 | Fixed. Team copy is browser-local. |
| F-1-7 | Fixed. Unsupported “safe” wording remains absent. |
| F-1-8 | Fixed. The action and three facts fit at 390 × 844. |
| F-1-9 | Fixed. The headline and packet section name the job directly. |
| F-1-10 | Fixed. Decorative map labels remain out of readable copy. |
| F-1-11 | Fixed. README exit meanings remain four short sentences. |
| F-1-12 | Fixed. Job name, anonymous input, and GitHub-only action terms are consistent. |
| F-1-13 | Fixed. The demo exit says “View install command.” |
| F-1-14 | Fixed. Checkout wording is observable and the linked policy returns 200. |
| F-1-15 | Fixed. Local export/import/restore passed. |
| F-1-16 | Fixed. Source-repository mutation test passed. |
| F-2-1 | Fixed. The unbounded future-update entitlement is absent. |
| F-3-3 | Fixed. The unsupported payment-data statement is absent. |
| F-3-4 | Fixed. The preview label says “Sample drill result.” |
| F-3-5 | Fixed. “Five-file drill packet” defines the output on first use. |
| F-3-6 | Fixed. The demo heading names the sample packet. |

Earlier verification defects are also closed: clean claim setup, checkout,
mobile demo width, dark contrast, claim registration, invalid-license token
replacement, exit-code classification, real 404 status, strict TypeScript,
touch targets, and immutable hashed-asset caching all passed. The earlier
factory verifier timeout did not recur in fresh browser checks. F-5-8 is a new
rate-limit recovery defect, not the old replacement-token bug.

## Site copy and missing features

The live landing and README still match the recorded copy audit. No sentence
exceeds 22 words, no banned marketing word or metaphor heading was found, and
the terminology remains consistent. Every claim-like sentence maps to a
registered claim, though four registered tests are incomplete as described
above.

No AI feature is warranted. Workflow parsing and release classification need
deterministic behavior. Team history already supports the useful implied
import/export path.

## Required next work

Close all eight findings, add the missing adversarial and installed-artifact
tests, and run a real no-secret packet to completion on Docker. Then repeat all
16 exact claim commands, the full clean build, phone and desktop live checks,
and every earlier finding disposition. PASS requires zero findings and zero
untested claims.
