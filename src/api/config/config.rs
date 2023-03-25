
use confy::{load, store, ConfyError};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct ZiomConfig {
    api_key: String,
}

pub fn set_api_key(api_key: &Option<std::string::String>) {
    match api_key {
        Some(value) => {
            let ziom_cfg = ZiomConfig {
                api_key: value.to_string(),
            };
            let store = store("ziom", None, ziom_cfg);
            match store {
                Ok(_) => println!("Key properly set!"),
                Err(e) => println!("Error occured: {:?}", e),
            }
        }
        None => {
            let key = read_api_key();
            match key {
                Ok(value) => println!("Your API key: {}", value),
                Err(e) => println!("Error occured: {:?}", e),
            }
        }
    }
}

pub fn read_api_key() -> Result<String, ConfyError> {
    let ziom_cfg = confy::load::<ZiomConfig>("ziom", None)?;
    return Ok(ziom_cfg.api_key);
}
