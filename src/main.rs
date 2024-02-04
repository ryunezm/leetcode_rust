use crate::easy::_1_two_sum::two_sum;
use crate::easy::_9_palindrome_number::is_palindrome;
use crate::easy::_13_roman_to_integer::roman_to_int;
use crate::easy::_35_search_insert_position::search_insert;
use crate::easy::_118_pascal_triangle::generate;
use crate::easy::_342_power_of_four::is_power_of_four;
use crate::easy::_2591_distribute_money_to_maximum_children::dist_money;
use crate::easy::_3019_number_of_changing_keys::count_key_changes;
use crate::easy::_3024_type_of_triangle_ii::triangle_type;
use crate::easy::_3028_ant_on_the_boundary::return_to_boundary_count;

use crate::medium::divide_two_integers::divide;

mod easy;
mod medium;


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
    let a = 3;//2147483647;
    let b = 3;
    let c = divide(a,b);
    println!("{}/{} = {}", a, b, c);
    //println!("{}", i32::MAX/i32::MAX)

    //number_of_changing_keys
    let string1: String = "aAbBcC".to_string();
    println!("{}", count_key_changes(string1));

    //search_insert_position
    let nums: Vec<i32> = Vec::from([1, 3, 5, 6]);
    let target = 7;
    search_insert(nums, target);

    //type_of_triangle_ii
    let triangle: Vec<i32> = Vec::from([10, 4, 6]);
    println!("Type of triangle: {}", triangle_type(triangle));

    //ant_on_the_boundary
    let ants = Vec::from([2,3,-5]);
    println!("Ant returning to boundary: {}", return_to_boundary_count(ants));
}
