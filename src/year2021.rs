pub fn day01a(input: Vec<i32>) -> i32 {
    input
        .iter()
        .zip(input.iter().skip(1).collect::<Vec<&i32>>())
        .fold(0, |acc, p| if *p.0 < *p.1 { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01a_test() {
        assert_eq!(
            day01a(vec!(199, 200, 208, 210, 200, 207, 240, 260, 260, 263)),
            7
        );
    }
}
