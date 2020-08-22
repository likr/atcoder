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
        k: isize,
        p: [Usize1; n],
        c: [isize; n],
    }
    let mut components = UnionFind::new(n);
    for i in 0..n {
        components.union(i, p[i]);
    }
    let mut cycles = vec![0; n];
    let mut cycle_cost = vec![0isize; n];
    for i in 0..n {
        cycles[components.find(i)] += 1;
        cycle_cost[components.find(i)] += c[p[i]];
    }
    let mut cost = vec![0isize; n];
    for i in 0..n {
        let cc = cycle_cost[components.find(i)];
        let ci = cycles[components.find(i)];

        let l = min(k, k % ci + ci);
        let k = k - l;

        let mut s = vec![c[p[i]]];
        let mut j = p[i];
        for _ in 1..l {
            s.push(s[s.len() - 1] + c[p[j]]);
            j = p[j];
        }
        let mut c = *s.iter().max().unwrap();
        if cc > 0 {
            c += cc * (k / ci);
        }
        // eprintln!("{} {} {} {:?}", i, cc, ci, s);
        cost[i] = c;
    }
    println!("{}", cost.iter().max().unwrap());
}
