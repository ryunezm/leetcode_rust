/*
3019. Number of Changing Keys
https://leetcode.com/contest/weekly-contest-382/problems/number-of-changing-keys


You are given a 0-indexed string s typed by a user. Changing a key is defined as using a key
different from the last used key. For example, s = "ab" has a change of a key while s = "bBBb"
does not have any.

Return the number of times the user had to change the key.


Note: Modifiers like shift or caps lock won't be counted in changing the key that is if a user
typed the letter 'a' and then the letter 'A' then it will not be considered as a changing of key.

*/

pub fn count_key_changes(s: String) -> i32 {
    let mut cont = 0;
    for i in 1..s.len()  {
        let current_char = s.chars().nth(i).unwrap() as u8;
        let previous_char = s.chars().nth(i-1).unwrap() as u8;

        if (current_char != previous_char) && (current_char + 32 != previous_char) && (current_char != previous_char + 32) {
            cont+=1;
            // println!("{}. Current char: {} vs Previous char: {}", cont, current_char, previous_char)
        }
    }

    cont
}