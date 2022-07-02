use proconio::{input, marker::Chars};

fn main() {
  input! {
    n: usize,
    q: usize,
    s: Chars,
    qs: [(usize, usize); q],
  }
  let mut i = 0;
  for (t, x) in qs {
    if t == 1 {
      i = (i + n - x) % n
    } else {
      println!("{}", s[(i + x - 1) % n]);
    }
  }
}
