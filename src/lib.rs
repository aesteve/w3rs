#[macro_use]
extern crate nom;
extern crate colored;

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
use std::path::Path;

mod blocks;
pub mod map;
mod metadata;
pub mod race;
mod utils;

#[derive(Debug)]
pub struct Player {
    pub team_id: u8,
    pub player_id: u8,
    pub player_name: String,
    pub race: Race,
    pub color: u8,
    pub host: bool,
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
            let mut team_leave_blocks = leave_blocks.iter().filter_map(|l| {
                players
                    .iter()
                    .find(|p| p.player_id == l.player_id)
                    .map(|_| l)
            });
            if team_leave_blocks.any(|block| block.player_won()) {
                return GameOutcome::Winner(team_id);
            } else if team_leave_blocks.all(|block| block.player_lost()) {
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
                if host.player_id == slot.player_id {
                    Some(Player {
                        team_id: slot.team_id,
                        player_id: host.player_id,
                        player_name: host.player_name.clone(),
                        race: slot.race.clone(),
                        color: slot.color,
                        host: true,
                    })
                } else {
                    players_metadata.iter().find_map(|m| {
                        if m.player_id == slot.player_id && m.player_name != "" {
                            Some(Player {
                                team_id: slot.team_id,
                                player_id: m.player_id,
                                player_name: m.player_name.clone(),
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
            .group_by(|p| p.team_id as u16)
            .into_iter()
            .filter_map(|(team, players)| match team {
                25 => None,
                24 => None,
                12 => None,
                _ => Some((team, players.collect::<Vec<&Player>>())),
            })
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
        let description = format!("{} ({})", self.player_name, self.race);
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
            GameBlock::PlayerChatMsg(msg) => writeln!(f, "{}: {}", msg.player_id, msg.text)?,
            GameBlock::Leave(left) => writeln!(
                f,
                "{} left {:?}|{:?}",
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
    use crate::blocks::gameblock::GameBlock;
    use crate::Game;
    use humantime::format_duration;
    use itertools::Itertools;
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
}
