pub use crate::config::prelude::*;
use crate::Error as ValidationError;
use crate::RuleType;

use std::path::PathBuf;
use std::sync::Arc;

use quick_xml::de::from_str;
use quick_xml::Reader;
use tokio::{fs::File, io::AsyncReadExt, sync::watch};

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
    ///Spawns a task that watches for changes in the configuration file
    /// NOTE: Be careful to validate that ValidationRule handlers actually run in the separate thasks!
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

//     async fn quick_watcherr(path: &str) { ///tODO
//         //let mut reader: Reader<&[u8]> = Reader::from_str(path);
//         let mut one_sec = tokio::time::interval(std::time::Duration::from_secs(1));

// //         let xml = "<tag1>text1</tag1><tag1>text2</tag1>\
// //         <tag1>text3</tag1><tag1><tag2>text4</tag2></tag1>";

// // let mut reader = Reader::from_str(xml);
// // reader.config_mut().trim_text(true);


//         let reader = match Reader::from_file(path) {
//             Ok(mut r) => {r.config_mut().trim_text(true); r},
//             Err(e) => eprint!("Error: {}", e),
//         };

//         loop { 
//             one_sec.tick().await;

//             match reader.read_event(){

//             }
//         }
//         //example https://github.com/tafia/quick-xml/blob/8f91a9c20eb67666eaccbc4e37fbe5e3adde3a44/examples/read_texts.rs#L19
//     }

    async fn watch_config_changes(&self) {
        let mut one_sec = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            one_sec.tick().await;

            match Self::read_config(self.config_path.to_str().unwrap()).await {
                Ok(new_config) => {
                    if new_config != *self.rx.borrow() {
                        //clone ?
                        if let Err(e) = self.tx.send(new_config) {
                            eprintln!("Failed to send XML config update: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Config ERROR: {e}"),
            }
        }
    }
    async fn read_config(path: &str) -> Result<Config, ValidationError> {
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
