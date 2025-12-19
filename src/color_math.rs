use crate::color_spaces::{Color, Hsl, Rgb};
use rand::prelude::*;

// TODO

pub fn complement(hsl: Hsl) -> Hsl {
    let new_h = (hsl.h + 180).rem_euclid(360);
    Hsl::new(new_h, hsl.s, hsl.l)
}

pub fn triad(hsl: Hsl) -> (Hsl, Hsl) {
    let left = (hsl.h as i32 - 120).rem_euclid(360);
    let right = (hsl.h + 120).rem_euclid(360);

    let left = Hsl::new(left as u16, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}

pub fn square(hsl: Hsl) -> (Hsl, Hsl, Hsl) {
    let left = (hsl.h as i32 - 90).rem_euclid(360);
    let middle = (hsl.h + 180).rem_euclid(360);
    let right = (hsl.h + 90).rem_euclid(360);

    let left = Hsl::new(left as u16, hsl.s, hsl.l);
    let middle = Hsl::new(middle, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, middle, right)
}

pub fn analogous(hsl: Hsl) -> (Hsl, Hsl) {
    let left = (hsl.h as i32 - 30).rem_euclid(360);
    let right = (hsl.h + 30).rem_euclid(360);

    let left = Hsl::new(left as u16, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}

pub fn monochromatic(hsl: Hsl) -> Vec<Hsl> {
    let mut monochrome: Vec<Hsl> = Vec::new();

    for l in ((hsl.l + 50)..=1000).step_by(50) {
        monochrome.push(Hsl::new(hsl.h, hsl.s, l));
    }

    for l in (50..hsl.l).rev().step_by(50) {
        monochrome.push(Hsl::new(hsl.h, hsl.s, l));
    }

    monochrome
}

pub fn gradient(hsl: Hsl, hsl2: Hsl, num: u32) -> Vec<Hsl> {
    assert!(hsl.h <= 360 && hsl2.h <= 360);
    assert!(hsl.s <= 1000 && hsl2.s <= 1000);
    assert!(hsl.l <= 1000 && hsl2.l <= 1000);
    assert!(num > 1);

    let num = num as i32;
    let mut gradient: Vec<Hsl> = Vec::with_capacity(num as usize);

    let hue = if hsl.s == 0 {
        hsl2.h as i32
    } else {
        hsl.h as i32
    };

    let mut h = hue;
    let mut s = hsl.s as i32;
    let mut l = hsl.l as i32;

    let end_h = hsl2.h as i32;
    let s_interval = (hsl2.s as i32 - s) / num;
    let l_interval = (hsl2.l as i32 - l) / num;
    let h_difference = end_h - hue;

    let h_interval = if h_difference > 180 {
        (360 - h_difference) / num
    } else if h_difference < -180 {
        (360 + h_difference) / num
    } else {
        h_difference / num
    };

    for _ in 0..num {
        h = (h + h_interval).rem_euclid(360);
        s = s + s_interval;
        l = l + l_interval;

        gradient.push(Hsl::new(h as u16, s as u16, l as u16));
    }

    gradient
}

pub fn three_node_distance_rgb(rgb1: Rgb, rgb2: Rgb) -> u32 {
    let r = (rgb1.r as i32 - rgb2.r as i32).pow(2);

    let g = (rgb1.g as i32 - rgb2.g as i32).pow(2);

    let b = (rgb1.b as i32 - rgb2.b as i32).pow(2);

    let distance = r + g + b;
    distance as u32
}

pub fn n_color_average_complement(nodes: Vec<Color>) -> Color {
    let mut complements = Vec::new();
    for node in nodes {
        complements.push(complement(node.hsl));
    }
    let rgb = match complements.pop() {
        Some(val) => val.encode().get_rgb(),
        None => panic!("no colors to complement"),
    };
    let mut r = rgb.r as u32;
    let mut g = rgb.g as u32;
    let mut b = rgb.b as u32;
    for complement in complements {
        let rgb = complement.encode().get_rgb();
        r = (r + rgb.r as u32) / 2;
        g = (r + rgb.g as u32) / 2;
        b = (r + rgb.b as u32) / 2;
    }

    Color::new(Rgb::new(r as u8, g as u8, b as u8).encode())
}

pub fn generate_color() -> Color {
    let mut rng = rand::rng();
    let h = rng.random_range(0..361);
    let s = rng.random_range(0..1001);
    let l = rng.random_range(0..1001);
    Color::new(Hsl::new(h, s, l).encode())
}

pub fn generate_palette(current_palette: &mut Vec<Color>, num: u8) {
    let mut rng = rand::rng();
    let mut temp_palette = Vec::with_capacity(num as usize);
    for i in [0..num].iter() {
        let method = rng.random_range(0..5);
        let index = rng.random_range(0..current_palette.len());
        match method {
            0 => temp_palette.push(Color::new(complement(current_palette[index].hsl).encode())),
            1 => temp_palette.push(generate_color()),
            2 => temp_palette.push(n_color_average_complement(*current_palette))
            3 => {
                let (hsl1, hsl2) = triad(current_palette[index].hsl);
                temp_palette.push(Color::new(hsl1.encode()));
                temp_palette.push(Color::new(hsl2.encode()));
                i.skip(2);
            }
            4 => {
                let (hsl1, hsl2, hsl3) = square(current_palette[index].hsl);
                temp_palette.push(Color::new(hsl1.encode()));
                temp_palette.push(Color::new(hsl2.encode()));
                temp_palette.push(Color::new(hsl3.encode()));
                i.skip(3);
            }
            _ => panic!("generate palette generated an invalid number")
        }
    }
}

// -----------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_complement() {
        let tests: HashMap<Hsl, Hsl> = HashMap::from([
            (Hsl::new(60, 842, 319), Hsl::new(240, 842, 319)),
            (Hsl::new(250, 842, 319), Hsl::new(70, 842, 319)),
        ]);
        for (hsl, desired_result) in tests {
            println!("input: {:?}, desired_result: {:?}", hsl, desired_result);
            let result = complement(hsl);
            assert_eq!(result, desired_result);
        }
    }

    #[test]
    fn test_triad() {
        let tests: HashMap<Hsl, (Hsl, Hsl)> = HashMap::from([(
            Hsl::new(60, 842, 319),
            (Hsl::new(300, 842, 319), Hsl::new(180, 842, 319)),
        )]);
        for (hsl, desired_results) in tests {
            let (desired_result1, desired_result2) = desired_results;
            println!(
                "input: {:?}, desired_result1: {:?}, desired_result2: {:?}",
                hsl, desired_result1, desired_result2
            );
            let results = triad(hsl);
            let (result1, result2) = results;
            if !(result1 == desired_result1 && result2 == desired_result2) {
                panic!("result1: {:?}, result2: {:?}", result1, result2);
            }
        }
    }

    #[test]
    fn test_square() {
        let tests: HashMap<Hsl, (Hsl, Hsl, Hsl)> = HashMap::from([]);

        for (hsl, desired_results) in tests {
            let (desired_result1, desired_result2, desired_result3) = desired_results;
            println!(
                "input: {:?}, desired_result1: {:?}, desired_result2: {:?}, desired_result3: {:?}",
                hsl, desired_result1, desired_result2, desired_result3
            );
            let results = square(hsl);
            let (result1, result2, result3) = results;
            if !(result1 == desired_result1
                && result2 == desired_result2
                && result3 == desired_result3)
            {
                panic!(
                    "result1: {:?}, result2: {:?}, result3: {:?}",
                    result1, result2, result3
                );
            }
        }
    }

    #[test]
    fn test_analogous() {
        let tests: HashMap<Hsl, (Hsl, Hsl)> = HashMap::from([
            (
                Hsl::new(60, 842, 319),
                (Hsl::new(30, 842, 319), Hsl::new(90, 842, 319)),
            ),
            (
                Hsl::new(30, 599, 599),
                (Hsl::new(0, 599, 599), Hsl::new(60, 599, 599)),
            ),
            (
                Hsl::new(0, 100, 100),
                (Hsl::new(330, 100, 100), Hsl::new(30, 100, 100)),
            ),
        ]);

        for (hsl, desired_result) in tests {
            let (desired_left, desired_right) = desired_result;
            println!("input: {:?}", hsl);
            let (left, right) = analogous(hsl);
            if !(left == desired_left && right == desired_right) {
                panic!(
                    "desired_left: {:?}, left: {:?}, desired_right: {:?}, right: {:?}",
                    desired_left, left, desired_right, right
                );
            }
        }
    }

    #[test]
    fn test_monochromatic_basic() {
        let hsl = Hsl::new(120, 500, 300);
        let result = monochromatic(hsl);

        let expected_l: Vec<u16> = (350..=1000)
            .step_by(50)
            .chain((50..300).rev().step_by(50))
            .collect();

        assert_eq!(result.len(), expected_l.len());

        for (i, l) in expected_l.iter().enumerate() {
            assert_eq!(result[i].h, 120);
            assert_eq!(result[i].s, 500);
            assert_eq!(result[i].l, *l);
        }
    }

    #[test]
    fn test_monochromatic_l_near_bounds() {
        let hsl = Hsl::new(10, 200, 50);
        let result = monochromatic(hsl);

        let expected: Vec<u16> = (100..=1000).step_by(50).collect();
        assert_eq!(result.iter().map(|c| c.l).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_gradient_basic_linear() {
        let hsl1 = Hsl::new(100, 200, 300);
        let hsl2 = Hsl::new(200, 400, 500);

        let result = gradient(hsl1, hsl2, 5);
        assert_eq!(result.len(), 5);

        let s_interval = (400 - 200) / 5;
        let l_interval = (500 - 300) / 5;
        let h_interval = (200 - 100) / 5;

        let mut expected_h = 100;
        let mut expected_s = 200;
        let mut expected_l = 300;

        for (i, color) in result.iter().enumerate() {
            expected_h = (expected_h + h_interval) % 360;
            expected_s += s_interval;
            expected_l += l_interval;

            assert_eq!(color.h, expected_h as u16, "hue mismatch at step {i}");
            assert_eq!(color.s, expected_s as u16, "hue mismatch at step {i}");
            assert_eq!(color.l, expected_l as u16, "hue mismatch at step {i}");
        }
    }

    #[test]
    fn test_gradient_hue_wrapping_forward() {
        let hsl1 = Hsl::new(350, 500, 500);
        let hsl2 = Hsl::new(250, 500, 500);

        let result = gradient(hsl1, hsl2, 5);
        let h_interval = (250 - 350) / 5;

        let mut expected = 350;
        for (i, c) in result.iter().enumerate() {
            expected = (expected + h_interval) % 360;
            assert_eq!(c.h, expected as u16, "hue wrap mismatch at step {i}");
        }

        let hsl1 = Hsl::new(350, 500, 500);
        let hsl2 = Hsl::new(150, 500, 500);

        let result = gradient(hsl1, hsl2, 5);
        let h_interval = (360 + (150 - 350)) / 5;

        let mut expected = 350;
        for (i, c) in result.iter().enumerate() {
            expected = (expected + h_interval) % 360;
            assert_eq!(c.h, expected as u16, "hue wrap mismatch at step {i}");
        }
    }

    #[test]
    fn test_gradient_hue_wrapping_backward() {
        let hsl1 = Hsl::new(10, 500, 500);
        let hsl2 = Hsl::new(270, 500, 500);

        let result = gradient(hsl1, hsl2, 5);
        let h_interval = (360 + (10 - 270)) / 5;

        let mut expected = 10;
        for (i, c) in result.iter().enumerate() {
            expected = (expected + h_interval) % 360;
            assert_eq!(
                c.h, expected as u16,
                "hue backward wrap mismatch at step {i}"
            );
        }
    }

    #[test]
    fn test_gradient_saturation_zero_uses_second_hue() {
        let hsl1 = Hsl::new(180, 0, 300);
        let hsl2 = Hsl::new(90, 500, 900);

        let result = gradient(hsl1, hsl2, 4);

        let expected = 90;

        for c in result {
            assert_eq!(c.h, expected as u16);
        }
    }

    #[test]
    #[should_panic]
    fn test_gradient_asserts() {
        let hsl1 = Hsl::new(400, 300, 300);
        let hsl2 = Hsl::new(100, 300, 300);

        gradient(hsl1, hsl2, 5);
    }
}
