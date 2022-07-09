use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
  input! {
    s: Chars,
    t: Chars,
  }
  if s.iter().dedup().collect_vec() != t.iter().dedup().collect_vec() {
    println!("No");
    return ;
  }
  let sc = cnt(&s);
  let tc = cnt(&t);
  if sc.len() != tc.len() {
    println!("No");
    return ;
  }
  if sc.into_iter().zip(tc).all(|(s, t)| (s == 1 && t == 1) || (s > 1 && t > 1 && s <= t)) {
    println!("Yes");
  } else {
    println!("No");
  }
}

fn cnt(s: &Vec<char>) -> Vec<usize> {
  let mut prev = &s[0];
  let mut acc = 1;
  let mut t = vec![];
  for c in &s[1..] {
    if c == prev {
      acc += 1;
    } else {
      t.push(acc);
      acc = 1;
      prev = c;
    }
  }
  t.push(acc);
  t
}