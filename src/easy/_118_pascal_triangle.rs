/*
118. Pascal's Triangle
https://leetcode.com/problems/pascals-triangle
#Array #Dinamic_Programming

Given an integer numRows, return the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

        1
       1 1
      1 2 1
     1 3 3 1
    1 4 6 4 1


Constraints:
-> 1 <= numRows <= 30

*/

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    fn combination(n: i32, r: i32) -> i32 {
        (1..=r).fold(1, |acc, i| acc * (n - i + 1) / i)

        /* Same as:
        let mut result = 1;
        for i in 1..=r {
               result = result * (n - i + 1) / i;
        }
        result
        */
    }

    let mut matrix: Vec<Vec<i32>> = vec![vec![]; num_rows as usize];

    for i in 0..num_rows {
        for j in 0..i + 1 {
            matrix[i as usize].push(combination(i, j));
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_case_1() {
        let num_rows = 1;
        assert_eq!(generate(num_rows), vec![[1]])
    }

    #[test]
    fn generate_case_2() {
        let num_rows = 5;
        assert_eq!(generate(num_rows), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]])
    }
}

