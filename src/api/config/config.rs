use confy::store;
use serde_derive::{Deserialize, Serialize};
use std;

#[derive(Serialize, Deserialize)]
struct ZiomConfig {
    api_key: String,
}

pub fn set_api_key(api_key: &Option<std::string::String>) {
    match api_key {
        Some(value) => {
            let key = "ZIOM-OPEN-API";
            let ziom_cfg = ZiomConfig {
                api_key: key.to_string(),
            };
            let store = store("ziom", None, ziom_cfg);
            match store {
                Ok(_) => println!("Key properly set!"),
                Err(e) => println!("Error occured: {e:?}"),
            }
        }
        None => {
            println!("Please provide your OpenAI API key!")
        }
    }
}
