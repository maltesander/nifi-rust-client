//! Hand-written source for the canonical-superset dynamic client (Phase 4a).
//!
//! This file is the *source of truth* for `DynamicClientV2`. At build time the
//! `build.rs` script copies this directory's contents into
//! `$OUT_DIR/dynamic_v2/` so the generated `mod.rs` (emitted by
//! `nifi-openapi-gen::emit_dynamic_v2`) can include this module via
//! `pub mod client;`. **Do not** declare this module from `lib.rs` — it is
//! reached via the `__dynamic_v2` doc-hidden re-export wired up by
//! `build_api::generate_lib_rs_fragment`.

pub mod client;

pub use client::DynamicClientV2;
