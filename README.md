# CI Provider Failover Drill

Prove one GitHub Actions job can run on another runner.

`cifail` turns one workflow job into five files for a fixed Docker image. It
lists required files and network hosts, and marks GitHub-only actions for replacement.

It does not store secrets, change CI settings, or cut over providers. Release
and publish commands stay blocked unless you pass `--allow-release`.

## Who it is for

This is for maintainers whose critical checks or releases depend on GitHub
Actions. The generated packet can run on a laptop or any runner with Docker.

## Try the sample

```sh
cargo run -- demo
```

The command copies a sample repository to a new temporary directory, analyzes
its `release-check` job, and prints the packet path. Nothing touches your repo.
The website recording uses the same bundled sample. Open `/demo` on the live
site or run `npm run dev` and visit `http://localhost:5173/demo`.

## Install

Build the single binary from source:

```sh
cargo install --path .
cifail --help
```

The first release is `0.1.0`. The factory publishes packages after handoff;
this repository does not use registry credentials.

## Generate and run a packet

Generate a packet without running the selected commands:

```sh
cifail drill \
  --workflow .github/workflows/release.yml \
  --job release-check \
  --image 'node:22-bookworm@sha256:<64-hex-digest>' \
  --out .ci-failover
```

The packet contains:

- `Dockerfile`: a pinned, provider-neutral runner image.
- `run.sh`: translated shell steps with anonymous inputs.
- `.env.example`: placeholder secret variables, never names or values.
- `drill.json`: machine-readable checks for reporting.
- `report.md`: the local and generic-runner drill report.

Inspect those files, then run the packet locally:

```sh
cifail drill [same options] --execute
```

Execution needs Docker. Exit `0` means ready or passed. Exit `2` means bad
input. Exit `3` means blocked. Exit `4` means Docker failed. Add `--json` for
script output.

Release commands such as `npm publish`, `docker push`, and `gh release create`
are omitted by default. Use `--allow-release` only in a disposable test target.

## Develop and verify

Requires Rust 1.85+ and Node 20+.

```sh
npm ci
npm test
npm run build
```

`npm run build` compiles the release binary and writes the static site to
`dist/site/`. The factory deploys that directory as a static site. `cargo
package --allow-dirty` verifies the Rust package can be published.

## Privacy and pricing

The free CLI has no telemetry and makes no product requests. Inputs and drill
packets stay on your machine. Docker or package commands may use the network
only when you explicitly execute a packet.

The optional Team license is a one-time $49 purchase. It adds local drill
history with JSON export and import in the site preview. Scheduled runs need an
organization runner and are planned, not included in v1. Payment opens Sociobot
checkout.

See [Privacy](https://ci-provider-failover-drill.sociobot.in/privacy) and
[Terms](https://ci-provider-failover-drill.sociobot.in/terms).

## License

MIT. See [LICENSE](LICENSE).
