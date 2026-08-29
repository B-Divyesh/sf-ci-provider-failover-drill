# Polish 3 — cumulative finding closure

Repair source: candidate `b7e18eebc30a85be83434b23929a2f15b8435fbd`
and adversarial review `d7fe3c4bc5cddc1f778d2b428ff4cccf97e36afb`.
The repair also rechecked every finding from reviews 1 and 2.

## Finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 / F-3-1 | Replaced literal command matching with shell-word classification. It unwraps assignments, `env`, `command`, `exec`, `sudo`, nested wrappers, common launcher prefixes, and shell `-c` forms. Unknown wrapper targets fail closed. | Clean-clone `@claim:release-safety`; Rust `recognizes_release_commands_through_options_wrappers_and_shells` and `wrapper_inspection_fails_closed_when_the_target_is_unclear`; live blocked sample in [`live-demo-mobile-390.png`](qa-evidence/polish-3/live-demo-mobile-390.png). |
| F-1-2 / F-3-2 | Network inference now consumes the same unwrapped command facts, so wrapped npm publish forms report `registry.npmjs.org`. | Clean-clone `@claim:inspection-report`; demo network result in [`live-demo-mobile-390.png`](qa-evidence/polish-3/live-demo-mobile-390.png); live `/demo` checked in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json). |
| F-1-3 | Kept the syscall-level connection recorder around both CLI generation paths and verified all output stays in the selected packet/demo area. | Clean-clone `@claim:local-privacy` and `@claim:offline-generation`; live website request origins in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json). |
| F-1-4 | Kept the CLI demo in a new temporary directory, matched its facts to the browser sample, and documented the separate `demo:` namespace. | Clean-clone `@claim:cli-demo-isolation`; [`demo.md`](demo.md); live isolated sample in [`live-demo-mobile-390.png`](qa-evidence/polish-3/live-demo-mobile-390.png). |
| F-1-5 | Kept route-specific static HTML and rendered metadata, real route files, History API focus restoration, and the designed HTTP 404. | `static deployment keeps routes real and hashed assets immutable`; live raw/rendered checks for all routes, focus, and 404 in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json). |
| F-1-6 | Kept the Team promise browser-local and retained JSON export/import. | Clean-clone `@claim:team-history`; live `/team` semantics in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json). |
| F-1-7 | Kept unsupported “safe” language out and described the observable blocked publish step. | `round-three copy uses defined packet terms and observable checkout wording`; [`copy-audit.md`](copy-audit.md); live landing in [`screenshot-desktop.png`](qa-evidence/polish-3/screenshot-desktop.png). |
| F-1-8 | Kept the action and all three facts before the map at 390 × 844. | `mobile first screen keeps its action and facts visible`; exact live bounds in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json); [`live-landing-mobile-390.png`](qa-evidence/polish-3/live-landing-mobile-390.png). |
| F-1-9 | Kept the direct headings “Prove one CI job runs elsewhere” and “Generate and run the packet.” | [`copy-audit.md`](copy-audit.md); live landing in [`screenshot-desktop.png`](qa-evidence/polish-3/screenshot-desktop.png). |
| F-1-10 | Kept decorative map lore out of readable copy while preserving the cartographic visual identity. | `landing and legal routes pass an accessibility smoke test`; [`copy-audit.md`](copy-audit.md); live landing in [`screenshot-desktop.png`](qa-evidence/polish-3/screenshot-desktop.png). |
| F-1-11 | Kept exit-code documentation as four short sentences. | Clean-clone `@claim:exit-codes`; [`copy-audit.md`](copy-audit.md); README clean-clone review. |
| F-1-12 | Kept **job name**, **anonymous input**, and **GitHub-only action** consistent across help, reports, docs, demo, and claims. | `public CLI and demo copy use the documented terms`; [`copy-audit.md`](copy-audit.md); live demo in [`live-demo-mobile-390.png`](qa-evidence/polish-3/live-demo-mobile-390.png). |
| F-1-13 | Kept the demo exit action named **View install command** and made it clear only `demo:` data. | Clean-clone `@claim:demo-sandbox`; live control size and label in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json). |
| F-1-14 | Kept checkout wording observable and retained the live Sociobot terms/refund-policy link. | Clean-clone `@claim:paid-contract`; live checkout 303 and terms 200 in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json); [`live-terms-desktop.png`](qa-evidence/polish-3/live-terms-desktop.png). |
| F-1-15 | Kept validated local history export, delete, merge/replace import, and restore. | Clean-clone `@claim:team-history`; live `/team` route checked in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json). |
| F-1-16 | Kept the source-repository snapshot and zero-mutation assertion around normal packet generation. | Clean-clone `@claim:no-ci-mutation`; live boundary copy in [`screenshot-desktop.png`](qa-evidence/polish-3/screenshot-desktop.png). |
| F-2-1 | Kept the finite entitlement: “It covers the browser tools shown on the Team page.” The untestable future-update promise remains absent. | Clean-clone `@claim:paid-contract`; live terms assertion in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json); [`live-terms-desktop.png`](qa-evidence/polish-3/live-terms-desktop.png). |
| F-3-3 | Replaced the unregistered payment-data statement with “Payment opens Sociobot checkout.” | `round-three copy uses defined packet terms and observable checkout wording`; live assertion in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json); [`live-privacy-desktop.png`](qa-evidence/polish-3/live-privacy-desktop.png). |
| F-3-4 | Replaced the generic “The product” eyebrow with “Sample drill result.” | `round-three copy uses defined packet terms and observable checkout wording`; live landing in [`screenshot-desktop.png`](qa-evidence/polish-3/screenshot-desktop.png). |
| F-3-5 | Defined the first packet reference as a “five-file drill packet.” | `round-three copy uses defined packet terms and observable checkout wording`; first-screen live check in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json); [`live-landing-mobile-390.png`](qa-evidence/polish-3/live-landing-mobile-390.png). |
| F-3-6 | Replaced the route metaphor with “The sample packet is ready to inspect.” | Clean-clone `@claim:demo-sandbox`; live heading check in [`live-route-check.json`](qa-evidence/polish-3/live-route-check.json); [`live-demo-mobile-390.png`](qa-evidence/polish-3/live-demo-mobile-390.png). |

## Clean-clone verification

Clean clone: `/tmp/cifail-polish3-clean-jPKHe4` at repair commit `efe9fd0`.

- All 16 commands in `.factory/claims.json` ran separately and passed.
- `npm test`: 5 Rust unit tests, 2 CLI integration tests, and 25 Chromium
  tests passed.
- `npm run lint`, `npm run build`, and `cargo package --locked` passed.
- The production bundle is 7.01 KiB gzip JavaScript and 3.78 KiB gzip CSS.
- Browser coverage includes axe on every route, dark mode, keyboard focus,
  mobile first-screen bounds, touch targets, same-origin privacy, offline CLI
  generation, demo isolation, raw metadata, and real 404 routing.

## Live verification

- Cold checks cover `/`, `/?demo=1`, `/demo`, `/team`, `/privacy`, `/terms`,
  and `/missing-place` at
  <https://ci-provider-failover-drill.sociobot.in>.
- The route report records correct status, title, canonical, description, one
  `h1`, one `main`, no overflow, no unexpected console error, and zero serious
  or critical axe violations.
- The query demo preserves real browser data, clears `demo:` keys, makes only
  same-origin requests, and keeps 44 px controls.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100,
  SEO 100; LCP 1.6 s, CLS 0, total blocking time 70 ms. Evidence:
  [`lighthouse-mobile.json`](qa-evidence/polish-3/lighthouse-mobile.json).
- Factory URL verification passed in 586 ms with no console, title, language,
  landmark, alt-text, or button-label defect. Evidence:
  [`verify.json`](qa-evidence/polish-3/verify.json).

No finding from reviews 1, 2, or 3 remains open.
