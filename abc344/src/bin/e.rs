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
        a: [usize; n],
        q: usize,
    }
    let mut values = vec![a[0]];
    let mut next = vec![None];
    let mut prev = vec![None];
    let mut head = Some(0);
    let mut index = HashMap::new();
    index.insert(a[0], 0);

    let mut last = 0;
    for i in 1..n {
        next[last] = Some(i);
        values.push(a[i]);
        next.push(None);
        prev.push(Some(last));
        last = i;
        index.insert(a[i], last);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
                y: usize,
            }
            let j = values.len();
            values.push(y);
            next.push(next[index[&x]]);
            prev.push(Some(index[&x]));
            if let Some(k) = next[index[&x]] {
                prev[k] = Some(j);
            }
            next[index[&x]] = Some(j);
            index.insert(y, j);
        } else {
            input! {
                x: usize,
            }
            if let Some(k) = next[index[&x]] {
                prev[k] = prev[index[&x]];
            }
            if let Some(k) = prev[index[&x]] {
                next[k] = next[index[&x]];
            } else {
                head = next[index[&x]];
            }
        }
    }
    let mut ans = vec![];
    let mut node = head;
    while let Some(k) = node {
        ans.push(format!("{}", values[k]));
        node = next[k];
    }
    println!("{}", ans.join(" "));
}
