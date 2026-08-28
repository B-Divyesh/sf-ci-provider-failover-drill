# Polish 1 — review finding closure

Candidate repaired from `b4c784bf8c3f12b69df8764887e12da89270b733` and adversarial
review `d5de579f0fbfc61ef06afec149f5403384457001`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Added executable-aware command inspection for npm options and aliases, pnpm, yarn, git tags, and documented release tools. | `@claim:release-safety`; Rust unit `recognizes_release_commands_with_options_aliases_and_continuations` |
| F-1-2 | Network host detection now uses the same command facts and identifies npm registry use with options and aliases. | `@claim:inspection-report` |
| F-1-3 | Added a local egress recorder test around `cifail drill` and `cifail demo`. | `@claim:local-privacy` |
| F-1-4 | Registered and tested CLI demo isolation and its sample facts from a caller directory. | `@claim:cli-demo-isolation` |
| F-1-5 | Static route preparation now writes per-route title, description, canonical, Open Graph, and Twitter metadata. | static-route Playwright assertion; `/demo`, `/team`, `/privacy`, `/terms`, `/404.html` bodies |
| F-1-6 | Rewrote the Team promise as local browser records and supplied file export/import. | `@claim:team-history` |
| F-1-7 | Removed unsupported “safe” language and named the blocked `npm publish` behavior. | copy audit; landing and demo browser checks |
| F-1-8 | Mobile hero places the required facts before the map and asserts they fit in 390×844. | `mobile first screen keeps its action and facts visible` |
| F-1-9 | Replaced metaphor headings with the direct job and packet wording. | copy audit; landing browser check |
| F-1-10 | Replaced decorative field-map labels with section labels that state product information. | copy audit; accessibility scan |
| F-1-11 | Split README exit-code wording into four short sentences. | `.factory/copy-audit.md` |
| F-1-12 | Standardized product, job name, anonymous input, and GitHub-only action terminology. | README and copy audit |
| F-1-13 | Renamed the demo exit action to **View install command**. | `@claim:demo-sandbox` |
| F-1-14 | Replaced merchant/refund assertions with the tested checkout action and a Sociobot terms/refund-policy link. | `@claim:paid-contract`; `/terms` link crawl in browser suite |
| F-1-15 | Implemented local JSON history export, merge/replace import, validation, and round trip. | `@claim:team-history` |
| F-1-16 | Added the mutation claim and a source-repository snapshot test. | `@claim:no-ci-mutation` |

## Verification evidence

- `npm test`: 6 Rust tests and 23 Playwright tests passed.
- Every command declared in `.factory/claims.json` was run from the clean
  dependency install and passed; each command rebuilds its own binary and site.
- `npm run lint`, `npm run build`, and `cargo package --locked --allow-dirty`
  passed. The release site has 7.0 KB gzip JavaScript and 3.8 KB gzip CSS.
- Local screenshots and live URL evidence are recorded in the handoff after deployment.
