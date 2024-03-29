use flate2::read::ZlibDecoder;
use nom::bytes::complete::take;
use nom::multi::many0;
use nom::{number::complete::le_u16, IResult};
use std::io;
use std::io::Read;

#[derive(PartialEq, Eq)]
pub(crate) struct CompressedDataBlock {
    pub block_size: u16,
    pub block_decompressed_size: u16,
    pub compressed: Vec<u8>,
}

impl CompressedDataBlock {
    pub fn inflate(&self) -> Result<Vec<u8>, io::Error> {
        let mut decoded = Vec::with_capacity(self.block_decompressed_size as usize);
        let mut decoder = ZlibDecoder::new(&self.compressed[..]);
        decoder.read_to_end(&mut decoded)?;
        Ok(decoded)
    }
}

pub(crate) fn deflate_game(blocks: &[CompressedDataBlock]) -> Result<Vec<u8>, io::Error> {
    let mut decoded = Vec::new();
    for block in blocks {
        decoded.extend(block.inflate()?);
    }
    Ok(decoded)
}

pub(crate) fn compressed_data_blocks(input: &[u8]) -> IResult<&[u8], Vec<CompressedDataBlock>> {
    many0(compressed_data_block)(input)
}

fn compressed_data_block(input: &[u8]) -> IResult<&[u8], CompressedDataBlock> {
    let (rest, block_size) = le_u16(input)?;
    let (rest, _) = take(2usize)(rest)?;
    let (rest, block_decompressed_size) = le_u16(rest)?;
    let (rest, _) = take(4usize)(rest)?; // str ignored
    let (rest, _) = take(2usize)(rest)?; // bytes ignored
    let (rest, compressed) = take(block_size as usize)(rest)?;
    Ok((
        rest,
        CompressedDataBlock {
            block_size,
            block_decompressed_size,
            compressed: compressed.to_vec(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::blocks::compressedblock::{compressed_data_blocks, deflate_game};
    use crate::metadata::replay::parse_header;
    use crate::tests::replay_bytes;

    #[test]
    fn data_blocks_test() {
        let file = replay_bytes("reforged2010.w3g");
        let (rest, headers) = parse_header(&file).unwrap();
        let (rest, blocks) = compressed_data_blocks(rest).unwrap();
        assert_eq!(0, rest.len());
        assert_eq!(headers.compressed_data_block_count as usize, blocks.len());
        let decoded = deflate_game(&blocks).unwrap();
        assert_eq!(
            blocks
                .iter()
                .map(|m| m.block_decompressed_size as u64)
                .sum::<u64>(),
            decoded.len() as u64
        );
    }
}
