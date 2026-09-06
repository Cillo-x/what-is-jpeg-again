use alloc::vec::Vec;

use crate::{
    BitWriter, BlockU8, C_AC_HFT, C_DC_HFT, Dct, QC, QY, Y_AC_HFT, Y_DC_HFT,
    calculate_size_and_amp, quant, quant_mut, rgb_to_ycbcr, rle, write_app0, write_dht, write_dqt,
    write_eoi, write_sof0, write_soi, write_sos, zigzag_scan,
};

pub struct JpegEncoder<D>
where
    D: Dct,
{
    dct: D,
    y_blk: BlockU8,
    cb_blk: BlockU8,
    cr_blk: BlockU8,
    y_dc_prev: i16,
    cb_dc_prev: i16,
    cr_dc_prev: i16,
}

impl<D> JpegEncoder<D>
where
    D: Dct,
{
    pub fn new(dct: D) -> Self {
        Self {
            dct,
            y_blk: [[0; 8]; 8],
            cb_blk: [[0; 8]; 8],
            cr_blk: [[0; 8]; 8],
            y_dc_prev: 0,
            cb_dc_prev: 0,
            cr_dc_prev: 0,
        }
    }

    pub fn encode(&mut self, rgb: &[u8], width: usize, height: usize) -> Vec<u8> {
        assert_eq!(width % 8, 0);
        assert_eq!(height % 8, 0);

        self.y_dc_prev = 0;
        self.cb_dc_prev = 0;
        self.cr_dc_prev = 0;

        let mut buf = Vec::new();
        write_soi(&mut buf);
        write_app0(&mut buf);

        let qy_zigzag = zigzag_scan(&QY).map(|x| x as u8);
        let qc_zigzag = zigzag_scan(&QC).map(|x| x as u8);
        write_dqt(&mut buf, &qy_zigzag, &qc_zigzag);
        write_sof0(&mut buf, width as u16, height as u16);
        write_dht(&mut buf);
        write_sos(&mut buf);

        let mut bw = BitWriter::new(buf);
        let ycbcr_img = rgb_to_ycbcr(rgb, width, height);
        for by in 0..height / 8 {
            for bx in 0..width / 8 {
                let blk_y = by * 8;
                let blk_x = bx * 8;

                for r in 0..8 {
                    let row_offset = (blk_y + r) * width;
                    for c in 0..8 {
                        let src_idx = row_offset + blk_x + c;
                        self.y_blk[r][c] = ycbcr_img.y[src_idx];
                        self.cb_blk[r][c] = ycbcr_img.cb[src_idx];
                        self.cr_blk[r][c] = ycbcr_img.cr[src_idx];
                    }
                }
                // now we get 3 channel block
                let mut y = self.dct.forward_dct(&self.y_blk);
                let mut cr = self.dct.forward_dct(&self.cr_blk);
                let mut cb = self.dct.forward_dct(&self.cb_blk);

                quant_mut(&mut y, &quant::QY);
                quant_mut(&mut cr, &quant::QC);
                quant_mut(&mut cb, &quant::QC);

                let y_zig = zigzag_scan(&y);
                let cr_zig = zigzag_scan(&cr);
                let cb_zig = zigzag_scan(&cb);

                // Y panel
                let (y_dc_diff, y_ac_pairs) = rle(y_zig, self.y_dc_prev);
                self.y_dc_prev = y[0][0];
                let (size, amp) = calculate_size_and_amp(y_dc_diff);
                bw.write_bits(
                    Y_DC_HFT.codes[size as usize],
                    Y_DC_HFT.sizes[size as usize] as usize,
                );
                bw.write_bits(amp, size as usize);
                for (k, v) in y_ac_pairs {
                    let (s, v) = calculate_size_and_amp(v);
                    let symbol = (k << 4 | s) as usize;
                    bw.write_bits(Y_AC_HFT.codes[symbol], Y_AC_HFT.sizes[symbol] as usize);
                    bw.write_bits(v, s as usize);
                }
                // Cb Panel
                let (cb_dc_diff, cb_ac_pairs) = rle(cb_zig, self.cb_dc_prev);
                self.cb_dc_prev = cb[0][0];
                let (size, amp) = calculate_size_and_amp(cb_dc_diff);
                bw.write_bits(
                    C_DC_HFT.codes[size as usize],
                    C_DC_HFT.sizes[size as usize] as usize,
                );
                bw.write_bits(amp, size as usize);
                for (k, v) in cb_ac_pairs {
                    let (s, v) = calculate_size_and_amp(v);
                    let symbol = (k << 4 | s) as usize;
                    assert!(
                        C_AC_HFT.sizes[symbol] != 0,
                        "Cb block ({bx}, {by}): missing AC Huffman symbol {symbol:#04x}"
                    );
                    bw.write_bits(C_AC_HFT.codes[symbol], C_AC_HFT.sizes[symbol] as usize);
                    bw.write_bits(v, s as usize);
                }
                // Cr Panel
                let (cr_dc_diff, cr_ac_pairs) = rle(cr_zig, self.cr_dc_prev);
                self.cr_dc_prev = cr[0][0];
                let (size, amp) = calculate_size_and_amp(cr_dc_diff);
                bw.write_bits(
                    C_DC_HFT.codes[size as usize],
                    C_DC_HFT.sizes[size as usize] as usize,
                );
                bw.write_bits(amp, size as usize);
                for (k, v) in cr_ac_pairs {
                    let (s, v) = calculate_size_and_amp(v);
                    let symbol = (k << 4 | s) as usize;
                    bw.write_bits(C_AC_HFT.codes[symbol], C_AC_HFT.sizes[symbol] as usize);
                    bw.write_bits(v, s as usize);
                }
            }
        }
        let mut buf = bw.finish();
        write_eoi(&mut buf);
        buf
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{File, write},
        io::Read,
    };

    use crate::NaiveDct;

    use super::*;

    #[test]
    fn test_smoke() {
        JpegEncoder::new(NaiveDct);
    }

    #[test]
    fn test_demo() {
        let mut file = File::open("../example/demo.bin").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let mut encoder = JpegEncoder::new(NaiveDct);
        // let mut bw = BitWriter::new(Vec::with_capacity(4 * 1024));
        let outbuf = encoder.encode(&data, 256, 256);
        // let outbuf = bw.finish();
        write("../playground/raw.jpg", outbuf).unwrap();
    }
}
