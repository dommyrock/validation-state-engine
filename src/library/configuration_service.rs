pub use crate::config::prelude::*;
use crate::library::{Error, Result};

use std::path::PathBuf;
use std::sync::Arc;

use quick_xml::{de::from_str, events::Event};
use quick_xml::{DeError, Reader};
use tokio::{fs::File, io::AsyncReadExt, sync::watch};

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
            .expect("Failed to read initial configuration");

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

    async fn read_config(path: &str) -> Result<Config> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let mut reader = Reader::from_str(&contents);
        reader.config_mut().trim_text(true);
        let config: Config = from_str(&contents)?;

        loop {
            match reader.read_event() {//TODO: nonsense code ---refactor with custom deserializer
                Ok(Event::Start(e)) if e.name().as_ref() == b"rule" => {
                    let rule_text = reader
                        .read_text(e.name())
                        // .map_err(|e| format!("Error reading text at position {}: {:?}", reader.buffer_position(), e))?;
                        .map_err(|e| {
                            quick_xml::DeError::Custom(format!(
                                "Error reading text at position {}: {:?}",
                                reader.buffer_position(),
                                e
                            ))
                        })?;
                    //config.config_rules.push(RuleType::new(rule_text));
                }
                Ok(Event::Eof) => break,
                // )));
                Err(e) => {
                    let cus_err = format!(
                        "PRINT Config ERROR: {e} {:?}",
                        reader.error_position() as usize,
                    );
                    println!("{:?}", cus_err);
                    return Err(Error::SerdeError(quick_xml::DeError::Custom(cus_err)));
                    // return Err(Error::IoError(std::io::Error::new(
                    //     std::io::ErrorKind::InvalidData,
                    //     format!("Config ERROR at position {}: {:?}", reader.error_position(), e),
                    // )));
                    //v2 return  Err(Error::XMLError(e)); // reader.error_pos() is the position of the error
                    //v1 return Err(Error::XMLError(quick_xml::Error::Custom(format!(
                    //     "Config ERROR at position {}: {:?}",
                    //     reader.buffer_position(),
                    //     e
                    // ))));
                }
                _ => (),
            }
        }

        Ok(config)
    }

    async fn watch_config_changes(&self) {
        let mut one_sec = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            one_sec.tick().await;

            match Self::read_config(self.config_path.to_str().unwrap()).await {
                Ok(new_config) => {
                    // Preserve the non-deserialized config_rules from the current config
                    // let current_config = self.rx.borrow();
                    // new_config.config_rules = current_config.config_rules.clone();

                    if new_config != *self.rx.borrow() {
                        //clone ?
                        if let Err(e) = self.tx.send(new_config) {
                            eprintln!("Failed to send XML config update: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Config ERROR: {e}"),
                // Err(e) => eprintln!("{:?}",DeError::Custom(format!("Config ERROR: {:?}", e))),
            }
        }
    }
    async fn read_config_v1(path: &str) -> core::result::Result<Config, Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let config: Config = from_str(&contents)?;
        Ok(config)
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
