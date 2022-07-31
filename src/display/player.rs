use crate::player::Player;
use colored::{Color, Colorize};
use std::fmt;
use std::fmt::{Display, Formatter};

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
