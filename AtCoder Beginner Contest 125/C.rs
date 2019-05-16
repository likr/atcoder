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
    if b > a {
        gcd(b, a)
    } else if b == 0 {
        a
    } else if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut l = Vec::new();
    l.resize(n + 2, 1);
    let mut r = Vec::new();
    r.resize(n + 2, 1);

    l[0] = 0;
    r[n + 1] = 0;
    for i in 1..n + 1 {
        l[i] = gcd(a[i - 1], l[i - 1]);
    }
    for i in (1..n + 1).rev() {
        r[i] = gcd(a[i - 1], r[i + 1]);
    }
    let g = (1..n + 1).map(|i| gcd(l[i - 1], r[i + 1])).max().unwrap();
    println!("{}", g);
}
