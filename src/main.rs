mod library;
use library::ConfigurationService::Configuration;
use library::RuleType;
use library::{
    BusinessRuleService::BusinessRuleService, ConfigurationService::ConfigurationService,
};

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let cfg = Configuration {
        rules: vec![
            RuleType::SideJobPrevention,
            RuleType::ExhaustionPrevention,
            RuleType::LastMinuteBookingPrevention,
        ],
    };

    let config_service = ConfigurationService::new("validator_config.xml".to_string()).await;
    let service = BusinessRuleService::new(Arc::clone(&config_service), cfg).await;

    // Get a receiver to watch for configuration changes
    let mut config_rx = config_service.subscribe();

    loop {
        // Wait for configuration changes
        if config_rx.changed().await.is_ok() {
            // Spawn tasks for the new configuration (memory mapped XML parsed BusinessRule state changes)
            let rules = config_rx.borrow().clone().rules;

            println!(
                "\nConfiguration change detected - spawning {} new tasks...",
                rules.len()
            );

            let mut tasks = Vec::with_capacity(rules.len());

            for (_i, rule) in rules.iter().enumerate() {
                let service_clone = Arc::clone(&service);

                let task_name = format!("{:?}", rule);//temp placeholder

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
