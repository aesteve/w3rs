use crate::blocks::chat::{player_chat_msg, PlayerChatMsgBlock};
use crate::blocks::command::{parse_command, CommandData};
use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::multi::many0;
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

impl LeaveGameBlock {
    pub fn is_draw(&self) -> bool {
        self.result[0] == 10 // no matter if reason[0] is 0x01 or 0x0C
    }
    /// true if we're SURE the player won, false otherwise
    pub fn player_won(&self) -> bool {
        self.result[0] == 9
    }
    /// true if we're SURE the player lost, false otherwise
    pub fn player_lost(&self) -> bool {
        self.result[0] == 8 || self.result[0] == 13
    }
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

pub(crate) fn parse_game_blocks(input: &[u8]) -> IResult<&[u8], Vec<GameBlock>> {
    many0(parsed_game_block)(input)
}

fn parsed_game_block(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, kind) = le_u8(input)?;
    match kind {
        23 => parse_leave_block(rest),
        26 | 27 => ignore_4(rest),
        30 | 31 => time_slot_block(rest),
        32 => player_chat_msg(rest),
        34 => unknown_022(rest),
        35 => unknown_023(rest),
        47 => forced_game_end_cd(rest),
        _ => Ok((rest, GameBlock::Unknown)),
    }
}

fn unknown_022(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, length) = le_u8(input)?;
    let (rest, _) = take(length as usize)(rest)?;
    Ok((rest, GameBlock::Unknown))
}

fn unknown_023(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, _) = take(8usize)(input)?;
    Ok((rest, GameBlock::Unknown))
}

fn forced_game_end_cd(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, _) = take(8usize)(input)?;
    Ok((rest, GameBlock::Unknown))
}

fn time_slot_block(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, byte_count) = le_u16(input)?;
    let (rest, time_increment) = le_u16(rest)?;
    let (rest, command) = map_res(take((byte_count - 2) as usize), parse_time_blocks)(rest)?;
    Ok((
        rest,
        GameBlock::TimeSlot(TimeSlotBlock {
            byte_count,
            time_increment,
            command: command.1,
        }),
    ))
}

fn parse_leave_block(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, reason) = take(4usize)(input)?;
    let (rest, player_id) = le_u8(rest)?;
    let (rest, result) = take(4usize)(rest)?;
    let (rest, _) = take(4usize)(rest)?;
    Ok((
        rest,
        GameBlock::Leave(LeaveGameBlock {
            player_id,
            reason: reason[0..4].try_into().unwrap(),
            result: result[0..4].try_into().unwrap(),
        }),
    ))
}

fn ignore_4(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, _) = take(4usize)(input)?;
    Ok((rest, GameBlock::Ignored))
}
