// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;

// Internal Dependencies ------------------------------------------------------
use super::textevent::*;

#[derive(Serialize, Deserialize)]
pub struct Storage{
    pub events: HashMap<NaiveDate, Vec<TextEvent>>,
}

impl Storage {
    pub fn new(e : HashMap<NaiveDate, Vec<TextEvent>>) -> Self {
        Self {
            events: e,
        }
    }
} 
