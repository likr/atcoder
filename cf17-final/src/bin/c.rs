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

fn diff(di: usize, dj: usize) -> usize {
    let d = max(di, dj) - min(di, dj);
    min(d, 24 - d)
}

fn min_diff(d: &Vec<usize>) -> usize {
    let n = d.len();
    let mut max_diff = INF;
    for i in 0..n {
        for j in i + 1..n {
            if diff(d[i], d[j]) < max_diff {
                max_diff = diff(d[i], d[j]);
            }
        }
    }
    max_diff
}

fn reverse(d: usize) -> usize {
    24 - d
}

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }
    d.push(0);
    let mut count = HashMap::new();
    for &di in &d {
        *count.entry(di).or_insert(0) += 1;
    }
    let mut one = HashSet::new();
    let mut two = HashSet::new();
    for (&dk, &count) in count.iter() {
        if count > 2 {
            println!("0");
            return;
        } else if count == 2 {
            two.insert(dk);
        } else if count == 1 {
            one.insert(dk);
        }
    }
    let mut y = vec![];
    let mut checked = HashSet::new();
    for i in 0..=n {
        if two.contains(&d[i]) && !checked.contains(&d[i]) {
            checked.insert(d[i]);
            d[i] = reverse(d[i]);
        } else if one.contains(&d[i]) {
            y.push(i)
        }
    }
    // eprintln!("{:?} {:?}", one, two);
    // eprintln!("{:?}", d);
    let m = y.len();
    let mut e = d.clone();
    let mut result = min_diff(&d);
    for x in 0..2usize.pow(m as u32) {
        for i in 0..m {
            if x & 1 << i > 0 {
                e[y[i]] = reverse(d[y[i]]);
            } else {
                e[y[i]] = d[y[i]];
            }
            result = max(result, min_diff(&e));
        }
    }
    println!("{}", result);
}
