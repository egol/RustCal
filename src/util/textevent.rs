#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextEvent{
    pub content: String,
    pub status: i8,
    pub completed: bool,
}

impl TextEvent {
    pub fn new(s: String) -> Self {
        Self {
            content: s,
            status: 0,
            completed: false,
        }
    }

}