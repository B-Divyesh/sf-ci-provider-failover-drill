# Adversarial first-read review 1

## Verdict: FAIL

- Product: CI Provider Failover Drill
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Candidate: `bc742b3809e53b5f4dfbff6c6094deb845bc0cd8`
- Reviewed: 28 August 2026 UTC
- Work order: `ci-provider-failover-drill-review-1`
- Findings: 2 blocking, 5 high, 9 medium

The first screen and one-click demo are understandable and usable. The product
still fails because a valid npm publish command bypasses the advertised safety
block. The same command also defeats the advertised network-needs report. The
listed tests pass because they cover only the exact `npm publish` spelling in
the bundled sample.

## Findings

### Blocking

#### F-1-1 — a valid publish command bypasses the release safety block

**Exact claims:** landing fact “Release steps stay blocked”; boundary “Publish
commands need an explicit flag”; README “Release and publish commands stay
blocked unless you pass `--allow-release`”; claim `release-safety`.

The command below is valid npm syntax and was tested without
`--allow-release`:

```yaml
- name: Publish with npm flags
  run: npm --access public publish
```

The CLI returned `ready: true`, `commands_included: 1`, and
`commands_blocked: 0`. Generated `run.sh` contained:

```sh
npm --access public publish
```

The implementation searches for the literal substring `npm publish`. Moving a
valid npm option before the subcommand bypasses it. A maintainer relying on the
stated default can publish from a drill packet.

**Fix:** parse shell commands into executable and arguments, then recognize
publish/release operations regardless of option order and aliases. Cover at
least `npm --access public publish`, `npm pub`, `pnpm publish`,
`yarn npm publish`, line continuations, `git push --tags`, and the documented
release tools. Keep an unknown/high-risk command blocked or explicitly marked
for review. Add each form to `@claim:release-safety` and assert it is absent
from `run.sh` without the flag and present only with the flag.

#### F-1-2 — a valid npm command produces a false “Network: None” report

**Exact claims:** README “It checks files, secret references, network needs,
and provider-only actions before an outage makes them urgent”; claim
`inspection-report`.

The same `npm --access public publish` workflow produced
`network_hosts: []`, and `report.md` said “Network hosts — None.” Publishing to
npm requires registry access. The network detector also depends on literal
substrings such as `npm publish`, so the packet reports a false requirement.

**Fix:** use the parsed command model from F-1-1 for network inference. Assert
that option order and aliases still report `registry.npmjs.org`. Expand
`@claim:inspection-report` beyond the single bundled spelling.

### High

#### F-1-3 — the CLI privacy test does not prove the privacy copy

**Exact copy:** README “The free CLI has no telemetry and makes no product
requests”, “Inputs and drill packets stay on your machine”, and “Docker or
package commands may use the network only when you explicitly execute a
packet”; Privacy heading “Your workflow stays on your machine.”

The `offline-generation` test only points `HTTP_PROXY` and `HTTPS_PROXY` at an
unreachable address. That does not detect direct sockets, DNS, protocols that
ignore those variables, or telemetry sent without a proxy. The claim contract
requires observable egress evidence for privacy claims.

**Fix:** add a `local-privacy` claim and run both `cifail drill` and
`cifail demo` in a sandbox that records or denies all network syscalls. Assert
zero connection attempts and assert all created files remain under the chosen
packet or demo directory.

#### F-1-4 — CLI demo isolation and sample-equivalence claims are unlisted

**Exact copy:** README “Nothing touches your repo” and “The website recording
uses the same bundled sample”; terminal SVG “nothing was saved to your
repository.” The existing `demo-sandbox` entry explicitly tests only a fresh
browser context at `/demo`.

Manual verification passed: running `cargo run -- demo` from a temporary
caller directory left a sentinel unchanged and created the packet under a new
`/tmp/cifail-demo-*` directory. That does not satisfy the requirement that
every public claim have a registered repeatable test.

**Fix:** add `cli-demo-isolation` to `claims.json`. Its test should run the
literal documented command from a non-repository temporary directory, snapshot
that directory before and after, validate the reported temporary path, and
compare the CLI sample facts with `/demo`.

#### F-1-5 — cold deep-link metadata identifies every route as the landing page

**Location:** raw HTML responses for `/demo`, `/team`, `/privacy`, `/terms`,
and an unknown route.

Every response contains the landing values before JavaScript runs:

```html
<title>CI Failover Drill — prove one job runs elsewhere</title>
<link rel="canonical" href="https://ci-provider-failover-drill.sociobot.in/" />
<meta property="og:title" content="Prove your CI escape route" />
```

The SPA updates the title, description, and canonical in a browser, but it
never updates Open Graph/Twitter fields. Non-JavaScript crawlers canonicalize
Privacy, Terms, Demo, Team, and 404 to `/` and show the landing social card
copy.

**Fix:** make `prepare-static-routes.mjs` write route-specific title,
description, canonical, Open Graph, and Twitter metadata into every static
route file. Add response-body assertions for every route, not only post-script
DOM assertions.

#### F-1-6 — “shared drill record” promises sharing that Team does not provide

**Exact quote/location:** landing Team heading “Keep a shared drill record.”
The supporting copy says “browser-only drill history,” and `/team` stores data
only in `localStorage`.

A buyer can reasonably read “shared” as usable by more than one person or
browser. No share, sync, import, or history export exists.

**Fix:** for the current product, rewrite the heading to “Keep drill records
in this browser.” If sharing is intended, implement the local file round-trip
described in F-1-15 before restoring “shared.”

#### F-1-7 — “safe” is a broad, unsupported adjective

**Exact copy:** “See a safe release-check packet in one click”, “Inspect a
safe failover packet”, and metadata “safe, provider-neutral container drill.”

“Safe” does not name the protection, and F-1-1 demonstrates that a publish
command can remain executable. It therefore overstates the result.

**Fix:** use observable wording: “See a sample packet with one blocked
`npm publish` step” and “Inspect the sample failover packet.” Remove “safe” from
metadata until the intended safety boundary is defined and comprehensively
tested.

### Medium

#### F-1-8 — the three required first-screen facts are below the mobile fold

At 390×844, the primary action is visible, but the three facts begin at CSS
pixels 1050, 1050, and 1083. They appear after the map image, not on the first
screen. The test named “mobile first screen keeps its action and facts visible”
only uses Playwright `toBeVisible`, which does not mean inside the viewport.

**Fix:** place `.plain-facts` before the map at 390 px and assert each fact has
`top >= 0` and `bottom <= innerHeight` without scrolling.

#### F-1-9 — the primary headline and a section heading use a metaphor

**Exact copy:** “Prove your CI escape route” and “Make the escape route
repeatable.” A cold visitor can infer the meaning from surrounding copy, so
this is not the mandatory first-read blocker, but the headings do not name the
job directly.

**Fix:** use “Prove one CI job runs elsewhere” and “Generate and run the
packet.”

#### F-1-10 — decorative map lore adds labels with no product information

**Exact copy:** “Outage route / field check 01”, “PLATE 02”, “KNOWN ROUTE”,
“ROUTE 03”, “THREE LEGS”, “FIELD KIT”, and the decorative coordinates.

These strings would make sense on the map art but not in a heading/list read by
a visitor. They consume first-screen space and conflict with the requirement
that every line carry usable information.

**Fix:** remove them from readable copy or make purely visual marks
`aria-hidden`. Where a label is needed, use “Sample result”, “How it works”,
and “Install.”

#### F-1-11 — one README sentence exceeds the 22-word cap

**Exact quote:** “The command exits `0` when the packet is ready or the drill
passes, `2` for input problems, `3` when safety checks block the drill, and `4`
when Docker returns a failure.” (31 words)

**Fix:** “Exit `0` means ready or passed. Exit `2` means bad input. Exit `3`
means blocked. Exit `4` means Docker failed.”

#### F-1-12 — product and technical terms change without explanation

**Locations:** page title “CI Failover Drill” versus product name “CI Provider
Failover Drill”; “job key” versus “job”; “anonymous input labels”, “anonymous
secret inputs”, and “secret inputs”; “provider actions” versus “provider-only
actions”; README heading “Use”; “publishable crate.”

The variations make a short, technical product harder to scan and break its
own terminology table.

**Fix:** use the full product name in metadata; use “job name”, “anonymous
input”, and “GitHub-only action” everywhere. Rename “Use” to “Generate and run
a packet.” Rewrite the introduction as “`cifail` turns one workflow job into
five files for a fixed Docker image.” Rewrite the inspection sentence as “It
lists required files and network hosts, and marks GitHub-only actions for
replacement.” Rewrite “publishable crate” as “Rust package can be published.”

#### F-1-13 — “Start for real” does not name the button result

**Location:** persistent demo banner. The button scrolls to the install
command; it does not start a drill.

**Fix:** rename it “View install command.”

#### F-1-14 — merchant and refund statements are not covered by a claim

**Exact copy:** “Sociobot is the merchant of record”, “Refunds are handled
there”, and Terms “Its checkout handles payment, receipts, and refunds.” The
`paid-contract` test confirms price text and a checkout URL. It does not verify
these legal/service statements, and “there” has no clear referent.

**Fix:** replace the landing text with the tested action “Payment opens
Sociobot checkout.” Link to an actual refund policy from Terms, or add a
maintainable contract check for the statements that remain.

#### F-1-15 — local Team history has no import/export round trip

The brief and paid Team section position drill history as an organization
record, but records are trapped in one browser. The downloadable item is a
blank Markdown template, not the saved history.

**Fix:** add “Export drill history” and “Import drill history” for a documented
JSON file, with merge/replace choice and validation. Keep it local-first and
add a claim test that exports, clears, imports, and reproduces the same records.
AI would not improve this workflow and should not be added.

#### F-1-16 — “no automatic cutover” and “does not change CI settings” are unlisted claims

**Exact copy:** landing “No automatic cutover” and README “It does not store
secrets, change CI settings, or cut over providers.” No claim entry asserts
that normal drill generation leaves the source repository and provider state
unchanged.

**Fix:** add a `no-ci-mutation` claim. Snapshot the source repository, run a
normal drill from a clean sandbox, and assert no source file changed and no
provider/network request occurred. Keep the existing direct wording after that
test exists.

## First-read result

### 390×844, before scrolling

- What it does, in my words: tests whether one GitHub Actions job can be
  represented as a Docker failover packet for another runner.
- For whom: GitHub Actions maintainers preparing one critical job for an
  outage.
- First click: **Try it with sample data**.

The exact visible text that answers these questions is “Prove your CI escape
route”, “For GitHub Actions maintainers who need one critical job to run during
an outage”, and “Try it with sample data”, followed by “See a safe
release-check packet in one click.” All three questions are answerable without
scrolling, so the mandatory first-read blocker is not raised. F-1-8 records the
separate first-screen facts failure.

### 1440×900, before scrolling

The same three answers are visible. The map caption also shows “Selected job →
pinned runner → drill report.” All three facts are visible on desktop.

There was no horizontal overflow and no console error in either context.

## Copy audit

Counts treat hyphenated terms and inline code as one word. Alt text is included
because it is landing-page copy. “—” means no copy finding.

### Landing page sentences

| # | Sentence | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove your CI escape route. | 5 | F-1-9 |
| 2 | For GitHub Actions maintainers who need one critical job to run during an outage. | 14 | — |
| 3 | See a safe release-check packet in one click. | 8 | F-1-7 |
| 4 | A mapped route crosses between two isolated CI provider regions. | 10 | — |
| 5 | One command reads one job. | 5 | — |
| 6 | It produces a packet for Docker on another runner. | 9 | — |
| 7 | Terminal demo showing a ready packet and one blocked release step. | 11 | — |
| 8 | Point the CLI at a workflow and job key. | 9 | F-1-12 |
| 9 | Review files, network hosts, anonymous inputs, and skipped actions. | 9 | F-1-12 |
| 10 | Use the pinned container on a laptop or independent Docker runner. | 11 | — |
| 11 | Build the single binary from this repository. | 7 | — |
| 12 | No automatic cutover. | 3 | F-1-16 |
| 13 | You decide where and when to run. | 7 | — |
| 14 | No secret storage. | 3 | F-1-3 |
| 15 | Reports use anonymous input labels. | 5 | F-1-12 |
| 16 | No universal conversion. | 3 | F-1-12 |
| 17 | Provider actions become named assumptions. | 5 | F-1-12 |
| 18 | No surprise releases. | 3 | F-1-1 |
| 19 | Publish commands need an explicit flag. | 6 | F-1-1 |
| 20 | Team adds browser-only drill history and organization report templates. | 9 | — |
| 21 | Sociobot is the merchant of record. | 6 | F-1-14 |
| 22 | Refunds are handled there. | 4 | F-1-14 |
| 23 | The free CLI keeps packet export and safety checks. | 9 | — |
| 24 | Prove one GitHub Actions job can run elsewhere. | 8 | — |

### README sentences

| # | Sentence | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove one GitHub Actions job can run on another runner. | 10 | — |
| 2 | `cifail` turns one selected workflow job into a pinned container command and a portable drill packet. | 16 | F-1-12 |
| 3 | It checks files, secret references, network needs, and provider-only actions before an outage makes them urgent. | 16 | F-1-2, F-1-12 |
| 4 | It does not store secrets, change CI settings, or cut over providers. | 12 | F-1-3, F-1-12, F-1-16 |
| 5 | Release and publish commands stay blocked unless you pass `--allow-release`. | 10 | F-1-1 |
| 6 | This is for maintainers whose critical checks or releases depend on GitHub Actions. | 13 | — |
| 7 | The generated packet can run on a laptop or any runner with Docker. | 13 | — |
| 8 | The command copies a sample repository to a new temporary directory, analyzes its `release-check` job, and prints the packet path. | 20 | — |
| 9 | Nothing touches your repo. | 4 | F-1-4 |
| 10 | The website recording uses the same bundled sample. | 8 | F-1-4 |
| 11 | Open `/demo` on the live site or run `npm run dev` and visit `http://localhost:5173/demo`. | 14 | — |
| 12 | Build the single binary from source. | 6 | — |
| 13 | The first release is `0.1.0`. | 5 | — |
| 14 | The factory publishes packages after handoff; this repository does not use registry credentials. | 13 | — |
| 15 | Generate a packet without running the selected commands. | 8 | — |
| 16 | `Dockerfile`: a pinned, provider-neutral runner image. | 6 | F-1-12 |
| 17 | `run.sh`: translated shell steps with anonymous secret inputs. | 8 | F-1-12 |
| 18 | `.env.example`: placeholder secret variables, never names or values. | 8 | F-1-12 |
| 19 | `drill.json`: machine-readable checks for reporting. | 5 | — |
| 20 | `report.md`: the local and generic-runner drill report. | 7 | F-1-12 |
| 21 | Inspect those files, then run the packet locally. | 8 | — |
| 22 | Execution needs Docker. | 3 | — |
| 23 | The command exits `0` when the packet is ready or the drill passes, `2` for input problems, `3` when safety checks block the drill, and `4` when Docker returns a failure. | 31 | F-1-11 |
| 24 | Add `--json` for script output. | 5 | — |
| 25 | Release commands such as `npm publish`, `docker push`, and `gh release create` are omitted by default. | 16 | F-1-1 |
| 26 | Use `--allow-release` only in a disposable test target. | 8 | — |
| 27 | Requires Rust 1.85+ and Node 20+. | 6 | — |
| 28 | `npm run build` compiles the release binary and writes the static site to `dist/site/`. | 14 | — |
| 29 | The factory deploys that directory as a static site. | 9 | — |
| 30 | `cargo package --allow-dirty` verifies the publishable crate. | 7 | F-1-12 |
| 31 | The free CLI has no telemetry and makes no product requests. | 11 | F-1-3 |
| 32 | Inputs and drill packets stay on your machine. | 8 | F-1-3 |
| 33 | Docker or package commands may use the network only when you explicitly execute a packet. | 15 | F-1-3 |
| 34 | The optional Team license is a one-time $49 purchase. | 9 | — |
| 35 | It adds saved drill history and organization report templates in the site preview. | 13 | F-1-12 |
| 36 | Scheduled runs need an organization runner and are planned, not included in v1. | 13 | — |
| 37 | Sociobot is the merchant of record. | 6 | F-1-14 |
| 38 | See Privacy and Terms. | 4 | — |
| 39 | MIT. | 1 | — |
| 40 | See LICENSE. | 2 | — |

### Headings, labels, and controls

| Copy | Result | Proposed replacement |
| --- | --- | --- |
| Prove your CI escape route | F-1-9: metaphor | Prove one CI job runs elsewhere |
| Outage route / field check 01 | F-1-10: decorative lore | GitHub Actions outage drill, or delete |
| PLATE 02 / KNOWN ROUTE | F-1-10: decorative lore | Sample result, or make visual-only |
| ROUTE 03 / THREE LEGS | F-1-10: decorative lore | How it works, or make visual-only |
| Make the escape route repeatable | F-1-9: metaphor | Generate and run the packet |
| FIELD KIT | F-1-10: decorative lore | Delete; “Install” already names the section |
| Keep a shared drill record | F-1-6: unsupported sharing | Keep drill records in this browser |
| Use (README heading) | F-1-12: meaningless out of context | Generate and run a packet |
| Start for real | F-1-13: action result is unclear | View install command |
| No secrets stored | F-1-3: broader than the current packet scan | Secret names are removed from packet files |
| Release steps stay blocked | F-1-1: false for a valid npm form | Fix the blocker before restoring this fact |
| Try it with sample data | Pass | — |
| Copy install command / Copy demo command | Pass | — |
| Buy Team for $49 / Restore a license / Verify license | Pass | — |

## Demo and sandbox

**Browser demo: PASS.** The landing action opens `/demo` in one click. At
390×844, the first demo screen contains a realistic `release-check` workflow,
the `release.yml` filename, and visible workflow commands. The loaded result
contains 3 included shell steps, 1 blocked release step, 1 anonymous input,
the npm registry host, and all five packet filenames. The banner says “Demo —
sample data, nothing is saved” and provides Reset and exit actions.

Reset changed its label to “Demo reset.” A pre-seeded `team:drills` record and
`real:sentinel` value were byte-for-byte unchanged. No `demo:` key was created.
The complete live request log contained only
`https://ci-provider-failover-drill.sociobot.in`. There were no console or page
errors.

**CLI demo: functional PASS, claims registration FAIL (F-1-4).** From a new
temporary caller directory, `cargo run --manifest-path /work/repo/Cargo.toml --
demo --json` created exactly `Dockerfile`, `run.sh`, `.env.example`,
`drill.json`, and `report.md` under `/tmp/cifail-demo-*/failover-packet`. It
left the caller sentinel unchanged. `run.sh` contained `DRILL_SECRET_1` and
neither `NPM_TOKEN` nor `npm publish`.

## Claim execution

The checkout was clean at the supplied base before `npm ci`. Every exact test
command in `.factory/claims.json` was then run separately. All listed commands
returned zero:

| Claim id | Listed test | Result |
| --- | --- | --- |
| `packet-generation` | `npm run test:claims -- --grep @claim:packet-generation` | PASS |
| `release-safety` | `npm run test:claims -- --grep @claim:release-safety` | PASS, but claim false outside narrow fixture: F-1-1 |
| `secret-redaction` | `npm run test:claims -- --grep @claim:secret-redaction` | PASS |
| `offline-generation` | `npm run test:claims -- --grep @claim:offline-generation` | PASS, insufficient privacy evidence: F-1-3 |
| `demo-sandbox` | `npm run test:claims -- --grep @claim:demo-sandbox` | PASS |
| `privacy-local` | `npm run test:claims -- --grep @claim:privacy-local` | PASS |
| `paid-license` | `npm run test:claims -- --grep @claim:paid-license` | PASS |
| `team-history` | `npm run test:claims -- --grep @claim:team-history` | PASS |
| `paid-contract` | `npm run test:claims -- --grep @claim:paid-contract` | PASS |
| `runner-contract` | `npm run test:claims -- --grep @claim:runner-contract` | PASS |
| `inspection-report` | `npm run test:claims -- --grep @claim:inspection-report` | PASS, but claim false outside narrow fixture: F-1-2 |
| `exit-codes` | `npm run test:claims -- --grep @claim:exit-codes` | PASS |
| `license-verdict-cache` | `npm run test:claims -- --grep @claim:license-verdict-cache` | PASS |

The full `npm test` run also passed 5 Rust tests and 20 Chromium tests. Passing
the sample assertions does not override the reproduced claim failures.

## Earlier-finding audit

No `.factory/review-*.md` or `.factory/polish-*.md` existed before this review.
The handoff and all three verification reports were read. Each earlier defect
was checked again on the live site and in the current code:

| Earlier finding | Current evidence | Result |
| --- | --- | --- |
| Claim commands time out from a clean checkout | `pretest:claims` builds first; all 13 exact commands passed | Fixed |
| Team checkout returns 404 | live endpoint returned 303 to hosted Dodo checkout | Fixed |
| `/demo` overflows at 390 px | `scrollWidth === innerWidth === 390` | Fixed |
| Dark install section has serious contrast failures | live light/dark axe scan on six routes: zero violations | Fixed |
| Runner, inspection, exit-code, and cache claims were absent | all four entries and tags exist | Fixed as registration; F-1-2 finds incomplete inspection behavior |
| Replacing an invalid license does not recover | code clears the verdict and forces verification; regression test passed | Fixed |
| Missing job exits 3 instead of 2 | CLI and automated test returned 2 | Fixed |
| Unknown route is a soft 404 | live `/missing-place` returned HTTP 404 with designed page | Fixed |
| No strict TypeScript check | `npm run typecheck` exists and passed | Fixed |
| Mobile targets below 44×44 | no undersized interactive element on six live routes at 390 px | Fixed |
| Hashed assets lack immutable caching | live JS and CSS return one-year immutable caching | Fixed |
| URL verifier is unstable | factory verifier completed in 744 ms with zero errors | Fixed |

The second verification's checkout-only blocker is the same fixed 303 result.
The third verification and prior handoff listed no defects; the new
adversarial command forms in F-1-1 and F-1-2 were not exercised there.

## Structure, accessibility, and links

- Root, Demo, Team, Privacy, Terms, and the designed 404 each render one `h1`,
  one `main`, `lang="en"`, and a route-specific browser title after script
  execution.
- Direct routes return 200; an unknown route returns 404. Deep links load,
  browser back restores the route, and route changes focus the new `h1`.
- Header and footer are consistent. Privacy and Terms are reachable. The
  sitemap lists all five real routes. Favicon, apple-touch icon, social card,
  robots, and sitemap all return 200.
- Every crawled same-origin link returned 200 or the intentional 404 test.
  Both `mailto:` links are explicit. Checkout returned 303 to the hosted
  provider.
- Live axe checks in light and dark mode found zero violations on all six
  routes. No interactive target measured below 44×44 at 390 px. Reduced motion
  sets smooth scrolling to `auto` and animation/transition durations to
  `0.01ms`.
- The topographic survey-map system is distinct and matches
  `.factory/design.md`; it is not a generic SaaS card/gradient layout.
- Initial JS is 20,100 bytes raw and 6,730 bytes gzip. No third-party font or
  script is loaded.
- The local production HTML, JS, and CSS SHA-256 values exactly match live:
  `67be3e…`, `b0e744…`, and `5a7400…`.

F-1-5 is the remaining structure failure; the post-script DOM checks conceal
incorrect cold response metadata.

## Missed leverage

F-1-15 is the one obvious missing extension: saved Team history needs a local
import/export round trip so an organization record can move between browsers
or people. The core job is deterministic workflow inspection; an AI step would
be decorative and would add privacy/cost complexity. No runtime AI or embedded
provider key was found.

## Verification commands

```sh
npm ci
# Every command from .factory/claims.json, run separately
npm test
npm run lint
npm run build
/opt/fleet/lib/verify-url.sh \
  https://ci-provider-failover-drill.sociobot.in /tmp/cifail-verify-url
```

Results: `npm test` passed 5 Rust and 20 Chromium tests; lint passed; the release
binary and `dist/site/` built; the live verifier passed. These gates do not
cover the adversarial command in F-1-1/F-1-2.

## What would make this perfect

Resolve every finding above, beginning with parsed-command safety and network
inference plus adversarial claim fixtures. Then make privacy tests observe all
CLI egress, register the CLI demo promises, emit correct metadata in each cold
route response, replace metaphors and decorative labels with direct copy, move
the facts into the mobile viewport, and add local Team history import/export.
Re-run the entire checklist from a clean checkout. The target is zero findings,
not merely a passing existing suite.
