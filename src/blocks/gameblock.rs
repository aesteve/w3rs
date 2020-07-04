use crate::blocks::chat::{player_chat_msg, PlayerChatMsgBlock};
use crate::blocks::command::{parse_command, CommandData};
use nom::{
    number::complete::{le_u16, le_u8},
    IResult,
};
use std::convert::TryInto;

#[derive(Debug)]
pub enum GameBlock {
    Leave(LeaveGameBlock),
    TimeSlot(TimeSlotBlock),
    PlayerChatMsg(PlayerChatMsgBlock),
    Ignored,
    Unknown,
}

impl GameBlock {
    pub fn should_display(&self) -> bool {
        match self {
            GameBlock::Unknown | GameBlock::Ignored => false,
            GameBlock::TimeSlot(ts) => ts.command.is_some(),
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct LeaveGameBlock {
    pub player_id: u8,
    pub reason: [u8; 4],
    pub result: [u8; 4],
}

#[derive(Debug)]
pub struct TimeSlotBlock {
    byte_count: u16,
    pub time_increment: u16,
    pub command: Option<CommandData>,
}

pub(crate) fn parse_time_blocks(input: &[u8]) -> IResult<&[u8], Option<CommandData>> {
    if input.is_empty() {
        Ok((&[], None))
    } else {
        parse_command(input).map(|(b, cd)| (b, Some(cd)))
    }
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
            >> command: map_res!(take!(byte_count - 2), parse_time_blocks)
            >> (GameBlock::TimeSlot(TimeSlotBlock {
                byte_count,
                time_increment,
                command: command.1
            }))
    )?;
    Ok((input, res))
}

fn parse_leave_block(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (input, res) = do_parse!(
        input,
        reason: take!(4)
            >> player_id: le_u8
            >> result: take!(4)
            >> ignored: take!(4)
            >> (GameBlock::Leave(LeaveGameBlock {
                player_id,
                reason: reason[0..4].try_into().unwrap(),
                result: result[0..4].try_into().unwrap(),
            }))
    )?;
    Ok((input, res))
}

fn ignore_4(input: &[u8]) -> IResult<&[u8], GameBlock> {
    do_parse!(input, skipped: take!(4) >> (GameBlock::Ignored))
}
