use proconio::input;

fn main() {
  input! {
    n: u32,
  }
  let hh = 21 + n/60;
  let mm = n%60;
  println!("{0: >02}:{1: >02}", hh, mm);
}