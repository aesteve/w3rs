use crate::action::Action;
use crate::blocks::chat::Addressee;
use crate::blocks::command::GameComponent;
use std::time::Duration;

#[derive(PartialEq, Clone, Debug)]
pub enum Event {
    ChatMsg {
        addressee: Addressee,
        message: String,
    },
    Action {
        selection: Vec<GameComponent>,
        action: Action,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub struct GameEvent {
    pub player_id: u8,
    pub time: Duration,
    pub event: Event,
}
