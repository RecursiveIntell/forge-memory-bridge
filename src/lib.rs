//! # forge-memory-bridge
//!
//! Transforms Forge export envelopes into projection
//! import batches for consumption by
//! `semantic-memory`'s projection importer.
//!
//! ## Authority boundary
//!
//! This crate owns the **transformation** from Forge's export vocabulary to
//! memory's import vocabulary. It does NOT:
//! - evaluate comparability or decide promotion,
//! - guess missing semantics from live memory,
//! - become a query service,
//! - persist any state of its own.
//!
//! ## Phase status: current / implemented now

mod batch;
mod error;
#[allow(deprecated)]
mod legacy;
mod transform;

pub use batch::*;
pub use error::BridgeError;
pub use transform::*;

/// Compatibility-only legacy bridge helpers.
///
/// The canonical path is `ExportEnvelopeV3` -> `transform_envelope_v3()` ->
/// `ProjectionImportBatchV3`.
/// V1/V2 helpers remain only for migration compatibility and should not be
/// used for new integrations.
#[doc(hidden)]
#[deprecated(
    since = "0.2.0",
    note = "Legacy bridge helpers are compatibility-only. Use `transform_envelope_v3()` + `ProjectionImportBatchV3`."
)]
#[allow(deprecated)]
pub mod compat {
    pub use crate::legacy::{
        transform_legacy_envelope, upgrade_legacy_envelope, LegacyEpisodeMeta,
        LegacyImportEnvelopeV1, LegacyImportRecord,
    };
}

#[deprecated(
    since = "0.1.0",
    note = "Legacy bridge helpers are compatibility-only. Use `transform_envelope_v3()` + `ProjectionImportBatchV3`."
)]
#[doc(hidden)]
#[allow(deprecated)]
pub use compat::transform_legacy_envelope;
#[deprecated(
    since = "0.1.0",
    note = "Legacy bridge helpers are compatibility-only. Use `transform_envelope_v3()` + `ProjectionImportBatchV3`."
)]
#[doc(hidden)]
#[allow(deprecated)]
pub use compat::upgrade_legacy_envelope;
#[deprecated(
    since = "0.1.0",
    note = "Legacy bridge helpers are compatibility-only. Use `transform_envelope_v3()` + `ProjectionImportBatchV3`."
)]
#[doc(hidden)]
#[allow(deprecated)]
pub use compat::LegacyEpisodeMeta;
#[deprecated(
    since = "0.1.0",
    note = "Legacy bridge helpers are compatibility-only. Use `transform_envelope_v3()` + `ProjectionImportBatchV3`."
)]
#[doc(hidden)]
#[allow(deprecated)]
pub use compat::LegacyImportEnvelopeV1;
#[deprecated(
    since = "0.1.0",
    note = "Legacy bridge helpers are compatibility-only. Use `transform_envelope_v3()` + `ProjectionImportBatchV3`."
)]
#[doc(hidden)]
#[allow(deprecated)]
pub use compat::LegacyImportRecord;
