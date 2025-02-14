/*
9. Palindrome Number
https://leetcode.com/problems/palindrome-number
#Math

Given an integer x, return true if x is a palindrome, and false otherwise.


Constraints:
-> -2^31 <= x <= 2^31 - 1


Follow up: Could you solve it without converting the integer to a string?

*/

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        false
    } else if x >= 0 && x < 10 {
        true
    } else {
        let mut aux: i32 = x;
        let mut inverted: i32 = 0;
        while aux > 0 {
            inverted = inverted * 10 + aux % 10;
            aux /= 10;
        }
        inverted == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_case_1() {
        let x = 121;
        assert_eq!(is_palindrome(x), true)
    }

    #[test]
    fn is_palindrome_case_2() {
        let x = -121;
        assert_eq!(is_palindrome(x), false)
    }

    #[test]
    fn is_palindrome_case_3() {
        let x = 10;
        assert_eq!(is_palindrome(x), false)
    }

    #[test]
    fn is_palindrome_case_4() {
        let x = 222242222;
        assert_eq!(is_palindrome(x), true)
    }
}
