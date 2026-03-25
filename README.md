# forge-memory-bridge

Transform Forge export envelopes into projection import batches for `semantic-memory`.

## What it does

`forge-memory-bridge` sits between Forge's export pipeline and `semantic-memory`'s projection importer. It validates export envelopes, transforms each record into the import projection format, and preserves provenance (envelope ID, content digest, lineage, trace context) end-to-end.

The bridge **does not**:
- Evaluate comparability or decide promotion
- Guess missing semantics from live memory
- Act as a query service
- Persist any state of its own

## Versioned contracts

The canonical path uses V3 types throughout:

```text
ExportEnvelopeV3 → transform_envelope_v3() → ProjectionImportBatchV3
```

V1 and V2 paths are preserved as deprecated compatibility layers for migration only.

### Batch schema versions

| Schema | Constant | Status |
|---|---|---|
| `ProjectionImportBatchV3` | `PROJECTION_IMPORT_BATCH_V3_SCHEMA` | **Canonical** — kernel-ready, carries support algebra, contradiction witnesses, retraction lineage, intervention/control artifacts (v13–v15) |
| `ProjectionImportBatchV2` | `PROJECTION_IMPORT_BATCH_V2_SCHEMA` | Compatibility — adds export metadata and evidence bundles over V1 |
| `ProjectionImportBatchV1` | `PROJECTION_IMPORT_BATCH_V1_SCHEMA` | Compatibility — minimal projection records only |

### Import record types

Each batch contains typed `ImportProjectionRecord` variants:

- **ClaimVersion** — claim projection with lifecycle state, freshness, contradiction status, and supersession lineage
- **RelationVersion** — relation projection with scope, validity, and contradiction tracking
- **Episode** — causal episode with document linkage and experiment provenance
- **EntityAlias** — entity merge/alias with durable review state and human-confirmation flags
- **EvidenceRef** — opaque evidence reference with audit-only dereference

## Error handling

All public functions return `Result<_, BridgeError>`. The `BridgeError` enum covers:

- `InvalidEnvelope` — structurally invalid export
- `IncompatibleVersion` — schema version mismatch
- `DigestMismatch` — content digest verification failure
- `DigestComputationFailed` — digest could not be computed
- `InvalidRecord` — malformed record within an envelope
- `TransformFailed` — transformation logic failure

## Dependencies

- `semantic-memory-forge` — export envelope types and validation
- `stack-ids` — typed identifiers (ClaimId, EnvelopeId, TraceCtx, etc.)
- `serde` / `serde_json` — serialization
- `thiserror` — error derivation
- `chrono` — timestamps
- `uuid` — ID generation
- `schemars` — JSON Schema derivation

## License

MIT
