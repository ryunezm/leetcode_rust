use crate::two_sum::two_sum;

mod two_sum;

fn main() {
    let ans: Vec<i32> = vec![1, 2, 3, 9, 9, 0, 4];
    let vector:Vec<i32> = two_sum(ans, 19);
    println!("{:?}", vector)


}
