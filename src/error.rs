//! Error types for the bridge crate.

use semantic_memory_forge::ExportEnvelopeError;
use thiserror::Error;

/// Errors produced by the forge-memory-bridge.
#[derive(Debug, Error)]
pub enum BridgeError {
    /// The export envelope is structurally invalid.
    #[error("invalid envelope: {reason}")]
    InvalidEnvelope { reason: String },

    /// Schema version mismatch.
    #[error("incompatible version: expected {expected}, got {actual}")]
    IncompatibleVersion { expected: String, actual: String },

    /// Content digest does not match computed value.
    #[error("digest mismatch: expected {expected}, got {actual}")]
    DigestMismatch { expected: String, actual: String },

    /// Failed to compute content digest.
    #[error("digest computation failed: {reason}")]
    DigestComputationFailed { reason: String },

    /// A record in the envelope is malformed.
    #[error("invalid record: {reason}")]
    InvalidRecord { reason: String },

    /// Transformation from export to import failed.
    #[error("transform failed: {reason}")]
    TransformFailed { reason: String },
}

impl BridgeError {
    /// Stable error kind discriminant.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::InvalidEnvelope { .. } => "invalid_envelope",
            Self::IncompatibleVersion { .. } => "incompatible_version",
            Self::DigestMismatch { .. } => "digest_mismatch",
            Self::DigestComputationFailed { .. } => "digest_computation_failed",
            Self::InvalidRecord { .. } => "invalid_record",
            Self::TransformFailed { .. } => "transform_failed",
        }
    }
}

impl From<ExportEnvelopeError> for BridgeError {
    fn from(value: ExportEnvelopeError) -> Self {
        match value {
            ExportEnvelopeError::InvalidEnvelope { reason } => Self::InvalidEnvelope { reason },
            ExportEnvelopeError::IncompatibleVersion { expected, actual } => {
                Self::IncompatibleVersion { expected, actual }
            }
            ExportEnvelopeError::DigestMismatch { expected, actual } => {
                Self::DigestMismatch { expected, actual }
            }
            ExportEnvelopeError::DigestComputationFailed { reason } => {
                Self::DigestComputationFailed { reason }
            }
        }
    }
}
