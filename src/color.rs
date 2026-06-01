// https://stackoverflow.com/a/9493060, translation validated by Google Gemini™
/// `h`, `s`, `l` in range `0.0..=1.0`
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    if s == 0.0 {
        let achromatic = (l * 255.0).round() as u8;
        [achromatic, achromatic, achromatic]
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        [
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        ]
    }
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> u8 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    (if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    } * 255.0)
        .round() as u8
}
