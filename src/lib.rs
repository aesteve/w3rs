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
    use crate::game::{Game, GameOutcome};
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

    #[test]
    fn test_outcome_one_on_one() {
        let one_on_one_replay = Path::new("./replays-ignore/Replay_2020_06_29_0026.w3g");
        let one_on_one_game = Game::parse(one_on_one_replay);
        assert_eq!(GameOutcome::Winner(0), one_on_one_game.outcome())
    }
}
