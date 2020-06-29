use nom::IResult;

named!(zero_terminated<&[u8], &[u8]>,
    terminated!(take_while!(|b: u8| b != 0), tag!([0])));

#[derive(Debug)]
pub(crate) struct MapInfo {
    pub(crate) name: String, // zeroTerminated
}

pub(crate) fn parse_map_info(input: &[u8]) -> IResult<&[u8], MapInfo> {
    do_parse!(
        input,
        skipped: take!(13)
            >> name: zero_terminated
            >> creator: zero_terminated
            >> (MapInfo {
                name: String::from_utf8_lossy(name).to_string(),
            })
    )
}
