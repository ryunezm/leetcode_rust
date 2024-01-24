use crate::easy::palindrome_number::is_palindrome;
use crate::easy::roman_to_integer::roman_to_int;
use crate::easy::two_sum::two_sum;
mod easy;

mod medium;

fn main() {
    let ans: Vec<i32> = vec![1, 2, 3, 9, 9, 0, 4];
    let vector:Vec<i32> = two_sum(ans, 19);
    println!("{:?}", vector);

    let num_pal = 1410110141;
    let num_pal_bool = is_palindrome(num_pal);
    println!("Is {} a palindrome? {}", num_pal, num_pal_bool);

    let num_rom: String = String::from("VIIII");
    let num_int = roman_to_int(num_rom);
    println!("{}", num_int)

}
