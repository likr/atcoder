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
        k: usize,
        t: usize,
        a: [usize; t],
    }
    let mut heap = a
        .iter()
        .enumerate()
        .map(|(i, &ai)| (ai, i))
        .collect::<BinaryHeap<(usize, usize)>>();
    let mut e = vec![0; k];
    let (ai, i) = heap.pop().unwrap();
    e[0] = i;
    if ai > 1 {
        heap.push((ai - 1, i));
    }
    for l in 1..k {
        let (ai, i) = heap.pop().unwrap();
        if e[l - 1] == i {
            if let Some((aj, j)) = heap.pop() {
                e[l] = j;
                heap.push((ai, i));
                if aj > 1 {
                    heap.push((aj - 1, j));
                }
            } else {
                e[l] = i;
                if ai > 1 {
                    heap.push((ai - 1, i));
                }
            }
        } else {
            e[l] = i;
            if ai > 1 {
                heap.push((ai - 1, i));
            }
        }
    }
    let result = (1..k).filter(|&i| e[i - 1] == e[i]).count();
    println!("{}", result);
}
