use crate::utils::zero_terminated;
use nom::IResult;

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
