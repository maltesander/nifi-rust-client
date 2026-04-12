pub mod cargo_features;
pub mod cleanup;
pub mod docker_compose;
pub mod lib_rs;

pub use cargo_features::{
    emit_cargo_features_client, emit_cargo_features_tests,
    patch_client_cargo_features, patch_tests_cargo_features,
};
pub use cleanup::{migrate_flat_layout, remove_stale_generated_files, remove_stale_version_dirs};
pub use docker_compose::{emit_docker_compose_default, replace_image_tag_default};
pub use lib_rs::{discover_versions, emit_lib_rs_feature_flags, generate_lib_rs_content};
