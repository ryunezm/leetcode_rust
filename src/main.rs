use crate::two_sum::two_sum;
use crate::palindrome_number::is_palindrome;
mod two_sum;
mod palindrome_number;

fn main() {
    let ans: Vec<i32> = vec![1, 2, 3, 9, 9, 0, 4];
    let vector:Vec<i32> = two_sum(ans, 19);
    println!("{:?}", vector);

    let a = is_palindrome(1410110141);
    println!("{}", a);
}
