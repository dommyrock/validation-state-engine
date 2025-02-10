use crate::RuleType;
use std::path::PathBuf;
use std::sync::Arc;

use quick_xml::de::from_str;
use serde::de::{Deserializer, Error};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt, sync::watch};

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
    #[serde(rename = "@Type")] // Fix for XML attribute
    pub rule_type: RuleType,
    #[serde(rename = "@Enabled", deserialize_with = "parse_bool")]
    pub enabled: bool, // Needs custom parsing if you want a `bool`
    #[serde(rename = "@FallbackShiftStatusId", default)]
    pub fallback_shift_status_id: Option<i32>,
    #[serde(rename = "@PositionTypeIDs", deserialize_with = "parse_csv_string", default)]
    pub position_type_ids: Vec<i32>, // Will be empty vec when attribute is missing
    #[serde(rename = "@FromMatchStatusId",default)]
    pub from_match_status_id: Option<i32>,
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
        _ => Err(D::Error::custom(format!("Invalid boolean value: {}", s))),
    }
}

// TODO :Check dependency injection exampple to fugure out if will use trait objects or generics

// pub trait ConfigurationProvider {
//     async fn new(config_path: String) -> Arc<Self>;
//     fn subscribe(&self) -> watch::Receiver<Configuration>;
// }

pub struct ConfigurationService {
    config_path: PathBuf,
    tx: watch::Sender<Config>,
    rx: watch::Receiver<Config>,
}

impl ConfigurationService {
    pub async fn new(config_path: String, config_rules: Vec<RuleType>) -> Arc<Self> {
        let mut initial_config = Self::read_config(&config_path)
            .await
            .expect("Failed to read initial configuration");

        initial_config.config_rules = config_rules;

        let (tx, rx) = watch::channel(initial_config);
        //TODO : 'tx' --> In the future i might have separate task worker that updates xml file at random (to simulate "simulation engine" that updates the XML file)

        let service = Arc::new(Self {
            config_path: PathBuf::from(config_path),
            tx,
            rx,
        });

        // Spawn the file watcher
        let service_clone = Arc::clone(&service);

        tokio::spawn(async move {
            service_clone.watch_config_changes().await;
        });

        service
    }

    pub fn subscribe(&self) -> watch::Receiver<Config> {
        self.rx.clone()
    }

    async fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        //let _config_v1: Configuration = from_str(&contents)?;

        let config: Config = from_str(&contents)?;

        //TODO: I need to create struct For Each rule type and have it tested in unit tests quick_xml::de::from_str; to vllidate correct parsing of XML file
        //DOCS: https://crates.io/crates/quick-xml (check the 'Serde' section)

        Ok(config)
    }

    async fn watch_config_changes(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            interval.tick().await;

            match Self::read_config(self.config_path.to_str().unwrap()).await {
                Ok(new_config) => {
                    if new_config != *self.rx.borrow() {
                        //clone ?
                        if let Err(e) = self.tx.send(new_config) {
                            eprintln!("Failed to send XML config update: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Error reading configuration: {}", e),
            }
        }
    }
}

//TODO : Since we're cnverting XML content to Potential ValidationRuleError .
/*
-> IMPLEMENT 'FROM' TRAIT FOR ANY
-> Need to check error type that we're working here so that we can convert that error into ours or inverse.

example-ref : https://learning-rust.github.io/docs/custom-error-types/ (last section) */

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_rule_validation_settings_parsing() {
        let service = ConfigurationService::new(
            "test_config.xml".to_string(),
            vec![RuleType::SideJobPrevention],
        )
        .await;

        // Get config via subscription
        let receiver = service.subscribe();
        let config = receiver.borrow().clone();

        dbg!(&config);

        assert_eq!(
            config.validation_rules.groups.validation_rules_groups[0].validation_rules[0].rule_type,
            RuleType::SideJobPrevention
        );
        assert_eq!(
            config.validation_rules.groups.validation_rules_groups[0].validation_rules[0].enabled,
            true
        );
        assert_eq!(
            config.validation_rules.groups.validation_rules_groups[0].validation_rules[0]
                .position_type_ids,
            vec![1]
        );
        assert_eq!(
            config.validation_rules.groups.validation_rules_groups[0].validation_rules[0]
                .from_match_status_id,
            Some(0)
        );
    }
}

// TODO: Test config file update

// let updated_content = r#"<?xml version="1.0" encoding="UTF-8"?>
//     <Configuration>
//         <rules>
//             <RuleType>ValidationRule</RuleType>
//         </rules>
//     </Configuration>"#;

// // fs::write(&tmp_file, updated_content).unwrap();

// // Wait for watcher to detect change
// tokio::time::sleep(std::time::Duration::from_secs(2)).await;

// let updated_config = receiver.borrow().clone();
// assert_eq!(updated_config.rules.len(), 1);
// assert!(matches!(updated_config.rules[0], RuleType::ValidationRule));

/*Serde Deserializer DOCS: https://serde.rs/impl-deserialize.html*/
