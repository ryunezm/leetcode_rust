/*
2520. Count the Digits That Divide a Number
https://leetcode.com/problems/count-the-digits-that-divide-a-number
#Math


Given an integer num, return the number of digits in num that divide num.

An integer val divides nums if nums % val == 0.


Constraints:
-> 1 <= num <= 10^9
-> num does not contain 0 as one of its digits.

*/

pub fn count_digits(num: i32) -> i32 {
    let mut cont = 0;
    let digits = to_digits(num);

    for i in 0..digits.len() {
        if num % digits[i] == 0 {
            cont = cont + 1;
        }
    }

    return cont;

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
    fn cn_case_1() {
        let num = 111;
        assert_eq!(count_digits(num), 3);
    }

    #[test]
    fn cn_case_2() {
        let num = 121;
        assert_eq!(count_digits(num), 2);
    }

    #[test]
    fn cn_case_3() {
        let num = 1248;
        assert_eq!(count_digits(num), 4);
    }
}
