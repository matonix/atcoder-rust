use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
  input! {
    n: usize,
    a: [Chars; n],
  }
  let a = a.into_iter().map(|s| s.iter().map(|c| c.to_digit(10).unwrap() as usize).collect_vec() ).collect_vec();
  let mut m = 0;
  for i in 0..n {
    for j in 0..n {
      m = m.max(a[i][j]);
    }
  }
  let mut s = vec![];
  for i in 0..n {
    for j in 0..n {
      if a[i][j] == m {
        s.push((i,j));
      }
    }
  }
  let mut ans = 0;
  for (i,j) in s {
    for x in -1..=1 {
      for y in -1..=1 {
        if !(x == 0 && y == 0) {
          let mut cand = 0;
          for k in 0..n {
            let v = a[(i as isize+x*k as isize).rem_euclid(n as isize) as usize][(j as isize+y*k as isize).rem_euclid(n as isize) as usize];
            cand = cand * 10 + v;
            ans = ans.max(cand);
          }
        }
      }
    }
  }
  println!("{}", ans);
}