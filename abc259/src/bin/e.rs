use std::collections::{HashMap, HashSet};
use proconio::input;

fn main() {
  input! {
    n: usize,
  }
  //                   p       e      i
  let mut map: HashMap<usize, (usize, Option<usize>)> = HashMap::new();
  for i in 0..n {
    input! {
      m: usize,
      pe: [(usize, usize); m]
    }
    for j in 0..m {
      if let Some(ei) = map.get_mut(&pe[j].0) {
        if ei.0 < pe[j].1 {
          *ei = (pe[j].1, Some(i));
        } else if ei.0 == pe[j].1 {
          ei.1 = None; // >= 2
        }
      } else {
        map.insert(pe[j].0, (pe[j].1, Some(i)));
      }
    } 
  }
  let ans = map.values().flat_map(|(_, i)| i).collect::<HashSet<_>>();
  println!("{}", (ans.len() + 1).min(n));
}