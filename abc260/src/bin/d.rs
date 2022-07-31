use std::cmp::Ordering;

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
      n: usize,
      k: usize,
      p: [usize; n],
  }
  let mut ans = vec![-1isize; n];
  let mut board: Vec<MinScored<usize, Vec<usize>>> = Vec::new();
  for (i, &x) in p.iter().enumerate() {
    if board.len() == 0 {
      if k == 1 {
        ans[x-1] = i as isize + 1;
      } else {
        board.push(MinScored(x, vec![x]));
      }
    } else {
      let pos = board.binary_search_by_key(&x, |ms| ms.0).unwrap_or_else(|e| e);
      if pos == board.len() {
        board.push(MinScored(x, vec![x]));
      } else {
        let e = &mut board[pos];
        e.0 = x; // 順序は変わらない
        e.1.push(x);
        if e.1.len() == k {
          for j in e.1.iter() {
            ans[j-1] = i as isize + 1;
          }
          board.remove(pos);
        }
      }
    }
    // dbg!(&board);
  }
  println!(
    "{}",
    ans
      .into_iter()
      .map(|v| v.to_string())
      .collect_vec()
      .join("\n")
  );
}

// ヒープに入れるコンテナ
// https://github.com/petgraph/petgraph/blob/944057cdc57824f90a36f838f3ee7f148157c801/src/scored.rs
#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
  #[inline]
  fn eq(&self, other: &MinScored<K, T>) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
  #[inline]
  fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
  #[inline]
  fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
    let a = &self.0;
    let b = &other.0;
    if a == b {
      Ordering::Equal
    } else if a < b {
      Ordering::Greater
    } else if a > b {
      Ordering::Less
    } else if a.ne(a) && b.ne(b) {
      // these are the NaN cases
      Ordering::Equal
    } else if a.ne(a) {
      // Order NaN less, so that it is last in the MinScore order
      Ordering::Less
    } else {
      Ordering::Greater
    }
  }
}
