use crate::ConfigurationService;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;

use super::ConfigurationService::Configuration;

pub struct BusinessRuleService {
    config_rx: watch::Receiver<Configuration>,
}

impl BusinessRuleService {
    pub async fn new(
        config_service: Arc<ConfigurationService>,
        rule_cfg: Configuration,
    ) -> Arc<Self> {
        let config_rx = config_service.subscribe();
        Arc::new(Self { config_rx })
    }

    // Process a single set of rules to completion
    pub async fn process_rules(&self, task_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let rules = self.config_rx.borrow().clone().rules;
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
}
