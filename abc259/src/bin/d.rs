use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
  input! {
    n: usize,
    s: (i64, i64),
    t: (i64, i64),
    cs: [(i64, i64, i64); n],
  }
  // make graph
  let mut uf = UnionFind::<usize>::new(n);
  for i in 0..n {
    for j in 0..i {
      if is_connect(cs[i], cs[j]) {
        uf.union(i, j);
      }
    }
  }

  // find path
  if let Some(si) = cs.iter().find_position(|&v| on_cir(s, *v)) {
    if let Some(ti) = cs.iter().find_position(|&v| on_cir(t, *v)) {
      if uf.equiv(si.0, ti.0) {
        println!("Yes");
        return ;
      }
    } 
  }
  println!("No");
}

fn is_connect(u: (i64, i64, i64), v: (i64, i64, i64)) -> bool {
  let sqdist = (u.0 - v.0).pow(2) + (u.1 - v.1).pow(2);
  (u.2 - v.2).pow(2) <= sqdist && sqdist <= (u.2 + v.2).pow(2)
}

fn on_cir(p: (i64, i64), v: (i64, i64, i64)) -> bool {
  (p.0 - v.0).pow(2) + (p.1 - v.1).pow(2) == (v.2).pow(2)
}