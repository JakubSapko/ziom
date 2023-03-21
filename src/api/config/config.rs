use std::env;
use env_perm::{check_or_set};

pub fn set_api_key(api_key: &Option<std::string::String>)  {
    
    match api_key {
        Some(value) => {
            let key = "ZIOM-OPEN-API";
            check_or_set(key, value);
            println!("Key set successfully!");
        },
        None => {
            println!("Please provide your OpenAI API key!")
        },
    }
}