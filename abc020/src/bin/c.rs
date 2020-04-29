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
        t: usize,
        s: [Chars; h],
    }
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
            }
            if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }
    let mut l = 1;
    let mut r = t;
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, start.0, start.1)));
        let mut available = false;
        let mut visited = HashSet::new();
        while let Some(Reverse((d, i, j))) = heap.pop() {
            if (i, j) == goal {
                eprintln!("{}", d);
                available = d <= t;
                break;
            }
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            if i > 0 {
                heap.push(Reverse((
                    d + if s[i - 1][j] == '#' { m } else { 1 },
                    i - 1,
                    j,
                )));
            }
            if i < h - 1 {
                heap.push(Reverse((
                    d + if s[i + 1][j] == '#' { m } else { 1 },
                    i + 1,
                    j,
                )));
            }
            if j > 0 {
                heap.push(Reverse((
                    d + if s[i][j - 1] == '#' { m } else { 1 },
                    i,
                    j - 1,
                )));
            }
            if j < w - 1 {
                heap.push(Reverse((
                    d + if s[i][j + 1] == '#' { m } else { 1 },
                    i,
                    j + 1,
                )));
            }
        }
        if available {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
