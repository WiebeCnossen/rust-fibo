extern crate num;

use num::bigint::BigInt;
use std::io;

fn fibo(n: u16) -> BigInt {
  (0..n).fold(
    (BigInt::from(0), BigInt::from(1)),
    |(prev, cur), _| { let next = prev + &cur; (cur, next) }
  ).1
}

fn read_line() -> String {
  let mut line = String::new();
  line.clear();
  io::stdin().read_line(&mut line).ok();
  line.pop();
  line
}

fn main() {
  loop {
    println!("Give a number or quit:");
    let line = read_line();
    if line == "quit" { break; }
    match u16::from_str_radix(&line, 10) {
      Ok(n) => println!("Fibo[{}] = {}", n, fibo(n)),
      Err(error) => println!("Not understood: {}", error)
    }
  }
}
