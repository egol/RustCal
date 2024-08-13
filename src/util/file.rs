// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::fs;

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;

// Internal dependencies ------------------------------------------------------
use crate::util::textevent::*;

// TODO: allow for save file naming and directory selection
// TODO: allow for configuring time based auto save?
/// Writes storage hash map to json
/// responsible for saving user data
pub fn save_data(s: &HashMap<NaiveDate, Vec<TextEvent>>) {
    let j = serde_json::to_string(&s).expect("could not serialize hashmap");
    fs::write("test.json", j).expect("Unable to write file");
}

// Converts json save file back into a hashmap
// that is used to load previous data
pub fn read() -> HashMap<NaiveDate, Vec<TextEvent>>{
    if let Ok(mut f) = File::open("test.json") {
        let mut contents = String::new();
        if f.read_to_string(&mut contents).is_err(){
            return HashMap::new();
        }
        if let Ok(p) = serde_json::from_str::<HashMap<NaiveDate, Vec<TextEvent>>>(&contents){
            return p;
        }
        else{
            return HashMap::new();
        }
    } else {
        HashMap::new()
    }
}