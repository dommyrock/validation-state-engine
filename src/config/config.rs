use crate::RuleType;

use serde::de::{Deserializer, Error};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    #[serde(rename = "ValidationRules")]
    pub validation_rules: ValidationRulesContainer,
    #[serde(skip_deserializing)]
    pub config_rules: Vec<RuleType>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationRulesContainer {
    #[serde(rename = "Groups")]
    pub groups: GroupsContainer,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GroupsContainer {
    #[serde(rename = "ValidationRulesGroup")]
    pub validation_rules_groups: Vec<ValidationRulesGroupSettings>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationRulesGroupSettings {
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "ValidationRule")]
    pub validation_rules: Vec<ValidationRuleSettings>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationRuleSettings {
    #[serde(rename = "@Type")]
    pub rule_type: RuleType,
    #[serde(rename = "@Enabled", deserialize_with = "parse_bool")]
    pub enabled: bool,
    #[serde(rename = "@FallbackShiftStatusId", default)]
    pub fallback_shift_status_id: Option<i32>,
    #[serde(
        rename = "@PositionTypeIDs",
        deserialize_with = "parse_csv_string",
        default
    )]
    pub position_type_ids: Vec<i32>,
    #[serde(rename = "@FromMatchStatusId", default)]
    pub from_match_status_id: Option<i32>,
    #[serde(rename = "Rules")]
    pub rules: RulesContainer,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RulesContainer {
    #[serde(rename = "Rule", default)]
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Rule {
    #[serde(
        rename = "@ForCandidateStatusIds",
        deserialize_with = "parse_csv_string",
        default
    )]
    for_candidate_status_ids: Vec<i32>,
    #[serde(rename = "@Enforce", deserialize_with = "parse_bool", default)]
    enforce: bool,
}

fn parse_csv_string<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().map_err(Error::custom))
        .collect()
}

fn parse_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        "" => Ok(false),
        " " => Ok(false),
        _ => Err(D::Error::custom(format!("Invalid boolean value: {}", s))),
    }
}
