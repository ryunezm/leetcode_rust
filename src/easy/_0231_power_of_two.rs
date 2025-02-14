/*
231. Power of Two
https://leetcode.com/problems/power-of-two
#Math #BitManipulation #Recursion


Given an integer n, return true if it is a power of two. Otherwise, return false.

An integer n is a power of two, if there exists an integer x such that n == 2^x.


Constraints:
-> -2^31 <= n <= 2^31 - 1
*/

pub fn is_power_of_two(n: i32) -> bool {
    if n < 0 {
        false
    } else if n == 1 {
        true
    } else {
        let n_binary: Vec<u32> = format!("{:b}", n)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        if n > 1 && n_binary.len() >= 1 && n_binary.iter().sum::<u32>() == 1 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_power_of_two_case_0() {
        let n = 0;
        assert_eq!(is_power_of_two(n), false)
    }

    #[test]
    fn is_power_of_two_case_1() {
        let n = 1;
        assert_eq!(is_power_of_two(n), true)
    }

    #[test]
    fn is_power_of_two_case_2() {
        let n = 16;
        assert_eq!(is_power_of_two(n), true)
    }

    #[test]
    fn is_power_of_two_case_3() {
        let n = 3;
        assert_eq!(is_power_of_two(n), false)
    }

    #[test]
    fn is_power_of_two_case_4() {
        let n = 32;
        assert_eq!(is_power_of_two(n), true)
    }

    #[test]
    fn is_power_of_two_case_5() {
        let n = 2097152;
        assert_eq!(is_power_of_two(n), true)
    }

    #[test]
    fn is_power_of_two_case_6() {
        let n = 2097153;
        assert_eq!(is_power_of_two(n), false)
    }

    #[test]
    fn is_power_of_two_case_7() {
        let n = 1073741824;
        assert_eq!(is_power_of_two(n), true)
    }

    #[test]
    fn is_power_of_two_case_8() {
        let n = 1073741825;
        assert_eq!(is_power_of_two(n), false)
    }

    #[test]
    fn is_power_of_two_case_9() {
        let n = -2147483648;
        assert_eq!(is_power_of_two(n), false)
    }

    #[test]
    fn is_power_of_two_case_10() {
        let n = -2147483646;
        assert_eq!(is_power_of_two(n), false)
    }
}
