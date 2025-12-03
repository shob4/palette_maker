use crate::color_spaces::{Hsl, Rgb};

// TODO
// [] tests for analagous
// [] tests for monochromatic
// [] change to color?
// [] return passed hsl/rgb?
// [] figure out how to make gradients
//  [] figure out how to go arounda gradient

pub fn complement(hsl: Hsl) -> Hsl {
    let new_h = (hsl.h + 180) % 360;
    Hsl::new(new_h, hsl.s, hsl.l)
}

pub fn triad(hsl: Hsl) -> (Hsl, Hsl) {
    let left = (hsl.h + 240) % 360;
    let right = (hsl.h + 120) % 360;

    let left = Hsl::new(left, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}

pub fn square(hsl: Hsl) -> (Hsl, Hsl, Hsl) {
    let left = (hsl.h + 90) % 360;
    let middle = (hsl.h + 180) % 360;
    let right = (hsl.h + 270) % 360;

    let left = Hsl::new(left, hsl.s, hsl.l);
    let middle = Hsl::new(middle, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, middle, right)
}

pub fn analogous(hsl: Hsl) -> (Hsl, Hsl) {
    let left = (hsl.h + 30) % 360;
    let right = (hsl.h + 330) % 360;

    let left = Hsl::new(left, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}

pub fn monochromatic(hsl: Hsl) -> Vec<Hsl> {
    let mut monochrome: Vec<Hsl> = Vec::new();
    let current_l = hsl.l;

    while current_l + 50 <= 1000 {
        let current_l = current_l + 50;
        monochrome.push(Hsl::new(hsl.h, hsl.s, current_l));
    }

    let current_l = hsl.l;
    while current_l - 50 != 0 {
        let current_l = current_l - 50;
        monochrome.push(Hsl::new(hsl.h, hsl.s, current_l));
    }

    return monochrome;
}

pub fn gradient(hsl: Hsl, hsl2: Hsl, num: u32) -> Vec<Hsl> {
    assert!(hsl.h <= 360 && hsl2.h <= 360);
    assert!(hsl.s <= 1000 && hsl2.s <= 1000);
    assert!(hsl.l <= 1000 && hsl2.l <= 1000);
    assert!(num > 1);

    let hue: i32;

    if hsl.s == 0 {
        hue = hsl2.h as i32;
    } else {
        hue = hsl.h as i32;
    }

    let mut gradient: Vec<Hsl> = Vec::new();
    let destination_h = hsl2.h as i32;
    let s_interval = (destination_h - hue) / num as i32;
    let l_interval = (destination_h - hue) / num as i32;
    let h_difference = destination_h - hue;
    let h_interval: i32;

    if h_difference > 180 {
        h_interval = (360 - h_difference) / num as i32;
    } else if h_difference < -180 {
        h_interval = (360 + h_difference) / num as i32;
    } else {
        h_interval = h_difference / num as i32;
    }

    let current_h = hue;
    let current_s = hsl.s as i32;
    let current_l = hsl.l as i32;

    for _ in 0..num {
        let current_h = if current_h + h_interval < 0 {
            current_h + h_interval + 360
        } else {
            current_h + h_interval % 360
        };
        let current_s = current_s + s_interval;
        let current_l = current_l + l_interval;
        gradient.push(Hsl::new(
            current_h as u16,
            current_s as u16,
            current_l as u16,
        ));
    }

    return gradient;
}

pub fn three_node_distance_rgb(rgb1: Rgb, rgb2: Rgb) -> u32 {
    let r = match (rgb1.r as i32 - rgb2.r as i32).checked_pow(2) {
        Some(val) => val.abs(),
        None => todo!("figure out how to handle overflow"),
    };
    let g = match (rgb1.g as i32 - rgb2.g as i32).checked_pow(2) {
        Some(val) => val.abs(),
        None => todo!("figure out how to handle overflow"),
    };
    let b = match (rgb1.b as i32 - rgb2.b as i32).checked_pow(2) {
        Some(val) => val.abs(),
        None => todo!("figure out how to handle overflow"),
    };
    let distance = r + g + b;
    return distance as u32;
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
    fn test_monochromatic() {}

    #[test]
    fn test_gradient() {}
}
