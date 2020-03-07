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

fn check(a: &mut Vec<isize>, negative: bool) -> isize {
    let n = a.len();
    let mut s = vec![0isize; n];
    s[0] = a[0];
    for i in 1..n {
        s[i] = s[i - 1] + a[i];
    }

    let mut count = 0isize;
    let mut b = 0isize;
    let c = if negative && s[0] >= 0 {
        -s[0] - 1
    } else if !negative && s[0] <= 0 {
        -s[0] + 1
    } else {
        0
    };
    s[0] += c;
    count += c.abs();
    b += c;

    for i in 1..n {
        // eprintln!("{} {}", b, count);
        let c = if s[i - 1] < 0 && s[i] + b <= 0 {
            -(s[i] + b) + 1
        } else if s[i - 1] > 0 && s[i] + b >= 0 {
            -(s[i] + b) - 1
        } else {
            0
        };
        s[i] = s[i] + b + c;
        count += c.abs();
        b += c;
    }
    // eprintln!("{:?}", s);
    count
}

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let x = check(&mut a.clone(), true);
    let y = check(&mut a.clone(), false);
    println!("{}", min(x, y));
}
