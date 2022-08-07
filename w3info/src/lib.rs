#![recursion_limit = "256"]
pub mod w3info_parsed_replay;
pub mod w3info_replay;

#[cfg(test)]
mod tests {
    use crate::w3info_parsed_replay::W3ParsedReplay;
    use crate::w3info_replay::W3InfoReplay;
    use itertools::Itertools;
    use std::fs;
    use std::path::PathBuf;
    use w3rs::game::Game;

    fn w3info_base_dir() -> PathBuf {
        let mut base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base_dir.push("replays-w3info");
        base_dir
    }

    fn w3info_replays_dir() -> PathBuf {
        let mut replays_dir = w3info_base_dir();
        replays_dir.push("analysed");
        replays_dir
    }

    fn w3info_replays_metadata() -> Vec<(W3InfoReplay, W3ParsedReplay)> {
        let mut analysed_list_file = w3info_base_dir();
        analysed_list_file.push("analysed.jsonl");
        let jsonl = fs::read_to_string(analysed_list_file)
            .expect("Could not read analysed.jsonl file containing the w3info replays metadata");
        let mut res = Vec::new();
        for mut pair in &jsonl.lines().chunks(2usize) {
            let replay_metadata = pair.next().unwrap();
            let game_analysed = pair.next().unwrap();
            res.push((
                serde_json::from_str(replay_metadata).unwrap(),
                serde_json::from_str(game_analysed).unwrap(),
            ));
        }
        res
    }

    fn w3info_replay(name: &str) -> PathBuf {
        let mut base = w3info_base_dir();
        base.push(name);
        base
    }

    #[test]
    fn check_w3info_parsing() {
        for (metadata, game_info) in w3info_replays_metadata() {
            let id = metadata.id;
            let replay_file = w3info_replay(id.to_string().as_str());
            let parsed_w3rs = Game::parse(replay_file);
            // TODO: check same parsing
        }
    }
}
