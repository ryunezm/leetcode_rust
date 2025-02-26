/*
13. Roman to Integer
https://leetcode.com/problems/roman-to-integer
#Hash_Table #Math #String

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII,
which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral
for four is not IIII. Instead, the number four is written as IV. Because the one is before the
five we subtract it making four. The same principle applies to the number nine, which is written
as IX. There are six instances where subtraction is used:

I can be placed before V (5) and X (10) to make 4 and 9.
X can be placed before L (50) and C (100) to make 40 and 90.
C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer.

Constraints:
-> 1 <= s.length <= 15
-> s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
-> It is guaranteed that s is a valid roman numeral in the range [1, 3999].
*/

pub fn roman_to_int(s: String) -> i32 {
    let s_as_vec: Vec<char> = s.chars().collect();
    let mut result = 0;
    let mut prev_value = 0;

    for i in (0..s_as_vec.len()).rev() {
        let value = match s_as_vec[i] {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid!"),
        };

        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }

        prev_value = value;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_roman_to_int_case_1() {
        let s = String::from("III");
        assert_eq!(roman_to_int(s), 3)
    }

    #[test]
    fn test_roman_to_int_case_2() {
        let s = String::from("LVIII");
        assert_eq!(roman_to_int(s), 58)
    }

    #[test]
    fn test_roman_to_int_case_3() {
        let s = String::from("MCMXCIV");
        assert_eq!(roman_to_int(s), 1994)
    }

    #[test]
    fn test_roman_to_int_case_4() {
        let s = String::from("MMMCMXCIX");
        assert_eq!(roman_to_int(s), 3999)
    }

    #[test]
    fn test_roman_to_int_case_5() {
        let s = String::from("CMXCIX");
        assert_eq!(roman_to_int(s), 999)
    }
}
