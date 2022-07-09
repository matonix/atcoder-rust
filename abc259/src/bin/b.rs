use std::f64::consts::PI;
use proconio::input;
use num::Complex;

fn main() {
  input! {
    a: f64,
    b: f64,
    d: f64,
  }
  let ans = Complex::new(a, b) * Complex::from_polar(&1.0, &(d / 180.0 * PI));
  println!("{} {}", ans.re, ans.im);
}