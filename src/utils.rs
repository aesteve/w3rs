named!(pub(crate) zero_terminated<&[u8], &[u8]>,
    terminated!(take_while!(|b: u8| b != 0), tag!([0])));

pub fn decode(encoded: &[u8]) -> Vec<u8> {
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
