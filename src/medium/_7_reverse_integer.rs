/*
7. Reverse Integer
https://leetcode.com/problems/reverse-integer
#Math


Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value
to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

Constraints:
-> -2^31 <= x <= 2^31 - 1 or -2147483648 <= x <= 2147483647

*/

pub fn reverse(x: i32) -> i32 {
    if x == 0 || x == i32::MAX || x == i32::MIN { return 0; }
    let digits: i32 = count_digits(x);
    let pot: Vec<i32> = pow_10(digits);
    let pot_reversed: Vec<i32> = pot.iter().rev().cloned().collect();
    let inv_digits: Vec<i32> = extract_digits(x);
    let mut result: i32 = 0;

    for i in 0..digits {
        let product = inv_digits[i as usize].checked_mul(pot_reversed[i as usize + 1]);

        let product = match product {
            Some(p) => p,
            None => return 0,
        };

        if let Some(new_result) = result.checked_add(product) { result = new_result; } else { return 0; }
    }

    return if x < 0 {
        let another_product = result.checked_mul(-1);

        let another_product = match another_product {
            Some(p) => p,
            None => return 0,
        };

        another_product
    } else { result };

    fn count_digits(mut n: i32) -> i32 {
        n = n.abs();
        (n as f64).log10().floor() as i32 + 1
    }

    fn extract_digits(n: i32) -> Vec<i32> {
        let n = n.abs();
        let digits: i32 = count_digits(n);
        let pot_10: Vec<i32> = pow_10(digits);
        let mut aux_cal: Vec<i32> = vec![0; digits as usize];

        for i in 0..digits as usize {
            if i == 0 { aux_cal[i] = (n % pot_10[i + 1]) / pot_10[i]; }
            else if i>8  { if n<2000000000 { aux_cal[i] = 1; } else { aux_cal[i] = 2} }
            else { aux_cal[i] = (n % pot_10[i + 1] - n % pot_10[i]) / pot_10[i]; }
        }

        aux_cal
    }

    fn pow_10(i: i32) -> Vec<i32> {
        let i = (i + 1).abs();
        let mut vec: Vec<i32> = vec![1; i as usize];
        for j in 1..i as usize {
            vec[j] = (vec[j - 1] as i32).checked_mul(10_i32).unwrap_or(1);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let n = i32::MAX;
        assert_eq!(reverse(n), 0)
    }

    #[test]
    fn test_min() {
        let n = i32::MIN;
        assert_eq!(reverse(n), 0)
    }

    #[test]
    fn test_overflow_case_1() {
        let n = -2147483647;
        assert_eq!(reverse(n), 0)
    }

    #[test]
    fn test_overflow_case_2() {
        let n = 123;
        assert_eq!(reverse(n), 321)
    }

    #[test]
    fn test_overflow_case_3() {
        let n = -20;
        assert_eq!(reverse(n), -2)
    }

    #[test]
    fn test_overflow_case_4() {
        let n = 1534236469;
        assert_eq!(reverse(n), 0)
    }

    #[test]
    fn test_overflow_case_5() {
        let n: i32 = 2147483642;
        assert_eq!(reverse(n), 0)
    }

    #[test]
    fn test_overflow_case_6() {
        let n: i32 = -2147483412;
        assert_eq!(reverse(n), -2143847412)
    }

    #[test]
    fn test_overflow_case_7() {
        let n: i32 = -2147483411;
        assert_eq!(reverse(n), -1143847412)
    }
}