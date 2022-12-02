use itertools::Itertools;

pub fn day01a(input: Vec<String>) -> i32 {
    input
        .split(|line| line == &String::from(""))
        .map(|elf_calories| {
            elf_calories
                .iter()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn day01b(input: Vec<String>) -> i32 {
    input
        .split(|line| line == &String::from(""))
        .map(|elf_calories| {
            elf_calories
                .iter()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01a_test() {
        assert_eq!(
            day01a(vec![
                String::from("1000"),
                String::from("2000"),
                String::from("3000"),
                String::from(""),
                String::from("4000"),
                String::from(""),
                String::from("5000"),
                String::from("6000"),
                String::from(""),
                String::from("7000"),
                String::from("8000"),
                String::from("9000"),
                String::from(""),
                String::from("10000")
            ]),
            24000
        );
    }

    #[test]
    fn day01b_test() {
        assert_eq!(
            day01b(vec![
                String::from("1000"),
                String::from("2000"),
                String::from("3000"),
                String::from(""),
                String::from("4000"),
                String::from(""),
                String::from("5000"),
                String::from("6000"),
                String::from(""),
                String::from("7000"),
                String::from("8000"),
                String::from("9000"),
                String::from(""),
                String::from("10000")
            ]),
            45000
        );
    }
}
