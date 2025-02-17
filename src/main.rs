mod config;
mod library;

use config::config::{ValidationRuleSettings, ValidationRulesGroupSettings};
use library::{
    configuration_service::ConfigurationService, rule_validation_service::RuleValidationService,
    RuleType,
};
use std::sync::{Arc, LazyLock};

static CONFIG_RULES: LazyLock<Vec<RuleType>> = LazyLock::new(|| {
    vec![
        RuleType::SideJobPrevention,
        RuleType::ExhaustionPrevention,
        RuleType::LastMinuteActionPreventionForBooking,
    ]
});

#[tokio::main]
async fn main() -> ! {
    let cfg = &*CONFIG_RULES;
    let config_service = ConfigurationService::new("validator_config.xml".to_string()).await;
    let service = RuleValidationService::new(Arc::clone(&config_service)).await;

    // Get a receiver to watch for configuration changes
    let mut config_rx = config_service.subscribe();

    let filter_configured_groups =
        |x: &ValidationRulesGroupSettings| -> Vec<ValidationRuleSettings> {
            x.validation_rules
                .iter()
                .filter(|rule| cfg.contains(&rule.rule_type) && rule.enabled)
                .cloned()
                .collect()
        };

    loop {
        //Wait for configuration changes / Spawn task handlers for processing Rule changes
        if config_rx.changed().await.is_ok() {
            let config_clone = config_rx.borrow().clone();

            let validation_rule_groups: Vec<Vec<ValidationRuleSettings>> = config_clone
                .validation_rules
                .groups
                .validation_rules_groups
                .iter()
                .map(filter_configured_groups)
                .filter(|x: &Vec<ValidationRuleSettings>| !x.is_empty())
                .collect();

            println!("Fetched {:#?} ", validation_rule_groups);

            println!(
                "\nConfiguration change detected - spawning {} new tasks ...",
                cfg.len()
            );

            let mut tasks = Vec::with_capacity(cfg.len());

            for (_i, rule) in cfg.iter().enumerate() {
                let service_clone = Arc::clone(&service);

                let task_name = format!("{:?}", rule); //temp placeholder

                let task = tokio::spawn(async move {
                    if let Err(e) = service_clone.process_rules(&task_name).await {
                        eprintln!("{} encountered an error: {}", task_name, e);
                    }
                });

                tasks.push(task);
            }

            // Wait for all tasks to complete
            for (i, handle) in tasks.into_iter().enumerate() {
                match handle.await {
                    Ok(_) => println!("Task_{} completed successfully", i + 1),
                    Err(e) => eprintln!("Task_{} failed: {}", i + 1, e),
                }
            }

            println!("All tasks completed - waiting for next configuration change...");
        }
    }
}
