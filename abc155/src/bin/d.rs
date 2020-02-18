use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [isize; n],
    }
    a.sort();
    let mut rev_a = a.clone();
    rev_a.reverse();
    for i in 0..n {
        rev_a[i] *= -1;
    }

    let mut l = -10isize.pow(18u32);
    let mut r = 10isize.pow(18u32);
    while l < r - 1 {
        // eprintln!("{} {}", l, r);
        let mut count = 0;
        let m = (l + r) / 2;
        for i in 0..n {
            if a[i] == 0 {
                if m >= 0 {
                    count += n - i - 1;
                }
            } else if a[i] < 0 {
                count += rev_a[..n - i - 1].upper_bound_by_key(&m, |&aj| -aj * a[i]);
            } else {
                count += a[i + 1..].upper_bound_by_key(&m, |&aj| aj * a[i]);
            }
            // eprintln!("  {} {}", a[i], count);
        }
        // eprintln!(" {} {}", m, count);
        if count < k {
            l = m;
        } else {
            r = m;
        }
    }
    // eprintln!("{} {}", l, r);
    println!("{}", r);
}
