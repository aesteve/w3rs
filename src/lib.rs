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
use nom::lib::std::fmt::Formatter;
use std::fmt::Display;
use std::fs::DirEntry;
use std::{fmt, fs};

use crate::race::Race;
use colored::Colorize;

mod blocks;
pub mod map;
mod metadata;
pub mod race;
mod utils;

#[derive(Debug)]
pub struct Player {
    team_id: u8,
    player_id: u8,
    player_name: String,
    race: Race,
    color: u8,
    host: bool,
}

#[derive(Debug)]
pub struct Game {
    name: String,
    game_type: Vec<String>,
    game_start_record: GameStartRecord,
    players: Vec<Player>,
    pos: GamePosData,
    map: MapInfo,
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
    pub fn parse(file: DirEntry) -> Game {
        println!("Read replay {:?}", file);
        let file = fs::read(file.path()).expect("Can read replay as bytes");
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
        let (_, _) = parse_game_blocks(rest).expect("Could not parse data blocks");
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
        }
    }

    pub fn players_by_team(&self) -> Vec<(u8, Vec<&Player>)> {
        self.players
            .iter()
            .group_by(|p| p.team_id)
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
        writeln!(f, "Warcraft 3 Reforged game. {:?}", self.game_type())?;
        writeln!(f, "\tMap: {}", self.map.name)?;
        for (team, players) in self.players_by_team() {
            write!(f, "\tTeam {:?}:", team + 1)?;
            write!(f, " [ ")?;
            for player in players {
                write!(f, "{} ", player)?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "")
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

impl fmt::Display for Player {
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

#[cfg(test)]
mod tests {
    use crate::Game;
    use std::fs;
    use std::path::Path;
    use std::time::SystemTime;

    fn parse_replays<P: AsRef<Path>>(path: P) -> Vec<Game> {
        fs::read_dir(path)
            .expect("Replays dir should exist")
            .map(|f| Game::parse(f.unwrap()))
            .collect()
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
