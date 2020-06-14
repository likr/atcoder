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
        s: [Chars; 5],
    }
    let pattern = [
        ".###..#..###.###.#.#.###.###.###.###.###.",
        ".#.#.##....#...#.#.#.#...#.....#.#.#.#.#.",
        ".#.#..#..###.###.###.###.###...#.###.###.",
        ".#.#..#..#.....#...#...#.#.#...#.#.#...#.",
        ".###.###.###.###...#.###.###...#.###.###.",
    ];
    let pattern = pattern
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..n {
        for j in 0..10 {
            let mut ok = true;
            for x in 0..5 {
                for y in 0..3 {
                    if s[x][4 * i + 1 + y] != pattern[x][4 * j + 1 + y] {
                        ok = false;
                    }
                }
            }
            if ok {
                print!("{}", j);
                break;
            }
        }
    }
    println!("");
}
