use crate::action::{from_parsed_action, Action};
use crate::blocks::chat::ChatMsgBlock;
use crate::blocks::command::{GameComponent, SelectedComponent};
use crate::blocks::compressedblock::{compressed_data_blocks, deflate_game};
use crate::blocks::gameblock::{parse_game_blocks, GameBlock, LeaveGameBlock};
use crate::event::{Event, GameEvent};
use crate::map::{parse_map_info, MapInfo};
use crate::metadata::game::{
    parse_game_metadata, parse_game_pos, parse_start_record, GamePosData, GameStartRecord,
};
use crate::metadata::player::{parse_players, parse_players_reforged, parse_players_slots};
use crate::metadata::replay::parse_header;
use crate::player::Player;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub game_type: Vec<String>,
    pub game_start_record: GameStartRecord,
    pub players: Vec<Player>,
    pub pos: GamePosData,
    pub map: MapInfo,
    pub(crate) blocks: Vec<GameBlock>,
}

#[derive(Debug, PartialEq)]
pub enum GameOutcome {
    Draw,
    Winner(u16), // team id
    Unknown,
}

impl Game {
    pub(crate) fn leave_blocks(&self) -> Vec<&LeaveGameBlock> {
        self.blocks
            .iter()
            .filter_map(|b| match b {
                GameBlock::Leave(l) => Some(l),
                _ => None,
            })
            .collect()
    }

    pub fn outcome(&self) -> GameOutcome {
        let leave_blocks = self.leave_blocks();
        if leave_blocks.iter().any(|l| l.is_draw()) {
            return GameOutcome::Draw;
        }
        let players_by_team = self.players_by_team();
        let mut teams_who_lost: Vec<u16> = Vec::new();
        let all_teams: Vec<u16> = players_by_team.iter().map(|t| t.0).collect();
        for (team_id, players) in players_by_team {
            let team_leave_blocks: Vec<&&LeaveGameBlock> = leave_blocks
                .iter()
                .filter_map(|l| {
                    players
                        .iter()
                        .find_map(|p| if p.id == l.player_id { Some(l) } else { None })
                })
                .collect();
            if team_leave_blocks.iter().any(|block| block.player_won()) {
                return GameOutcome::Winner(team_id);
            } else if team_leave_blocks.iter().all(|block| block.player_lost()) {
                teams_who_lost.push(team_id);
            }
        }
        if !all_teams.is_empty() && teams_who_lost.len() == all_teams.len() - 1 {
            return GameOutcome::Winner(
                *(all_teams
                    .iter()
                    .find(|t| !teams_who_lost.contains(&&t))
                    .unwrap()),
            );
        }
        GameOutcome::Unknown
    }
}

#[derive(Debug)]
pub enum GameContext {
    Ladder,
    Custom,
    SinglePlayer,
    Unknown(String),
}

#[derive(Debug)]
pub enum GameType {
    OneOnOne,
    TwoOnTwo,
    ThreeOnThree,
    FourOnFour,
    FFA,
    Unknown,
}

impl Game {
    pub fn parse<P: AsRef<Path> + Debug>(file: P) -> Game {
        println!("Read replay {:?}", file);
        let file = fs::read(file).expect("Can read replay as bytes");
        let (rest, _) = parse_header(&file[..]).expect("Could not parse replay header");
        let (_, blocks) = compressed_data_blocks(rest).expect("Could not parse data blocks");
        let decoded = deflate_game(&blocks)
            .expect("Could not deflate blocks of replay, is zlib available on system?");
        let (rest, metadata) =
            parse_game_metadata(&decoded).expect("Could not parse game metadata");
        let (rest, players_metadata) =
            parse_players(rest).expect("could not parse players metadata");
        // println!("rest is: {rest:?}");
        let mut rest = rest;
        if rest[0] != 0x19 {
            // TODO: handle Reforged-style players (Protobuf w/ icon, etc.)
            let (b, _players_reforged_metadata) =
                parse_players_reforged(rest).expect("Could not parse players metadata (reforged)");
            rest = b;
        }

        let (rest, game_start_record) =
            parse_start_record(rest).expect("Could not parse start record");
        let (rest, players_slots) = parse_players_slots(game_start_record.slot_record_count)(rest)
            .expect("Could not parse player slot");
        let (rest, game_pos_data) = parse_game_pos(rest).expect("Could not parse game start pos");
        let (_, map) = parse_map_info(&crate::utils::decode(&metadata.encoded_map_info)[..])
            .expect("Could not decode map data");
        let host = metadata.host.clone();
        let (_, blocks) = parse_game_blocks(rest).expect("Could not parse data blocks");
        let players: Vec<Player> = players_slots
            .iter()
            .flat_map(|slot| {
                if host.id == slot.player_id {
                    Some(Player {
                        team_id: slot.team_id,
                        id: host.id,
                        name: host.name.clone(),
                        race: slot.race.clone(),
                        color: slot.color,
                        host: true,
                    })
                } else {
                    players_metadata.iter().find_map(|m| {
                        if m.id == slot.player_id && !m.name.is_empty() {
                            Some(Player {
                                team_id: slot.team_id,
                                id: m.id,
                                name: m.name.clone(),
                                race: slot.race.clone(),
                                color: slot.color,
                                host: false,
                            })
                        } else {
                            None
                        }
                    })
                }
            })
            .collect();
        Game {
            name: metadata.game_name.clone(),
            game_type: metadata.game_type(),
            players,
            game_start_record,
            pos: game_pos_data,
            map,
            blocks,
        }
    }

    pub fn players_by_team(&self) -> Vec<(u16, Vec<&Player>)> {
        self.players
            .iter()
            .unique()
            .group_by(|p| p.team_id as u16)
            .into_iter()
            .filter_map(|(team, p)| {
                let players = p.collect::<Vec<&Player>>();
                if players.is_empty()
                    || (players.len() == 1 && players[0].name == "d")
                    || players.iter().any(|p| p.is_observer())
                {
                    None
                } else {
                    Some((team, players))
                }
            })
            .filter(|group| group.1.len() > 1 || group.1[0].name != "d")
            .collect()
    }

    pub fn player(&self, player_id: u8) -> Option<&Player> {
        self.players.iter().find(|p| p.id == player_id)
    }

    pub fn game_type(&self) -> (GameContext, GameType) {
        let context = match self.game_type[0].as_str() {
            "09" => GameContext::Custom,
            "01" => GameContext::Ladder,
            "1d" => GameContext::SinglePlayer,
            str => GameContext::Unknown(str.to_string()),
        };
        let by_team = self.players_by_team();
        let typ = if by_team.len() > 2 && by_team.iter().all(|(_, p)| p.len() == 1) {
            GameType::FFA
        } else if by_team.len() == 2 {
            match by_team[0].1.len() {
                1 => GameType::OneOnOne,
                2 => GameType::TwoOnTwo,
                3 => GameType::ThreeOnThree,
                4 => GameType::FourOnFour,
                _ => GameType::Unknown,
            }
        } else {
            GameType::Unknown
        };
        (context, typ)
    }

    pub fn events(&self) -> Vec<GameEvent> {
        let mut time = Duration::from_millis(0);
        let mut game_components: HashMap<u32, GameComponent> = HashMap::new();
        let mut player_selection: HashMap<u8, Vec<SelectedComponent>> = HashMap::new();
        let mut events: Vec<GameEvent> = Vec::new();
        let mut player_hotkey_groups: HashMap<u8, Vec<SelectedComponent>> = HashMap::new();
        for block in &self.blocks {
            match block {
                GameBlock::TimeSlot(ts_block) => {
                    time += Duration::from_millis(ts_block.time_increment as u64);
                    if let Some(cmd) = &ts_block.command {
                        let player = cmd.player;
                        let actions = &cmd.actions;
                        for action in actions {
                            // Update selection
                            if let Some(selection) = action.selection(&mut player_hotkey_groups) {
                                for selected in selection.clone() {
                                    if let Some(component) = selected.kind {
                                        game_components.insert(selected.id_1, component.clone());
                                        game_components.insert(selected.id_2, component.clone());
                                    }
                                }
                                if selection.is_empty() {
                                    player_selection.remove(&player);
                                } else {
                                    let enhanced_selection: Vec<SelectedComponent> = selection
                                        .iter()
                                        .map(|comp| SelectedComponent {
                                            id_1: comp.id_1,
                                            id_2: comp.id_2,
                                            kind: game_components
                                                .get(&comp.id_1)
                                                .or_else(|| game_components.get(&comp.id_2))
                                                .map(GameComponent::clone),
                                        })
                                        .collect();
                                    player_selection.insert(player, enhanced_selection);
                                }
                            }
                            if let Some(selected_units) = player_selection.get(&player) {
                                let parsed =
                                    from_parsed_action(selected_units, action, &game_components);
                                if let Some(action) = parsed {
                                    events.push(GameEvent {
                                        time,
                                        player_id: player,
                                        event: Event::Action {
                                            selection: selected_units
                                                .iter()
                                                .flat_map(|s| s.kind.as_ref())
                                                .map(GameComponent::clone)
                                                .collect(),
                                            action,
                                        },
                                    })
                                }
                            }
                        }
                    }
                }
                GameBlock::PlayerChatMsg(msg) => {
                    if let ChatMsgBlock::Msg(addressee) = &msg.kind {
                        events.push(GameEvent {
                            player_id: msg.player_id,
                            time,
                            event: Event::ChatMsg {
                                addressee: addressee.clone(),
                                message: msg.text.clone(),
                            },
                        })
                    }
                }
                _ => {}
            }
        }
        events
    }
}

pub(crate) fn non_noisy(event: &&GameEvent) -> bool {
    match &event.event {
        // avoid noisy actions
        Event::Action {
            action,
            selection: _,
        } => !matches!(
            action,
            Action::Move(_)
                | Action::SetRallyPoint(_)
                | Action::RightClick { at: _, target: _ }
                | Action::Attack { at: _, target: _ }
        ),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::display::player::player_msg_color;
    use crate::event::{Event, GameEvent};
    use crate::game::{non_noisy, Game};
    use colored::{Color, Colorize};
    use humantime::format_duration;
    use std::path::Path;

    #[test]
    fn parse_replay_events() {
        let game = Game::parse(Path::new("./replays-ignore/vs_HAPPY_1_TS.w3g"));
        println!("Analyzed game:");
        println!("{}", game);
        let events = game.events();
        for event in events.iter().filter(non_noisy).collect::<Vec<&GameEvent>>() {
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
}
