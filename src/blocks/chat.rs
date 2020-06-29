use crate::blocks::gameblock::GameBlock;
use crate::utils::zero_terminated;
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
pub(crate) struct PlayerChatMsgBlock {
    player_id: u8,
    kind: ChatMsgBlock,
    text: String,
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

named!(
    parse_message<&[u8], ChatMsgBlock>,
    switch!(take!(1),
        [16] => call!(startup_chat_msg) |
        [32] => call!(addressee_chat_msg) |
        _ => call!(unknown_chat_msg)
    )
);

fn startup_chat_msg(input: &[u8]) -> IResult<&[u8], ChatMsgBlock> {
    do_parse!(input, (ChatMsgBlock::StartupMsg))
}

fn addressee_chat_msg(input: &[u8]) -> IResult<&[u8], ChatMsgBlock> {
    do_parse!(
        input,
        to: take!(4) >> (ChatMsgBlock::Msg(Addressee::from_u8(&to)))
    )
}

fn unknown_chat_msg(input: &[u8]) -> IResult<&[u8], ChatMsgBlock> {
    do_parse!(input, (ChatMsgBlock::Unknown))
}

pub(crate) fn player_chat_msg(input: &[u8]) -> IResult<&[u8], GameBlock> {
    let (input, meta) = do_parse!(
        input,
        player_id: le_u8
            >> byte_count: le_u16
            >> (PlayerChatMsgMetadata {
                player_id,
                byte_count,
            })
    )?;
    let (input, msg) = parse_message(input)?;
    let (input, text) = do_parse!(
        input,
        b: zero_terminated >> (String::from_utf8_lossy(b).to_string())
    )?;
    let block = GameBlock::PlayerChatMsg(PlayerChatMsgBlock {
        player_id: meta.player_id,
        kind: msg,
        text,
    });
    Ok((input, block))
}
