use itertools::Itertools;

pub fn day01a(input: Vec<i32>) -> i32 {
    input
        .iter()
        .zip(input.iter().skip(1).collect::<Vec<&i32>>())
        .fold(0, |acc, p| if *p.0 < *p.1 { acc + 1 } else { acc })
}

pub fn day01b(input: Vec<i32>) -> i32 {
    let chunked: Vec<i32> = input
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .into_iter()
        .map(|t| t.0 + t.1 + t.2)
        .collect();
    day01a(chunked)
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

    #[test]
    fn day01b_test() {
        assert_eq!(
            day01b(vec!(199, 200, 208, 210, 200, 207, 240, 260, 260, 263)),
            5
        );
    }
}
