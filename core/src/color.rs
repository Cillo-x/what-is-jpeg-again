pub struct YCbCrImage {
    pub width: usize,
    pub height: usize,
    pub y: Vec<u8>,
    pub cb: Vec<u8>,
    pub cr: Vec<u8>,
}

pub fn rgb_to_ycbcr(rgb: &[u8], width: usize, height: usize) -> YCbCrImage {
    let size = width * height * 3;
    let mut y = Vec::with_capacity(size);
    let mut cb = Vec::with_capacity(size);
    let mut cr = Vec::with_capacity(size);

    for chunk in rgb.chunks_exact(3) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;

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
    fn test_rgb_to_ycrcb() {
        let test_rgb = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 128, 128, 128];
        let expected_y = vec![76, 150, 29, 128];
        let expected_cb = vec![85, 44, 255, 128];
        let expected_cr = vec![255, 21, 107, 128];

        let img = rgb_to_ycbcr(&test_rgb, 4, 4);
        assert_eq!(img.y, expected_y);
        assert_eq!(img.cb, expected_cb);
        assert_eq!(img.cr, expected_cr);
    }
}
