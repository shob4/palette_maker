use crate::color_spaces::{Hsl, Rgb};

// TODO
// [] add analagous
// [] add monochromatic

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

pub fn analogous(hsl: Hsl) {}

pub fn monochromatic(hsl: Hsl) {}

pub fn three_node_distance(rgb1: Rgb, rgb2: Rgb) -> u32 {
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
}
