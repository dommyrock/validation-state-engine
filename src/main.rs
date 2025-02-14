mod library;

use library::{
    configuration_service::ConfigurationService, rule_types::RuleType,
    rule_validation_service::RuleValidationService,
};
use std::sync::Arc;
pub use self::library::errors::{Error, Result};

#[tokio::main]
async fn main() {
    let cfg = vec![
        RuleType::SideJobPrevention,
        RuleType::ExhaustionPrevention,
        RuleType::LastMinuteActionPreventionForBooking,
    ];

    let config_service = ConfigurationService::new("validator_config.xml".to_string(), cfg).await;
    let service = RuleValidationService::new(Arc::clone(&config_service)).await;

    // Get a receiver to watch for configuration changes
    let mut config_rx = config_service.subscribe();

    loop {
        //Wait for configuration changes
        if config_rx.changed().await.is_ok() {
            // Spawn tasks for the new configuration (memory mapped XML parsed BusinessRule state changes)
            let rules = config_rx.borrow().clone().config_rules;

            println!(
                "\nConfiguration change detected - spawning {} new tasks ...",
                rules.len()
            );

            let mut tasks = Vec::with_capacity(rules.len());

            for (_i, rule) in rules.iter().enumerate() {
                let service_clone = Arc::clone(&service);

                let task_name = format!("{:?}", rule); //temp placeholder

                let task = tokio::spawn(async move {
                    if let Err(e) = service_clone.process_rules(&task_name).await {
                        //TODO this should have custom error type and push to Vec<CustomError>
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
