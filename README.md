# lib-otlp-std

`handl-plugin-lib-otlp-std` — concrete OTLP/HTTP exporter using `ureq`.

Ships both the `handl-packages/otlp_std` .hnl package and the Rust plugin
crate under `src/`.

## Layout

```text
handl-packages/otlp_std/   the .hnl package
src/                      the Rust plugin crate
workflows/gate.yaml                     bupha entry (handl-gates) → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate  # full gate (bupha entry)
```

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`). Deps resolve from git, so no sibling checkouts are needed;
a plain clone test-runs as-is.

## License

MIT.
