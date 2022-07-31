use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
      n: usize,
      a: [usize; n],
  }
  let m = 998244353;
  let mut ans = 0;
  for i in 1..=n {
    let mut amod = a.iter().map(|x| x % i).collect_vec();
    amod.sort_unstable();
    let cnt = dedup_with_count(&amod);
    dbg!(cnt);
  }
  println!("{}", ans);
}

// Require: vector v is sorted
fn dedup_with_count<T: PartialEq + Copy>(v: &Vec<T>) -> Vec<(T, usize)> {
  let mut result = vec![];
  let mut c = 1usize;
  if let Some(y) = v.get(0) {
    let mut y = y;
    for x in &v[1..] {
      if *x == *y {
        c += 1;
      } else {
        result.push((*y, c));
        y = x;
        c = 1;
      }
    }
    result.push((*y, c));
  }
  result
}
