/*
387. First Unique Character in a String
https://leetcode.com/problems/first-unique-character-in-a-string
#Hash_Table #String #Queue #Counting


Given a string s, find the first non-repeating character in it and return its index.
If it does not exist, return


Constraints:
-> 1 <= s.length <= 10^5
-> s consists of only lowercase English letters.

*/


pub fn first_uniq_char(s: String) -> i32 {

    let mut count_char = std::collections::HashMap::new();

    for (index,char) in s.chars().enumerate() { // Count the frequency of each character.
        let count = count_char.entry(char).or_insert(0);
        *count += 1;
    }

    for (index, char) in s.chars().enumerate(){ // Find the first and unique character.
        if *count_char.get(&char).unwrap() == 1 {
            return index as i32;
        }
    }

    -1
}

/*
    --- --- --- TIME LIMIT EXCEEDED --- --- ---
    let mut skip_list = Vec::new();

    for i in 0..s.len() {
        if !skip_list.contains(&i) {
            let mut is_unique = true;

            for j in i + 1..s.len() {
                if s.chars().nth(i) == s.chars().nth(j) {
                    skip_list.push(i);
                    skip_list.push(j);
                    is_unique = false;
                    //break;
                }
            }

            if is_unique { return i as i32; } // Found the first unique character, return its index.
        }
    }

    -1 // No unique character found.
    --- --- --- TIME LIMIT EXCEEDED --- --- ---
*/
