# Demo contract

## Browser

- URL: `https://ci-provider-failover-drill.sociobot.in/demo`
- Local URL: `http://127.0.0.1:5173/demo` after `npm run dev`
- One-click entry: `/?demo=1` opens the sample without setup.
- Stable route: `/demo` opens the same isolated sample.
- Reset: select **Reset demo** in the persistent demo banner.
- Exit: select **View install command** to reach the install command.
- Storage namespace: `demo:`. The v1 demo is read-only and creates no keys.
  Demo startup does not read or verify a stored Team license. Reset and exit
  clear only `demo:` keys, leaving real browser data unchanged.

The sample is a Node package with a `release-check` job. It has checkout,
runtime setup, install, test, registry identity, and publish steps. The result
shows three shell steps, one blocked publish step, one anonymous input,
and the five packet files.

## CLI

Run:

```sh
cargo run -- demo
```

The same sample lives in `examples/sample-repo/`. The command copies it to a
new `cifail-demo-*` temporary directory, generates the packet there, and prints
that path. Pass `--out <path>` for a stable verification path. It never reads
or writes the caller's repository.
