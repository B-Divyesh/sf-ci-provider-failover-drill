# Adversarial first-read review 4

## Verdict: PASS

- Candidate: `f8edf1989b8f49a3709243f962a9c7cee03304c0`
- Live URL: <https://ci-provider-failover-drill.sociobot.in>
- Reviewed: 29 August 2026 UTC
- Findings: 0 blocking, 0 high, 0 minor

No product code was changed during this review. No unresolved finding remains.

## Cold first read

At 390 × 844, before scrolling, the product is clear: it checks that one CI
job can run elsewhere; it is for GitHub Actions maintainers with a critical
job during an outage; the first action is **Try it with sample data**. The
exact copy is “Prove one CI job runs elsewhere.”, “For GitHub Actions
maintainers who need one critical job to run during an outage.”, and “Try it
with sample data”. “See a sample five-file drill packet with one blocked npm
publish step.” explains the result. The facts fit at y=746–796; no overflow or
console error occurred.

At 1440 × 900 the same information, action, result note, and facts were
visible. “Selected job → pinned runner → drill report” also explains the map.
No overflow or console error occurred. The first-read blocker does not apply.

## Copy audit

Counts use whitespace-separated words; inline code and hyphenated terms count
as one. “—” means no flag. All sentences are ≤22 words. The audit found no
banned adjective, unexplained metaphor heading, inconsistent term, or
non-result-naming button.

### Landing page

| # | Sentence | Words | Flag |
| ---: | --- | ---: | --- |
| 1 | Prove one CI job runs elsewhere. | 6 | — |
| 2 | For GitHub Actions maintainers who need one critical job to run during an outage. | 14 | — |
| 3 | See a sample five-file drill packet with one blocked npm publish step. | 12 | — |
| 4 | A mapped route crosses between two isolated CI provider regions. | 10 | — |
| 5 | One command reads one job. | 5 | — |
| 6 | It produces a packet for Docker on another runner. | 9 | — |
| 7 | Terminal demo showing a ready packet and one blocked release step. | 11 | — |
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
| 18 | No surprise releases. | 3 | — |
| 19 | Publish commands need an explicit flag. | 6 | — |
| 20 | Team saves drill history locally. | 5 | — |
| 21 | Export it to move it between browsers. | 7 | — |
| 22 | Payment opens Sociobot checkout. | 4 | — |
| 23 | The free CLI keeps packet export and safety checks. | 9 | — |
| 24 | Prove one GitHub Actions job can run elsewhere. | 8 | — |

Facts are useful fragments: “Free local drill”, “No secrets stored”, and
“Release steps stay blocked”. “Five-file drill packet” defines the output on
first use. The section headings name their content, and the actions include
“Try it with sample data”, “Copy install command”, “Buy Team for $49”, and
“Restore a license”.

### README

| # | Sentence or labelled description | Words | Flag |
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
| 11 | Open `/?demo=1` on the live site or run `npm run dev` and visit `http://localhost:5173/?demo=1`. | 14 | — |
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
| 28 | Release commands such as `npm publish`, `docker push`, and `gh release create` are omitted by default. | 16 | — |
| 29 | Use `--allow-release` only in a disposable test target. | 8 | — |
| 30 | The scan unwraps `env`, `command`, `exec`, `sudo`, and shell `-c` prefixes. | 11 | — |
| 31 | Unclear dynamic wrappers are omitted for review. | 7 | — |
| 32 | Requires Rust 1.85+ and Node 20+. | 6 | — |
| 33 | `npm run build` compiles the release binary and writes the static site to `dist/site/`. | 14 | — |
| 34 | The factory deploys that directory as a static site. | 9 | — |
| 35 | `cargo package --allow-dirty` verifies the Rust package can be published. | 10 | — |
| 36 | The free CLI has no telemetry and makes no product requests. | 11 | — |
| 37 | Inputs and drill packets stay on your machine. | 8 | — |
| 38 | Docker or package commands may use the network only when you explicitly execute a packet. | 15 | — |
| 39 | The optional Team license is a one-time $49 purchase. | 9 | — |
| 40 | It adds local drill history with JSON export and import in the site preview. | 14 | — |
| 41 | Scheduled runs need an organization runner and are planned, not included in v1. | 13 | — |
| 42 | Payment opens Sociobot checkout. | 4 | — |
| 43 | See Privacy and Terms. | 4 | — |
| 44 | MIT. | 1 | — |
| 45 | See LICENSE. | 2 | — |

The terminology remains **packet**, **job name**, **drill**, **runner**,
**anonymous input**, **GitHub-only action**, and **Team**.

## Demo, claims, and sandbox checks

The landing action opened `/?demo=1` in one click. The first demo screen
showed the realistic `release-check` sample, 3 included shell steps, 1 blocked
release step, 1 anonymous input, `registry.npmjs.org`, and five packet files.
The persistent banner said “Demo — sample data, nothing is saved”. Reset
removed a seeded `demo:old` key and preserved `cifail:real`; full demo traffic
stayed on `https://ci-provider-failover-drill.sociobot.in`.

From a fresh temporary caller directory, `cargo run --quiet --locked
--manifest-path <clean-clone>/Cargo.toml -- demo --json` created
`/tmp/cifail-demo-*/failover-packet`, reported the documented 3/1/1 sample,
and left the caller's sole sentinel file unchanged.

Clean clone: `/tmp/cifail-review4-clean-psaXDX` at `f8edf19`. The 16
registered claim tests all passed: `packet-generation`, `release-safety`,
`secret-redaction`, `offline-generation`, `local-privacy`,
`cli-demo-isolation`, `no-ci-mutation`, `demo-sandbox`, `privacy-local`,
`paid-license`, `team-history`, `paid-contract`, `runner-contract`,
`inspection-report`, `exit-codes`, and `license-verdict-cache`. The tagged
aggregate rerun passed 16/16. `npm test` passed 5 Rust unit tests, 2 CLI tests,
and 25 Chromium tests; `npm run lint` and `npm run build` passed. The build
produced `dist/site/` with 7.01 KiB gzip JS and 3.78 KiB gzip CSS.

Every claim-like live sentence maps to a registered observable test. The prior
untestable payment-data wording remains absent; the page says “Payment opens
Sociobot checkout.” No unlisted claim was found.

## Earlier findings

Every earlier review, polish report, and handoff was read, then rechecked on
the live site and in current code. No earlier finding is reopened.

| Earlier ID | Confirmed result |
| --- | --- |
| F-1-1, F-1-2 | Wrapper-, alias-, option-, and shell-aware release and network tests pass. |
| F-1-3, F-1-4, F-1-16 | Local egress, CLI-demo isolation, and source immutability claims pass. |
| F-1-5 | Raw and rendered metadata is route-specific. |
| F-1-6, F-1-15 | Team is browser-local and supports JSON export/import/restore. |
| F-1-7 to F-1-14 | Direct, defined copy; mobile facts; terminology; demo exit label; and checkout wording remain correct. |
| F-2-1 | No future-update entitlement remains. |
| F-3-3 to F-3-6 | No payment-data statement; preview, packet definition, and demo-result wording remain fixed. |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/team`, `/privacy`, and `/terms` returned 200; an unknown
  route returned the designed HTTP 404 and a return action.
- Each route has a specific title, description, canonical, OG/Twitter data,
  one `h1`, one `main`, favicon assets, and `lang="en"`.
- `robots.txt`, `sitemap.xml`, all internal routes/assets, and the Sociobot
  terms link returned 200. Checkout returned its expected 303.
- Header, skip link, and footer are consistent. Demo navigation focused its
  `h1`; Back restored the landing `h1` and polite route status. Direct
  `/#install` loaded at the install section.
- The full test suite reports no serious or critical axe issue. Mobile layout
  has no horizontal overflow and controls meet the 44 px test.
- The survey-grid, contour-route, original topographic art, and
  serif/monospace system match `.factory/design.md` and are distinct from a
  generic SaaS template.

## Missed leverage

No AI addition is warranted. Safety classification and workflow inspection are
deterministic, and a model would weaken the boundary. The useful implied
portability feature—moving Team history between browsers—is already available
through validated JSON export/import. No provider key or decorative AI feature
was found.

## What would make this perfect

Nothing is outstanding in the reviewed v1 scope. Future command parsers,
providers, storage behavior, or paid wording should retain the same claim and
demo-isolation coverage.
