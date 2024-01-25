/*
2591. Distribute Money to Maximum Children
https://leetcode.com/problems/distribute-money-to-maximum-children/

You are given an integer money denoting the amount of money (in dollars) that you have and another
integer children denoting the number of children that you must distribute the money to.

You have to distribute the money according to the following rules:

- All money must be distributed.
- Everyone must receive at least 1 dollar.
- Nobody receives 4 dollars.

Return the maximum number of children who may receive exactly 8 dollars if you distribute the money
according to the aforementioned rules. If there is no way to distribute the money, return -1.

Constraints:
1 <= money <= 200
2 <= children <= 30

*/

pub fn dist_money(money: i32, children: i32) -> i32 {
    let money_to_dist = money - children;
    let mut cont = 0;


    if money == 4 && children == 1 || money_to_dist < 0 { return -1; } else if money_to_dist >= 0 && money_to_dist < 7 { return 0; } else {
        for i in 7..=money_to_dist {
            if i % 7 == 0 && i <= children * 7 { cont += 1; }
            if i > children * 7 { break; }
        }

        if money_to_dist == (children - 1) * 7 + 3 { cont -= 1; }
        if money_to_dist > children * 7 { cont -= 1; }
    }
    cont
}