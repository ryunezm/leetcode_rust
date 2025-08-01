/*
728. Self Dividing Numbers
https://leetcode.com/problems/self-dividing-numbers


A self-dividing number is a number that is divisible by every digit it contains.

For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.
A self-dividing number is not allowed to contain the digit zero.

Given two integers left and right, return a list of all the self-dividing numbers in the range
[left, right] (both inclusive).


Constraints:
1 <= left <= right <= 10^4
*/

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut ans = vec![];

    for i in left..right + 1 {
        if has_zero(i) {
            continue;
        }

        let mut sum = 0;
        for x in to_digits(i) {
            sum = sum + i % x;
        }

        if sum == 0 {
            ans.push(i);
        }
    }

    return ans;

    fn has_zero(num: i32) -> bool {
        num.to_string().contains('0')
    }

    fn to_digits(num: i32) -> Vec<i32> {
        num.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdn_case_1_basic() {
        let left = 10;
        let right = 20;
        assert_eq!(self_dividing_numbers(left, right), [11, 12, 15]);
    }

    #[test]
    fn sdn_case_2_digits() {
        let left = 1;
        let right = 9;
        assert_eq!(
            self_dividing_numbers(left, right),
            [1, 2, 3, 4, 5, 6, 7, 8, 9]
        )
    }

    #[test]
    fn sdn_case_3_one() {
        let left = 11;
        let right = 11;
        assert_eq!(self_dividing_numbers(left, right), [11]);
    }

    #[test]
    fn sdn_case_4_zero() {
        let left = 49;
        let right = 50;
        assert_eq!(self_dividing_numbers(left, right), []);
    }

    #[test]
    fn snd_case6_leetcode_1() {
        let left = 1;
        let right = 22;
        assert_eq!(
            self_dividing_numbers(left, right),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        )
    }

    #[test]
    fn sdn_case_6_leetcode_2() {
        let left = 47;
        let right = 85;
        assert_eq!(self_dividing_numbers(left, right), [48, 55, 66, 77])
    }
}
