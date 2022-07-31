use crate::race::Race;
use crate::utils::zero_terminated_string;
use nom::bytes::complete::take;
use nom::number::complete::le_u32;
use nom::{number::complete::le_u8, IResult};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PlayerMetaData {
    pub id: u8,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
// TODO: that's a protobuf payload to be parsed
pub(crate) struct ReforgedPlayerMetaData {
    pub id: u32,
    pub batte_tag: String,
    pub clan: String,
    pub portrait: String,
    pub team: u32,
    pub unknown: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct PlayerSlotMetaData {
    pub(crate) player_id: u8,
    slot_status: u8,
    computer_flag: u8,
    pub(crate) team_id: u8,
    pub(crate) color: u8,
    pub(crate) race: Race,
    ai_strength: u8,
    handicap_flag: u8,
}

pub(crate) fn parse_player_metadata(input: &[u8]) -> IResult<&[u8], PlayerMetaData> {
    let (rest, id) = le_u8(input)?;
    let (rest, name) = zero_terminated_string(rest)?;
    let (rest, add_data_flag) = le_u8(rest)?;
    let (rest, _) = take(add_data_flag as usize)(rest)?;
    Ok((rest, PlayerMetaData { id, name }))
}

pub(crate) fn parse_reforged_player_metadata(
    _input: &[u8],
) -> IResult<&[u8], ReforgedPlayerMetaData> {
    // TODO: protobuf parsing
    unimplemented!("Protobuf parsing of Reforged players hasn't been implemented yet");
}

pub(crate) fn parse_players(input: &[u8]) -> IResult<&[u8], Vec<PlayerMetaData>> {
    let mut rest = input;
    let mut players = Vec::new();
    let mut fst = rest[0];
    while fst == 0x16 {
        // https://gist.github.com/ForNeVeR/48dfcf05626abb70b35b8646dd0d6e92#file-w3g_format-txt-L627
        let (bytes, _) = take(1usize)(rest)?;
        let (bytes, player) = parse_player_metadata(bytes)?;
        let (bytes, _) = take(4usize)(bytes)?;
        rest = bytes;
        players.push(player);
        fst = bytes[0];
    }
    Ok((rest, players))
}

// const REFORGED_PLAYER_SUBTYPE: u8 = 0x3;
const REFORGED_PLAYER_SUBTYPE: u8 = 0x0; // FIXME: use the one above

pub(crate) fn parse_players_reforged(input: &[u8]) -> IResult<&[u8], Vec<ReforgedPlayerMetaData>> {
    let mut rest = input;
    let mut players = Vec::new();
    while rest[0] == 0x39 {
        // https://gist.github.com/ForNeVeR/48dfcf05626abb70b35b8646dd0d6e92#file-w3g_format-txt-L627
        let (bytes, _) = take(1usize)(rest)?;
        let (bytes, subtype) = le_u8(bytes)?;
        let (bytes, following_bytes) = le_u32(bytes)?;
        let (bytes, payload) = take(following_bytes)(bytes)?;
        if subtype == REFORGED_PLAYER_SUBTYPE {
            // TODO: Protobuf parsing here
            let (_, player) = parse_reforged_player_metadata(payload)?;
            players.push(player);
        }
        rest = bytes;
        // players.push(player);
    }
    Ok((rest, players))
}

pub(crate) fn parse_players_slots(
    nb_players: u8,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<PlayerSlotMetaData>> {
    move |input| {
        std::iter::repeat(parse_player_slot_record)
            .take(nb_players as usize)
            .try_fold((input, Vec::new()), |(data, mut acc), parser| {
                parser(data).map(|(i, o)| {
                    acc.push(o);
                    (i, acc)
                })
            })
    }
}

fn parse_player_slot_record(input: &[u8]) -> IResult<&[u8], PlayerSlotMetaData> {
    let (rest, player_id) = le_u8(input)?;
    let (rest, _) = take(1usize)(rest)?;
    let (rest, slot_status) = le_u8(rest)?;
    let (rest, computer_flag) = le_u8(rest)?;
    let (rest, team_id) = le_u8(rest)?;
    let (rest, color) = le_u8(rest)?;
    let (rest, race_flag) = le_u8(rest)?;
    let (rest, ai_strength) = le_u8(rest)?;
    let (rest, handicap_flag) = le_u8(rest)?;
    Ok((
        rest,
        PlayerSlotMetaData {
            player_id,
            slot_status,
            computer_flag,
            team_id,
            color,
            race: Race::from_u8(race_flag),
            ai_strength,
            handicap_flag,
        },
    ))
}

#[cfg(test)]
mod tests {
    // fn test_replay(file: DirEntry) {
    //     let file = fs::read(file.path()).expect("Can read replay as bytes");
    //     let (rest, _) = parse_header(&file[..]).unwrap();
    //     let (_, blocks) = compressed_data_blocks(rest).unwrap();
    //     let decoded = deflate_game(&blocks).unwrap();
    //     let (rest, metadata) = parse_game_metadata(&decoded).unwrap();
    //     let (_, players) = parse_players(rest).unwrap();
    //     assert_eq!(metadata.nb_players as usize - 1, players.len()); // host has been parsed already
    // }
    //
    // fn test_replays<P: AsRef<Path>>(path: P) {
    //     for file in fs::read_dir(path)
    //         .expect("Replays dir should exist")
    //         .map(|m| m.unwrap())
    //     {
    //         test_replay(file);
    //     }
    // }
    //
    // #[test]
    // fn data_blocks_test() {
    //     test_replays("./replays/");
    //     test_replays("./replays-ignore/");
    // }
}
