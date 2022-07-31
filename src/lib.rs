extern crate colored;
extern crate nom;

pub mod action;
mod blocks;
pub mod building;
pub mod environment;
pub mod event;
pub mod game;
pub mod item;
pub mod map;
mod metadata;
pub mod player;
pub mod race;
pub mod spell;
pub mod unit;
mod utils;

#[cfg(test)]
mod tests {
    use crate::blocks::gameblock::GameBlock;
    use crate::game::{Game, GameOutcome, GameType};
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
        let game = Game::parse(Path::new(
            "./replays-w3info/3210760876_FeaR_Kiosuke_Northern Isles.w3g",
        ));
        let players = game.players;
        assert_eq!(3, players.len());
        assert_eq!(2, players.iter().filter(|p| !p.is_observer()).count());
        assert_eq!(1, players.iter().filter(|p| p.is_observer()).count());
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

    #[test]
    fn parse_w3info() {
        let start = SystemTime::now();
        let games = parse_replays("./replays-w3info/");
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
        let one_on_one_replay = Path::new("./replays-ignore/Replay_2020_06_29_0026.w3g");
        let one_on_one_game = Game::parse(one_on_one_replay);
        assert_eq!(GameOutcome::Winner(0), one_on_one_game.outcome())
    }
}
