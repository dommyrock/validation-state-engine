use std::path::PathBuf;
use std::sync::Arc;
use crate::RuleType;

use tokio::{fs::File, io::AsyncReadExt, sync::watch};
use quick_xml::de::from_str;
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Configuration {
    pub rules: Vec<RuleType>,
}

pub struct ConfigurationService {
    config_path: PathBuf,
    _tx: watch::Sender<Configuration>,
    rx: watch::Receiver<Configuration>,
}

impl ConfigurationService {
    pub async fn new(config_path: String) -> Arc<Self> {
        let initial_config = Self::read_config(&config_path).await
            .expect("Failed to read initial configuration");
        
        let (_tx, rx) = watch::channel(initial_config);
        
        let service = Arc::new(Self {
            config_path: PathBuf::from(config_path),
            _tx,
            rx,
        });
        
        // Spawn the file watcher
        let service_clone = Arc::clone(&service);

        tokio::spawn(async move {
            service_clone.watch_config_changes().await;
        });
        
        service
    }
    
    pub fn subscribe(&self) -> watch::Receiver<Configuration> {
        self.rx.clone()
    }
    
    async fn read_config(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        let config: Configuration = from_str(&contents)?;
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
                    //TODO : See how I will read to speciffic Rule struct instead of 'RulesType enum' (which is only a marker for what rules are configured for prsing XML file)

                    // if let Ok(current_config) = self.rx.borrow().clone() {//Expected struc 'Configuration' got enum Result
                    //     if new_config != current_config {
                    //         let _ = self.tx.send(new_config);
                    //     }
                    // }
                }
                Err(e) => eprintln!("Error reading configuration: {}", e),
            }
        }
    }
}

//TODO : Since we're cnverting XML content to Potential BusinessRuleError . 
// -> We would need to impement the From trait .
// -> Need to check error type that we're working here so that we can convert that error into ours or inverse. 

// example-ref : https://learning-rust.github.io/docs/custom-error-types/ (last section)