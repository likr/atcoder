use std::cmp::{min, max};

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[derive(Clone, Debug)]
struct BitSet {
    x: Vec<u64>,
}

impl BitSet {
    fn new(n: usize) -> BitSet {
        BitSet {
            x: vec![0u64; (n + 64 - 1) / 64],
        }
    }

    fn insert(&mut self, v: usize) {
        self.x[v / 64] |= 1 << (v % 64);
    }

    fn has(&self, v: usize) -> bool {
        (self.x[v / 64] >> (v % 64)) & 1 > 0
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    max(a, b) - min(a, b)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    let m = 3 * 80 * 80 + 1;
    let mut table = vec![vec![BitSet::new(m); w]; h];
    table[0][0].insert(abs_diff(a[0][0], b[0][0]));
    for i in 1..h {
        let d = abs_diff(a[i][0], b[i][0]);
        for v in 0..m {
            if table[i - 1][0].has(v) {
                table[i][0].insert(d + v);
                table[i][0].insert(abs_diff(d, v));
            }
        }
    }
    for j in 1..w {
        let d = abs_diff(a[0][j], b[0][j]);
        for v in 0..m {
            if table[0][j - 1].has(v) {
                table[0][j].insert(d + v);
                table[0][j].insert(abs_diff(d, v));
            }
        }
    }
    for i in 1..h {
        for j in 1..w {
            let d = abs_diff(a[i][j], b[i][j]);
            for v in 0..m {
                if table[i - 1][j].has(v) {
                    table[i][j].insert(d + v);
                    table[i][j].insert(abs_diff(d, v));
                }
                if table[i][j - 1].has(v) {
                    table[i][j].insert(d + v);
                    table[i][j].insert(abs_diff(d, v));
                }
            }
        }
    }
    for v in 0..m {
        if table[h - 1][w - 1].has(v) {
            println!("{}", v);
            return;
        }
    }
}
