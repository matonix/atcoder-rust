use proconio::input;

fn main() {
  input! {
      n: usize,
      x: usize,
      y: usize,
  }
  let mut r = vec![0; n];
  let mut b = vec![0; n];
  r[n - 1] = 1;
  for i in (1..n).rev() {
    r[i - 1] += r[i];
    b[i] += r[i] * x;
    r[i - 1] += b[i];
    b[i - 1] += b[i] * y;
  }
  println!("{}", b[0]);
}
