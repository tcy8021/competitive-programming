fn read_input_to_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<T> = input
        .split(' ')
        .map(|s| s.trim_end().parse().unwrap())
        .collect();
    v
}

fn main() {
    let n: i64 = read_input_to_vec()[0];
    let mut v: Vec<Vec<i64>> = Vec::new();

    for _ in 0..n {
        v.push(read_input_to_vec());
    }

    println!("{:?}", v);
}
