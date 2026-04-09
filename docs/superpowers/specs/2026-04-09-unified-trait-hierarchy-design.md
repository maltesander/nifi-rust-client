# Unified Trait Hierarchy for Static and Dynamic Clients

**Date:** 2026-04-09
**Status:** Approved

## Problem

The static client uses a sub-struct builder pattern for path parameters:

```rust
client.controller_services_api().config(id).analyze_configuration(&body).await?
```

The dynamic client uses flat trait methods with all path params as arguments:

```rust
client.controller_services_api().analyze_configuration(id, body).await?
```

This makes switching between modes feel inconsistent. The goal is ergonomic parity — both clients should use the same builder-chain call pattern — while keeping the static client fully strict with no type compromises.

## Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Shared trait between static and dynamic? | No — parallel hierarchies | Static stays strict with version-specific types; dynamic uses dynamic union types |
| Shared trait across static versions? | No — per-version trait modules | Each version's traits reference its own concrete types; cross-version generic code is what dynamic mode is for |
| Body param ownership | `&T` (borrowed) in both | Matches existing static behavior; dynamic dispatch clones internally before converting |
| `impl Into` / `impl Borrow` | Deferred | `&T` is clean now; widening to `impl Borrow<T>` later is non-breaking |
| Dynamic `id` capture | `id: String` (owned in dispatch struct) | Avoids lifetime complexity in dispatch structs |
| Semver impact — static | Non-breaking (additive traits) | Existing inherent methods remain unchanged |
| Semver impact — dynamic | Breaking (flat → builder chain) | Major version bump required |

## Architecture

### Trait Structure

Two independent trait hierarchies with identical shape, different type namespaces.

**Static traits** — one module per version:

```
crate::v2_8_0::traits::controller_services::ControllerServicesApi
crate::v2_8_0::traits::controller_services::ControllerServicesConfigApi
crate::v2_8_0::traits::controller_services::ControllerServicesBulletinsApi
...
```

**Dynamic traits** — refactored from flat to hierarchical:

```
crate::dynamic::traits::controller_services::ControllerServicesApi
crate::dynamic::traits::controller_services::ControllerServicesConfigApi
crate::dynamic::traits::controller_services::ControllerServicesBulletinsApi
...
```

### Root Trait (per API group)

Contains GAT accessors for sub-resources and root-level async methods.

```rust
pub trait ControllerServicesApi {
    type ConfigApi<'b>: ControllerServicesConfigApi where Self: 'b;
    type BulletinsApi<'b>: ControllerServicesBulletinsApi where Self: 'b;
    type DescriptorsApi<'b>: ControllerServicesDescriptorsApi where Self: 'b;
    type ReferencesApi<'b>: ControllerServicesReferencesApi where Self: 'b;
    type RunStatusApi<'b>: ControllerServicesRunStatusApi where Self: 'b;
    type StateApi<'b>: ControllerServicesStateApi where Self: 'b;

    fn config<'b>(&'b self, id: &'b str) -> Self::ConfigApi<'b>;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::BulletinsApi<'b>;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::DescriptorsApi<'b>;
    fn references<'b>(&'b self, id: &'b str) -> Self::ReferencesApi<'b>;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::RunStatusApi<'b>;
    fn state<'b>(&'b self, id: &'b str) -> Self::StateApi<'b>;

    // Root methods (no sub-resource)
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<ControllerServiceDto, NifiError>;

    async fn update_controller_service(
        &self,
        id: &str,
        body: &ControllerServiceEntity,
    ) -> Result<ControllerServiceDto, NifiError>;

    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<ControllerServiceDto, NifiError>;
}
```

### Sub-Resource Trait

Leaf methods with the primary path param already captured.

```rust
pub trait ControllerServicesConfigApi {
    async fn analyze_configuration(
        &self,
        body: &ConfigurationAnalysisEntity,
    ) -> Result<ConfigurationAnalysisDto, NifiError>;

    async fn submit_config_verification_request(
        &self,
        body: &VerifyConfigRequestEntity,
    ) -> Result<VerifyConfigRequestDto, NifiError>;

    async fn delete_verification_request(
        &self,
        request_id: &str,
    ) -> Result<VerifyConfigRequestDto, NifiError>;

    async fn get_verification_request(
        &self,
        request_id: &str,
    ) -> Result<VerifyConfigRequestDto, NifiError>;
}
```

### Static Implementation

Existing sub-structs implement the traits. Inherent methods remain unchanged.

```rust
// crate::v2_8_0::api::controller_services

impl crate::v2_8_0::traits::ControllerServicesApi for ControllerServicesApi<'_> {
    type ConfigApi<'b> = ControllerServicesConfigApi<'b> where Self: 'b;

    fn config<'b>(&'b self, id: &'b str) -> Self::ConfigApi<'b> {
        ControllerServicesConfigApi { client: self.client, id }
    }

    async fn get_controller_service(&self, id: &str, ...) -> Result<...> {
        // delegates to existing inherent method
    }
}

impl crate::v2_8_0::traits::ControllerServicesConfigApi
    for ControllerServicesConfigApi<'_>
{
    async fn analyze_configuration(
        &self,
        body: &crate::v2_8_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_8_0::types::ConfigurationAnalysisDto, NifiError> {
        // delegates to existing inherent method
    }
}
```

### Dynamic Dispatch — Sub-Resource Structs

New dispatch structs capture `id` and version, routing to the correct static sub-struct.

```rust
// crate::dynamic::dispatch::controller_services

pub struct ControllerServicesConfigApiDispatch<'a> {
    client: &'a NifiClient,
    id: String,
    version: DetectedVersion,
}

impl crate::dynamic::traits::ControllerServicesConfigApi
    for ControllerServicesConfigApiDispatch<'_>
{
    async fn analyze_configuration(
        &self,
        body: &dynamic::types::ConfigurationAnalysisEntity,
    ) -> Result<dynamic::types::ConfigurationAnalysisDto, NifiError> {
        match self.version {
            DetectedVersion::V2_6_0 => {
                let api = crate::v2_6_0::api::controller_services::ControllerServicesConfigApi {
                    client: self.client,
                    id: &self.id,
                };
                Ok(api
                    .analyze_configuration(
                        &crate::v2_6_0::types::ConfigurationAnalysisEntity::try_from(
                            body.clone(),
                        )?,
                    )
                    .await?
                    .into())
            }
            // ... other versions
        }
    }
}
```

The existing top-level dispatch enum gains GAT accessors:

```rust
impl crate::dynamic::traits::ControllerServicesApi
    for ControllerServicesApiDispatch<'_>
{
    type ConfigApi<'b> = ControllerServicesConfigApiDispatch<'b> where Self: 'b;

    fn config<'b>(&'b self, id: &'b str) -> Self::ConfigApi<'b> {
        ControllerServicesConfigApiDispatch {
            client: self.client(),
            id: id.to_string(),
            version: self.version(),
        }
    }
}
```

### Dynamic Trait Default Impls

Sub-resource traits in the dynamic hierarchy provide default impls returning `UnsupportedEndpoint`, same as today:

```rust
pub trait ControllerServicesConfigApi {
    async fn analyze_configuration(
        &self,
        body: &ConfigurationAnalysisEntity,
    ) -> Result<ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_configuration".to_string(),
            version: "unknown".to_string(),
        })
    }
}
```

## Generator Changes

### New: `emit/traits.rs`

Emits per-version static trait modules.

**Input:** parsed `ApiTag` with `sub_groups` and `root_endpoints`.

**Output per version:** `crates/nifi-rust-client/src/vx_y_z/traits/{api_group}.rs` + `mod.rs`

**Logic:**
1. For each API group, emit a root trait with:
   - One GAT + accessor method per sub-group
   - One async method per root endpoint
2. For each sub-group, emit a sub-resource trait with leaf methods
3. Types reference `crate::vx_y_z::types::*`

### Modified: `emit/api.rs`

Adds trait impl blocks after existing inherent impls.

**Logic:**
1. Root struct gets `impl crate::vx_y_z::traits::{Group}Api for {Group}Api<'_>`
   - GAT accessors call the existing inherent accessor methods (e.g., `self.config(id)`)
   - Root async methods call the existing inherent methods (e.g., `self.get_controller_service(id, ui_only).await`)
   - This avoids code duplication — the trait impl is a thin forwarding layer
2. Each sub-struct gets `impl crate::vx_y_z::traits::{SubGroup}Api for {SubGroup}Api<'_>`
   - Methods call the existing inherent methods (e.g., `self.analyze_configuration(body).await`)
   - Same forwarding pattern — no duplicated HTTP logic

### Modified: `emit/dynamic/traits.rs`

Refactored from flat to hierarchical.

**Before:** One trait per API group, all path params as method args.

**After:** Root trait with GAT accessors + sub-resource traits. Primary path param removed from leaf method signatures. Default impls return `UnsupportedEndpoint`.

### Modified: `emit/dynamic/dispatch.rs`

**Added:** One dispatch struct per sub-resource group.

Each struct holds `client: &NifiClient`, `id: String`, `version: DetectedVersion` and implements the dynamic sub-resource trait with version-match dispatch.

**Modified:** Top-level dispatch enum gains GAT accessor methods that construct sub-resource dispatch structs.

### Modified: `emit/dynamic/impls.rs`

Simplified — per-version impls now implement sub-resource traits. The `id` param comes from `self.id` on the dispatch struct rather than from method args.

### New file layout

```
crates/nifi-rust-client/src/
  vx_y_z/
    api/              # existing, inherent impls + NEW trait impls appended
    types/            # existing, unchanged
    traits/           # NEW
      mod.rs          # re-exports all trait modules
      controller_services.rs
      process_groups.rs
      flow.rs
      ...
    mod.rs            # updated: pub mod traits
  dynamic/
    traits/           # REFACTORED: flat → hierarchical (same files, new structure)
    dispatch/         # MODIFIED: sub-resource dispatch structs added
    impls/            # MODIFIED: implements sub-resource traits
```

## Edge Cases

**Octet-stream methods** (`upload_nar`, `create_asset`, `upload_process_group`): Take `data: Vec<u8>` (owned) + `filename: Option<&str>`. Same signature in both trait hierarchies. No special handling.

**Void-returning methods** (`log_out`, `download_nar`): Return `Result<(), NifiError>`. No issue.

**Secondary path params** (`delete_verification_request(request_id: &str)`): Primary param captured in sub-struct, secondary stays as method param. Same in both hierarchies.

**Query param enums** (`DiagnosticLevel`, etc.): Static traits use version-specific enums. Dynamic traits use dynamic enums. Generator already distinguishes these.

**Flat API groups** (no sub-structs, e.g. `connections`, `labels`, `access`): Root trait only, no GATs. All methods take path params directly. Simpler case.

## Migration

**Static client — non-breaking:**
- Existing inherent methods remain untouched
- Traits are purely additive
- Users who want traits import `use crate::v2_8_0::traits::ControllerServicesConfigApi`
- No existing code breaks

**Dynamic client — breaking:**
- Flat calls like `.analyze_configuration(id, body)` become `.config(id).analyze_configuration(&body)`
- Body params change from owned `T` to borrowed `&T`
- Trait import paths change (same module, but trait names may change)
- Requires major version bump

## Final Step: Documentation Audit

After implementation is complete, verify all documentation reflects the current state of the repository:

1. **`AGENTS.md`** — update the "Dynamic mode" section to reflect hierarchical traits, update the dispatch layer description, remove or update the "path parameters differ" note (since they no longer differ), update code examples
2. **`crates/nifi-rust-client/README.md`** — update the "Dynamic mode" section with builder-chain examples, remove the "Path parameter differences" subsection (parity achieved), update trait import examples, verify the "When to use which" table still holds
3. **`README.md` (root)** — update the dynamic mode code example to use builder-chain, remove the note about path param differences in the "Two modes" paragraph
4. **`crates/nifi-openapi-gen/README.md`** — verify generator documentation reflects new emitters (`emit/traits.rs`, modified `emit/dynamic/*`)
5. **Version/changelog** — bump to next major version, document the breaking dynamic API change
6. **Generated test files** — verify wiremock tests still compile with trait changes (inherent methods unchanged, so tests should pass)

This is a clean-slate documentation pass — every README and AGENTS.md should accurately describe the post-implementation state of the repo for the next release.
