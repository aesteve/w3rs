use crate::race::Race;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
pub struct Player {
    pub team_id: u8,
    pub id: u8,
    pub name: String,
    pub race: Race,
    pub color: u8,
    pub host: bool,
}

impl Player {
    pub fn is_observer(&self) -> bool {
        self.team_id == 24
            || self.team_id == 25
            || self.team_id == 12
            || self.race == Race::Unknown && self.color > 6
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
