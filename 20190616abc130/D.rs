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

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut s1 = Vec::new();
    s1.resize(n, 0);
    s1[n - 1] = a[n - 1];
    for i in 1..n {
        s1[n - i - 1] = s1[n - i] + a[i];
    }

    let mut s2 = Vec::new();
    s2.resize(n, 0);
    for i in 1..n {
        s2[i] = s2[i - 1] + a[i - 1];
    }

    let mut count = 0;
    for i in 0..n {
        let mut l = i + 1;
        let mut h = n - 1;
        let mut m = (l + h) / 2;
        while l < h && !(s1[m - 1] - s2[i] >= k && s1[m] - s2[i] < k) {
            if s1[m] - s2[i] > k {
                l = m + 1;
            } else {
                h = m - 1;
            }
            m = (l + h) / 2;
        }
        println!("{} {} {}", i, m, m - i);
        count += m - i + 1;
    }
    println!("{}", count);
}
