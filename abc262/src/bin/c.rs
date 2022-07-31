use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut id = 0usize;
    let mut sp = 0usize;
    for i in 0..n {
        if a[i] == i + 1 {
            // identity
            id += 1;
        } else if a[a[i] - 1] == i + 1  {
            // swap pair
            sp += 1;
        }
    }
    id = id * (id - 1) / 2; // nC2
    sp /= 2; // dedup
    dbg!(id, sp);
    println!("{}", id + sp);
}
