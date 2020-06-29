use crate::blocks::chat::{player_chat_msg, PlayerChatMsgBlock};
use hex_string::u8_to_hex_string;
use nom::{
    number::complete::{le_u16, le_u8},
    IResult,
};
use std::iter::FromIterator;

#[derive(Debug)]
pub(crate) enum GameBlock {
    Leave(LeaveGameBlock),
    TimeSlot(TimeSlotBlock),
    PlayerChatMsg(PlayerChatMsgBlock),
    Ignored,
    Unknown,
}

#[derive(Debug)]
pub(crate) struct LeaveGameBlock {
    reason: String,
    player_id: u8,
    result: String, // .skip(4)
}

#[derive(Debug)]
pub(crate) struct TimeSlotBlock {
    byte_count: u16,
    time_increment: u16,
    actions: Vec<u8>, // .skip(4)
}

named!(
    parsed_game_block<&[u8], GameBlock>,
    switch!(take!(1),
        [23] => dbg!(call!(parse_leave_block)) |
        [26] => call!(ignore_4) | // 1st start
        [27] => call!(ignore_4) | // 2nd start
        [28] => call!(ignore_4) | // 3rd start
        [30] => call!(time_slot_block) |
        [31] => call!(time_slot_block) |
        [32] => call!(player_chat_msg) |
        [34] => call!(unknown_022) |
        [35] => call!(unknown_023) |
        [47]=> call!(forced_game_end_cd) |
        _ => value!(GameBlock::Unknown)
    )
);

named!(
    parsed_game_blocks<&[u8], (Vec<GameBlock>, &[u8])>,
    many_till!(parsed_game_block, eof!())
);

pub(crate) fn parse_game_blocks(input: &[u8]) -> IResult<&[u8], Vec<GameBlock>> {
    let (input, (res, _)) = parsed_game_blocks(input)?;
    Ok((input, res))
}

fn unknown_022(input: &[u8]) -> IResult<&[u8], GameBlock> {
    do_parse!(
        input,
        length: le_u8 >> skipped: dbg!(take!(length)) >> (GameBlock::Unknown)
    )
}

fn unknown_023(input: &[u8]) -> IResult<&[u8], GameBlock> {
    do_parse!(input, skipped: take!(8) >> (GameBlock::Unknown))
}

fn forced_game_end_cd(input: &[u8]) -> IResult<&[u8], GameBlock> {
    do_parse!(input, skipped: take!(8) >> (GameBlock::Unknown))
}

fn time_slot_block(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (input, res) = do_parse!(
        input,
        byte_count: le_u16
            >> time_increment: le_u16
            >> actions: dbg!(take!(byte_count - 2))
            >> (GameBlock::TimeSlot(TimeSlotBlock {
                byte_count,
                time_increment,
                actions: actions.to_vec()
            }))
    )?;
    Ok((input, res))
}

fn parse_leave_block(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (input, res) = do_parse!(
        input,
        reason: dbg!(take!(4))
            >> player_id: dbg!(le_u8)
            >> result: dbg!(take!(4))
            >> ignored: dbg!(take!(4))
            >> (GameBlock::Leave(LeaveGameBlock {
                player_id,
                reason: String::from_iter(reason.iter().flat_map(|c| u8_to_hex_string(c).to_vec())),
                result: String::from_iter(result.iter().flat_map(|c| u8_to_hex_string(c).to_vec())),
            }))
    )?;
    Ok((input, res))
}

fn ignore_4(input: &[u8]) -> IResult<&[u8], GameBlock> {
    do_parse!(input, skipped: take!(4) >> (GameBlock::Ignored))
}
