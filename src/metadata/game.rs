use crate::metadata::player::{parse_player_metadata, PlayerMetaData};
use crate::utils::{zero_terminated, zero_terminated_string};
use hex_string::u8_to_hex_string;
use nom::bytes::complete::{take, take_while};
use nom::{
    number::complete::{le_u16, le_u32, le_u8},
    IResult,
};
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub(crate) struct GameMetaData {
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

pub(crate) fn parse_game_metadata(input: &[u8]) -> IResult<&[u8], GameMetaData> {
    let (rest, _) = take(5usize)(input)?;
    let (rest, host) = parse_player_metadata(rest)?;
    let (rest, game_name) = zero_terminated_string(rest)?;
    let (rest, _) = zero_terminated_string(rest)?; // private string
    let (rest, encoded_map_info) = zero_terminated(rest)?;
    let (rest, nb_players) = le_u32(rest)?;
    let (rest, game_type) = take(4usize)(rest)?;
    let (rest, language) = take(4usize)(rest)?;
    Ok((
        rest,
        GameMetaData {
            host,
            game_name,
            encoded_map_info: encoded_map_info.to_vec(),
            nb_players,
            game_type: game_type.to_vec(),
            language: language.to_vec(),
        },
    ))
}

#[derive(Debug, PartialEq)]
pub struct GameStartRecord {
    data_byte_count: u16,
    pub(crate) slot_record_count: u8,
}

pub fn parse_start_record(input: &[u8]) -> IResult<&[u8], GameStartRecord> {
    let (rest, _) = take_while(|b: u8| b != 0x19)(input)?;
    let (rest, data_byte_count) = le_u16(&rest[1..])?;
    let (rest, slot_record_count) = le_u8(rest)?;
    Ok((
        rest,
        GameStartRecord {
            data_byte_count,
            slot_record_count,
        },
    ))
}

#[derive(Debug, PartialEq)]
pub struct GamePosData {
    random_seed: u32,
    select_mode: [char; 2], // hex, 1 byte
    start_spot_count: u8,
}

pub fn parse_game_pos(input: &[u8]) -> IResult<&[u8], GamePosData> {
    let (rest, random_seed) = le_u32(input)?;
    let (rest, select_mode) = le_u8(rest)?;
    let (rest, start_spot_count) = le_u8(rest)?;
    Ok((
        rest,
        GamePosData {
            random_seed,
            select_mode: u8_to_hex_string(&select_mode),
            start_spot_count,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::blocks::compressedblock::{compressed_data_blocks, deflate_game};
    use crate::metadata::game::parse_game_metadata;
    use crate::metadata::replay::parse_header;
    use crate::tests::{replays_dir, replays_ignore_dir, replays_w3info_dir};
    use std::ffi::OsStr;
    use std::fs;
    use std::fs::DirEntry;
    use std::path::Path;

    fn metadata_parsed_properly(file: DirEntry) {
        let file = fs::read(file.path()).expect("Can read replay as bytes");
        let (rest, _) = parse_header(&file[..]).unwrap();
        let (_, blocks) = compressed_data_blocks(rest).unwrap();
        let decoded = deflate_game(&blocks).unwrap();
        let res = parse_game_metadata(&decoded);
        assert!(res.is_ok());
    }

    fn replays_metadata_is_parsed<P: AsRef<Path>>(dir: P) {
        for file in fs::read_dir(dir)
            .expect("Replays dir should exist")
            .map(|m| m.unwrap())
            .filter(|f| f.path().is_file() && f.path().extension().unwrap() == OsStr::new("w3g"))
        {
            metadata_parsed_properly(file);
        }
    }

    #[test]
    fn data_blocks_test() {
        replays_metadata_is_parsed(replays_dir());
        replays_metadata_is_parsed(replays_ignore_dir());
        replays_metadata_is_parsed(replays_w3info_dir())
    }
}
