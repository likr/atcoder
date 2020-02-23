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
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let mut count = vec![0; 26];
    for i in 0..h {
        for j in 0..w {
            count[a[i][j] as usize - 'a' as usize] += 1;
        }
    }

    let mut c = (h / 2) * (w / 2);
    for k in 0..26 {
        if count[k] / 4 > c {
            count[k] -= 4 * c;
            c = 0;
        } else {
            c -= count[k] / 4;
            count[k] = count[k] % 4;
        }
    }
    eprintln!("{} {:?}", c, count);
    if c > 0 {
        eprintln!("1");
        println!("No");
        return;
    }

    let mut c = if h % 2 == 0 && w % 2 == 0 {
        0
    } else if h % 2 == 0 {
        h / 2
    } else if w % 2 == 0 {
        w / 2
    } else {
        h / 2 + w / 2
    };
    for k in 0..26 {
        if count[k] / 2 > c {
            count[k] -= 2 * c;
            c = 0;
        } else {
            c -= count[k] / 2;
            count[k] = count[k] % 2;
        }
    }
    eprintln!("{} {:?}", c, count);
    if c > 0 {
        eprintln!("2");
        println!("No");
    } else {
        println!("Yes");
    }
}
