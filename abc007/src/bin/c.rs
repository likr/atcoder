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
        row: usize,
        col: usize,
        sy: Usize1,
        sx: Usize1,
        gy: Usize1,
        gx: Usize1,
        c: [Chars; row],
    }
    let mut visited = vec![vec![false; col]; row];
    let mut f = HashSet::new();
    f.insert((sx, sy));
    let d = [(0, 1), (1, 0), (1, 2), (2, 1)];
    for k in 1.. {
        // eprintln!("{:?}", f);
        let mut g = HashSet::new();
        for &(x, y) in f.iter() {
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            for &(dx, dy) in &d {
                let x2 = x + dx - 1;
                let y2 = y + dy - 1;
                if !visited[y2][x2] && c[y2][x2] == '.' {
                    g.insert((x2, y2));
                }
            }
        }
        if g.contains(&(gx, gy)) {
            println!("{}", k);
            return;
        }
        f = g;
    }
}
