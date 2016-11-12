use std::io;
use std::str::FromStr;
use std::fmt::{Debug, Display};

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

pub fn read_line_as<T: FromStr>() -> Vec<T> {
    read_line()
        .split_whitespace()
        .map(|part|
             match part.parse::<T>() {
                 Ok(num) => num,
                 Err(err) => panic!()
             })
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
