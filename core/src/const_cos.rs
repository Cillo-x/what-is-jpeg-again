use core::f32::consts::PI;

const fn reduce_angle(x: f32) -> (f32, f32, bool) {
    // Reduce angle to [0, 2*PI)
    let mut r = x % (2.0 * PI);
    if r < 0.0 {
        r += 2.0 * PI;
    }

    // Reduce angle to [-PI/4, PI/4]
    if r <= PI / 4.0 {
        (r, 1.0, false)
    } else if r <= 3.0 * PI / 4.0 {
        // cos(x) = sin(PI/2 - x)
        (PI / 2.0 - r, 1.0, true)
    } else if r <= 5.0 * PI / 4.0 {
        // cos(x) = -cos(x - PI)
        (r - PI, -1.0, false)
    } else if r <= 7.0 * PI / 4.0 {
        (r - 1.5 * PI, 1.0, true)
    } else {
        (r - 2.0 * PI, 1.0, false)
    }
}

const fn sin_taylor(r: f32) -> f32 {
    let r2 = r * r;
    r * (1.0
        + r2 * (-1.0 / 6.0 + r2 * (1.0 / 120.0 + r2 * (-1.0 / 5040.0 + r2 * (1.0 / 362880.0)))))
}

const fn cos_taylor(r: f32) -> f32 {
    let r2 = r * r;
    1.0 + r2 * (-0.5 + r2 * (1.0 / 24.0 + r2 * (-1.0 / 720.0 + r2 * (1.0 / 40320.0))))
}

/// Computes an approximation of `cos(x)` at compile time.
///
/// `x` is in radians.
///
/// For good accuracy, `x` should be small in magnitude.
pub const fn const_cos(x: f32) -> f32 {
    let (r, sign, use_sin) = reduce_angle(x);
    let v = if use_sin {
        sin_taylor(r)
    } else {
        cos_taylor(r)
    };

    sign * v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32, tol: f32, msg: &str) {
        let diff = (actual - expected).abs();
        assert!(
            diff <= tol,
            "{msg}: actual = {actual}, expected = {expected}, diff = {diff}, tol = {tol}"
        )
    }

    #[test]
    fn test_smoke() {
        assert_eq!(const_cos(PI / 2.0), 0.0);
    }

    #[test]
    fn test_key_points() {
        let tol = 1e-5;

        assert_close(const_cos(0.0), 1.0, tol, "cos(0)");
        assert_close(const_cos(PI), -1.0, tol, "cos(pi)");
        assert_close(const_cos(-PI), -1.0, tol, "cos(-pi)");
        assert_close(const_cos(2.0 * PI), 1.0, tol, "cos(2pi)");
        assert_close(const_cos(0.5 * PI), 0.0, tol, "cos(pi/2)");
        assert_close(const_cos(1.5 * PI), 0.0, tol, "cos(3pi/2)");
        assert_close(const_cos(-0.5 * PI), 0.0, tol, "cos(-pi/2)");

        assert_close(const_cos(PI / 4.0), (PI / 4.0).cos(), tol, "cos(pi/4)");
        assert_close(
            const_cos(3.0 * PI / 4.0),
            (3.0 * PI / 4.0).cos(),
            tol,
            "cos(3pi/4)",
        );
        assert_close(
            const_cos(5.0 * PI / 4.0),
            (5.0 * PI / 4.0).cos(),
            tol,
            "cos(5pi/4)",
        );
        assert_close(
            const_cos(7.0 * PI / 4.0),
            (7.0 * PI / 4.0).cos(),
            tol,
            "cos(7pi/4)",
        );
    }
}
