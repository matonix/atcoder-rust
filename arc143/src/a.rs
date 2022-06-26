use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        ns: [isize; 3]
    }
    let ns = ns.into_iter().sorted().collect_vec();
    let mut ans = -1;
    if ns[0] + ns[1] < ns[2] {
    } else {
        ans = ns[2];
    }
    println!("{}", ans);
}
