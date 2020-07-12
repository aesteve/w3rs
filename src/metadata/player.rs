use crate::race::Race;
use crate::utils::zero_terminated_string;
use nom::bytes::complete::take;
use nom::{number::complete::le_u8, IResult};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PlayerMetaData {
    pub id: u8,
    pub name: String,
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

fn parse_player_metadata_in_list(input: &[u8]) -> IResult<&[u8], PlayerMetaData> {
    let (rest, _) = take(1usize)(input)?;
    let (rest, id) = le_u8(rest)?;
    let (rest, name) = zero_terminated_string(rest)?;
    let (rest, add_data_flag) = le_u8(rest)?;
    let (rest, _) = take(add_data_flag as usize)(rest)?;
    let (rest, _) = take(4usize)(rest)?;
    Ok((rest, PlayerMetaData { id, name }))
}

pub(crate) fn parse_players(
    nb_players: u32,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<PlayerMetaData>> {
    move |input| {
        std::iter::repeat(parse_player_metadata_in_list)
            .take(nb_players as usize)
            .try_fold((input, Vec::new()), |(data, mut acc), parser| {
                parser(data).map(|(i, o)| {
                    acc.push(o);
                    (i, acc)
                })
            })
    }
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
    use crate::blocks::compressedblock::{compressed_data_blocks, deflate_game};
    use crate::metadata::game::parse_game_metadata;
    use crate::metadata::player::parse_players;
    use crate::metadata::replay::parse_header;
    use std::fs;
    use std::fs::DirEntry;
    use std::path::Path;

    fn test_replay(file: DirEntry) {
        let file = fs::read(file.path()).expect("Can read replay as bytes");
        let (rest, _) = parse_header(&file[..]).unwrap();
        let (_, blocks) = compressed_data_blocks(rest).unwrap();
        let decoded = deflate_game(&blocks).unwrap();
        let (rest, metadata) = parse_game_metadata(&decoded).unwrap();
        let (_, players) = parse_players(metadata.nb_players - 1)(rest).unwrap();
        assert_eq!(metadata.nb_players as usize - 1, players.len()); // host has been parsed already
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
