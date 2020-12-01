fn run_length_encoding(s: &str) -> Vec<(char, i32)> {
    let mut res: Vec<(char, i32)> = Vec::new();
    s.chars().for_each(|c| match &mut res.last_mut() {
        Some(t) => {
            if c == t.0 {
                t.1 += 1;
            } else {
                res.push((c, 1));
            }
        }
        None => res.push((c, 1)),
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_length_encoding_test() {
        assert_eq!(
            run_length_encoding("aabubuaaauu"),
            vec![
                ('a', 2),
                ('b', 1),
                ('u', 1),
                ('b', 1),
                ('u', 1),
                ('a', 3),
                ('u', 2)
            ]
        )
    }
}
