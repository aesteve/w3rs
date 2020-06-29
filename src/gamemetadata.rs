use crate::players::{parse_player_metadata, PlayerMetaData};
use hex_string::u8_to_hex_string;
use nom::{
    number::complete::{le_u16, le_u32, le_u8},
    IResult,
};
use std::iter::FromIterator;
named!(zero_terminated<&[u8], &[u8]>,
    terminated!(take_while!(|b: u8| b != 0), tag!([0])));

#[derive(Debug, PartialEq)]
pub struct GameMetaData {
    pub host: PlayerMetaData,
    pub game_name: String,
    pub(crate) encoded_map_info: Vec<u8>,
    pub nb_players: u32,
    game_type: Vec<u8>,
    language: Vec<u8>,
}

impl GameMetaData {
    pub fn game_type(&self) -> Vec<String> {
        self.game_type
            .iter()
            .map(|hex| String::from_iter(u8_to_hex_string(hex).to_vec()))
            .collect()
    }
}

pub fn parse_game_metadata(input: &[u8]) -> IResult<&[u8], GameMetaData> {
    do_parse!(
        input,
        ignored: take!(5)
            >> host: parse_player_metadata
            >> game_name: zero_terminated
            >> discard: zero_terminated
            >> encoded_map_info: zero_terminated
            >> nb_players: le_u32
            >> game_type: take!(4)
            >> language: take!(4)
            >> (GameMetaData {
                host,
                game_name: String::from_utf8_lossy(game_name).to_string(),
                encoded_map_info: encoded_map_info.to_vec(),
                nb_players,
                game_type: game_type.to_vec(),
                language: language.to_vec(),
            })
    )
}

#[derive(Debug, PartialEq)]
pub struct GameStartRecord {
    check_game_start_record: u8,
    game_start_record: u8,
    data_byte_count: u16,
    pub(crate) slot_record_count: u8,
}

pub fn parse_start_record(input: &[u8]) -> IResult<&[u8], GameStartRecord> {
    do_parse!(
        input,
        check_game_start_record: le_u8
            >> ignored: take_while!(|b: u8| b != 25)
            >> game_start_record: le_u8
            >> data_byte_count: le_u16
            >> slot_record_count: le_u8
            >> (GameStartRecord {
                check_game_start_record,
                game_start_record,
                data_byte_count,
                slot_record_count
            })
    )
}

#[derive(Debug, PartialEq)]
pub struct GamePosData {
    random_seed: u32,
    select_mode: [char; 2], // hex, 1 byte
    start_spot_count: u8,
}

pub fn parse_game_pos(input: &[u8]) -> IResult<&[u8], GamePosData> {
    do_parse!(
        input,
        random_seed: le_u32
            >> select_mode: le_u8
            >> start_spot_count: le_u8
            >> (GamePosData {
                random_seed,
                select_mode: u8_to_hex_string(&select_mode),
                start_spot_count,
            })
    )
}

#[cfg(test)]
mod tests {
    use crate::compressedblocks::{compressed_data_blocks, deflate_game};
    use crate::gamemetadata::parse_game_metadata;
    use crate::headers::parse_header;
    use std::fs;
    use std::fs::DirEntry;
    use std::path::Path;

    fn test_replay(file: DirEntry) {
        let file = fs::read(file.path()).expect("Can read replay as bytes");
        let (rest, _) = parse_header(&file[..]).unwrap();
        let (_, blocks) = compressed_data_blocks(rest).unwrap();
        let decoded = deflate_game(&blocks).unwrap();
        parse_game_metadata(&decoded).unwrap();
    }

    fn test_replays<P: AsRef<Path>>(path: P) {
        for file in fs::read_dir(path)
            .expect("Replays dir should exist")
            .map(|m| m.unwrap())
        {
            test_replay(file);
        }
    }

    #[test]
    fn data_blocks_test() {
        test_replays("./replays/");
        test_replays("./replays-ignore/");
    }
}
