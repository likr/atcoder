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
        m: usize,
        st: [(Usize1, Usize1); m],
    }
    let mut events = BinaryHeap::new();
    for i in 0..m {
        events.push(Reverse((st[i].0, 0, i)));
        events.push(Reverse((st[i].1, 1, i)));
    }
    let mut result = (0..m).collect::<HashSet<_>>();
    let mut current = HashSet::new();
    for t in 0..n {
        while let Some(&Reverse((k, remove, i))) = events.peek() {
            if k != t || remove == 1 {
                break;
            }
            current.insert(i);
            events.pop();
        }
        if current.len() == 1 {
            for &i in &current {
                result.remove(&i);
            }
        }
        while let Some(&Reverse((k, remove, i))) = events.peek() {
            if k != t || remove == 0 {
                break;
            }
            current.remove(&i);
            events.pop();
        }
    }
    let mut result = result.into_iter().collect::<Vec<_>>();
    result.sort();
    println!("{}", result.len());
    for &i in &result {
        println!("{}", i + 1);
    }
}
