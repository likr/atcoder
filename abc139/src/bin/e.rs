use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
      n: usize,
      a: [[Usize1; n - 1]; n],
    }
    let mut candidate = (0..n).collect::<HashSet<_>>();
    let mut progress = vec![0; n];
    let mut finished = HashSet::new();
    let mut turn = 0;
    while finished.len() < n {
        let mut used = HashSet::new();
        for &i in &candidate {
            if finished.contains(&i) {
                continue;
            }
            let j = a[i][progress[i]];
            if !used.contains(&i)
                && !used.contains(&j)
                && !finished.contains(&j)
                && i == a[j][progress[j]]
            {
                used.insert(i);
                used.insert(j);
                progress[i] += 1;
                if progress[i] == n - 1 {
                    finished.insert(i);
                }
                progress[j] += 1;
                if progress[j] == n - 1 {
                    finished.insert(j);
                }
            }
        }
        if used.is_empty() {
            println!("-1");
            return;
        }
        candidate = used;
        turn += 1;
    }
    println!("{}", turn);
}
