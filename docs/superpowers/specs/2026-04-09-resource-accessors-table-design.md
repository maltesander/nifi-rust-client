# Auto-generated Resource Accessors Table

## Summary

Add a new doc emitter that auto-generates the `## Resource accessors` section in `crates/nifi-rust-client/README.md` from parsed OpenAPI specs, using the existing marker-based replacement mechanic.

## Markers

```html
<!-- RESOURCE_ACCESSORS_START -->
(generated table)
<!-- RESOURCE_ACCESSORS_END -->
```

Placed in `crates/nifi-rust-client/README.md`, replacing the current hand-written resource accessors section content.

## Table Format

```markdown
| Accessor | Resource | 2.6.0 | 2.7.0 | 2.8.0 |
|----------|----------|-------|-------|-------|
| `client.flow_api()` | Flow management | 15 | 15 | 16 |
| `client.ai_api()` | AI services | — | — | 3 |

> Numbers indicate the endpoint count available for each accessor in that NiFi version. — means the accessor is not available in that version.
```

### Column rules

- **Accessor**: derived from tag name using existing `to_snake_case` logic, formatted as `` `client.{snake_name}_api()` ``
- **Resource**: derived from tag name by inserting spaces between PascalCase words, then lowercasing all but the first word (e.g. `ControllerServices` → "Controller services")
- **Version columns**: capped at the **10 most recent versions** (semver-sorted, newest rightmost). Older versions that still compile are simply not shown in the table.
- **Version cells**: the endpoint count for that tag in that version if the tag exists, em dash `—` if the tag does not exist in that version.
- **Row order**: alphabetical by accessor name.
- **Legend**: a blockquote line immediately after the table explaining what the numbers and dashes mean.

## New File

`crates/nifi-openapi-gen/src/docs/resource_accessors.rs`

### Public API

```rust
pub fn update_resource_accessors_table(
    specs: &[(semver::Version, ApiSpec)],
    client_readme: &Path,
)
```

### Logic

1. Collect the union of all tag names across all specs.
2. Sort tags alphabetically.
3. Select up to 10 most recent versions (semver-sorted).
4. For each tag, for each selected version: look up the endpoint count from `spec.tags` (number of operations in that tag). Use `—` if the tag is absent.
5. Derive the accessor name: `to_snake_case(tag_name)` → `` `client.{snake}_api()` ``.
6. Derive the resource description: split PascalCase tag name into words, lowercase all but the first.
7. Build the markdown table string with header, separator, rows, and legend.
8. Call `update_file_between_markers()` with `RESOURCE_ACCESSORS_START` / `RESOURCE_ACCESSORS_END`.

## Integration

Called from `generate.rs` alongside the existing doc update calls (`update_versions_table`, `update_readme_examples`, etc.). No new dependencies — uses `ApiSpec`, `semver::Version`, and `util::update_file_between_markers`, all of which already exist.

## README Changes

In `crates/nifi-rust-client/README.md`:
- Replace the current hand-written `## Resource accessors` section content with the marker pair.
- The surrounding heading and any prose before/after the table remain hand-written.

## Out of Scope

The following sections remain hand-written:
- Client methods (`### Client methods`)
- Error handling (`## Error handling`)
- nifi-openapi-gen README intro and architecture table
