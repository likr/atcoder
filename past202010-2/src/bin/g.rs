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
        s: [Chars; n],
    }
    let mut dot_count = 1;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '.' {
                dot_count += 1;
            }
        }
    }
    debug!(dot_count);

    let mut t = vec![vec!['#'; m + 2]; n + 2];

    let mut result = 0;
    for y in 0..n {
        'outer: for x in 0..m {
            if s[y][x] == '.' {
                continue;
            }
            debug!(x, y);
            for i in 0..n {
                for j in 0..m {
                    t[i + 1][j + 1] = s[i][j];
                }
            }
            t[y + 1][x + 1] = '.';

            let mut dsu = Dsu::new(n * m);
            for i in 0..n {
                for j in 0..m {
                    if t[i + 1][j + 1] == '.' {
                        if t[i][j + 1] == '.' {
                            dsu.merge(i * m + j, (i - 1) * m + j);
                        }
                        if t[i + 1][j] == '.' {
                            dsu.merge(i * m + j, i * m + j - 1);
                        }
                        if t[i + 2][j + 1] == '.' {
                            dsu.merge(i * m + j, (i + 1) * m + j);
                        }
                        if t[i + 1][j + 2] == '.' {
                            dsu.merge(i * m + j, i * m + j + 1);
                        }
                    }
                }
            }

            debug!(t);

            for i in 0..n {
                for j in 0..m {
                    debug!(i, j, dsu.size(i * m + j));
                    if t[i + 1][j + 1] == '.' && dsu.size(i * m + j) != dot_count {
                        continue 'outer;
                    }
                }
            }
            result += 1;
        }
    }
    println!("{}", result);
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
use dsu::*;
