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
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut a = a;
    let mut b = b;
    let mut c = c;
    a.sort();
    b.sort();
    c.sort();
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);

    let mut b_index = vec![0; n];
    let mut c_offset = 0;
    for i in 0..n {
        let bi = b[i];
        while c_offset < n && bi >= c[c_offset] {
            c_offset += 1;
        }
        b_index[i] = c_offset;
    }

    // println!("{:?}", b_index);

    let mut result = 0;
    let mut b_offset = n as i64 - 1;
    let mut count = 0;
    for i in (0..n).rev() {
        let ai = a[i];
        while b_offset >= 0 && ai < b[b_offset as usize] {
            count += n - b_index[b_offset as usize];
            b_offset -= 1;
        }
        result += count;
        // println!("{} {} {} {}", i, b_offset, count, result);
    }

    println!("{}", result);
}
