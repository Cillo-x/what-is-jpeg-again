fn caculate_size_and_amp(raw: i16) -> (u8, u16) {
    if raw == 0 {
        return (0, 0);
    }

    let abs_val = raw.unsigned_abs();
    let size = abs_val.ilog2() + 1;
    let amp = if raw > 0 {
        abs_val
    } else {
        2u16.pow(size) - abs_val - 1
    };

    (size as u8, amp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caculate_size_and_amp() {
        let tests = [
            (-7, (3, 0b000)),
            (-6, (3, 0b001)),
            (-5, (3, 0b010)),
            (-4, (3, 0b011)),
            (4, (3, 0b100)),
            (5, (3, 0b101)),
            (6, (3, 0b110)),
            (7, (3, 0b111)),
        ];

        for t in tests {
            let (raw, expected) = t;
            assert_eq!(caculate_size_and_amp(raw), expected);
        }
    }
}
