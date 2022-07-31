use crate::race::Race;
use colored::{Color, Colorize};
use std::fmt;
use std::fmt::{Display, Formatter};
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

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let description = format!("{} ({})", self.name, self.race);
        if let Some(color) = player_msg_color(self) {
            write!(f, "{}", description.color(color))
        } else {
            write!(f, "{}", description)
        }
    }
}

pub fn player_msg_color(player: &Player) -> Option<Color> {
    match player.color {
        0 => Some(Color::Red),
        1 => Some(Color::Blue),
        2 => Some(Color::Cyan),
        3 => Some(Color::Magenta),
        4 => Some(Color::Yellow),

        6 => Some(Color::Green),

        _ => None,
    }
}
