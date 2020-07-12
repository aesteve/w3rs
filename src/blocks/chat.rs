use crate::blocks::gameblock::GameBlock;
use crate::utils::zero_terminated;
use nom::bytes::complete::take;
use nom::{
    number::complete::{le_u16, le_u8},
    IResult,
};

#[derive(Debug)]
pub(crate) struct PlayerChatMsgMetadata {
    player_id: u8,
    byte_count: u16,
}

#[derive(Debug)]
pub struct PlayerChatMsgBlock {
    pub player_id: u8,
    pub text: String,
    kind: ChatMsgBlock,
}

#[derive(Debug)]
pub(crate) enum ChatMsgBlock {
    StartupMsg,
    Msg(Addressee),
    Unknown,
}

#[derive(Debug)]
pub(crate) enum Addressee {
    All,
    Allies,
    Observers,
    Direct,
}

impl Addressee {
    fn from_u8(to: &[u8]) -> Addressee {
        match to[0..2] {
            [0, 0] => Addressee::All,
            [0, 1] => Addressee::Allies,
            [0, 2] => Addressee::Observers,
            _ => Addressee::Direct,
        }
    }
}

fn parse_message(input: &[u8]) -> IResult<&[u8], ChatMsgBlock> {
    let (rest, msg_type) = le_u8(input)?;
    match msg_type {
        16 => Ok((rest, ChatMsgBlock::StartupMsg)),
        32 => addressee_chat_msg(rest),
        _ => Ok((rest, ChatMsgBlock::Unknown)),
    }
}

fn addressee_chat_msg(input: &[u8]) -> IResult<&[u8], ChatMsgBlock> {
    let (rest, to) = take(4usize)(input)?;
    Ok((rest, ChatMsgBlock::Msg(Addressee::from_u8(&to))))
}

fn parse_msg_metadata(input: &[u8]) -> IResult<&[u8], PlayerChatMsgMetadata> {
    let (rest, player_id) = le_u8(input)?;
    let (rest, byte_count) = le_u16(rest)?;
    Ok((
        rest,
        PlayerChatMsgMetadata {
            player_id,
            byte_count,
        },
    ))
}

pub(crate) fn player_chat_msg(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (rest, meta) = parse_msg_metadata(input)?;
    let (rest, msg) = parse_message(rest)?;
    let (rest, text) = zero_terminated(rest)?;
    let block = GameBlock::PlayerChatMsg(PlayerChatMsgBlock {
        player_id: meta.player_id,
        kind: msg,
        text: String::from_utf8_lossy(text).to_string(),
    });
    Ok((rest, block))
}
