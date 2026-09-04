pub type AcPair = (u8, i16);
pub const ZRL: AcPair = (15, 0);
pub const EOB: AcPair = (0, 0);

pub fn rle(buf: [i16; 64], prev_dc: i16) -> (i16, Vec<AcPair>) {
    let diff_dc = buf[0] - prev_dc;

    let mut result = Vec::with_capacity(32);
    let mut zero_count = 0;

    for &coeff in buf.iter().skip(1) {
        if coeff == 0 {
            zero_count += 1;
            if zero_count == 15 {
                result.push(ZRL);
                zero_count = 0;
            }
        } else {
            result.push((zero_count, coeff));
            zero_count = 0;
        }
    }
    result.push(EOB);

    (diff_dc, result)
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
        assert_eq!(pairs[1], (1, 42))
    }
}
