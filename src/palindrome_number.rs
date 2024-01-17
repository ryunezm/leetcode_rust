/*
Given an integer x, return true if x is a palindrome, and false otherwise.
https://leetcode.com/problems/palindrome-number/
*/

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 { false } else if x >= 0 && x < 10 { true } else {
        let mut aux: i32 = x;
        let mut inverted: i32 = 0;
        while aux>0 {
            inverted = inverted * 10 + aux%10;
            aux /= 10;
        }
        inverted == x
    }
}
