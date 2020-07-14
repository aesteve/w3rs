extern crate colored;
extern crate nom;

use crate::blocks::{
    compressedblock::{compressed_data_blocks, deflate_game},
    gameblock::parse_game_blocks,
};
use crate::map::{parse_map_info, MapInfo};
use crate::metadata::{
    game::{parse_game_metadata, parse_game_pos, parse_start_record, GamePosData, GameStartRecord},
    player::{parse_players, parse_players_slots},
    replay::parse_header,
};
use itertools::Itertools;
use nom::lib::std::fmt::{Debug, Formatter};
use std::fmt::Display;
use std::{fmt, fs};

use crate::blocks::gameblock::{GameBlock, LeaveGameBlock};
use crate::race::Race;
use colored::Colorize;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub mod action;
mod blocks;
pub mod building;
pub mod item;
pub mod map;
mod metadata;
pub mod race;
pub mod spell;
pub mod unit;
mod utils;

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

#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub game_type: Vec<String>,
    pub game_start_record: GameStartRecord,
    pub players: Vec<Player>,
    pub pos: GamePosData,
    pub map: MapInfo,
    pub blocks: Vec<GameBlock>,
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
        if teams_who_lost.len() == all_teams.len() - 1 {
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
            parse_players(metadata.nb_players - 1)(rest).expect("could not parse players metadata");
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
                        if m.id == slot.player_id && m.name != "" {
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
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let outcome = self.outcome();
        writeln!(f, "Warcraft 3 Reforged game. {:?}", self.game_type())?;
        writeln!(f, "\tMap: {}", self.map.name)?;
        for (team, players) in self.players_by_team() {
            let team_won = outcome == GameOutcome::Winner(team);
            write!(f, "\tTeam {:?}:", team + 1)?;
            write!(f, " [ ")?;
            for player in players {
                write!(f, "{} ", player)?;
            }
            write!(f, "]")?;
            if team_won {
                write!(f, " ✅")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let short = match self {
            Race::Human => "HU",
            Race::Orc => "OR",
            Race::NightElf => "NE",
            Race::Undead => "UD",
            Race::Random => "??",
            Race::Unknown => "N/A",
        };
        write!(f, "{}", short)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let description = format!("{} ({})", self.name, self.race);
        match self.color {
            0 => write!(f, "{}", description.red()),
            1 => write!(f, "{}", description.blue()),
            2 => write!(f, "{}", description.cyan()),
            3 => write!(f, "{}", description.purple()),
            4 => write!(f, "{}", description.yellow()),

            6 => write!(f, "{}", description.green()),

            _ => write!(f, "{}", description),
        }
    }
}

impl Display for GameBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameBlock::PlayerChatMsg(msg) => writeln!(f, "Player {}: {}", msg.player_id, msg.text)?,
            GameBlock::Leave(left) => writeln!(
                f,
                "Player {} left {:?}|{:?}",
                left.player_id, left.reason, left.result
            )?,
            GameBlock::TimeSlot(ts_block) => {
                if let Some(cmd) = &ts_block.command {
                    writeln!(f, "Player {}:", cmd.player)?;
                    for action in &cmd.actions {
                        writeln!(f, "\t{:?}", action)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::command::{GameComponent, SelectedComponent};
    use crate::blocks::gameblock::GameBlock;
    use crate::{Game, GameOutcome};
    use humantime::format_duration;
    use itertools::Itertools;
    use nom::lib::std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::time::{Duration, SystemTime};

    fn parse_replays<P: AsRef<Path>>(path: P) -> Vec<Game> {
        fs::read_dir(path)
            .expect("Replays dir should exist")
            .map(|f| f.unwrap().path())
            .sorted()
            .map(Game::parse)
            .collect()
    }

    #[test]
    #[ignore]
    fn parse_ts_blocks() {
        let game = Game::parse(Path::new("./replays-ignore/Replay_2020_06_29_0026.w3g"));
        println!("Blocks:");
        let mut time = Duration::from_millis(0);
        for block in &game.blocks {
            if let GameBlock::TimeSlot(ts_block) = block {
                time += Duration::from_millis(ts_block.time_increment as u64);
            }
            if block.should_display() {
                println!("[{}] {}", format_duration(time), block);
            }
        }
    }

    #[test]
    fn fold_units() {
        let game = Game::parse(Path::new("./replays-ignore/Replay_2020_06_29_0026.w3g"));
        let mut time = Duration::from_millis(0);
        let mut game_components: HashMap<u32, GameComponent> = HashMap::new();
        let mut player_selection: HashMap<u8, Vec<SelectedComponent>> = HashMap::new();
        for block in &game.blocks {
            if let GameBlock::TimeSlot(ts_block) = block {
                time += Duration::from_millis(ts_block.time_increment as u64);
                if let Some(cmd) = &ts_block.command {
                    let player = cmd.player;
                    let actions = &cmd.actions;
                    for action in actions {
                        if let Some(selection) = action.selection() {
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
                        if action.should_display() {
                            println!("[{}] Player {}:", format_duration(time), player);
                            println!("\t Selection: {:?}", player_selection.get(&player));
                            println!("\t {:?}", action);
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn parse_all() {
        let start = SystemTime::now();
        let mut games = parse_replays("./replays/");
        games.extend(parse_replays("./replays-ignore"));
        for game in &games {
            println!("{}", game);
        }
        let nb = games.len();
        let elapsed = start.elapsed().unwrap().as_millis();
        println!(
            "{} games in: {:?}ms (avg: {}ms)",
            nb,
            elapsed,
            elapsed / (nb as u128)
        );
    }

    #[test]
    #[ignore]
    fn test_outcome_one_on_one() {
        let one_on_one_replay = Path::new("./replays-ignore/Replay_2020_06_29_0026.w3g");
        let one_on_one_game = Game::parse(one_on_one_replay);
        assert_eq!(GameOutcome::Winner(0), one_on_one_game.outcome())
    }
}
