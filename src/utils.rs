use nom::bytes::complete::{tag, take_while};
use nom::sequence::terminated;
use nom::IResult;

pub(crate) fn zero_terminated_string(input: &[u8]) -> IResult<&[u8], String> {
    let (rest, byte_str) = zero_terminated(input)?;
    Ok((rest, String::from_utf8_lossy(byte_str).to_string()))
}

pub(crate) fn zero_terminated(input: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_while(|b: u8| b != 0), tag([0]))(input)
}

/// See: https://gist.github.com/dengzhp/1185519#file-w3g_format-txt-L435
pub(crate) fn decode(encoded: &[u8]) -> Vec<u8> {
    let mut decoded_string: Vec<u8> = Vec::new();
    let mut mask: usize = 0;
    let mut pos: usize = 0;
    let mut dpos: usize = 0;

    while encoded.get(pos).is_some() {
        if pos % 8 == 0 {
            mask = *(encoded.get(pos).unwrap()) as usize;
        } else {
            if (mask & (0x1 << (pos % 8))) == 0 {
                decoded_string.insert(dpos, encoded.get(pos).unwrap() - 1);
            } else {
                decoded_string.insert(dpos, *encoded.get(pos).unwrap());
            }
            dpos += 1;
        }
        pos += 1;
    }
    decoded_string
}
