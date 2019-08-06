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

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a
    }
    gcd(b, a % b)
}

fn ext_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a
    }
    let d = ext_gcd(b, a % b, y, x);
    *y -= a / b * *x;
    d
}

fn exp(i: usize, j: usize, a: usize, b: usize, c: usize, n: usize, table: &mut Vec<(usize, usize)>) -> (usize, usize) {
    if i == n || j == n {
        return (0, j);
    }
    let (ni, di) = exp(i + 1, j, a, b, c, n, table);
    let (nj, dj) = exp(i, j + 1, a, b, c, n, table);
    let num = 100 * (a * ni + b * nj + 100); 
    let den = (100 - c) * (100 * di + 100 * dj + 100);
    table[i * n + j] = (num, den);
    table[i * n + j]
}

fn main() {
    // input! {
    //     n: usize,
    //     a: usize,
    //     b: usize,
    //     c: usize,
    // }
    //
    let mut x = 0;
    let mut y = 0;
    println!("{:?}", ext_gcd(93, 16, &mut x, &mut y));
    println!("{} {}", x, y);
    // let mut table = Vec::new();
    // table.resize(n * n);
    // println!("{:?}", exp(0, 0, a, b, c, n, &mut table));
}
