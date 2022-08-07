extern crate colored;
extern crate nom;

pub mod action;
mod blocks;
pub mod building;
pub mod display;
pub mod environment;
pub mod event;
pub mod game;
pub mod item;
pub mod map;
pub mod metadata;
pub mod player;
pub mod race;
pub mod spell;
pub mod unit;
mod utils;

#[cfg(test)]
pub mod tests {
    use crate::blocks::gameblock::GameBlock;
    use crate::game::{Game, GameOutcome, GameType};
    use humantime::format_duration;
    use itertools::Itertools;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime};

    pub(crate) fn crate_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    pub(crate) fn replays_ignore_dir() -> PathBuf {
        let mut path = crate_root();
        path.push("replays-ignore");
        path
    }

    pub(crate) fn replays_dir() -> PathBuf {
        let mut path = crate_root();
        path.push("replays");
        path
    }

    pub(crate) fn replays_w3info_dir() -> PathBuf {
        let mut path = crate_root();
        path.push("w3info");
        path.push("replays-w3info");
        path
    }

    pub(crate) fn replay(name: &str) -> PathBuf {
        let mut dir = replays_dir();
        dir.push(name);
        dir
    }

    pub(crate) fn replay_bytes(name: &str) -> Vec<u8> {
        fs::read(replay(name)).unwrap_or_else(|_| panic!("Could not read {}", name))
    }

    pub(crate) fn w3info_replay(name: &str) -> PathBuf {
        let mut dir = replays_w3info_dir();
        dir.push(name);
        dir
    }

    pub(crate) fn ignored_replay(name: &str) -> PathBuf {
        let mut dir = replays_ignore_dir();
        dir.push(name);
        dir
    }

    fn parse_replays<P: AsRef<Path>>(dir: P) -> Vec<Game> {
        fs::read_dir(dir)
            .expect("Replays dir should exist")
            .filter_map(|f| f.ok())
            .map(|f| f.path())
            .filter(|f| f.exists() && f.is_file() && f.extension().unwrap() == OsStr::new("w3g"))
            .sorted()
            .map(Game::parse)
            .collect()
    }

    #[test]
    fn parse_ts_blocks() {
        let game = Game::parse(ignored_replay("Replay_2020_06_29_0026.w3g"));
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
    fn parse_players_properly() {
        let game = Game::parse(w3info_replay("3210760876_FeaR_Kiosuke_Northern Isles.w3g"));
        let players = game.players;
        assert_eq!(3, players.len());
        assert_eq!(2, players.iter().filter(|p| !p.is_observer()).count());
        assert_eq!(1, players.iter().filter(|p| p.is_observer()).count());
    }

    #[test]
    fn parse_all() {
        let start = SystemTime::now();
        let mut games = parse_replays(replays_dir());
        games.extend(parse_replays(replays_ignore_dir()));
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
    fn parse_w3info() {
        let start = SystemTime::now();
        let games = parse_replays(replays_w3info_dir());
        for game in &games {
            let (_, game_type) = game.game_type();
            assert!(matches!(game_type, GameType::OneOnOne));
            let outcome = game.outcome();
            assert!(matches!(outcome, GameOutcome::Winner(_)));
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
    fn test_outcome_one_on_one() {
        let one_on_one_game = Game::parse(ignored_replay("Replay_2020_06_29_0026.w3g"));
        assert_eq!(GameOutcome::Winner(0), one_on_one_game.outcome())
    }
}
