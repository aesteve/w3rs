use crate::utils::zero_terminated;
use nom::{
    number::complete::{le_u16, le_u32},
    IResult,
};

const REPLAY_PREFIX: &str = "Warcraft III recorded game";

#[derive(Debug, PartialEq, Eq)]
pub struct FileMetaData {
    pub offset: u32,
    pub compressed_size: u32,
    pub header_version: String,
    pub decompressed_size: u32,
    pub compressed_data_block_count: u32,
    pub replay_metadata: ReplayMetaData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReplayMetaData {
    pub game_identifier: String,
    pub version: u32,
    pub build_no: u16,
    pub flags: String,
    pub replay_length_ms: u32,
    pub checksum: u32,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], FileMetaData> {
    do_parse!(
        input,
        ignored: tag!(REPLAY_PREFIX)
            >> magic: zero_terminated
            >> offset: le_u32
            >> compressed_size: le_u32
            >> header_version: take!(4u8)
            >> decompressed_size: le_u32
            >> compressed_data_block_count: le_u32
            >> replay_metadata: parse_replay_metadata
            >> (FileMetaData {
                offset,
                compressed_size,
                header_version: String::from_utf8_lossy(header_version).to_string(),
                decompressed_size,
                compressed_data_block_count,
                replay_metadata,
            })
    )
}

fn parse_replay_metadata(input: &[u8]) -> IResult<&[u8], ReplayMetaData> {
    do_parse!(
        input,
        game_identifier: take!(4)
            >> version: le_u32
            >> build_no: le_u16
            >> flags: take!(2)
            >> replay_length_ms: le_u32
            >> checksum: le_u32
            >> (ReplayMetaData {
                game_identifier: String::from_utf8_lossy(game_identifier).to_string(),
                version,
                build_no,
                flags: String::from_utf8_lossy(flags).to_string(),
                replay_length_ms,
                checksum
            })
    )
}
