/*
14. Longest Common Prefix
https://leetcode.com/problems/longest-common-prefix
#String #Trie


Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".


Constraints:
1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters.

*/
/*
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ans_limit = strs[0].len();
    let mut ans = String::from("");
    let array_size = strs.len();

    for i in 0..array_size {
        if strs[i].len() <= ans_limit { ans_limit = strs[i].len(); }
    }

    for j in 0 .. ans_limit {
        for k in 0.. array_size {
            if strs[0][k] != strs[j][k] { break; }
        }
        ans.push(strs[0][j])
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcp_case_1() {
        let strs: Vec<String> = Vec::from(["flower".to_string(), "flow".to_string(), "flight".to_string()]);
        assert_eq!(longest_common_prefix(strs), "fl");
    }

    #[test]
    fn lcp_case_2() {
        let strs: Vec<String> = Vec::from(["dog".to_string(), "racecar".to_string(), "car".to_string()]);
        assert_eq!(longest_common_prefix(strs), "");
    }
}

*/