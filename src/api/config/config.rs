use confy;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct ZiomConfig {
    api_key: String,
}

pub fn handle_config(api_key: &Option<std::string::String>) {
    match api_key {
        Some(value) => {
            write_api_key(value);
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
fn write_api_key(key: &String) {
    let ziom_cfg = ZiomConfig {
        api_key: key.to_string(),
    };
    let store = confy::store("ziom", None, ziom_cfg);
    match store {
        Ok(_) => println!("Key properly set!"),
        Err(e) => println!("Error occured: {:?}", e),
    }
}

pub fn read_api_key() -> Result<String, confy::ConfyError> {
    let ziom_cfg = confy::load::<ZiomConfig>("ziom", None)?;
    return Ok(ziom_cfg.api_key);
}
