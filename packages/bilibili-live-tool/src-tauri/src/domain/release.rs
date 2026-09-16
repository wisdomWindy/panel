use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateStatus {
    Unavailable,
    Current,
    Available,
    Downloading,
    Downloaded,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCandidate {
    pub version: String,
    pub release_notes: Vec<String>,
    pub rollout_percent: u8,
    pub is_paused: bool,
    pub is_forced: bool,
    pub blacklisted_versions: Vec<String>,
}

pub fn is_valid_update_candidate(candidate: &UpdateCandidate) -> bool {
    candidate.rollout_percent <= 100
        && semver::Version::parse(&candidate.version).is_ok()
        && candidate
            .blacklisted_versions
            .iter()
            .all(|version| semver::Version::parse(version).is_ok())
}

pub fn is_update_eligible(
    candidate: &UpdateCandidate,
    current_version: &str,
    installation_bucket: u8,
) -> bool {
    if !is_valid_update_candidate(candidate) || installation_bucket > 99 || candidate.is_paused {
        return false;
    }
    is_update_forced(candidate, current_version) || installation_bucket < candidate.rollout_percent
}

pub fn is_update_forced(candidate: &UpdateCandidate, current_version: &str) -> bool {
    candidate.is_forced
        || candidate
            .blacklisted_versions
            .iter()
            .any(|value| value == current_version)
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginPermission {
    SanitizedEvent,
    CurrentRoomUid,
    Effect,
    Panel,
    DataSource,
    Network,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluginResource {
    SanitizedEvent,
    CurrentRoomUid,
    Effect,
    Panel,
    DataSource,
    DeclaredNetwork,
    Config,
    Database,
    Key,
    Login,
    Filesystem,
    UndeclaredNetwork,
}

pub fn can_access_plugin_resource(
    permissions: &[PluginPermission],
    resource: PluginResource,
) -> bool {
    let required = match resource {
        PluginResource::SanitizedEvent => Some(PluginPermission::SanitizedEvent),
        PluginResource::CurrentRoomUid => Some(PluginPermission::CurrentRoomUid),
        PluginResource::Effect => Some(PluginPermission::Effect),
        PluginResource::Panel => Some(PluginPermission::Panel),
        PluginResource::DataSource => Some(PluginPermission::DataSource),
        PluginResource::DeclaredNetwork => Some(PluginPermission::Network),
        PluginResource::Config
        | PluginResource::Database
        | PluginResource::Key
        | PluginResource::Login
        | PluginResource::Filesystem
        | PluginResource::UndeclaredNetwork => None,
    };
    required.is_some_and(|value| permissions.contains(&value))
}

pub fn is_valid_network_domain(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 253
        && value.is_ascii()
        && !value.contains(['/', ':', '@'])
        && value.split('.').all(|part| {
            !part.is_empty()
                && part.len() <= 63
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

pub fn is_valid_plugin_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
}

pub fn is_valid_wasm_module_name(value: &str) -> bool {
    let path = std::path::Path::new(value);
    !value.is_empty()
        && value.is_ascii()
        && !value.contains(['/', '\\'])
        && path.file_name().and_then(|name| name.to_str()) == Some(value)
        && path.extension().and_then(|extension| extension.to_str()) == Some("wasm")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(rollout_percent: u8) -> UpdateCandidate {
        UpdateCandidate {
            version: "1.1.0".into(),
            release_notes: vec![],
            rollout_percent,
            is_paused: false,
            is_forced: false,
            blacklisted_versions: vec![],
        }
    }

    #[test]
    fn rollout_pause_and_blacklist_are_deterministic() {
        assert!(!is_update_eligible(&candidate(0), "1.0.0", 0));
        assert!(is_update_eligible(&candidate(5), "1.0.0", 4));
        assert!(!is_update_eligible(&candidate(5), "1.0.0", 5));
        assert!(is_update_eligible(&candidate(100), "1.0.0", 99));

        let mut paused = candidate(100);
        paused.is_paused = true;
        assert!(!is_update_eligible(&paused, "1.0.0", 0));

        let mut blacklisted = candidate(0);
        blacklisted.blacklisted_versions.push("1.0.0".into());
        assert!(is_update_eligible(&blacklisted, "1.0.0", 99));
        assert!(is_update_forced(&blacklisted, "1.0.0"));

        let mut forced = candidate(0);
        forced.is_forced = true;
        assert!(is_update_eligible(&forced, "1.0.0", 99));
        assert!(is_update_forced(&forced, "1.0.0"));

        let mut invalid_version = candidate(100);
        invalid_version.version = "latest".into();
        assert!(!is_valid_update_candidate(&invalid_version));

        let mut invalid_rollout = candidate(101);
        assert!(!is_valid_update_candidate(&invalid_rollout));
        invalid_rollout.rollout_percent = 100;
        invalid_rollout.blacklisted_versions.push("broken".into());
        assert!(!is_valid_update_candidate(&invalid_rollout));
    }

    #[test]
    fn forbidden_plugin_resources_are_always_denied() {
        let all = [
            PluginPermission::SanitizedEvent,
            PluginPermission::CurrentRoomUid,
            PluginPermission::Effect,
            PluginPermission::Panel,
            PluginPermission::DataSource,
            PluginPermission::Network,
        ];
        assert!(can_access_plugin_resource(
            &all,
            PluginResource::SanitizedEvent
        ));
        for resource in [
            PluginResource::Config,
            PluginResource::Database,
            PluginResource::Key,
            PluginResource::Login,
            PluginResource::Filesystem,
            PluginResource::UndeclaredNetwork,
        ] {
            assert!(!can_access_plugin_resource(&all, resource));
        }
    }

    #[test]
    fn network_allowlist_accepts_domains_only() {
        assert!(is_valid_network_domain("api.example.com"));
        assert!(!is_valid_network_domain("https://api.example.com"));
        assert!(!is_valid_network_domain("../secret"));
        assert!(!is_valid_network_domain(""));
    }

    #[test]
    fn permission_and_wasm_validation_fail_closed() {
        assert!(serde_json::from_str::<PluginPermission>("\"filesystem\"").is_err());
        assert!(is_valid_plugin_id("captain.effect-1"));
        assert!(!is_valid_plugin_id("../plugin"));
        assert!(is_valid_wasm_module_name("plugin.wasm"));
        assert!(!is_valid_wasm_module_name("plugin.js"));
        assert!(!is_valid_wasm_module_name("../plugin.wasm"));
    }
}
