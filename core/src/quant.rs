use crate::BlockI16;

pub const QY: BlockI16 = [
    [16, 11, 10, 16, 24, 40, 51, 61],
    [12, 12, 14, 19, 26, 58, 60, 55],
    [14, 13, 16, 24, 40, 57, 69, 56],
    [14, 17, 22, 29, 51, 87, 80, 62],
    [18, 22, 37, 56, 68, 109, 103, 77],
    [24, 35, 55, 64, 81, 104, 113, 92],
    [49, 64, 78, 87, 103, 121, 120, 101],
    [72, 92, 95, 98, 112, 100, 103, 99],
];

pub const QC: BlockI16 = [
    [17, 18, 24, 47, 99, 99, 99, 99],
    [18, 21, 26, 66, 99, 99, 99, 99],
    [24, 26, 56, 99, 99, 99, 99, 99],
    [47, 66, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
];

pub fn quant(blk: &BlockI16, table: &BlockI16) -> BlockI16 {
    let mut result = [[0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            result[i][j] = blk[i][j] / table[i][j];
        }
    }
    result
}

pub fn quant_mut(blk: &mut BlockI16, table: &BlockI16) {
    for i in 0..8 {
        for j in 0..8 {
            blk[i][j] /= table[i][j];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        let expected = [[1; 8]; 8];
        assert_eq!(quant(&QY, &QY), expected);
        assert_eq!(quant(&QC, &QC), expected);
    }
}
