# Adversarial first-read review 3

## Verdict: FAIL

- Product: CI Provider Failover Drill
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Candidate: `b7e18eebc30a85be83434b23929a2f15b8435fbd`
- Reviewed: 29 August 2026 UTC
- Work order: `ci-provider-failover-drill-review-3`
- Findings: 2 blocking, 1 high, 3 minor

The cold landing page and one-click demo are clear. All 16 registered claim
commands pass, as do the full test, lint, and build gates. The product still
fails because a valid shell wrapper bypasses the advertised release block and
causes a false network report. These reopen F-1-1 and F-1-2 under their
original IDs.

## Findings

### Blocking

#### F-1-1 — reopened in round 3 (round reference F-3-1): `env` bypasses release safety

**Exact claims:** landing “Release steps stay blocked”; boundary “Publish
commands need an explicit flag”; README “Release and publish commands stay
blocked unless you pass `--allow-release`”; claim `release-safety`.

A clean-clone binary was given a valid workflow step without
`--allow-release`:

```yaml
- run: env npm publish
```

The CLI returned `ready: true`, `commands_included: 4`, and
`commands_blocked: 0`. Generated `run.sh` contained `env npm publish`. The
implementation identifies only the first shell word as the executable, so it
sees `env` rather than the wrapped `npm` command. Running the packet can
publish despite the stated default.

**Concrete fix:** unwrap executable prefixes such as `env`, `command`,
`exec`, and `sudo`, including environment assignments, before classifying the
real command. Conservatively block a wrapper when its target cannot be parsed.
Add `env npm publish` and the other supported wrappers to
`@claim:release-safety`; assert that each is absent from `run.sh` unless the
explicit release flag is present.

#### F-1-2 — reopened in round 3 (round reference F-3-2): the same wrapper produces a false network report

**Exact claim:** README “It lists required files and network hosts, and marks
GitHub-only actions for replacement”; claim `inspection-report`.

In a clean temporary sample where the other npm steps were replaced by local
`echo` commands, `env npm publish` produced `network_hosts: []` and
`report.md` said “Network hosts — None.” The command requires an npm registry.
This can leave an independent runner without a required egress rule.

**Concrete fix:** feed wrapper-unwrapped command facts into both release
classification and network inference. Add the same `env npm publish` fixture
to `@claim:inspection-report` and require `registry.npmjs.org`.

### High

#### F-3-3 — the payment-data privacy statement is not registered or tested

**Exact quote/location:** `/privacy`, License checks: “The checkout site
handles payment details.”

No `.factory/claims.json` entry states who receives payment details. The
`paid-contract` test checks price, scope, free features, and the checkout link;
it does not test the payment-data boundary. A buyer can rely on this privacy
statement.

**Concrete fix:** replace it with the already registered, observable wording
“Payment opens Sociobot checkout.” If the stronger data-handling statement is
required, add a privacy claim and a checkout-boundary test before restoring it.

### Minor

#### F-3-4 — “The product” is a generic, information-free section label

**Exact quote/location:** landing preview eyebrow: “The product”.

The label could appear on any site and does not identify the section when
read out of context. The following heading already names the content.

**Concrete fix:** delete the eyebrow, or replace it with “Sample drill result.”

#### F-3-5 — the first-screen outcome introduces “packet” without defining it

**Exact quote/location:** first-screen action note: “See a sample packet with
one blocked npm publish step.”

“Packet” is a product-specific artifact, not a standard GitHub Actions term.
The README defines it as five files, but the first screen does not. A cold
visitor can choose the action, but cannot yet picture what will open.

**Concrete fix:** “See a sample five-file drill packet with one blocked npm
publish step.” Keep “packet” after this first definition.

#### F-3-6 — the demo result heading uses the route metaphor instead of the output name

**Exact quote/location:** `/demo` result heading: “The route is ready to
inspect”.

The user is inspecting a generated packet, not a route. The metaphor weakens
the otherwise consistent product terminology and makes the result heading less
useful out of context.

**Concrete fix:** “The sample packet is ready to inspect.”

## Cold first read

### 390 × 844, before scrolling

- What it does, in my words: checks whether one GitHub Actions job can run on
  another runner during an outage.
- For whom: GitHub Actions maintainers responsible for a critical job.
- First click: **Try it with sample data**.

The exact visible text was “Prove one CI job runs elsewhere,” “For GitHub
Actions maintainers who need one critical job to run during an outage,” and
“Try it with sample data.” The action note was also visible. All three facts
fit at y=746–796. There was no horizontal overflow or console error. The three
required questions are answerable, so this is not a first-read blocker.

### 1440 × 900, before scrolling

The same answers, action, three facts, and the caption “Selected job → pinned
runner → drill report” were visible. There was no console error or horizontal
overflow.

## Copy audit

Counts use visible whitespace-separated words; hyphenated terms count once.
Code commands are counted by their displayed words. No sentence exceeds 22
words and no banned marketing adjective appears.

### Landing-page sentences

| # | Sentence | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove one CI job runs elsewhere. | 6 | — |
| 2 | For GitHub Actions maintainers who need one critical job to run during an outage. | 14 | — |
| 3 | See a sample packet with one blocked npm publish step. | 10 | F-3-5: undefined product term |
| 4 | A mapped route crosses between two isolated CI provider regions. | 10 | —; image alt text |
| 5 | One command reads one job. | 5 | — |
| 6 | It produces a packet for Docker on another runner. | 9 | — after the first-use fix in F-3-5 |
| 7 | Terminal demo showing a ready packet and one blocked release step. | 11 | —; image alt text |
| 8 | Point the CLI at a workflow and job name. | 9 | — |
| 9 | Review files, network hosts, anonymous inputs, and GitHub-only actions. | 9 | — |
| 10 | Use the pinned container on a laptop or independent Docker runner. | 11 | — |
| 11 | Build the single binary from this repository. | 7 | — |
| 12 | No automatic cutover. | 3 | — |
| 13 | You decide where and when to run. | 7 | — |
| 14 | No secret storage. | 3 | — |
| 15 | Reports use anonymous input labels. | 5 | — |
| 16 | No universal conversion. | 3 | — |
| 17 | GitHub-only actions become named assumptions. | 5 | — |
| 18 | No surprise releases. | 3 | F-1-1 |
| 19 | Publish commands need an explicit flag. | 6 | F-1-1 |
| 20 | Team saves drill history locally. | 5 | — |
| 21 | Export it to move it between browsers. | 7 | — |
| 22 | Payment opens Sociobot checkout. | 4 | — |
| 23 | The free CLI keeps packet export and safety checks. | 9 | — |
| 24 | Prove one GitHub Actions job can run elsewhere. | 8 | — |

### README sentences and labelled file descriptions

| # | Sentence or labelled description | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove one GitHub Actions job can run on another runner. | 10 | — |
| 2 | `cifail` turns one workflow job into five files for a fixed Docker image. | 13 | — |
| 3 | It lists required files and network hosts, and marks GitHub-only actions for replacement. | 13 | F-1-2 |
| 4 | It does not store secrets, change CI settings, or cut over providers. | 12 | — |
| 5 | Release and publish commands stay blocked unless you pass `--allow-release`. | 10 | F-1-1 |
| 6 | This is for maintainers whose critical checks or releases depend on GitHub Actions. | 13 | — |
| 7 | The generated packet can run on a laptop or any runner with Docker. | 13 | — |
| 8 | The command copies a sample repository to a new temporary directory, analyzes its `release-check` job, and prints the packet path. | 20 | — |
| 9 | Nothing touches your repo. | 4 | — |
| 10 | The website recording uses the same bundled sample. | 8 | — |
| 11 | Open `/demo` on the live site or run `npm run dev` and visit `http://localhost:5173/demo`. | 14 | — |
| 12 | Build the single binary from source. | 6 | — |
| 13 | The first release is `0.1.0`. | 5 | — |
| 14 | The factory publishes packages after handoff; this repository does not use registry credentials. | 13 | — |
| 15 | Generate a packet without running the selected commands. | 8 | — |
| 16 | `Dockerfile`: a pinned, provider-neutral runner image. | 6 | — |
| 17 | `run.sh`: translated shell steps with anonymous inputs. | 7 | — |
| 18 | `.env.example`: placeholder secret variables, never names or values. | 8 | — |
| 19 | `drill.json`: machine-readable checks for reporting. | 5 | — |
| 20 | `report.md`: the local and generic-runner drill report. | 7 | — |
| 21 | Inspect those files, then run the packet locally. | 8 | — |
| 22 | Execution needs Docker. | 3 | — |
| 23 | Exit `0` means ready or passed. | 6 | — |
| 24 | Exit `2` means bad input. | 5 | — |
| 25 | Exit `3` means blocked. | 4 | — |
| 26 | Exit `4` means Docker failed. | 5 | — |
| 27 | Add `--json` for script output. | 5 | — |
| 28 | Release commands such as `npm publish`, `docker push`, and `gh release create` are omitted by default. | 16 | F-1-1 |
| 29 | Use `--allow-release` only in a disposable test target. | 8 | — |
| 30 | Requires Rust 1.85+ and Node 20+. | 6 | — |
| 31 | `npm run build` compiles the release binary and writes the static site to `dist/site/`. | 14 | — |
| 32 | The factory deploys that directory as a static site. | 9 | — |
| 33 | `cargo package --allow-dirty` verifies the Rust package can be published. | 10 | — |
| 34 | The free CLI has no telemetry and makes no product requests. | 11 | — |
| 35 | Inputs and drill packets stay on your machine. | 8 | — |
| 36 | Docker or package commands may use the network only when you explicitly execute a packet. | 15 | — |
| 37 | The optional Team license is a one-time $49 purchase. | 9 | — |
| 38 | It adds local drill history with JSON export and import in the site preview. | 14 | — |
| 39 | Scheduled runs need an organization runner and are planned, not included in v1. | 13 | — |
| 40 | Payment opens Sociobot checkout. | 4 | — |
| 41 | See Privacy and Terms. | 4 | — |
| 42 | MIT. | 1 | — |
| 43 | See LICENSE. | 2 | — |

### Headings, labels, and actions

| Copy | Words | Result |
| --- | ---: | --- |
| CI / FAILOVER | 3 | Wordmark; accessible name is the full product name |
| Demo / Install / Team / Privacy | 1 each | Clear navigation labels |
| CI provider failover drill | 4 | Product identifier |
| Try it with sample data | 5 | Result-naming action |
| Selected job → pinned runner → drill report | 8 | Informative caption |
| Sample result / Release check | 2 each | Informative preview labels |
| The product | 2 | F-3-4: generic label |
| See what the drill catches | 5 | Names the preview purpose |
| How it works / Three steps | 3 / 2 | Clear section labels |
| Generate and run the packet | 5 | Clear heading |
| Select one job / Inspect the packet / Run it elsewhere | 3 each | Clear step headings |
| Install / Rust 1.85+ / Run the first drill | 1 / 2 / 4 | Clear install labels |
| Copy install command | 3 | Result-naming button |
| Boundaries / Know what it does not do | 1 / 6 | Clear together; h2 stands alone |
| Team license / Keep drill records in this browser | 2 / 6 | Clear section labels |
| Buy Team for $49 / Restore a license | 4 / 3 | Result-naming actions |

README headings are descriptive: “Who it is for,” “Try the sample,”
“Install,” “Generate and run a packet,” “Develop and verify,” “Privacy and
pricing,” and “License.” No README button exists. Technical nouns are suitable
for the named maintainer audience after “packet” is defined.

## Demo and sandbox

The landing action opened `/demo` in one click. At 390 × 844, the first demo
screen already showed a realistic `release-check` workflow. The populated
result showed 3 included shell steps, 1 blocked release step, 1 anonymous
input, `registry.npmjs.org`, and the five packet files.

The persistent banner read “Demo — sample data, nothing is saved” and offered
**Reset demo** and **View install command**. A seeded `demo:test` key was
removed by Reset while a seeded `cifail:real-test` key remained. The fresh
context had no cookies or storage before entry. Every request during landing
and demo entry stayed on the product origin.

Running `cargo run --quiet --manifest-path /work/repo/Cargo.toml -- demo` from
a fresh temporary caller directory produced the 3/1/1 sample under a new
`/tmp/cifail-demo-*` directory. The caller sentinel remained the only file.

## Claims and local verification

The repository was cloned with `git clone --no-local` into
`/tmp/cifail-review3-claims-wi6n7Q` at the candidate commit. After `npm ci`,
every command in `.factory/claims.json` ran separately:

| Claim | Result |
| --- | --- |
| `packet-generation` | PASS |
| `release-safety` | PASS, but incomplete coverage exposed by F-1-1 |
| `secret-redaction` | PASS |
| `offline-generation` | PASS |
| `local-privacy` | PASS |
| `cli-demo-isolation` | PASS |
| `no-ci-mutation` | PASS |
| `demo-sandbox` | PASS |
| `privacy-local` | PASS |
| `paid-license` | PASS |
| `team-history` | PASS |
| `paid-contract` | PASS |
| `runner-contract` | PASS |
| `inspection-report` | PASS, but incomplete coverage exposed by F-1-2 |
| `exit-codes` | PASS |
| `license-verdict-cache` | PASS |

No listed command failed. The direct counterexample shows that passing the
single bundled sample is insufficient to prove the public release and network
claims. F-3-3 is the only claim-like live sentence found without a matching
registered claim.

The same clean clone also passed `npm test` (6 Rust tests and 24 Chromium
tests), `npm run lint`, and `npm run build`. The build produced `dist/site/`
with 6.98 KiB gzip JavaScript and 3.78 KiB gzip CSS.

## Earlier-finding regression check

Every earlier finding was checked on the live site and in current source or
tests. Two are reopened.

| Earlier finding | Round-3 verification |
| --- | --- |
| F-1-1 | **BLOCKING, reopened.** `env npm publish` bypasses the release classifier. |
| F-1-2 | **BLOCKING, reopened.** The same wrapped command can report no network host. |
| F-1-3 | Fixed: the connection-recorder claim test passed for drill and demo. |
| F-1-4 | Fixed: the CLI demo used a new temporary directory and left its caller unchanged. |
| F-1-5 | Fixed: raw and rendered metadata are route-specific on all routes and the 404. |
| F-1-6 | Fixed: Team says browser-local and the export/import round trip passed. |
| F-1-7 | Fixed: unsupported “safe” copy is absent from the live interface. |
| F-1-8 | Fixed: the action and three facts fit within 390 × 844. |
| F-1-9 | Fixed: the landing and packet headings use direct job wording. |
| F-1-10 | Fixed: the earlier readable map lore is absent. |
| F-1-11 | Fixed: README exit meanings remain split into short sentences. |
| F-1-12 | Fixed: job name, anonymous input, and GitHub-only action terminology remains consistent. |
| F-1-13 | Fixed: the demo exit control is “View install command.” |
| F-1-14 | Fixed: landing/Terms use direct checkout wording and the linked policy returned 200. |
| F-1-15 | Fixed: Team export, delete, import, and restore passed. |
| F-1-16 | Fixed: source-repository immutability claim passed. |
| F-2-1 | Fixed: Terms says “It covers the browser tools shown on the Team page,” and the old future promise is absent. |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/team`, `/privacy`, and `/terms` returned 200. A missing path
  returned a designed HTTP 404 with a return action.
- Every route has a route-specific title, one `h1`, one `main`, a description,
  canonical, Open Graph/Twitter metadata, SVG/favicon assets, `lang="en"`, and
  no serious or critical axe result.
- `robots.txt` and `sitemap.xml` list the public routes. Static responses carry
  CSP, `X-Content-Type-Options`, and the expected route metadata.
- SPA navigation and browser Back restored the route and focused the new
  `h1`; the polite route status updated. Deep links loaded directly.
- The full link crawl returned 200 for internal destinations and the Sociobot
  terms page. The checkout returned an intentional 303. Mail links were
  explicit. No unexpected dead link was found.
- The factory URL verifier passed with no console error, missing alt text, or
  unlabeled button. Reduced motion, dark mode, mobile touch targets, and mobile
  overflow checks passed in the repository suite.
- The ivory survey grid, rust contour route, topographic plate, serif/monospace
  type, clipped map shapes, and route-trace motion match `.factory/design.md`.
  The result is visibly product-specific rather than a generic SaaS template.

No structural finding is raised.

## Missed leverage

No missing AI feature is justified. This job is deterministic parsing and
safety classification; model output would weaken the safety boundary. The
useful implied extension—moving Team history between browsers—already has JSON
export/import. No decorative AI or embedded provider key was found.

## What would make this perfect

Close the two reopened safety/reporting defects by classifying wrapped shell
commands and adding adversarial fixtures to both claims. Replace the unlisted
payment-data statement with tested wording, define “packet” on first use,
remove the generic preview label, and name the demo result directly. Then
repeat every claim command plus the `env npm publish` counterexample from a
clean clone. Zero findings is the pass threshold.
