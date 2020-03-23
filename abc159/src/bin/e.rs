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
        k: usize,
        s: [Chars; h],
    }
    let mut result = INF;
    'x: for x in 0..2usize.pow(h as u32 - 1) {
        let mut count = 0;
        let mut ranges = vec![];
        let mut j = 0;
        for i in 0..h - 1 {
            if x & 1 << i > 0 {
                count += 1;
                ranges.push((j, i + 1));
                j = i + 1;
            }
        }
        ranges.push((j, h));

        let m = ranges.len();
        let mut range_count = vec![0; m];
        'outer: for j in 0..w {
            for l in 0..m {
                let (i0, i1) = ranges[l];
                for i in i0..i1 {
                    if s[i][j] == '1' {
                        range_count[l] += 1;
                    }
                }
            }
            for l in 0..m {
                if range_count[l] > k {
                    // eprintln!(" {} {:?}", j, range_count);
                    count += 1;
                    for l in 0..m {
                        range_count[l] = 0;
                        let (i0, i1) = ranges[l];
                        for i in i0..i1 {
                            if s[i][j] == '1' {
                                range_count[l] += 1;
                            }
                        }
                        if range_count[l] > k {
                            continue 'x;
                        }
                    }
                    continue 'outer;
                }
            }
        }

        // eprintln!("{} {:?}", count, ranges);
        result = min(result, count);
    }
    println!("{}", result);
}
