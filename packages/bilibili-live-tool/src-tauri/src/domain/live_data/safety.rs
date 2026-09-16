use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use regex::Regex;

use crate::domain::validation::SensitiveWord;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensitiveWordCategory {
    Illegal,
    Advertising,
    PersonalAttack,
    Custom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensitiveMatchType {
    Exact,
    Fuzzy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitiveRule {
    pub id: i64,
    pub word: String,
    pub category: SensitiveWordCategory,
    pub match_type: SensitiveMatchType,
    pub is_enabled: bool,
}

impl SensitiveRule {
    pub fn new(
        id: i64,
        word: &str,
        category: SensitiveWordCategory,
        match_type: SensitiveMatchType,
        is_enabled: bool,
    ) -> Result<Self, SensitiveMatcherError> {
        if id <= 0 {
            return Err(SensitiveMatcherError::InvalidRule);
        }
        let word = SensitiveWord::parse(word)
            .map_err(|_| SensitiveMatcherError::InvalidRule)?
            .as_str()
            .to_owned();
        Ok(Self {
            id,
            word,
            category,
            match_type,
            is_enabled,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SensitiveMatcherError {
    #[error("sensitive rule is invalid")]
    InvalidRule,
    #[error("sensitive matcher could not be compiled")]
    Compile,
}

pub struct SensitiveMatcher {
    exact: Option<AhoCorasick>,
    exact_rule_ids: Vec<i64>,
    fuzzy: Vec<(i64, Regex)>,
}

impl SensitiveMatcher {
    pub fn empty() -> Self {
        Self {
            exact: None,
            exact_rule_ids: Vec::new(),
            fuzzy: Vec::new(),
        }
    }

    pub fn compile(rules: &[SensitiveRule]) -> Result<Self, SensitiveMatcherError> {
        let exact_rules: Vec<_> = rules
            .iter()
            .filter(|rule| rule.is_enabled && rule.match_type == SensitiveMatchType::Exact)
            .collect();
        let exact = if exact_rules.is_empty() {
            None
        } else {
            Some(
                AhoCorasickBuilder::new()
                    .match_kind(MatchKind::LeftmostLongest)
                    .build(exact_rules.iter().map(|rule| rule.word.as_str()))
                    .map_err(|_| SensitiveMatcherError::Compile)?,
            )
        };
        let fuzzy = rules
            .iter()
            .filter(|rule| rule.is_enabled && rule.match_type == SensitiveMatchType::Fuzzy)
            .map(|rule| {
                Regex::new(&rule.word)
                    .map(|pattern| (rule.id, pattern))
                    .map_err(|_| SensitiveMatcherError::Compile)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            exact,
            exact_rule_ids: exact_rules.iter().map(|rule| rule.id).collect(),
            fuzzy,
        })
    }

    pub fn match_rule(&self, content: &str) -> Option<i64> {
        if let Some(rule_match) = self
            .exact
            .as_ref()
            .and_then(|matcher| matcher.find(content))
        {
            return self
                .exact_rule_ids
                .get(rule_match.pattern().as_usize())
                .copied();
        }
        self.fuzzy
            .iter()
            .find_map(|(id, pattern)| pattern.is_match(content).then_some(*id))
    }
}

impl Default for SensitiveMatcher {
    fn default() -> Self {
        Self::empty()
    }
}
