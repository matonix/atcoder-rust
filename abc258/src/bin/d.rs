use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
  input! {
    n: usize,
    x: usize,
    ab: [(usize, usize); n],
  }
  let absum = ab.iter().map(|(a, b)| *a + *b).cumsum::<usize>().collect_vec();
  let ans = (1..=n).map(|k| (x - k) * ab[k - 1].1).zip(absum).map(|(xkb, abs)| xkb + abs).min().unwrap();
  println!("{}", ans);
}