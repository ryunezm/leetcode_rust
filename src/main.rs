mod easy;
mod medium;
use crate::easy::palindrome_number::is_palindrome;
use crate::easy::roman_to_integer::roman_to_int;
use crate::easy::two_sum::two_sum;
use crate::easy::pascal_triangle::generate;
use crate::easy::power_of_four::is_power_of_four;
fn main() {

    //two_sum
    let ans: Vec<i32> = vec![1, 2, 3, 9, 9, 0, 4];
    let vector:Vec<i32> = two_sum(ans, 19);
    println!("{:?}", vector);

    //palindrome_numbers
    let num_pal = 1410110141;
    let num_pal_bool = is_palindrome(num_pal);
    println!("Is {} a palindrome? {}", num_pal, num_pal_bool);

    //roman_to_integers
    let num_rom: String = String::from("VIII");
    let num_int = roman_to_int(num_rom);
    println!("{}", num_int);

    //pascal_triangle
    let num_rows = 10;
    let triangle = generate(num_rows);
    println!("{:?}", triangle);

    //power_of_four
    let n_pof = 64;
    let pof = is_power_of_four(n_pof);
    println!("{} is a power of four? {}",n_pof, pof)
}
