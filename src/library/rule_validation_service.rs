use crate::config::prelude::*;
use crate::library::configuration_service::ValidationRulesGroupSettings;
use crate::library::Result;
use crate::{ConfigurationService, CONFIG_RULES};

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;

pub struct RuleValidationService {
    config_rx: watch::Receiver<Config>,
}

impl RuleValidationService {
    pub async fn new(config_service: Arc<ConfigurationService>) -> Arc<Self> {
        let config_rx = config_service.subscribe();
        Arc::new(Self { config_rx })
    }

    // Process a single set of rules to completion
    pub async fn process_rules(&self, task_name: &str) -> Result<()> {
        let rules_from_config: Vec<ValidationRulesGroupSettings> = self
            .config_rx
            .borrow()
            .clone()
            .validation_rules
            .groups
            .validation_rules_groups;

        println!(
            "Task {} starting to process rules:",
            task_name //, rules_from_config
        );

        for rule in rules_from_config {
            // println!("Task {} processing rule: {:?}", task_name, rule);
            println!("Task {} processing rule", task_name);
            //DO WORK HERE ...
        }

        println!("Task {} completed processing all rules", task_name);
        Ok(())
    }

    //TODO: Implement a method to get the current set of rules

    //TODO: Implement a method to get specific rule by ID (enum RuleType)
}
