use ac_library::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }
    // let mut ans = 0;
    // for i2 in 1..n {
    //     for j2 in 1..m {
    //         for i in 0..=i2 {
    //             for j in 0..=j2 {
    //                 let mut s = 0;
    //                 let mut t = INF;
    //                 for x in i..=i2 {
    //                     for y in j..=j2 {
    //                         s += a[x][y];
    //                         t = min(t, a[x][y]);
    //                     }
    //                 }
    //                 if t * s > ans {
    //                     println!("{} {} {} {} {} {} {}", i, j, i2, j2, t, s, t * s);
    //                 }
    //                 ans = max(ans, t * s);
    //             }
    //         }
    //     }
    // }

    let mut acc = vec![vec![0; m + 1]; n + 1];
    let mut a_values = vec![];
    for i in 0..n {
        for j in 0..m {
            a_values.push(a[i][j]);
            acc[i + 1][j + 1] = a[i][j] + acc[i + 1][j] + acc[i][j + 1] - acc[i][j];
        }
    }
    a_values.sort();
    a_values.dedup();
    let mut ans = 0;
    for &ai in a_values.iter() {
        let mut cols = vec![];
        for j in 0..m {
            let mut tree = Segtree::<Additive<usize>>::new(n);
            for i in 0..n {
                if a[i][j] < ai {
                    tree.set(i, 1);
                }
            }
            cols.push(tree);
        }
        for i in 0..n {
            let mut bottom = Segtree::<Min<usize>>::new(m + 1);
            for j in 0..m {
                if a[i][j] >= ai {
                    bottom.set(j + 1, cols[j].max_right(i, |&v| v == 0));
                } else {
                    bottom.set(j + 1, i);
                }
            }
            for j in 0..m {
                let i1 = i;
                let i2 = bottom.get(j + 1);
                let j1 = bottom.min_left(j + 1, |&v| v >= i2);
                let j2 = bottom.max_right(j + 1, |&v| v >= i2) - 1;
                let j1 = if j1 > 0 { j1 - 1 } else { j1 };
                ans = max(
                    ans,
                    ai * (acc[i2][j2] + acc[i1][j1] - acc[i1][j2] - acc[i2][j1]),
                );
            }
        }
    }
    println!("{}", ans);
}
