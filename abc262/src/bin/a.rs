use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let ans = ((n + 1) / 4) * 4 + 2;
    println!("{}", ans);
}
