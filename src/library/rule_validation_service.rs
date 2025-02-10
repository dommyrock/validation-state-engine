use crate::library::configuration_service::ValidationRulesGroupSettings;
use crate::ConfigurationService;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;

use super::configuration_service::Config;

pub struct RuleValidationService {
    config_rx: watch::Receiver<Config>,
}

impl RuleValidationService {
    pub async fn new(config_service: Arc<ConfigurationService>) -> Arc<Self> {
        let config_rx = config_service.subscribe();
        Arc::new(Self { config_rx })
    }

    // Process a single set of rules to completion
    pub async fn process_rules(&self, task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        //Parsed XML Rules to memore
        let rules: Vec<ValidationRulesGroupSettings> = self
            .config_rx
            .borrow()
            .clone()
            .validation_rules
            .groups
            .validation_rules_groups;

        // let eval = rules.iter().filter(|x| rule_c)
        println!("Task {} starting to process rules: {:?}", task_name, rules);

        // Process each rule
        for rule in rules {
            // Simulate some actual processing work
            println!("Task {} processing rule: {:?}", task_name, rule);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        println!("Task {} completed processing all rules", task_name);
        Ok(())
    }

    //TODO: Implement a method to get the current set of rules

    //TODO: Implement a method to get specific rule by ID (enum RuleType)
}
