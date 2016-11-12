

pub mod console {
use std::io;
pub fn readln() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
