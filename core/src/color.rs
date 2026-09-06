use alloc::vec::Vec;

pub struct YCbCrImage {
    pub width: usize,
    pub height: usize,
    pub y: Vec<u8>,
    pub cb: Vec<u8>,
    pub cr: Vec<u8>,
}

pub fn rgb_to_ycbcr(rgb: &[u8], width: usize, height: usize) -> YCbCrImage {
    let expected = width * height * 3;
    assert_eq!(
        rgb.len(),
        expected,
        "RGB data len {} != expected {}",
        rgb.len(),
        expected
    );

    let mut y = Vec::with_capacity(expected);
    let mut cb = Vec::with_capacity(expected);
    let mut cr = Vec::with_capacity(expected);

    for i in (0..expected).step_by(3) {
        let r = rgb[i] as f32;
        let g = rgb[i + 1] as f32;
        let b = rgb[i + 2] as f32;

        y.push((0.299 * r + 0.587 * g + 0.114 * b).round() as u8);
        cb.push((-0.1687 * r - 0.3313 * g + 0.5 * b + 128.0).round() as u8);
        cr.push((0.5 * r - 0.4187 * g - 0.0813 * b + 128.0).round() as u8);
    }

    YCbCrImage {
        width,
        height,
        y,
        cb,
        cr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        // 1x1 pixel pure White RGB data
        let rgb = vec![255, 255, 255];
        let img = rgb_to_ycbcr(&rgb, 1, 1);
        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);

        assert_eq!(img.y.len(), 1);
        assert_eq!(img.cb.len(), 1);
        assert_eq!(img.cr.len(), 1);

        assert_eq!(img.y[0], 255);
        assert_eq!(img.cb[0], 128);
        assert_eq!(img.cr[0], 128);
    }

    #[test]
    #[should_panic]
    fn test_no_enough_data() {
        let rgb = [0u8; 2];
        rgb_to_ycbcr(&rgb, 1, 1);
    }

    #[test]
    fn test_rgb_to_ycbcr() {
        // R G B
        let test_rgb = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128];
        let expected_y = vec![76, 150, 29, 128];
        let expected_cb = vec![85, 44, 255, 128];
        let expected_cr = vec![255, 21, 107, 128];

        let img = rgb_to_ycbcr(&test_rgb, 2, 2);
        assert_eq!(img.y, expected_y);
        assert_eq!(img.cb, expected_cb);
        assert_eq!(img.cr, expected_cr);
    }
}
