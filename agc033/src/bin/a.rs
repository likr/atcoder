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
        h: usize,
        w: usize,
        mut a: [Chars; h],
    }
    let mut next = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                next.insert((i, j));
            }
        }
    }
    let mut b_count = 0;
    let mut repeat = 0;
    while b_count < h * w {
        let mut next_next = HashSet::new();
        for &(i, j) in &next {
            a[i][j] = '#';
            b_count += 1;
        }
        for &(i, j) in &next {
            if i > 0 && a[i - 1][j] == '.' {
                next_next.insert((i - 1, j));
            }
            if i < h - 1 && a[i + 1][j] == '.' {
                next_next.insert((i + 1, j));
            }
            if j > 0 && a[i][j - 1] == '.' {
                next_next.insert((i, j - 1));
            }
            if j < w - 1 && a[i][j + 1] == '.' {
                next_next.insert((i, j + 1));
            }
        }
        // eprintln!("{}", b_count);
        // for row in a.iter() {
        //     eprintln!("{:?}", row);
        // }
        // eprintln!("{:?}", next_next);
        next = next_next;
        repeat += 1;
    }
    println!("{}", repeat - 1);
}
