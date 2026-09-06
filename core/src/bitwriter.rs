pub struct BitWriter {
    buf: Vec<u8>,
    current_byte: u8,
    bits_filled: usize,
    /// Number of bytes written to 'buf', including JPEG byte-stuffing bytes
    pub written: usize,
}

impl BitWriter {
    pub fn new(buf: Vec<u8>) -> Self {
        BitWriter {
            buf,
            bits_filled: 0,
            current_byte: 0,
            written: 0,
        }
    }

    pub fn write_bits(&mut self, value: u16, mut n: usize) {
        while n > 0 {
            let bits_to_take = core::cmp::min(n, 8 - self.bits_filled);
            let shift = n - bits_to_take;

            let bits = ((value >> shift) & ((1 << bits_to_take) - 1)) as u8;
            self.current_byte |= bits << (8 - self.bits_filled - bits_to_take);

            self.bits_filled += bits_to_take;
            n -= bits_to_take;

            if self.bits_filled == 8 {
                self.flush_byte();
            }
        }
    }

    fn flush_byte(&mut self) {
        if self.bits_filled > 0 {
            self.buf.push(self.current_byte);
            self.written += 1;

            // JPEG byte stuffing: insert 0x00 after 0xFF to avoid marker collision
            if self.current_byte == 0xFF {
                self.buf.push(0x00);
                self.written += 1;
            }

            self.current_byte = 0;
            self.bits_filled = 0;
        }
    }

    /// Flush the current byte, padding unused bits with `1` (required for JPEG)
    pub fn flush(&mut self) {
        if self.bits_filled > 0 {
            self.current_byte |= (1 << (8 - self.bits_filled)) - 1;
            self.flush_byte();
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.flush();
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        let mut bwriter = BitWriter::new(Vec::new());
        bwriter.write_bits(1, 8);
        bwriter.flush();
        assert_eq!(bwriter.buf[0], 1)
    }

    #[test]
    fn test_part_byte() {
        let mut bwriter = BitWriter::new(Vec::new());
        bwriter.write_bits(0b1010, 4);
        bwriter.flush();
        assert_eq!(bwriter.buf[0], 0b1010_1111)
    }

    #[test]
    fn test_cross_one_byte_write() {
        let mut bwriter = BitWriter::new(Vec::new());
        bwriter.write_bits(0b1010, 4);
        bwriter.write_bits(0, 8);
        bwriter.flush();
        assert_eq!(bwriter.buf[0], 0b1010_0000);
        assert_eq!(bwriter.buf[1], 0b0000_1111);
    }

    #[test]
    fn test_cross_two_byte_write() {
        let mut bwriter = BitWriter::new(Vec::new());
        bwriter.write_bits(0b1010, 4);
        bwriter.write_bits(0b1001_1111_0000_101, 15);
        bwriter.flush();
        assert_eq!(bwriter.buf[0], 0b1010_1001);
        assert_eq!(bwriter.buf[1], 0b1111_0000);
        assert_eq!(bwriter.buf[2], 0b101_11111);
    }

    #[test]
    fn test_finish() {
        let mut bwriter = BitWriter::new(Vec::new());
        bwriter.write_bits(0b1001_1111_0000_101, 15);
        let buf = bwriter.finish();
        assert_eq!(buf[0], 0b1001_1111);
        assert_eq!(buf[1], 0b0000_1011);
    }

    #[test]
    fn test_0xff_escape() {
        let mut bwriter = BitWriter::new(Vec::new());
        bwriter.write_bits(0b1111, 4);
        bwriter.write_bits(0b1111_01, 6);
        bwriter.flush();
        assert_eq!(bwriter.written, 3);
        assert_eq!(bwriter.buf[1], 0);
        assert_eq!(bwriter.buf[2], 0b0111_1111)
    }
}
