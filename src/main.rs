use std::cmp;

fn read_input<T>() -> Vec<T>
where
    T: std::fmt::Debug + std::str::FromStr,
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
    let _n: i64 = read_input()[0];
    let v: Vec<i64> = read_input();

    let mut result = 1000000000;
    for mut a in v {
        let mut div_cnt = 0;
        while a % 2 == 0 {
            a /= 2;
            div_cnt += 1;
        }
        result = cmp::min(result, div_cnt);
    }

    println!("{}", result);
}
