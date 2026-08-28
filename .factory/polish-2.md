# Polish 2 — cumulative finding closure

Repair source: candidate `0cf5ddf944b4598aa0d0f53cfd7c1be0807da3fd`
and review `3f4db1a152d124a44b3eef823f5d11065cab456e`.
Production deployment: `45fb8fc1-6e03-40cf-a09f-9d28bb3dc669`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept executable-aware blocking for npm option order and aliases, pnpm, Yarn, tag pushes, Cargo, Docker, GitHub releases, and line continuations. | Clean-clone `@claim:release-safety`; Rust `recognizes_release_commands_with_options_aliases_and_continuations`. |
| F-1-2 | Kept network inference on the parsed command model, including npm option order and aliases. | Clean-clone `@claim:inspection-report`. |
| F-1-3 | Kept the syscall connection recorder around both CLI generation paths. | Clean-clone `@claim:local-privacy`. |
| F-1-4 | Kept the CLI demo in a new temporary directory and matched its facts to the browser sample. | Clean-clone `@claim:cli-demo-isolation`; [live demo screenshot](qa-evidence/polish-2/live-demo-mobile-390.png). |
| F-1-5 | Kept route-specific static HTML metadata, SPA metadata updates, focus restoration, and a real 404 response. | `static deployment keeps routes real and hashed assets immutable`; [live raw and rendered route report](qa-evidence/polish-2/live-route-check.json). |
| F-1-6 | Kept Team history local to the browser with JSON export and import. | Clean-clone `@claim:team-history`. |
| F-1-7 | Kept unsupported “safe” wording out of the product and named the blocked release behavior. | `.factory/copy-audit.md`; [live landing screenshot](qa-evidence/polish-2/screenshot-desktop.png). |
| F-1-8 | Kept all three facts before the map on a 390 × 844 first screen. | `mobile first screen keeps its action and facts visible`; exact live bounds in [live route report](qa-evidence/polish-2/live-route-check.json); [mobile screenshot](qa-evidence/polish-2/live-landing-mobile-390.png). |
| F-1-9 | Kept the direct task headings “Prove one CI job runs elsewhere” and “Generate and run the packet.” | `.factory/copy-audit.md`; [live mobile screenshot](qa-evidence/polish-2/live-landing-mobile-390.png). |
| F-1-10 | Kept map lore out of readable copy while retaining the cartographic visual system. | `.factory/copy-audit.md`; `landing and legal routes pass an accessibility smoke test`; [live desktop screenshot](qa-evidence/polish-2/screenshot-desktop.png). |
| F-1-11 | Kept the README exit meanings as four short sentences. | `.factory/copy-audit.md`; clean-clone `@claim:exit-codes`. |
| F-1-12 | Finished the terminology fix in CLI help, generated reports, the terminal recording, demo docs, and claims: **job name**, **anonymous input**, and **GitHub-only action**. | `public CLI and demo copy use the documented terms`; clean-clone `@claim:packet-generation` and `@claim:inspection-report`. |
| F-1-13 | Kept the demo exit action named **View install command**. | Clean-clone `@claim:demo-sandbox`; control label and 55.6 px touch height in [live route report](qa-evidence/polish-2/live-route-check.json). |
| F-1-14 | Kept direct checkout wording and the live Sociobot terms/refund-policy link. | Clean-clone `@claim:paid-contract`; live link results: Terms 200 and checkout 303 in [live route report](qa-evidence/polish-2/live-route-check.json). |
| F-1-15 | Kept validated local JSON export, deletion, import, and restore. | Clean-clone `@claim:team-history`. |
| F-1-16 | Kept the source-repository snapshot and zero-mutation assertion. | Clean-clone `@claim:no-ci-mutation`. |
| F-2-1 | Replaced the unbounded “future v1 updates” entitlement with “It covers the browser tools shown on the Team page.” The paid-contract test now requires that scope and rejects the old promise. | Clean-clone `@claim:paid-contract`; [live Terms screenshot](qa-evidence/polish-2/live-terms-desktop.png); `terms.futurePromiseAbsent: true` in [live route report](qa-evidence/polish-2/live-route-check.json). |

## Verification

- A fresh clone of pushed repair `f721ea11335f4a6cc5435aa27b5903c9389482b7`
  ran all 16 commands in `.factory/claims.json` separately; all passed.
- The clean clone then passed `npm test`, `npm run lint`, `npm run build`, and
  `cargo package --locked`. That run had 6 Rust tests and 24 browser tests.
- The final browser suite has 24 tests, including axe checks on every route,
  dark mode, keyboard focus, mobile first-screen bounds, touch targets, privacy,
  offline generation, demo isolation, and real 404 routing.
- Production cold checks found no unexpected console errors, no serious or
  critical axe violations, one `h1` and one `main` per route, correct raw and
  rendered metadata, and same-origin-only demo requests.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1.5 s, CLS 0, total blocking time 10 ms. Full evidence:
  [lighthouse-mobile.json](qa-evidence/polish-2/lighthouse-mobile.json).

No finding from either review remains open.
