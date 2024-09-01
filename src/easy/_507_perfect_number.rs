/*
507. Perfect Number
https://leetcode.com/problems/perfect-number
#Math


A perfect number is a positive integer that is equal to the sum of its positive divisors,
excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.

Given an integer n, return true if n is a perfect number, otherwise return false.


Constraints:
-> 1 <= num <= 10^8
*/

pub fn check_perfect_number(num: i32) -> bool {
    let mut suma = 0;
    for i in 1..((num / 2)+1) { if num % i == 0 { suma = suma + i; } }
    if suma == num { return true; }
    false
}

/*
It's better using binary.
    6       ->  110             (2-1)
    28      ->  11100           (3-2)
    496     ->  111110000       (5-4)
    8128    ->  1111111000000   (7-6)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_perfect_number_case_1() {
        let n = 1;
        assert_eq!(check_perfect_number(n), false);
    }

    #[test]
    fn check_perfect_number_case_2() {
        let n = 2;
        assert_eq!(check_perfect_number(n), false);
    }

    #[test]
    fn check_perfect_number_case_3() {
        let n = 3;
        assert_eq!(check_perfect_number(n), false);
    }

    #[test]
    fn check_perfect_number_case_4() {
        let n = 6;
        assert_eq!(check_perfect_number(n), true);
    }

    #[test]
    fn check_perfect_number_case_5() {
        let n = 28;
        assert_eq!(check_perfect_number(n), true);
    }

    #[test]
    fn check_perfect_number_case_6() {
        let n = 496;
        assert_eq!(check_perfect_number(n), true);
    }

    #[test]
    fn check_perfect_number_case_7() {
        let n = 8128;
        assert_eq!(check_perfect_number(n), true);
    }

}