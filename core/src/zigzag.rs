use crate::BlockI16;

const JPEG_ZIGZAG_LUT: [usize; 64] = [
    0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20,
    13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59,
    52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
];

pub fn zigzag_scan(blk: &BlockI16) -> [i16; 64] {
    let mut result = [0; 64];
    for i in 0..64 {
        let natural_index = JPEG_ZIGZAG_LUT[i];
        result[i] = blk[natural_index / 8][natural_index % 8]
    }
    result
}

#[cfg(test)]
mod tests {
    use core::array;

    use super::*;

    #[test]
    fn test_smoke() {
        let blk: BlockI16 = [
            [11, 12, 16, 17, 99, 99, 99, 99],
            [13, 15, 18, 99, 99, 99, 99, 99],
            [14, 19, 99, 99, 99, 99, 99, 99],
            [20, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
        ];
        assert_eq!(zigzag_scan(&blk)[0], 11)
    }

    #[test]
    fn test_zigzag_enc() {
        let blk: BlockI16 = [
            [11, 12, 16, 17, 99, 99, 99, 99],
            [13, 15, 18, 99, 99, 99, 99, 99],
            [14, 19, 99, 99, 99, 99, 99, 99],
            [20, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
            [99, 99, 99, 99, 99, 99, 99, 99],
        ];
        let expected: [i16; 64] = array::from_fn(|i| if i < 10 { 11 + i as i16 } else { 99 });
        let buf = zigzag_scan(&blk);
        assert_eq!(buf, expected)
    }
}
