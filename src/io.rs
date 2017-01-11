use std::io;

fn trim_eol(mut s: String) -> String {
  match s.pop() {
    None => s,
    Some('\n') | Some('\r') => trim_eol(s),
    Some(c) => { s.push(c); s }
  }
}

pub fn read_line() -> String {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  trim_eol(line)
}
