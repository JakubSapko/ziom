use std::env;

pub fn set_api_key(api_key: &Option<std::string::String>)  {
    
    match api_key {
        Some(value) => {
            let key = "ZIOM-OPEN-API";
            env::set_var(key, value);
            println!("Key set successfully!");
        },
        None => {
            println!("Please provide your OpenAI API key!")
        },
    }
}