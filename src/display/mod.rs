use crate::display::player::player_msg_color;
use crate::event::{Event, GameEvent};
use crate::game::{non_noisy, Game};
use colored::{Color, Colorize};
use humantime::format_duration;
use std::thread::sleep;
use std::time::Duration;
pub mod action;
pub mod chat;
pub mod command;
pub mod game;
pub mod player;
pub mod race;

pub fn live_display(game: &Game) {
    let events = game.events();
    let mut last_time = Duration::from_millis(0);
    for event in events.iter().filter(non_noisy).collect::<Vec<&GameEvent>>() {
        let time = event.time;
        sleep(time - last_time);
        last_time = time;
        let color = game
            .player(event.player_id)
            .and_then(player_msg_color)
            .unwrap_or(Color::White);
        print!(
            "{}",
            format!(
                "[{}] Player {}: ",
                format_duration(event.time),
                event.player_id
            )
            .color(color)
        );
        match &event.event {
            Event::ChatMsg { addressee, message } => println!("{} {}", addressee, message),
            Event::Action { selection, action } => {
                println!("{}", format!("{:?} {}", selection, action).color(color));
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::display::live_display;
    use crate::game::Game;
    use crate::tests::w3info_replay;

    #[test]
    #[ignore]
    fn live_display_w3info() {
        let game = Game::parse(w3info_replay("3210760876_FeaR_Kiosuke_Northern Isles.w3g"));
        live_display(&game);
    }
}
