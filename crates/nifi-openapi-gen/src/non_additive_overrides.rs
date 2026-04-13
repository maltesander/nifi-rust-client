//! Human-approved exceptions to the non-additive-change detector.
//!
//! Ships empty. Entries are added only in response to a real
//! non-additive panic, after a human has verified that the change is
//! safe to absorb into the canonical superset. Each entry carries a
//! `reason` string that is logged at generation time so accepted
//! deviations stay visible.
//!
//! The table is keyed per rule + per location. There is no global
//! "disable detector" hatch.

use crate::non_additive_detector::NonAdditiveChange;
use crate::parser::HttpMethod;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonAdditiveOverride {
    EndpointRemoved {
        method: HttpMethod,
        path: String,
        reason: String,
    },
    TypeRemoved {
        type_name: String,
        reason: String,
    },
    FieldRemoved {
        type_name: String,
        field: String,
        reason: String,
    },
    FieldTypeChanged {
        type_name: String,
        field: String,
        reason: String,
    },
    EnumVariantRemoved {
        enum_name: String,
        variant: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct NonAdditiveOverrides {
    entries: Vec<NonAdditiveOverride>,
}

impl NonAdditiveOverrides {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn allow(&mut self, entry: NonAdditiveOverride) {
        self.entries.push(entry);
    }

    /// Returns `true` if any override silences the given change.
    pub fn allows(&self, change: &NonAdditiveChange) -> bool {
        self.entries.iter().any(|o| matches_change(o, change))
    }

    pub fn iter(&self) -> impl Iterator<Item = &NonAdditiveOverride> {
        self.entries.iter()
    }
}

fn matches_change(override_entry: &NonAdditiveOverride, change: &NonAdditiveChange) -> bool {
    match (override_entry, change) {
        (
            NonAdditiveOverride::EndpointRemoved {
                method: om,
                path: op,
                ..
            },
            NonAdditiveChange::EndpointRemoved { method, path, .. },
        ) => om == method && op == path,
        (
            NonAdditiveOverride::TypeRemoved { type_name: ot, .. },
            NonAdditiveChange::TypeRemoved { type_name, .. },
        ) => ot == type_name,
        (
            NonAdditiveOverride::FieldRemoved {
                type_name: ot,
                field: of,
                ..
            },
            NonAdditiveChange::FieldRemoved {
                type_name, field, ..
            },
        ) => ot == type_name && of == field,
        (
            NonAdditiveOverride::FieldTypeChanged {
                type_name: ot,
                field: of,
                ..
            },
            NonAdditiveChange::FieldTypeChanged {
                type_name, field, ..
            },
        ) => ot == type_name && of == field,
        (
            NonAdditiveOverride::EnumVariantRemoved {
                enum_name: oe,
                variant: ov,
                ..
            },
            NonAdditiveChange::EnumVariantRemoved {
                enum_name, variant, ..
            },
        ) => oe == enum_name && ov == variant,
        _ => false,
    }
}
