use crate::utils::zero_terminated;
use nom::bytes::complete::take;
use nom::IResult;

#[derive(Debug)]
pub struct MapInfo {
    pub name: String, // zeroTerminated
}

pub(crate) fn parse_map_info(input: &[u8]) -> IResult<&[u8], MapInfo> {
    let (rest, _) = take(13usize)(input)?;
    let (rest, name) = zero_terminated(rest)?;
    let (rest, _) = zero_terminated(rest)?; // creator name?
    Ok((
        rest,
        MapInfo {
            name: String::from_utf8_lossy(name).to_string(),
        },
    ))
}
