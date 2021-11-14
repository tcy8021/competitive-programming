#![allow(dead_code)]

// 標準入力を空白で区切って Vec<T> を返す(T は int, float, string では動作確認済み)
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
