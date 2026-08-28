# Adversarial first-read review 2

## Verdict: FAIL

- Product: CI Provider Failover Drill
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Candidate reviewed: `0cf5ddf944b4598aa0d0f53cfd7c1be0807da3fd`
- Reviewed: 28 August 2026 UTC
- Work order: `ci-provider-failover-drill-review-2`
- Findings: 0 blocking, 1 high, 0 medium, 0 low

The core CLI, landing page, and isolated demo are clear and work end to end.
The review fails because the Terms page adds a commercial entitlement that is
neither listed nor testable in `.factory/claims.json`. The product cannot pass
the claims contract while that promise remains.

## Findings

### High

#### F-2-1 — the paid entitlement to future updates is unlisted and untestable

**Exact quote/location:** `/terms`, Team purchase: “Team costs $49 as a
one-time purchase. It covers the current browser tools and future v1 updates.”

`paid-contract` is the only related claim. It promises only “Team costs $49
once while packet export and safety checks remain free,” and its tagged test
checks the price, one-time wording, free boundary, and checkout link. It does
not establish an entitlement to any future update, its scope, or its duration.

A buyer can rely on “future v1 updates” when deciding whether to buy. It is a
contractual promise, not an implementation detail, so it must not be left
outside the claims register.

**Concrete fix:** remove the untestable future promise. Rewrite the second
sentence as: “It covers the browser tools shown on the Team page.” If a future
update entitlement is intended, define its version/time scope, add it to
`paid-contract`, and add an observable contract test for that exact scope.

## Cold first read

### 390 × 844, before scrolling

- **What it does:** turns one CI job into a packet that can be run on another
  runner, so a maintainer can test a failover path.
- **For whom:** GitHub Actions maintainers with one critical job to keep
  running during an outage.
- **First click:** **Try it with sample data**.

The first screen states this directly with “Prove one CI job runs elsewhere,”
“For GitHub Actions maintainers who need one critical job to run during an
outage,” and “Try it with sample data.” The adjacent sentence says what opens:
“See a sample packet with one blocked npm publish step.” The three product
facts were inside the 844 px viewport at y=746–796. This is **not** a
first-read blocker.

### 1440 × 900, before scrolling

The same answers, action, all three facts, and the explanatory map caption are
visible. The page had no console errors or horizontal overflow in either fresh
context.

## Copy audit

Counts treat inline code and hyphenated terms as one word. The tables include
image alt text and the shared landing footer sentence. Headings, labels, and
controls are checked after the tables because they are fragments rather than
sentences.

### Landing page sentences

| # | Sentence | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove one CI job runs elsewhere. | 6 | — |
| 2 | For GitHub Actions maintainers who need one critical job to run during an outage. | 14 | — |
| 3 | See a sample packet with one blocked npm publish step. | 10 | — |
| 4 | A mapped route crosses between two isolated CI provider regions. | 10 | — |
| 5 | One command reads one job. | 5 | — |
| 6 | It produces a packet for Docker on another runner. | 9 | — |
| 7 | Terminal demo showing a ready packet and one blocked release step. | 11 | — |
| 8 | Point the CLI at a workflow and job name. | 9 | — |
| 9 | Review files, network hosts, anonymous inputs, and GitHub-only actions. | 9 | — |
| 10 | Use the pinned container on a laptop or independent Docker runner. | 11 | — |
| 11 | Build the single binary from this repository. | 7 | — |
| 12 | You decide where and when to run. | 7 | — |
| 13 | Reports use anonymous input labels. | 5 | — |
| 14 | GitHub-only actions become named assumptions. | 5 | — |
| 15 | Publish commands need an explicit flag. | 6 | — |
| 16 | Team saves drill history locally. | 5 | — |
| 17 | Export it to move it between browsers. | 7 | — |
| 18 | Payment opens Sociobot checkout. | 4 | — |
| 19 | The free CLI keeps packet export and safety checks. | 9 | — |
| 20 | Prove one GitHub Actions job can run elsewhere. | 8 | — |

### README sentences and labelled fragments

| # | Sentence or labelled fragment | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove one GitHub Actions job can run on another runner. | 10 | — |
| 2 | `cifail` turns one workflow job into five files for a fixed Docker image. | 13 | — |
| 3 | It lists required files and network hosts, and marks GitHub-only actions for replacement. | 13 | — |
| 4 | It does not store secrets, change CI settings, or cut over providers. | 12 | — |
| 5 | Release and publish commands stay blocked unless you pass `--allow-release`. | 10 | — |
| 6 | This is for maintainers whose critical checks or releases depend on GitHub Actions. | 13 | — |
| 7 | The generated packet can run on a laptop or any runner with Docker. | 13 | — |
| 8 | The command copies a sample repository to a new temporary directory, analyzes its `release-check` job, and prints the packet path. | 20 | — |
| 9 | Nothing touches your repo. | 4 | — |
| 10 | The website recording uses the same bundled sample. | 8 | — |
| 11 | Open `/demo` on the live site or run `npm run dev` and visit `http://localhost:5173/demo`. | 12 | — |
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
| 28 | Release commands such as `npm publish`, `docker push`, and `gh release create` are omitted by default. | 12 | — |
| 29 | Use `--allow-release` only in a disposable test target. | 8 | — |
| 30 | Requires Rust 1.85+ and Node 20+. | 6 | — |
| 31 | `npm run build` compiles the release binary and writes the static site to `dist/site/`. | 12 | — |
| 32 | The factory deploys that directory as a static site. | 9 | — |
| 33 | `cargo package --allow-dirty` verifies the Rust package can be published. | 8 | — |
| 34 | The free CLI has no telemetry and makes no product requests. | 11 | — |
| 35 | Inputs and drill packets stay on your machine. | 8 | — |
| 36 | Docker or package commands may use the network only when you explicitly execute a packet. | 15 | — |
| 37 | The optional Team license is a one-time $49 purchase. | 9 | — |
| 38 | It adds local drill history with JSON export and import in the site preview. | 14 | — |
| 39 | Scheduled runs need an organization runner and are planned, not included in v1. | 13 | — |
| 40 | Payment opens Sociobot checkout. | 4 | — |
| 41 | MIT. | 1 | — |
| 42 | See LICENSE. | 2 | — |

No audited landing or README sentence exceeds 22 words. The technical nouns
are appropriate for the named GitHub Actions maintainer audience and are used
consistently: **packet**, **job name**, **drill**, **runner**, **anonymous
input**, and **GitHub-only action**. The only copy finding is F-2-1 on the
Terms page, outside these two required inventories.

Headings name their sections or actions; none are mood slogans or metaphors.
The visible actions name results: **Try it with sample data**, **Copy install
command**, **Copy demo command**, **View install command**, **Export drill
history**, **Import drill history**, **Delete local history**, and **Buy Team
for $49**. No non-result-naming button was found.

## Demo and sandbox

The primary action reached `/demo` in one click. Its first screen already
showed a realistic Node `release-check` workflow beside its packet result: 3
included shell steps, 1 blocked release step, 1 anonymous input,
`registry.npmjs.org`, and the five generated files. The persistent banner read
“Demo — sample data, nothing is saved,” with **Reset demo** and **View install
command**. Reset retained the banner and left localStorage empty; no `demo:`
keys existed.

A fresh `/demo` browser context made requests only to
`https://ci-provider-failover-drill.sociobot.in`; it logged no console or page
errors. The documented CLI default command was also run from a temporary
caller directory. It left the caller sentinel untouched, created its packet
under `/tmp/cifail-demo-…`, and reported the same 3/1/1 sample facts.

## Claims and local verification

Started from this checkout with `npm ci`, then executed every command listed
in `.factory/claims.json` separately. All 16 passed:

| Claim | Result |
| --- | --- |
| `packet-generation` | PASS |
| `release-safety` | PASS |
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
| `inspection-report` | PASS |
| `exit-codes` | PASS |
| `license-verdict-cache` | PASS |

`npm test` then passed 6 Rust tests and 23 Chromium tests. `npm run lint` and
`npm run build` passed; the release build produced `dist/site/`. The focused
CLI checks included option-order/alias release forms, a local connection
recorder, source-repository immutability, and default CLI-demo isolation.

The live landing, README, Privacy, Terms, Demo, and Team copy was cross-checked
against the register. All functional/privacy/price claims have an entry except
the future-update entitlement in F-2-1. The checkout URL returned HTTP 303 to
the hosted Dodo checkout; no purchase was attempted.

## Earlier-review regression check

Every finding in `review-1.md` was rechecked on the live deployment and in
source/tests. None is unfixed, half-fixed, or regressed.

| Earlier finding | Verification |
| --- | --- |
| F-1-1 | Release parsing covers option order, aliases, tags, and continuations; `@claim:release-safety` passed. |
| F-1-2 | The same variants identify `registry.npmjs.org`; `@claim:inspection-report` passed. |
| F-1-3 | `@claim:local-privacy` records no connection attempts for drill or demo. |
| F-1-4 | `@claim:cli-demo-isolation` preserves the caller directory and compares sample facts. |
| F-1-5 | Raw `/demo`, `/team`, `/privacy`, `/terms`, and 404 responses have route-specific title, description, canonical, OG, and Twitter metadata. |
| F-1-6 | The Team wording says local browser records and supplies JSON export/import. |
| F-1-7 | Unsupported “safe” wording is absent from live copy and source. |
| F-1-8 | At 390 × 844 all three hero facts are within the initial viewport. |
| F-1-9 | The hero and packet headings use direct task language. |
| F-1-10 | Decorative map lore is absent from the readable interface. |
| F-1-11 | README exit codes are four short sentences. |
| F-1-12 | Current copy uses the documented terminology consistently. |
| F-1-13 | The demo exit control is **View install command**. |
| F-1-14 | Checkout wording is direct and Terms links to Sociobot’s live terms/refund policy. |
| F-1-15 | Team export, clear, import, and restore round trip passes `@claim:team-history`. |
| F-1-16 | `@claim:no-ci-mutation` snapshots the source repository around packet generation. |

## Structure, accessibility, and identity

- `/`, `/demo`, `/team`, `/privacy`, and `/terms` returned 200. An unknown
  route returned a styled HTTP 404 with a return action.
- Every checked route has one `main`, one `h1`, the expected route title,
  description, canonical, OG/Twitter fields, favicon, and `lang="en"`.
  `robots.txt` and `sitemap.xml` list the public routes.
- In SPA navigation, `/` → `/demo` and browser Back each focused the new
  `h1` and updated the polite route-status region. Deep links work.
- The keyboard skip link works. The full test suite reports no serious or
  critical axe findings; mobile demo controls meet the 44 px check; reduced
  motion and dark-mode checks pass.
- Crawled actionable links returned 200, the hosted checkout returned 303,
  and mail links were explicit. The unknown-route skip link is an in-document
  fragment, not a navigated destination.
- The page uses its documented cartographic system: ivory paper, contour-red
  route marks, a field-map hero, Georgia/monospace pairing, and reduced-motion
  route tracing. It does not present as a generic SaaS hero or feature-card
  template.

## Missed leverage

No missing AI feature was found. The brief is a deterministic local CLI
inspection task; an AI step would be decorative and would add privacy/cost
surface without improving the primary drill. The packet, report, Docker
contract, and Team JSON import/export cover the implied useful actions.

## What would make this perfect

Remove or precisely define and test the future-v1-updates entitlement in
F-2-1. With that one contractual copy issue resolved, this review has no other
remaining finding.
