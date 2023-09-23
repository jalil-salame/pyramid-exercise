fn main() {
    let height = std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|v| v.parse().unwrap())
        .unwrap_or(12usize);

    let mut s = String::with_capacity(height * 2 + 1);
    s.push('#');
    for i in 0..height {
        println!("{s:>width$}", width = height + i);
        s.push_str("##");
    }

    s.clear();
    for _ in 0..height {
        s.push('#');
        println!("{s:>width$}", width = height);
    }
}
