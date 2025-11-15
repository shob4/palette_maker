use crate::color_spaces::Hsl;

// TODO
// [] add square
// [] add analagous
// [] add monochromatic

pub fn complement(hsl: Hsl) -> Hsl {
    let new_h = (hsl.h + 180) % 360;
    Hsl::new(new_h, hsl.s, hsl.l)
}

pub fn triad(hsl: Hsl) -> (Hsl, Hsl) {
    let left = hsl.h - 60;
    let right = hsl.h + 60;

    let left = Hsl::new(left, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}
pub fn square(hsl: Hsl) {}
pub fn analogous(hsl: Hsl) {}
pub fn monochromatic(hsl: Hsl) {}

// -----------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_complement() {
        let tests: HashMap<Hsl, Hsl> = HashMap::from([]);
        for (hsl, desired_result) in tests {
            println!("input: {:?}, desired_result: {:?}", hsl, desired_result);
            let result = complement(hsl);
            assert_eq!(result, desired_result);
        }
    }

    #[test]
    fn test_triad() {
        let tests: HashMap<Hsl, (Hsl, Hsl)> = HashMap::from([]);
        for (hsl, desired_results) in tests {
            let (desired_result1, desired_result2) = desired_results;
            println!(
                "input: {:?}, desired_result1: {:?}, desired_result2: {:?}",
                hsl, desired_result1, desired_result2
            );
            let results = triad(hsl);
            let (result1, result2) = results;
            if !((result1 == desired_result1 || result1 == desired_result2)
                && (result2 == desired_result2 || result2 == desired_result1))
            {
                panic!("result1: {:?}, result2: {:?}", result1, result2);
            }
        }
    }
}
