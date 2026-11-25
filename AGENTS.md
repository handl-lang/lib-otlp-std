# lib-otlp-std

`handl-plugin-lib-otlp-std` — concrete OTLP/**HTTP** exporter backend
for `lib-otlp` using `ureq`: spans, logs, metrics over HTTP.

## Layout

| path | role |
| --- | --- |
| `src/lib.rs` | crate root — `otlp_std_op_registry` over `handl-ops` |
| `src/http_state.rs` | `HttpOtlpState` — the ureq-backed exporter state |
| `handl-packages/otlp_std/` | the `otlp_std.*` .hnl surface |
| `Cargo.toml`, `xtask/` | crate manifest + `cargo xtask` gate |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `workflows/gate.yaml` | manifest / hnl gate / bupha entry |

## Contract

- Migrated to the `op`/`handl` declaration form (ADR-0077 Phase 4)
  and the kernel op ABI (`handl-ops`): interface declared in
  `handl-packages/otlp/otlp.hnl`, bindings in
  `handl-packages/otlp_std/otlp_std.hnl`.
- Rust handls wrap `HttpOtlpState`, registered in a kernel
  `OpRegistry` via `otlp_std_op_registry`, which delegates to the
  shared `otlp_op_registry` builder in `handl-plugin-lib-otlp-c`.
  The host imports via `OpRegistry::import_kernel`; module/plugin
  wrappers live in consumer repos (`handl-kit-server`,
  `handl-plugin-lib-stdlib`).
- Deps resolve from git — a plain clone test-runs as-is.

## Verification

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
