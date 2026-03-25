#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextEvent {
    #[serde(default)]
    pub id: u64,
    pub content: String,
    pub status: i8,
    pub completed: bool,
}

impl TextEvent {
    pub fn new(s: String) -> Self {
        Self::with_id(0, s)
    }

    pub fn with_id(id: u64, s: String) -> Self {
        Self {
            id,
            content: s,
            status: 0,
            completed: false,
        }
    }
}
