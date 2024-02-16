/*
2108. Find First Palindromic String in the Array
https://leetcode.com/problems/find-first-palindromic-string-in-the-array
#Array #TwoPointers #String


Given an array of strings words, return the first palindromic string in the array. If there is no
such string, return an empty string "".

A string is palindromic if it reads the same forward and backward.


Constraints:
-> 1 <= words.length <= 100
-> 1 <= words[i].length <= 100
-> words[i] consists only of lowercase English letters.
*/

pub fn first_palindrome(words: Vec<String>) -> String {
    for word in words {
        if word.chars().eq(word.chars().rev()){
            return word
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_palindrome() {
        let words = vec![
            String::from("hello"),
            String::from("level"),
            String::from("world"),
            String::from("radar"),
        ];
        assert_eq!(first_palindrome(words), String::from("level"));
    }

    #[test]
    fn test_first_palindrome_empty_input() {
        let words: Vec<String> = vec![];
        assert_eq!(first_palindrome(words), String::new());
    }

    #[test]
    fn test_first_palindrome_no_palindrome() {
        let words = vec![
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
        ];
        assert_eq!(first_palindrome(words), String::new());
    }
}