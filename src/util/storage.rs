// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;

// Internal Dependencies ------------------------------------------------------
use super::textevent::*;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub events: HashMap<NaiveDate, Vec<TextEvent>>,
    #[serde(skip)]
    next_event_id: u64,
}

impl Storage {
    pub fn new(mut e: HashMap<NaiveDate, Vec<TextEvent>>) -> Self {
        let next_event_id = Self::normalize_event_ids(&mut e);
        Self {
            events: e,
            next_event_id,
        }
    }

    fn normalize_event_ids(events: &mut HashMap<NaiveDate, Vec<TextEvent>>) -> u64 {
        let mut next_event_id = events
            .values()
            .flat_map(|day_events| day_events.iter().map(|event| event.id))
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        for day_events in events.values_mut() {
            for event in day_events.iter_mut() {
                if event.id == 0 {
                    event.id = next_event_id;
                    next_event_id = next_event_id.saturating_add(1);
                }
            }
        }

        next_event_id.max(1)
    }

    pub fn events_for_date(&self, date: NaiveDate) -> Vec<TextEvent> {
        self.events.get(&date).cloned().unwrap_or_default()
    }

    pub fn add_event(&mut self, date: NaiveDate) -> u64 {
        let id = self.next_event_id;
        self.next_event_id = self.next_event_id.saturating_add(1);
        self.events
            .entry(date)
            .or_default()
            .push(TextEvent::with_id(id, String::new()));
        id
    }

    pub fn set_content(&mut self, date: NaiveDate, id: u64, content: String) -> bool {
        if let Some(event) = self.find_event_mut(date, id) {
            event.content = content;
            return true;
        }

        false
    }

    pub fn set_status(&mut self, date: NaiveDate, id: u64, status: i8) -> bool {
        if let Some(event) = self.find_event_mut(date, id) {
            event.status = status;
            return true;
        }

        false
    }

    pub fn set_completed(&mut self, date: NaiveDate, id: u64, completed: bool) -> bool {
        if let Some(event) = self.find_event_mut(date, id) {
            event.completed = completed;
            return true;
        }

        false
    }

    pub fn remove_event(&mut self, date: NaiveDate, id: u64) -> bool {
        let mut removed = false;
        let mut empty_after_remove = false;

        if let Some(events) = self.events.get_mut(&date) {
            let original_len = events.len();
            events.retain(|event| event.id != id);
            removed = events.len() != original_len;
            empty_after_remove = events.is_empty();
        }

        if empty_after_remove {
            self.events.remove(&date);
        }

        removed
    }

    fn find_event_mut(&mut self, date: NaiveDate, id: u64) -> Option<&mut TextEvent> {
        self.events
            .get_mut(&date)
            .and_then(|events| events.iter_mut().find(|event| event.id == id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_ids_to_legacy_events() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 25).unwrap();
        let storage = Storage::new(HashMap::from([(
            date,
            vec![TextEvent::new("first".into()), TextEvent::new("second".into())],
        )]));

        let events = storage.events_for_date(date);
        assert_eq!(events.len(), 2);
        assert_ne!(events[0].id, 0);
        assert_ne!(events[1].id, 0);
        assert_ne!(events[0].id, events[1].id);
    }

    #[test]
    fn mutates_events_by_stable_id() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 25).unwrap();
        let mut storage = Storage::new(HashMap::new());
        let event_id = storage.add_event(date);

        assert!(storage.set_content(date, event_id, "task".into()));
        assert!(storage.set_status(date, event_id, 2));
        assert!(storage.set_completed(date, event_id, true));

        let event = storage.events_for_date(date).pop().unwrap();
        assert_eq!(event.content, "task");
        assert_eq!(event.status, 2);
        assert!(event.completed);
    }

    #[test]
    fn remove_event_cleans_up_empty_dates() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 25).unwrap();
        let mut storage = Storage::new(HashMap::new());
        let event_id = storage.add_event(date);

        assert!(storage.remove_event(date, event_id));
        assert!(storage.events_for_date(date).is_empty());
        assert!(!storage.events.contains_key(&date));
    }
}
