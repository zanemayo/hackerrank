use std::io;
use std::HashSet;
mod console;

fn how_many_ways(coins: &Vec<u32>, ref memo: &mut HashSet<(u32, u32), u32>, index: usize) -> u32 {
0
}

fn main () {
    //let line = console::read_line_as::<usize>();
    let line: Vec<usize> = console::read_line_as();
    let amount = line[0];
    let num_coins = line[1];
    println!("hello world: {} {}", amount, num_coins);
    let coins: Vec<usize> = console::read_line_as();

    let num_ways = how_many_ways(&coins, HashSet::new<(u32, u32), u32>(), coins.len());
    println!("num ways: {}", num_ways);

}
