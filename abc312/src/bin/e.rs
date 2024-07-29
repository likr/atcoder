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
        rect: [[[usize; 3]; 2]; n],
    }
    let mut grid = vec![vec![vec![INF; 100]; 100]; 100];
    for i in 0..n {
        for x in rect[i][0][0]..rect[i][1][0] {
            for y in rect[i][0][1]..rect[i][1][1] {
                for z in rect[i][0][2]..rect[i][1][2] {
                    grid[x][y][z] = i;
                }
            }
        }
    }
    let mut ans = vec![HashSet::new(); n];
    for x in 0..100 {
        for y in 0..100 {
            for z in 0..100 {
                if x + 1 < 100
                    && grid[x][y][z] != INF
                    && grid[x + 1][y][z] != INF
                    && grid[x][y][z] != grid[x + 1][y][z]
                {
                    ans[grid[x][y][z]].insert(grid[x + 1][y][z]);
                    ans[grid[x + 1][y][z]].insert(grid[x][y][z]);
                }
                if y + 1 < 100
                    && grid[x][y][z] != INF
                    && grid[x][y + 1][z] != INF
                    && grid[x][y][z] != grid[x][y + 1][z]
                {
                    ans[grid[x][y][z]].insert(grid[x][y + 1][z]);
                    ans[grid[x][y + 1][z]].insert(grid[x][y][z]);
                }
                if z + 1 < 100
                    && grid[x][y][z] != INF
                    && grid[x][y][z + 1] != INF
                    && grid[x][y][z] != grid[x][y][z + 1]
                {
                    ans[grid[x][y][z]].insert(grid[x][y][z + 1]);
                    ans[grid[x][y][z + 1]].insert(grid[x][y][z]);
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", ans[i].len());
    }
}
