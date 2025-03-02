pub use crate::config::prelude::*;
use crate::library::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use quick_xml::{de::from_str, events::Event, Reader};
use tokio::{fs::File, io::AsyncReadExt, sync::watch};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug)]
pub struct ConfigurationService {
    config_path: PathBuf,
    tx: watch::Sender<Config>,
    rx: watch::Receiver<Config>,
}

impl ConfigurationService {
    ///Spawns a task that watches for changes in the configuration file </br>
    /// NOTE: Be careful to validate that ValidationRule handlers actually run in the separate thasks!
    pub async fn new(config_path: String) -> Arc<Self> {
        let initial_config = Self::read_config(&config_path)
            .await
            .map_err(|e| format!("Initial config failed to load. / {e}"))
            .unwrap();

        let (tx, rx) = watch::channel(initial_config);
        //TODO : 'tx' --> In the future i might have separate task worker that updates xml file at random (to simulate "simulation engine" that updates the XML file)

        let service = Arc::new(Self {
            config_path: PathBuf::from(config_path),
            tx,
            rx,
        });

        let service_clone = Arc::clone(&service);

        // Spawn the file watcher
        tokio::spawn(async move {
            println!("Spawning CONFIG watcher ...");
            service_clone.watch_config_changes().await;
        });

        service
    }

    pub fn subscribe(&self) -> watch::Receiver<Config> {
        self.rx.clone()
    }

    /// reads config once.
    async fn read_config(path: &str) -> Result<Config> {
        println!("Reader -- reading config ... ");
        let xml = std::fs::read_to_string(path)?;
        let mut reader = Reader::from_str(&xml);

        // loop {
        //    //TODO: config.rs test suite code goes here
        // }

        let _config: Config = from_str(&xml)?;
        Ok(_config)
    }

    // **example** of attribute parsing

    // fn handle_terminal(e: BytesStart) -> RtResult<String> {
    //     let name = String::from_utf8(e.name().0.into())?;
    //     let action = find_ros_action(name.as_str())
    //         .ok_or(RuntimeError::WrongArgument(format!(r#"ros analogue not found for node {:?}. Check the import "ros::nav2""#, name)))?;
    //     let mut args = vec![];

    //     for attr_res in e.attributes() {
    //         let attr = attr_res?;
    //         let key = String::from_utf8(attr.key.0.to_vec())?;
    //         let v_str = String::from_utf8(attr.value.to_vec())?;
    //         if v_str.starts_with("{") && v_str.ends_with("}") {
    //             let value = v_str.trim_start_matches("{").trim_end_matches("}");
    //             args.push(RtArgument::new(key, RtValue::Pointer(value.to_string())).to_string());
    //         } else {
    //             let param = find_action(&action, key.clone())?;
    //             let argument = RtArgument::new(key, convert_arg(&action, v_str, param)?);
    //             args.push(argument.to_string());
    //         }
    //     }

    //     Ok(format!("{}({})", action.name, args.join(", ")))
    // }

    /// continuously reads config
    async fn watch_config_changes(&self) {
        let mut one_sec = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            one_sec.tick().await;

            match Self::read_config(self.config_path.to_str().unwrap()).await {
                Ok(_new_cfg) => {}
                Err(e) => eprintln!(
                    "--> Config ERROR: {e}" //TODO: At this point we exited the Reader buffer and should have
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::RuleType;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_rule_validation_settings_parsing() {
        let service = ConfigurationService::new("test_config.xml".to_string()).await;

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
