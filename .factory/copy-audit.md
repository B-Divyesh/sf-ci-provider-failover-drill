# Copy audit — polish 3

Audited 29 August 2026 against the rendered landing page and README. Counts
treat inline code and hyphenated terms as one word. No sentence exceeds 22
words. No banned plain-words term appears.

## Landing-page sentences

| Sentence | Words | Flag |
| --- | ---: | --- |
| Prove one CI job runs elsewhere. | 6 | — |
| For GitHub Actions maintainers who need one critical job to run during an outage. | 14 | — |
| See a sample five-file drill packet with one blocked npm publish step. | 12 | — |
| A mapped route crosses between two isolated CI provider regions. | 10 | — |
| One command reads one job. | 5 | — |
| It produces a packet for Docker on another runner. | 9 | — |
| Terminal demo showing a ready packet and one blocked release step. | 11 | — |
| Point the CLI at a workflow and job name. | 9 | — |
| Review files, network hosts, anonymous inputs, and GitHub-only actions. | 9 | — |
| Use the pinned container on a laptop or independent Docker runner. | 11 | — |
| Build the single binary from this repository. | 7 | — |
| You decide where and when to run. | 7 | — |
| Reports use anonymous input labels. | 5 | — |
| GitHub-only actions become named assumptions. | 5 | — |
| Publish commands need an explicit flag. | 6 | — |
| Team saves drill history locally. | 5 | — |
| Export it to move it between browsers. | 7 | — |
| Payment opens Sociobot checkout. | 4 | — |
| The free CLI keeps packet export and safety checks. | 9 | — |
| Prove one GitHub Actions job can run elsewhere. | 8 | — |

First-screen labels and facts are short fragments:

| Copy | Words | Flag |
| --- | ---: | --- |
| Try it with sample data | 5 | — |
| Free local drill | 3 | — |
| No secrets stored | 3 | — |
| Release steps stay blocked | 4 | — |

The first screen defines a packet as a five-file drill packet before using the
shorter term elsewhere. The preview label is “Sample drill result.”

## README sentences and labelled descriptions

| Sentence or description | Words | Flag |
| --- | ---: | --- |
| Prove one GitHub Actions job can run on another runner. | 10 | — |
| `cifail` turns one workflow job into five files for a fixed Docker image. | 13 | — |
| It lists required files and network hosts, and marks GitHub-only actions for replacement. | 13 | — |
| It does not store secrets, change CI settings, or cut over providers. | 12 | — |
| Release and publish commands stay blocked unless you pass `--allow-release`. | 10 | — |
| This is for maintainers whose critical checks or releases depend on GitHub Actions. | 13 | — |
| The generated packet can run on a laptop or any runner with Docker. | 13 | — |
| The command copies a sample repository to a new temporary directory, analyzes its `release-check` job, and prints the packet path. | 20 | — |
| Nothing touches your repo. | 4 | — |
| The website recording uses the same bundled sample. | 8 | — |
| Open `/?demo=1` on the live site or run `npm run dev` and visit `http://localhost:5173/?demo=1`. | 14 | — |
| Build the single binary from source. | 6 | — |
| The first release is `0.1.0`. | 5 | — |
| The factory publishes packages after handoff; this repository does not use registry credentials. | 13 | — |
| Generate a packet without running the selected commands. | 8 | — |
| `Dockerfile`: a pinned, provider-neutral runner image. | 6 | — |
| `run.sh`: translated shell steps with anonymous inputs. | 7 | — |
| `.env.example`: placeholder secret variables, never names or values. | 8 | — |
| `drill.json`: machine-readable checks for reporting. | 5 | — |
| `report.md`: the local and generic-runner drill report. | 7 | — |
| Inspect those files, then run the packet locally. | 8 | — |
| Execution needs Docker. | 3 | — |
| Exit `0` means ready or passed. | 6 | — |
| Exit `2` means bad input. | 5 | — |
| Exit `3` means blocked. | 4 | — |
| Exit `4` means Docker failed. | 5 | — |
| Add `--json` for script output. | 5 | — |
| Release commands such as `npm publish`, `docker push`, and `gh release create` are omitted by default. | 16 | — |
| Use `--allow-release` only in a disposable test target. | 8 | — |
| The scan unwraps `env`, `command`, `exec`, `sudo`, and shell `-c` prefixes. | 11 | — |
| Unclear dynamic wrappers are omitted for review. | 7 | — |
| Requires Rust 1.85+ and Node 20+. | 6 | — |
| `npm run build` compiles the release binary and writes the static site to `dist/site/`. | 14 | — |
| The factory deploys that directory as a static site. | 9 | — |
| `cargo package --allow-dirty` verifies the Rust package can be published. | 10 | — |
| The free CLI has no telemetry and makes no product requests. | 11 | — |
| Inputs and drill packets stay on your machine. | 8 | — |
| Docker or package commands may use the network only when you explicitly execute a packet. | 15 | — |
| The optional Team license is a one-time $49 purchase. | 9 | — |
| It adds local drill history with JSON export and import in the site preview. | 14 | — |
| Scheduled runs need an organization runner and are planned, not included in v1. | 13 | — |
| Payment opens Sociobot checkout. | 4 | — |
| See Privacy and Terms. | 4 | — |
| MIT. | 1 | — |
| See LICENSE. | 2 | — |

## Changed route copy

| Route | Copy | Words | Flag |
| --- | --- | ---: | --- |
| Demo | The sample packet is ready to inspect. | 7 | — |
| Privacy | The last verdict is cached for one day. | 8 | — |
| Privacy | Payment opens Sociobot checkout. | 4 | — |
| Terms | It covers the browser tools shown on the Team page. | 10 | — |

## Terminology

| Concept | One term |
| --- | --- |
| Generated portable output | packet |
| Selected GitHub Actions unit | job name |
| Rehearsal of the alternate runner | drill |
| Container host outside GitHub Actions | runner |
| Credential placeholder | anonymous input |
| Provider-bound dependency | GitHub-only action |
| Paid browser feature set | Team |

Catalog description: “Prove one GitHub Actions job runs on an independent
Docker runner.” It starts with a verb and is under 120 characters.
