// Hand-written — not generated. See dynamic/mod.rs for the generated counterpart.

use semver::Version;

/// Controls how the dynamic client resolves a detected NiFi server version
/// to a supported client version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VersionResolutionStrategy {
    #[default]
    Strict,
    Closest,
    Latest,
}

pub(crate) fn resolve_version(
    detected_str: &str,
    strategy: VersionResolutionStrategy,
    strict_match: impl FnOnce(&str) -> Result<super::DetectedVersion, crate::NifiError>,
    supported_versions: &[(&str, super::DetectedVersion)],
) -> Result<super::DetectedVersion, crate::NifiError> {
    // All strategies agree on an exact match
    match strict_match(detected_str) {
        Ok(v) => return Ok(v),
        Err(e) => {
            if strategy == VersionResolutionStrategy::Strict {
                return Err(e);
            }
        }
    }

    let detected =
        Version::parse(detected_str).map_err(|_| crate::NifiError::UnsupportedVersion {
            detected: detected_str.to_string(),
        })?;

    // Filter to same major version only
    let same_major: Vec<(Version, super::DetectedVersion)> = supported_versions
        .iter()
        .filter_map(|(ver_str, dv)| {
            let v = Version::parse(ver_str).ok()?;
            if v.major == detected.major {
                Some((v, *dv))
            } else {
                None
            }
        })
        .collect();

    if same_major.is_empty() {
        return Err(crate::NifiError::UnsupportedVersion {
            detected: detected_str.to_string(),
        });
    }

    let resolved = match strategy {
        VersionResolutionStrategy::Strict => unreachable!("Strict already returned early"),
        VersionResolutionStrategy::Closest => {
            // Find minimum |detected.minor - supported.minor|; ties go to lower minor, then lower patch
            same_major
                .into_iter()
                .min_by(|(a, _), (b, _)| {
                    let dist_a = (detected.minor as i64 - a.minor as i64).unsigned_abs();
                    let dist_b = (detected.minor as i64 - b.minor as i64).unsigned_abs();
                    dist_a
                        .cmp(&dist_b)
                        .then(a.minor.cmp(&b.minor))
                        .then(a.patch.cmp(&b.patch))
                })
                .map(|(_, dv)| dv)
                .expect("same_major is non-empty")
        }
        VersionResolutionStrategy::Latest => {
            // Pick highest minor within same major
            same_major
                .into_iter()
                .max_by_key(|(v, _)| v.minor)
                .map(|(_, dv)| dv)
                .expect("same_major is non-empty")
        }
    };

    tracing::warn!(
        detected = %detected_str,
        resolved = %resolved,
        ?strategy,
        "NiFi version not directly supported, using fallback"
    );

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamic::DetectedVersion;

    /// Stub strict_match mirroring the generated version_from_str behavior (major.minor matching).
    fn stub_strict_match(version: &str) -> Result<DetectedVersion, crate::NifiError> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 2 {
            return Err(crate::NifiError::UnsupportedVersion {
                detected: version.to_string(),
            });
        }
        let major_minor = format!("{}.{}", parts[0], parts[1]);
        match major_minor.as_str() {
            "2.6" => Ok(DetectedVersion::V2_6_0),
            "2.7" => Ok(DetectedVersion::V2_7_2),
            "2.8" => Ok(DetectedVersion::V2_8_0),
            _ => Err(crate::NifiError::UnsupportedVersion {
                detected: version.to_string(),
            }),
        }
    }

    fn supported() -> Vec<(&'static str, DetectedVersion)> {
        vec![
            ("2.6.0", DetectedVersion::V2_6_0),
            ("2.7.2", DetectedVersion::V2_7_2),
            ("2.8.0", DetectedVersion::V2_8_0),
        ]
    }

    #[test]
    fn strict_exact_match() {
        let result = resolve_version(
            "2.8.1",
            VersionResolutionStrategy::Strict,
            stub_strict_match,
            &supported(),
        );
        assert_eq!(result.unwrap(), DetectedVersion::V2_8_0);
    }

    #[test]
    fn strict_unknown_errors() {
        let result = resolve_version(
            "2.5.0",
            VersionResolutionStrategy::Strict,
            stub_strict_match,
            &supported(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn closest_picks_nearest_lower() {
        // 2.5.0 with [2.6, 2.7, 2.8] — closest is 2.6
        let result = resolve_version(
            "2.5.0",
            VersionResolutionStrategy::Closest,
            stub_strict_match,
            &supported(),
        );
        assert_eq!(result.unwrap(), DetectedVersion::V2_6_0);
    }

    #[test]
    fn closest_equidistant_picks_lower() {
        // 2.7.0 with [2.6, 2.8] (no exact 2.7 match) — equidistant, picks lower (2.6)
        let versions = vec![
            ("2.6.0", DetectedVersion::V2_6_0),
            ("2.8.0", DetectedVersion::V2_8_0),
        ];
        let result = resolve_version(
            "2.7.0",
            VersionResolutionStrategy::Closest,
            |v| {
                Err(crate::NifiError::UnsupportedVersion {
                    detected: v.to_string(),
                })
            },
            &versions,
        );
        assert_eq!(result.unwrap(), DetectedVersion::V2_6_0);
    }

    #[test]
    fn closest_cross_major_errors() {
        let result = resolve_version(
            "1.25.0",
            VersionResolutionStrategy::Closest,
            stub_strict_match,
            &supported(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn latest_picks_highest() {
        // 2.5.0 with [2.6, 2.7, 2.8] — latest is 2.8
        let result = resolve_version(
            "2.5.0",
            VersionResolutionStrategy::Latest,
            stub_strict_match,
            &supported(),
        );
        assert_eq!(result.unwrap(), DetectedVersion::V2_8_0);
    }

    #[test]
    fn latest_cross_major_errors() {
        let result = resolve_version(
            "1.25.0",
            VersionResolutionStrategy::Latest,
            stub_strict_match,
            &supported(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn default_is_strict() {
        assert_eq!(
            VersionResolutionStrategy::default(),
            VersionResolutionStrategy::Strict
        );
    }

    #[test]
    fn malformed_version_errors() {
        let result = resolve_version(
            "bad",
            VersionResolutionStrategy::Closest,
            stub_strict_match,
            &supported(),
        );
        assert!(result.is_err());
    }
}
