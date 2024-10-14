/*
https://leetcode.com/problems/happy-number
#Math #HashTable #TwoPointers


Write an algorithm to determine if a number n is happy.

A happy number is a number defined by the following process:

-> Starting with any positive integer, replace the number by the sum of the squares of its digits.

-> Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a
cycle which does not include 1.

-> Those numbers for which this process ends in 1 are happy.

Return true if n is a happy number, and false if not.

Constraints:
1 <= n <= 2^31 - 1
*/

pub fn is_happy(n: i32) -> bool {
    let mut set_cache_numbers: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let mut set_cache_square: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut aux: i32;
    let mut number: i32 = n;
    set_cache_numbers.insert(n);
    if n == 1 { return true }

    loop {
        aux = sum_digits_pow_2(&extract_digits(number), &mut set_cache_square);
        if aux == 1 { return true }
        if set_cache_numbers.contains(&aux) { return false; }
        set_cache_numbers.insert(aux);
        number = aux;
    }


    fn count_digits(mut n: i32) -> i32 {
        n = n.abs();
        (n as f64).log10().floor() as i32 + 1
    }

    fn pow_10(i: i32) -> Vec<i32> {
        let i = (i + 1).abs() as usize;
        let mut vec: Vec<i32> = vec![1; i];
        for j in 1..i {
            vec[j] = vec[j - 1].checked_mul(10).unwrap_or(1);
        }
        vec
    }

    fn extract_digits(n: i32) -> Vec<i32> {
        let n: i32 = n.abs();
        let digits: i32 = count_digits(n);
        let pot_10: Vec<i32> = pow_10(digits);
        let mut aux_cal: Vec<i32> = vec![0; digits as usize];

        for i in 0..digits as usize {
            if i == 0 { aux_cal[i] = (n % pot_10[i + 1]) / pot_10[i]; } else if i > 8 { if n < 2000000000 { aux_cal[i] = 1; } else { aux_cal[i] = 2 } } else { aux_cal[i] = (n % pot_10[i + 1] - n % pot_10[i]) / pot_10[i]; }
        }

        aux_cal
    }

    fn sum_digits_pow_2(vec: &[i32], square_cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        vec
            .iter()
            .map(|&x| { *square_cache.entry(x).or_insert_with(|| x * x) })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_happy_case_1() {
        assert_eq!(is_happy(1), true);
    }

    #[test]
    fn is_happy_case_2() {
        assert_eq!(is_happy(2), false);
    }
    #[test]
    fn is_happy_case_3() {
        assert_eq!(is_happy(4), false);
    }

    #[test]
    fn is_happy_case_4() {
        assert_eq!(is_happy(13), true);
    }

    #[test]
    fn is_happy_case_5() {
        assert_eq!(is_happy(16), false);
    }

    #[test]
    fn is_happy_case_6(){
        assert_eq!(is_happy(97), true)
    }
}