use core::f32::consts::{FRAC_1_SQRT_2, PI};

pub trait Dct {
    fn forward_dct(&self, blk: &BlockU8) -> BlockI16;
}

type Block<T> = [[T; 8]; 8];
pub type BlockU8 = Block<u8>;
pub type BlockI16 = Block<i16>;

pub struct NaiveDct;

impl Dct for NaiveDct {
    fn forward_dct(&self, blk: &BlockU8) -> BlockI16 {
        let temp = blk.map(|row| row.map(|p| p as f32 - 128.0));
        let mut result: BlockI16 = [[0; 8]; 8];

        // let c = |k: usize| -> f32 { if k == 0 { 1.0 / 2.0f32.sqrt() } else { 1.0 } };
        const C: [f32; 8] = [FRAC_1_SQRT_2, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

        for u in 0..8 {
            for v in 0..8 {
                let mut sum = 0.0;
                for i in 0..8 {
                    for j in 0..8 {
                        let cos_x = ((2 * i + 1) as f32 * u as f32 * PI / 16.0).cos();
                        let cos_y = ((2 * j + 1) as f32 * v as f32 * PI / 16.0).cos();
                        sum += temp[i][j] * cos_x * cos_y;
                    }
                }
                result[u][v] = (0.25 * C[u] * C[v] * sum)
                    .clamp(i16::MIN as f32, i16::MAX as f32)
                    .round() as i16;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_constant_block(dct: impl Dct) {
        let blk = [[1 + 128; 8]; 8];
        let expected_dc = 8;
        let eps = 1;

        let coeffs = dct.forward_dct(&blk);
        assert_eq!(coeffs[0][0], expected_dc);
        for i in 0..8 {
            for j in 0..8 {
                if i == 0 && j == 0 {
                    continue;
                }
                assert!(
                    coeffs[i][j].abs() < eps,
                    "AC should be close to 0, got {}",
                    coeffs[i][j]
                );
            }
        }
    }

    #[test]
    fn test_smoke() {
        let blk = [[129; 8]; 8];
        let dct = NaiveDct;
        let coeffs = dct.forward_dct(&blk);
        assert_eq!(coeffs[0][0], 8);
    }

    #[test]
    fn test_naive_dct() {
        let dct = NaiveDct;
        test_constant_block(dct);
    }
}
