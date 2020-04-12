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
        k: usize,
        p: [Chars; n],
    }
    let p = p
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut count = HashMap::new();
    for pi in &p {
        *count.entry(pi[0]).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for &c in count.values() {
        heap.push(c);
    }

    let mut result = 0;
    loop {
        let mut cs = vec![];
        let mut count = 0;
        while let Some(c) = heap.pop() {
            cs.push(c);
            count += 1;
            if count == k {
                break;
            }
        }
        if count < k {
            break;
        }
        for c in cs {
            if c > 1 {
                heap.push(c - 1);
            }
        }
        result += 1;
    }
    println!("{}", result);
}
