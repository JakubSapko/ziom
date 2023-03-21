use std::env;

pub fn read_api_key() {
    let key = env::var("ZIOM-OPEN-API");
    match key {
        Ok(value) => {println!{"{}", value}},
        Err(_) => {println!{"ups"}},
    }
}