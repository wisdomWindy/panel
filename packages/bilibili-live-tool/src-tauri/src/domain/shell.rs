use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SHELL_COMMAND_NAMES: &[&str] = &[
    "shell_status",
    "config_overview",
    "config_open_directory",
    "config_export",
    "config_import",
    "config_reset",
    "locale_get",
    "locale_set_auto",
    "locale_save",
    "ui_get",
    "ui_preview",
    "ui_save",
    "ui_reset",
];

pub const SHELL_EVENT_NAMES: &[&str] = &[
    "shell_status_changed",
    "locale_changed",
    "ui_preferences_changed",
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LocaleCode {
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "en-US")]
    EnUs,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalePreferences {
    pub locale: LocaleCode,
    pub follow_system: bool,
}

impl Default for LocalePreferences {
    fn default() -> Self {
        Self {
            locale: LocaleCode::ZhCn,
            follow_system: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FontSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LayoutMode {
    Default,
    Custom,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FourRegionLayout {
    pub version: u8,
    pub navigation_width: u16,
    pub toolbar_height: u8,
    pub status_height: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiPreferences {
    pub theme: ThemeMode,
    pub accent_color: String,
    pub opacity: f64,
    pub font_size: FontSize,
    pub layout_mode: LayoutMode,
    pub custom_layout: Option<FourRegionLayout>,
    pub show_danmaku_level: bool,
    pub show_danmaku_medal: bool,
    pub show_danmaku_time: bool,
    pub max_danmaku_entries: u32,
}

impl Default for UiPreferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::System,
            accent_color: "#FB7299".to_owned(),
            opacity: 0.95,
            font_size: FontSize::Medium,
            layout_mode: LayoutMode::Default,
            custom_layout: None,
            show_danmaku_level: true,
            show_danmaku_medal: true,
            show_danmaku_time: true,
            max_danmaku_entries: 1_000,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ShellConfigScope {
    Room,
    SensitiveWords,
    Blacklist,
    GiftSounds,
    InterfaceLayout,
    Recording,
    #[cfg(feature = "member")]
    Ai,
    #[cfg(feature = "member")]
    Moderation,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigArchiveSection {
    pub scope: ShellConfigScope,
    pub entries: Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigArchiveV1 {
    pub version: u8,
    pub sections: Vec<ConfigArchiveSection>,
}

impl ShellConfigScope {
    pub const fn key_prefix(self) -> &'static str {
        match self {
            Self::Room => "room.",
            Self::SensitiveWords => "safety.sensitive.",
            Self::Blacklist => "safety.blacklist.",
            Self::GiftSounds => "media.gift_",
            Self::InterfaceLayout => "shell.",
            Self::Recording => "media.recording",
            #[cfg(feature = "member")]
            Self::Ai => "member.ai.",
            #[cfg(feature = "member")]
            Self::Moderation => "member.moderation.",
        }
    }
}

pub fn available_config_scopes() -> Vec<ShellConfigScope> {
    let scopes = vec![
        ShellConfigScope::Room,
        ShellConfigScope::SensitiveWords,
        ShellConfigScope::Blacklist,
        ShellConfigScope::GiftSounds,
        ShellConfigScope::InterfaceLayout,
        ShellConfigScope::Recording,
    ];
    #[cfg(feature = "member")]
    {
        let mut member_scopes = scopes;
        member_scopes.extend([ShellConfigScope::Ai, ShellConfigScope::Moderation]);
        member_scopes
    }
    #[cfg(not(feature = "member"))]
    {
        scopes
    }
}

pub const fn is_config_scope_available(scope: ShellConfigScope) -> bool {
    match scope {
        #[cfg(feature = "member")]
        ShellConfigScope::Ai | ShellConfigScope::Moderation => false,
        _ => true,
    }
}

pub fn executable_config_scopes() -> Vec<ShellConfigScope> {
    available_config_scopes()
        .into_iter()
        .filter(|scope| is_config_scope_available(*scope))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ShellValidationError {
    #[error("accent color must be #RRGGBB")]
    InvalidColor,
    #[error("opacity must be finite and between zero and one")]
    InvalidOpacity,
    #[error("custom layout does not match layout mode or bounds")]
    InvalidLayout,
    #[error("maximum danmaku entries must be between 1 and 10000")]
    InvalidMaxEntries,
    #[error("at least one available configuration scope is required")]
    InvalidConfigScope,
}

pub fn validate_ui_preferences(
    mut value: UiPreferences,
) -> Result<UiPreferences, ShellValidationError> {
    let accent = value.accent_color.trim().to_ascii_uppercase();
    let is_color = accent.len() == 7
        && accent.starts_with('#')
        && accent[1..].bytes().all(|byte| byte.is_ascii_hexdigit());
    if !is_color {
        return Err(ShellValidationError::InvalidColor);
    }
    if !value.opacity.is_finite() || !(0.0..=1.0).contains(&value.opacity) {
        return Err(ShellValidationError::InvalidOpacity);
    }
    let layout_is_valid = match (value.layout_mode, value.custom_layout) {
        (LayoutMode::Default, None) => true,
        (LayoutMode::Custom, Some(layout)) => {
            layout.version == 1
                && (176..=280).contains(&layout.navigation_width)
                && (48..=64).contains(&layout.toolbar_height)
                && (28..=40).contains(&layout.status_height)
        }
        _ => false,
    };
    if !layout_is_valid {
        return Err(ShellValidationError::InvalidLayout);
    }
    if !(1..=10_000).contains(&value.max_danmaku_entries) {
        return Err(ShellValidationError::InvalidMaxEntries);
    }
    value.accent_color = accent;
    Ok(value)
}

pub fn validate_config_scopes(
    scopes: &[ShellConfigScope],
) -> Result<Vec<ShellConfigScope>, ShellValidationError> {
    let mut normalized = Vec::new();
    for scope in scopes {
        if !is_config_scope_available(*scope) {
            return Err(ShellValidationError::InvalidConfigScope);
        }
        if !normalized.contains(scope) {
            normalized.push(*scope);
        }
    }
    if normalized.is_empty() {
        return Err(ShellValidationError::InvalidConfigScope);
    }
    Ok(normalized)
}

pub fn validate_config_archive(
    archive: ConfigArchiveV1,
) -> Result<ConfigArchiveV1, ShellValidationError> {
    if archive.version != 1 || archive.sections.is_empty() {
        return Err(ShellValidationError::InvalidConfigScope);
    }
    let scopes = archive
        .sections
        .iter()
        .map(|section| section.scope)
        .collect::<Vec<_>>();
    let normalized = validate_config_scopes(&scopes)?;
    if normalized.len() != archive.sections.len()
        || archive
            .sections
            .iter()
            .any(|section| !section.entries.is_array())
    {
        return Err(ShellValidationError::InvalidConfigScope);
    }
    Ok(archive)
}

pub fn resolve_system_locale(value: Option<&str>) -> LocaleCode {
    let normalized = value.unwrap_or_default().to_ascii_lowercase();
    if normalized.starts_with("en") {
        LocaleCode::EnUs
    } else {
        LocaleCode::ZhCn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezes_public_surface_and_source_defaults() {
        assert_eq!(SHELL_COMMAND_NAMES.len(), 13);
        assert_eq!(SHELL_EVENT_NAMES.len(), 3);
        assert_eq!(UiPreferences::default().theme, ThemeMode::System);
        assert_eq!(UiPreferences::default().accent_color, "#FB7299");
        assert_eq!(UiPreferences::default().opacity, 0.95);
        assert_eq!(UiPreferences::default().max_danmaku_entries, 1_000);
    }

    #[test]
    fn validates_and_canonicalizes_ui_preferences() {
        let valid = validate_ui_preferences(UiPreferences {
            accent_color: " #aabbcc ".to_owned(),
            ..UiPreferences::default()
        })
        .unwrap();
        assert_eq!(valid.accent_color, "#AABBCC");

        for opacity in [f64::NAN, -0.1, 1.1] {
            assert_eq!(
                validate_ui_preferences(UiPreferences {
                    opacity,
                    ..UiPreferences::default()
                }),
                Err(ShellValidationError::InvalidOpacity)
            );
        }
        assert_eq!(
            validate_ui_preferences(UiPreferences {
                layout_mode: LayoutMode::Custom,
                custom_layout: None,
                ..UiPreferences::default()
            }),
            Err(ShellValidationError::InvalidLayout)
        );
    }

    #[test]
    fn locale_falls_back_to_chinese_and_scopes_are_closed() {
        assert_eq!(resolve_system_locale(Some("en-GB")), LocaleCode::EnUs);
        assert_eq!(resolve_system_locale(Some("ja-JP")), LocaleCode::ZhCn);
        assert_eq!(resolve_system_locale(None), LocaleCode::ZhCn);
        assert!(validate_config_scopes(&[]).is_err());
        assert_eq!(
            validate_config_scopes(&[ShellConfigScope::Room, ShellConfigScope::Room]).unwrap(),
            vec![ShellConfigScope::Room]
        );
    }

    #[test]
    fn archive_requires_version_one_unique_available_array_sections() {
        let valid = ConfigArchiveV1 {
            version: 1,
            sections: vec![ConfigArchiveSection {
                scope: ShellConfigScope::InterfaceLayout,
                entries: serde_json::json!([]),
            }],
        };
        assert_eq!(validate_config_archive(valid.clone()).unwrap(), valid);
        assert!(validate_config_archive(ConfigArchiveV1 {
            version: 2,
            ..valid.clone()
        })
        .is_err());
        assert!(validate_config_archive(ConfigArchiveV1 {
            version: 1,
            sections: vec![valid.sections[0].clone(), valid.sections[0].clone()],
        })
        .is_err());
    }

    #[cfg(not(feature = "member"))]
    #[test]
    fn free_scope_projection_contains_exactly_six_items() {
        assert_eq!(available_config_scopes().len(), 6);
        assert_eq!(executable_config_scopes().len(), 6);
    }

    #[cfg(feature = "member")]
    #[test]
    fn member_scope_projection_contains_two_additional_items() {
        assert_eq!(available_config_scopes().len(), 8);
        assert_eq!(executable_config_scopes().len(), 6);
        assert!(!executable_config_scopes().contains(&ShellConfigScope::Ai));
        assert!(!executable_config_scopes().contains(&ShellConfigScope::Moderation));
        assert!(!is_config_scope_available(ShellConfigScope::Ai));
        assert!(!is_config_scope_available(ShellConfigScope::Moderation));
        assert!(validate_config_scopes(&[ShellConfigScope::Ai]).is_err());
    }
}
