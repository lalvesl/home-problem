use std::io::{Read, Write};

static LEVEL: i32 = 19;

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = zstd::stream::Encoder::new(Vec::new(), LEVEL).unwrap();
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decoder = zstd::stream::Decoder::new(data).unwrap();
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &[u8] = b"SOme tests potato potato
        11111111111111111111111111111111111111111111111
        *(&Y#)*&HY$(_I_G)RU(IGNJW_RE)(UG{(_#";

    #[test]
    fn test_zstd_compression() {
        let compressed = compress(SAMPLE_DATA);
        let decompressed = decompress(&compressed);

        assert_eq!(
            SAMPLE_DATA, decompressed,
            "Zstd decompression does not match original data!"
        );
        assert!(
            compressed.len() < SAMPLE_DATA.len(),
            "Zstd compression did not reduce size!"
        );
    }
}
