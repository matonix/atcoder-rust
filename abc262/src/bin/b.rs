use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut e = uv;
    e.append(&mut e.clone().into_iter().map(|(u, v)| (v, u)).collect_vec());
    let e: HashSet<(usize, usize)> = HashSet::from_iter(e.into_iter());
    let mut ans = 0;
    for a in 1..=n {
        for b in 1..=n {
            for c in 1..=n {
                if e.contains(&(a, b)) && e.contains(&(b, c)) && e.contains(&(c, a)) {
                    ans += 1;
                }
            }        
        }    
    }
    println!("{}", ans / 6);
}
