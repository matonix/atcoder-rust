use itertools::izip;
use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
      n: usize,
      x: usize,
      y: usize,
      z: usize,
      a: [isize; n],
      b: [isize; n],
  }
  let mut cands = izip!(
    1..=n,
    a.clone(),
    b.clone(),
    a.iter().zip(b).map(|(a, b)| a + b).collect_vec()
  ).collect_vec();
  cands.sort_by_key(|x| -x.1);
  cands[x..].sort_by_key(|x| x.0);
  cands[x..].sort_by_key(|x| -x.2);
  cands[x+y..].sort_by_key(|x| x.0);
  cands[x+y..].sort_by_key(|x| -x.3);
  let ans = cands[..x+y+z].into_iter().map(|x| x.0).sorted().collect_vec();
  for x in ans {
    println!("{}", x);
  }
}
