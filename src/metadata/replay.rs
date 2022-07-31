use crate::utils::zero_terminated;
use nom::bytes::complete::{tag, take};
use nom::{
    number::complete::{le_u16, le_u32},
    IResult,
};

const REPLAY_PREFIX: &str = "Warcraft III recorded game";

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FileMetaData {
    pub offset: u32,
    pub compressed_size: u32,
    pub header_version: String,
    pub decompressed_size: u32,
    pub compressed_data_block_count: u32,
    pub replay_metadata: ReplayMetaData,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReplayMetaData {
    pub game_identifier: String,
    pub version: u32,
    pub build_no: u16,
    pub flags: String,
    pub replay_length_ms: u32,
    pub checksum: u32,
}

pub(crate) fn parse_header(input: &[u8]) -> IResult<&[u8], FileMetaData> {
    let (rest, _) = tag(REPLAY_PREFIX)(input)?;
    let (rest, _) = zero_terminated(rest)?; // "magic" string
    let (rest, offset) = le_u32(rest)?;
    let (rest, compressed_size) = le_u32(rest)?;
    let (rest, header_version) = take(4usize)(rest)?;
    let (rest, decompressed_size) = le_u32(rest)?;
    let (rest, compressed_data_block_count) = le_u32(rest)?;
    let (rest, replay_metadata) = parse_replay_metadata(rest)?;
    Ok((
        rest,
        FileMetaData {
            offset,
            compressed_size,
            header_version: String::from_utf8_lossy(header_version).to_string(),
            decompressed_size,
            compressed_data_block_count,
            replay_metadata,
        },
    ))
}

fn parse_replay_metadata(input: &[u8]) -> IResult<&[u8], ReplayMetaData> {
    let (rest, game_identifier) = take(4usize)(input)?;
    let (rest, version) = le_u32(rest)?;
    let (rest, build_no) = le_u16(rest)?;
    let (rest, flags) = take(2usize)(rest)?;
    let (rest, replay_length_ms) = le_u32(rest)?;
    let (rest, checksum) = le_u32(rest)?;
    // let (rest, _checksum_sha1) = le_u32(rest)?;
    // let metadata = ReplayMetaData {
    //     game_identifier: String::from_utf8_lossy(game_identifier).to_string(),
    //     version,
    //     build_no,
    //     flags: String::from_utf8_lossy(flags).to_string(),
    //     replay_length_ms,
    //     checksum,
    // };
    // println!("metadata: {metadata:?}");
    Ok((
        rest,
        ReplayMetaData {
            game_identifier: String::from_utf8_lossy(game_identifier).to_string(),
            version,
            build_no,
            flags: String::from_utf8_lossy(flags).to_string(),
            replay_length_ms,
            checksum,
        },
    ))
}
