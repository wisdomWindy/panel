use serde::{Deserialize, Serialize};

pub use crate::application::live_data::SensitiveFileFormat;
use crate::domain::live_data::{SensitiveMatchType, SensitiveRule, SensitiveWordCategory};
use crate::infrastructure::text;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BlacklistFileRow {
    pub uid: String,
    pub username: String,
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum FileCodecError {
    #[error("file data is invalid")]
    InvalidData,
    #[error("csv data is invalid: {0}")]
    Csv(#[from] csv::Error),
    #[error("json data is invalid: {0}")]
    Json(#[from] serde_json::Error),
    #[error("encoded file is not UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SensitiveFileRow {
    word: String,
    category: String,
    match_type: String,
    enabled: bool,
}

pub fn encode_sensitive_words(
    rules: &[SensitiveRule],
    format: SensitiveFileFormat,
) -> Result<Vec<u8>, FileCodecError> {
    let rows: Vec<_> = rules.iter().map(SensitiveFileRow::from).collect();
    match format {
        SensitiveFileFormat::Csv => {
            let mut writer = csv::Writer::from_writer(Vec::new());
            for row in rows {
                writer.serialize(row)?;
            }
            writer
                .into_inner()
                .map(crate::infrastructure::text::with_utf8_bom)
                .map_err(|error| FileCodecError::Csv(error.into_error().into()))
        }
        SensitiveFileFormat::Json => Ok(serde_json::to_vec_pretty(&rows)?),
        SensitiveFileFormat::Txt => {
            let mut output = String::new();
            for row in rows {
                output.push_str(&serde_json::to_string(&row)?);
                output.push('\n');
            }
            Ok(output.into_bytes())
        }
    }
}

pub fn decode_sensitive_words(
    content: &[u8],
    format: SensitiveFileFormat,
) -> Result<Vec<SensitiveRule>, FileCodecError> {
    let rows = match format {
        // Files we export carry a UTF-8 BOM for Excel; strip it before parsing so
        // a round-trip does not turn the first header into "\u{FEFF}word".
        SensitiveFileFormat::Csv => csv::Reader::from_reader(text::strip_utf8_bom(content))
            .deserialize()
            .collect::<Result<Vec<SensitiveFileRow>, _>>()?,
        SensitiveFileFormat::Json => serde_json::from_slice(content)?,
        SensitiveFileFormat::Txt => String::from_utf8(content.to_vec())?
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(serde_json::from_str)
            .collect::<Result<Vec<SensitiveFileRow>, _>>()?,
    };
    rows.into_iter()
        .enumerate()
        .map(|(index, row)| row.into_rule((index + 1) as i64))
        .collect()
}

pub fn encode_blacklist_csv(rows: &[BlacklistFileRow]) -> Result<Vec<u8>, FileCodecError> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    for row in rows {
        writer.serialize(row)?;
    }
    writer
        .into_inner()
        .map(crate::infrastructure::text::with_utf8_bom)
        .map_err(|error| FileCodecError::Csv(error.into_error().into()))
}

pub fn decode_blacklist_csv(content: &[u8]) -> Result<Vec<BlacklistFileRow>, FileCodecError> {
    csv::Reader::from_reader(text::strip_utf8_bom(content))
        .deserialize()
        .collect::<Result<Vec<_>, _>>()
        .map_err(FileCodecError::from)
}

impl From<&SensitiveRule> for SensitiveFileRow {
    fn from(rule: &SensitiveRule) -> Self {
        Self {
            word: rule.word.clone(),
            category: match rule.category {
                SensitiveWordCategory::Illegal => "illegal",
                SensitiveWordCategory::Advertising => "advertising",
                SensitiveWordCategory::PersonalAttack => "personal_attack",
                SensitiveWordCategory::Custom => "custom",
            }
            .into(),
            match_type: match rule.match_type {
                SensitiveMatchType::Exact => "exact",
                SensitiveMatchType::Fuzzy => "fuzzy",
            }
            .into(),
            enabled: rule.is_enabled,
        }
    }
}

impl SensitiveFileRow {
    fn into_rule(self, id: i64) -> Result<SensitiveRule, FileCodecError> {
        let category = match self.category.as_str() {
            "illegal" => SensitiveWordCategory::Illegal,
            "advertising" => SensitiveWordCategory::Advertising,
            "personal_attack" => SensitiveWordCategory::PersonalAttack,
            "custom" => SensitiveWordCategory::Custom,
            _ => return Err(FileCodecError::InvalidData),
        };
        let match_type = match self.match_type.as_str() {
            "exact" => SensitiveMatchType::Exact,
            "fuzzy" => SensitiveMatchType::Fuzzy,
            _ => return Err(FileCodecError::InvalidData),
        };
        SensitiveRule::new(id, &self.word, category, match_type, self.enabled)
            .map_err(|_| FileCodecError::InvalidData)
    }
}
