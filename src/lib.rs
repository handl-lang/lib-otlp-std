//! # handl-lib-otlp-std
//!
//! Concrete OTLP/HTTP exporter using `ureq`.
//!
//! This crate has been migrated to the `op`/`handl` declaration form
//! (ADR-0077 Phase 4) and to the kernel op ABI (`handl-ops`): the OTLP
//! interface is declared in `handl-packages/otlp/otlp.hnl`, and this
//! crate's `handl` bindings are declared in
//! `handl-packages/otlp_std/otlp_std.hnl`. The Rust handls (wrapping
//! [`HttpOtlpState`]) are registered in a kernel [`handl_ops::OpRegistry`]
//! via [`otlp_std_op_registry`], which delegates to the shared
//! `otlp_op_registry` builder in `handl-plugin-lib-otlp-c`. The host
//! imports the registry via `OpRegistry::import_kernel`; the module/plugin
//! wrappers live in consumer repos (`handl-kit-server`,
//! `handl-plugin-lib-stdlib`).
//!
//! Provides [`HttpOtlpState`] ready for use with `ModuleRuntime`.

mod http_state;

use std::sync::Arc;

use handl_plugin_lib_otlp_c::{OtlpState, otlp_op_registry};

pub use handl_plugin_lib_otlp_c::{
    MetricKind, MetricOptions, OtlpConfig, OtlpError, OtlpEvent, OtlpResult, RecordedEvent,
    RecordedSpan, StartSpanOptions, StubOtlpState,
};
pub use http_state::HttpOtlpState;

/// The handl source declaring the otlp-std `handl` bindings — the
/// implementation-side declarations for the OTLP ops. Exposed for consumer
/// repos that build the host module/plugin wrappers.
pub const OTLP_STD_HNL: &str = include_str!("../handl-packages/otlp_std/otlp_std.hnl");

/// The op names declared in the otlp interface (re-exported for consumer
/// repos building host modules/plugins).
pub use handl_plugin_lib_otlp_c::OTLP_OPS;

/// Build the kernel [`handl_ops::OpRegistry`] for otlp-std, registering
/// the Rust handls (wrapping [`HttpOtlpState`]) under the op names
/// declared in the otlp interface.
pub fn otlp_std_op_registry() -> handl_ops::OpRegistry {
    let state: Arc<dyn OtlpState> = Arc::new(HttpOtlpState::new());
    otlp_op_registry(state)
}
