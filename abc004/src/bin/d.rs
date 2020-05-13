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
const INF: isize = std::isize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        r: isize,
        g: isize,
        b: isize,
    }
    let left = -1000;
    let right = 1000;

    let mut result = INF;
    for g_left in left..=right {
        let g_right = g_left + g - 1;

        let mut count = 0;
        for i in g_left..=g_right {
            count += i.abs();
        }

        let mut left_count = INF;
        for r_right in left..g_left {
            let r_left = r_right - r + 1;
            let mut c = 0;
            for i in r_left..=r_right {
                c += (i + 100).abs();
            }
            left_count = min(left_count, c);
        }
        count += left_count;

        let mut right_count = INF;
        for b_left in g_right + 1..=right {
            let b_right = b_left + b - 1;
            let mut c = 0;
            for i in b_left..=b_right {
                c += (i - 100).abs();
            }
            right_count = min(right_count, c);
        }
        count += right_count;

        result = min(result, count);
    }
    println!("{}", result);
}
