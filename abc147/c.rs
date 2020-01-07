use std::io::Read;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut claims: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for i in 0..n {
        let a_i: usize = iter.next().unwrap().parse().unwrap();
        for _ in 0..a_i {
            let xj = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let yj = iter.next().unwrap().parse::<usize>().unwrap();
            claims[i].push((xj, yj));
        }
    }

    let mut result = 0;
    let mut pattern = vec![0; n];
    let m = 2usize.pow(n as u32);
    'outer: for x in 0..m {
        let mut x = x;
        let mut count = 0;
        for i in 0..n {
            pattern[i] = x % 2;
            if pattern[i] == 1 {
                count += 1;
            }
            x /= 2;
        }
        if count < result {
            continue;
        }
        // println!("{:?}", pattern);
        for i in 0..n {
            if pattern[i] == 1 {
                for &(a, b) in &claims[i] {
                    if pattern[a] != b {
                        continue 'outer;
                    }
                }
            }
        }
        result = count;
    }
    println!("{}", result);
}
