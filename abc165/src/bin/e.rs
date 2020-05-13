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

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut result = vec![];
    if n % 2 == 1 {
        for i in 1..=m {
            result.push((i, n - i));
        }
    } else {
        let mut ds = HashSet::new();
        let mut l = 1;
        let mut r = n;
        for _ in 1..=m {
            let d1 = r - l - 1;
            let d2 = n - 2 - d1;
            if d1 == d2 || ds.contains(&d1) || ds.contains(&d2) {
                r -= 1;
            }
            ds.insert(r - l - 1);
            ds.insert(n - 2 - (r - l - 1));
            result.push((l, r));
            l += 1;
            r -= 1;
        }
    }

    // let mut pairs = HashSet::new();
    // let mut ans = (1..=n).collect::<Vec<_>>();
    // for _ in 0..n {
    //     eprintln!("{:?}", ans);
    //     for &(i, j) in &result {
    //         let x = min(ans[i - 1], ans[j - 1]);
    //         let y = max(ans[i - 1], ans[j - 1]);
    //         if pairs.contains(&(x, y)) {
    //             eprintln!("error: {} {}", x, y);
    //         }
    //         pairs.insert((x, y));
    //     }
    //     for i in 0..n {
    //         ans[i] += 1;
    //         if ans[i] > n {
    //             ans[i] = 1;
    //         }
    //     }
    // }

    for &(i, j) in &result {
        println!("{} {}", i, j);
    }
}
