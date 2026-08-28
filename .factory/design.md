# CI Provider Failover Drill — visual thesis

## Direction: topographic cartography

CI failover is route planning under pressure. The interface borrows from field
maps: contour lines show the terrain, survey marks identify verified facts,
and a red route joins a GitHub workflow to a provider-neutral runner. The site
should feel like an engineer's marked-up operations map, not a hosting vendor's
dashboard.

## Palette

Light mode is the primary treatment. Dark mode is a night-field-map variant.

| Token | Light | Dark | Use |
| --- | --- | --- | --- |
| paper | `#f3eedf` | `#101815` | page ground |
| sheet | `#fffaf0` | `#17231f` | raised work areas |
| ink | `#17231f` | `#f2eddf` | primary text |
| muted | `#53615a` | `#bac7bf` | secondary text |
| contour | `#b8432f` | `#f17a61` | route, action, focus |
| river | `#0d6670` | `#6fcbd0` | verified and informational states |
| moss | `#3d5a3e` | `#9fc59c` | successful checks |
| amber | `#8a5a00` | `#f2c36b` | cautions |
| danger | `#a12d2d` | `#ff8e85` | blocked steps |
| grid | `#c9c1ae` | `#32423c` | rules and contour detail |

Every text pairing meets WCAG AA. Status always has a word or symbol, never
color alone.

## Type

- Display: Georgia, `Times New Roman`, serif. Its engraved forms suggest map
  titles and keep the identity distinct without a font download.
- Body and UI: `ui-monospace`, SFMono-Regular, Menlo, Consolas, monospace. It
  matches manifests, terminals, coordinates, and tabular drill results.

The type scale is 14, 16, 20, 28, and clamp(40–72) px. Paragraph measure stays
below 68 characters. Numbers use tabular figures.

## Spacing and shape

An 8 px base grid drives spacing: 8, 16, 24, 32, 48, 64, 96. Sections follow
an offset two-column survey sheet rather than centered SaaS blocks. Corners are
small (2–8 px), like clipped map plates. Dashed rules, coordinate labels, and
registration crosses add structure. Buttons are rectangular route markers
with 48 px minimum height.

## Motion

The signature motion is a route trace: the failover path draws once when the
hero enters, then survey points settle into place over 600 ms. UI feedback uses
160–220 ms opacity and transform changes. Nothing loops. With
`prefers-reduced-motion: reduce`, the route and points appear complete and all
scroll behavior is instant.

## Asset plan and provenance

- `site/public/topographic-route.webp`: original raster hero plate generated
  for this product with `/opt/fleet/lib/gen-image.sh`, then compressed locally.
  Prompt: “An abstract topographic field map seen from above, warm ivory survey
  paper, dense precise rust-red contour lines forming two separated highlands,
  one teal river-like route crossing the gap through three survey waypoints,
  subtle graphite grid and registration marks, screen-print texture, editorial
  technical illustration, no words, no letters, no logos, no UI, no gradient,
  wide landscape composition.” Deployment metadata is stored beside the source
  generation during the build. Factory-generated; no third-party asset.
- Map marks, logo, favicon, terminal furniture, and contour separators are
  hand-made in SVG/CSS in this repository. No stock icons or external assets.
- The 1200×630 social card is composed from the same original cartographic
  plate plus live type, so it remains on-thesis and readable.

## Responsive behavior

At 390 px, the map plate follows the action and the terminal becomes a single
scrolling plane. Coordinate side labels disappear; route state labels remain.
The facts stack, but actions keep full 48 px targets. Desktop uses an uneven
5/7 column split to resemble a folded field sheet.
