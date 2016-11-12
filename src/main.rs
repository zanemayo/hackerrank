use std::io;
use std::collections::HashMap;

mod console;

fn how_many_ways(coins: &Vec<i64>, mut memo: &mut HashMap<(usize, i64), i64>, mut amount: i64, index: usize) -> i64 {
    if index == 1 { return if amount % coins[0] == 0 { 1 } else { 0 } }
    let mut num_ways = 0;
    while amount >= 0 {
        if memo.get(&(index - 1, amount)).is_none() {
          let ways = how_many_ways(&coins, &mut memo, amount, index - 1);
          memo.insert((index - 1, amount), ways);
        }
        num_ways += *memo.get(&(index - 1, amount)).unwrap();
        amount -= coins[index - 1];
    }
    return num_ways
}

fn main () {
    //let line = console::read_line_as::<usize>();
    let line: Vec<i64> = console::read_line_as();
    let amount = line[0];
    let num_coins = line[1];
    let coins: Vec<i64> = console::read_line_as();

    let num_ways = how_many_ways(&coins, &mut HashMap::new(), amount, coins.len());
    println!("{}", num_ways);

}
