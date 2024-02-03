mod easy;
mod medium;

use crate::easy::palindrome_number::is_palindrome;
use crate::easy::roman_to_integer::roman_to_int;
use crate::easy::two_sum::two_sum;
use crate::easy::pascal_triangle::generate;
use crate::easy::power_of_four::is_power_of_four;
use crate::easy::distribute_money_to_maximum_children::dist_money;
use crate::easy::number_of_changing_keys::count_key_changes;

use crate::medium::divide_two_integers::divide;
fn main() {


    //two_sum
    let ans: Vec<i32> = vec![1, 2, 3, 9, 9, 0, 4];
    let vector:Vec<i32> = two_sum(ans.clone(), 19);
    println!("Two_sum: {:?} {:?}", ans, vector);

    //palindrome_numbers
    let num_pal = 1410110141;
    let num_pal_bool = is_palindrome(num_pal);
    println!("Palindrome numbers: Is {} a palindrome? {}", num_pal, num_pal_bool);

    //roman_to_integers
    let num_rom: String = String::from("VIII");
    let num_int = roman_to_int(num_rom.clone());
    println!("Roman to integers: {} -> {}", num_rom, num_int);

    //pascal_triangle
    let num_rows = 4;
    let triangle = generate(num_rows);
    println!("{:?}", triangle);

    //power_of_four
    let n_pof = 64;
    let pof = is_power_of_four(n_pof);
    println!("{} is a power of four? {}",n_pof, pof);

    //distribute_money_to_maximum_children
    let money = 200;
    let children = 4;
    let max_children8 = dist_money(money, children);
    println!("${} distributed among {} children: {}", money, children, max_children8);


    //divide_two_integers
    let a = 2147483647;
    let b = 3;
    let c = divide(a,b);
    println!("{}/{} = {}", a, b, c);
    //println!("{}", i32::MAX/i32::MAX)

    //number_of_changing_keys
    let string1: String = "aAbBcC".to_string();
    println!("{}", count_key_changes(string1));


}
