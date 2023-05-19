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
        a: [usize; n],
    }
    let mut s = 0;
    let mut nums = vec![];
    for i in 0..n {
        s += a[i];
        nums.push(a[i]);
    }
    nums.sort();
    nums.dedup();
    for i in 0..nums.len() {
        nums.push(nums[i] + m);
    }
    let num_index = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect::<HashMap<_, _>>();
    let mut count = vec![0; nums.len()];
    for i in 0..n {
        count[num_index[&a[i]]] += 1;
        count[num_index[&(a[i] + m)]] += 1;
    }
    let mut acc = vec![0; count.len()];
    acc[0] = nums[0] * count[0];
    for i in 1..count.len() {
        acc[i] = acc[i - 1] + count[i] * (nums[i] % m);
    }
    let mut result = s;
    let mut j = 1;
    for i in 0..nums.len() / 2 {
        if j <= i {
            j = i + 1;
        }
        while j - i < nums.len() / 2 && nums[j] + 1 == nums[j + 1] {
            j += 1;
        }
        result = min(result, s - (acc[j] - acc[i]));
    }
    println!("{}", result);
}
