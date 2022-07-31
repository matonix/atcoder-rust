use std::collections::HashMap;
use proconio::{input, marker::Chars};

fn main() {
  input! {
      s: Chars,
  }
  let mut count = HashMap::new();
  for c in s {
    if let Some(v) = count.get_mut(&c) {
      *v += 1;
    } else {
      count.insert(c, 1);
    }
  }
  if let Some(e) = count.iter().find(|e| *e.1 == 1) {
    println!("{}", e.0);
  } else {
    println!("-1");
  }
}
