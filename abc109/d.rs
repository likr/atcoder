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
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut a = a;
    let mut result = vec![];
    for i in 0..h {
        for k in 0..w {
            let j = if i % 2 == 0 { k } else { w - 1 - k };
            if a[i][j] % 2 == 1 {
                let i_next = if (i % 2 == 0 && j == w - 1) || (i % 2 == 1 && j == 0) {
                    i + 1
                } else {
                    i
                };
                let j_next = if (i % 2 == 0 && j == w - 1) || (i % 2 == 1 && j == 0) {
                    j
                } else if i % 2 == 0 {
                    j + 1
                } else {
                    j - 1
                };
                if i_next < h {
                    a[i][j] -= 1;
                    a[i_next][j_next] += 1;
                    result.push((i, j, i_next, j_next));
                }
            }
        }
    }
    println!("{}", result.len());
    for (i0, j0, i1, j1) in result {
        println!("{} {} {} {}", i0 + 1, j0 + 1, i1 + 1, j1 + 1);
    }
}
