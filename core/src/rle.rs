/// AC coeff pair
/// `(run_length, raw_coefficient)`
pub type AcPair = (u8, i16);

pub const ZRL: AcPair = (15, 0);
pub const EOB: AcPair = (0, 0);

pub fn rle(buf: [i16; 64], dc_prev: i16) -> (i16, Vec<AcPair>) {
    let dc_diff = buf[0] - dc_prev;

    let mut result = Vec::with_capacity(32);
    let mut zero_count = 0;

    for &coeff in buf.iter().skip(1) {
        if coeff == 0 {
            zero_count += 1;
            if zero_count == 16 {
                result.push(ZRL);
                zero_count = 0;
            }
        } else {
            result.push((zero_count, coeff));
            zero_count = 0;
        }
    }
    // EOB represents the remaining zero coefficients. When the last AC
    // coefficient (index 63) is non-zero, the block is already complete and
    // writing EOB would be consumed as the next block's DC Huffman code.
    if zero_count > 0 {
        result.push(EOB);
    }

    (dc_diff, result)
}

#[cfg(test)]
mod tests {

    use core::array;

    use super::*;

    #[test]
    fn test_smoke() {
        let buf: [i16; 64] = array::from_fn(|i| match i {
            0 => 42,
            _ => 0,
        });
        let (dc, _) = rle(buf, 0);
        assert_eq!(dc, 42)
    }

    #[test]
    fn test_prev_block() {
        let buf: [i16; 64] = array::from_fn(|i| match i {
            0 => 42,
            _ => 0,
        });
        let (dc, _) = rle(buf, -1);
        assert_eq!(dc, 43);
    }

    #[test]
    fn test_eob() {
        let buf: [i16; 64] = array::from_fn(|i| match i {
            1..=9 => i as i16,
            _ => 0,
        });
        let (_, pairs) = rle(buf, 0);
        assert_eq!(*pairs.last().unwrap(), EOB)
    }

    #[test]
    fn test_zrl() {
        let buf: [i16; 64] = array::from_fn(|i| match i {
            1..=16 => 0,
            _ => 42,
        });
        let (_, pairs) = rle(buf, 0);
        assert_eq!(pairs[0], ZRL);
        assert_eq!(pairs[1], (0, 42))
    }

    #[test]
    fn test_fifteen_zeros_are_not_zrl() {
        let mut buf = [0i16; 64];
        buf[16] = 42; // 15 AC zeros, then a non-zero coefficient

        let (_, pairs) = rle(buf, 0);

        assert_eq!(&pairs[..2], &[(15, 42), ZRL]);
    }

    #[test]
    fn test_sixteen_zeros_use_zrl() {
        let mut buf = [0i16; 64];
        buf[17] = 42; // 16 AC zeros, then a non-zero coefficient

        let (_, pairs) = rle(buf, 0);

        assert_eq!(&pairs[..2], &[ZRL, (0, 42)]);
    }

    #[test]
    fn test_fifteen_trailing_zeros_are_eob() {
        let mut buf = [0i16; 64];
        buf[48] = 42; // 15 AC zeros after the last non-zero coefficient

        let (_, pairs) = rle(buf, 0);

        assert_eq!(*pairs.last().unwrap(), EOB);
        assert_eq!(pairs[pairs.len() - 2], (15, 42));
    }

    #[test]
    fn test_full_block_does_not_end_with_eob() {
        let buf = [42i16; 64];

        let (_, pairs) = rle(buf, 0);

        assert_eq!(pairs.len(), 63);
        assert_eq!(*pairs.last().unwrap(), (0, 42));
        assert!(!pairs.contains(&EOB));
    }
}
