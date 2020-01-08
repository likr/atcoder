use std::collections::HashSet;
use std::io::Read;

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

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let q = iter.next().unwrap().parse::<usize>().unwrap();

    let mut matrix = vec![vec![0; n + 1]; n + 1];
    for i in 0..q {
        let t = iter.next().unwrap().parse::<usize>().unwrap();
        if t == 1 {
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let y = iter.next().unwrap().parse::<usize>().unwrap();
            matrix[x][y] = 1;
        } else if t == 2 {
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            for y in 1..n + 1 {
                if matrix[y][x] == 1 {
                    matrix[x][y] = 1;
                }
            }
        } else {
            let mut s = HashSet::new();
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            for y in 1..n + 1 {
                if matrix[x][y] == 1 {
                    for z in 1..n + 1 {
                        if matrix[y][z] == 1 {
                            s.insert(z);
                        }
                    }
                }
            }
            for y in s {
                if y != x {
                    matrix[x][y] = 1;
                }
            }
        }
    }
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            if matrix[i][j] == 1 {
                print!("Y");
            } else {
                print!("N");
            }
        }
        println!("");
    }
}
