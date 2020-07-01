use petgraph::unionfind::*;
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
        xy: [(Usize1, Usize1); m],
    }
    let edges = xy.iter().map(|&(xi, yi)| (xi, yi)).collect::<HashSet<_>>();
    let mut result = 0;
    for x in 0..2usize.pow(n as u32) {
        let mut xs = vec![];
        let mut ys = vec![];
        for i in 0..n {
            if x & 1 << i == 0 {
                xs.push(i);
            } else {
                ys.push(i);
            }
        }

        let mut cs = UnionFind::new(n);
        for &i in &xs {
            for &j in &ys {
                if edges.contains(&(i, j)) || edges.contains(&(j, i)) {
                    cs.union(i, j);
                }
            }
        }
        let mut count = 0;
        for i in 0..n {
            if i == cs.find(i) {
                count += 1;
            }
        }
        if count == 1 {
            result += 1;
        }
    }
    println!("{}", result / 2);
}
