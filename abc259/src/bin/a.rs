use proconio::input;

fn main() {
  input! {
    _n: usize,
    m: usize,
    x: usize,
    t: usize,
    d: usize,
  }
  let ans = t.min(t - x * d + m * d);
  println!("{}", ans);
}