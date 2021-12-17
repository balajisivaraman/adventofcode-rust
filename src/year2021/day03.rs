use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
struct BitCount {
    zeros: u32,
    ones: u32,
}

impl BitCount {
    fn update_count(&mut self, bit: char) {
        if bit == '0' {
            self.zeros += 1;
        } else {
            self.ones += 1;
        }
    }

    fn most_common_bit(&self) -> char {
        if self.zeros > self.ones {
            '0'
        } else {
            '1'
        }
    }

    fn least_common_bit(&self) -> char {
        if self.zeros > self.ones {
            '1'
        } else {
            '0'
        }
    }
}

#[derive(Clone, Debug, Default)]
struct BitMap {
    internal: BTreeMap<usize, BitCount>,
}

impl BitMap {
    fn generate(input: &Vec<String>) -> BitMap {
        input.iter().fold(BitMap::default(), |mut acc, line| {
            line.chars().enumerate().for_each(|(i, c)| {
                let bit_count = acc.get_or_default(i);
                bit_count.update_count(c);
            });
            acc
        })
    }

    fn get_or_default(&mut self, index: usize) -> &mut BitCount {
        if !self.internal.contains_key(&index) {
            self.internal.insert(index, BitCount::default());
        }
        self.internal.get_mut(&index).unwrap()
    }

    fn get_most_common_bits(&self) -> String {
        self.internal
            .keys()
            .into_iter()
            .map(|k| self.internal.get(k).unwrap().most_common_bit())
            .collect()
    }

    fn get_least_common_bits(&self) -> String {
        self.internal
            .keys()
            .into_iter()
            .map(|k| self.internal.get(k).unwrap().least_common_bit())
            .collect()
    }

    fn get_most_common_bit_at_position(&self, pos: usize) -> char {
        self.internal.get(&pos).unwrap().most_common_bit()
    }

    fn get_least_common_bit_at_position(&self, pos: usize) -> char {
        self.internal.get(&pos).unwrap().least_common_bit()
    }
}

struct DiagnosticReport {
    input: Vec<String>,
    bit_map: BitMap,
}

impl DiagnosticReport {
    pub fn new(input: Vec<String>) -> DiagnosticReport {
        let bit_map = BitMap::generate(&input);
        DiagnosticReport { input, bit_map }
    }

    fn gamma_rate(&self) -> i32 {
        i32::from_str_radix(self.bit_map.get_most_common_bits().as_str(), 2).unwrap()
    }

    fn epsilon_rate(&self) -> i32 {
        i32::from_str_radix(self.bit_map.get_least_common_bits().as_str(), 2).unwrap()
    }

    fn oxygen_generator_rating(&self) -> i32 {
        let mut input = self.input.clone();
        let mut idx = 0;
        let mut bit_map = self.bit_map.clone();
        loop {
            input = input
                .into_iter()
                .filter(|s| {
                    s.chars().nth(idx).unwrap() == bit_map.get_most_common_bit_at_position(idx)
                })
                .collect();
            bit_map = BitMap::generate(&input);
            idx += 1;
            if input.len() == 1 {
                return i32::from_str_radix(input.get(0).unwrap(), 2).unwrap();
            }
        }
    }

    fn co2_scrubber_rating(&self) -> i32 {
        let mut input = self.input.clone();
        let mut idx = 0;
        let mut bit_map = self.bit_map.clone();
        loop {
            input = input
                .into_iter()
                .filter(|s| {
                    s.chars().nth(idx).unwrap() == bit_map.get_least_common_bit_at_position(idx)
                })
                .collect();
            bit_map = BitMap::generate(&input);
            idx += 1;
            if input.len() == 1 {
                return i32::from_str_radix(input.get(0).unwrap(), 2).unwrap();
            }
        }
    }
}

pub fn day03a(input: Vec<String>) -> i32 {
    let diagnostic_report = DiagnosticReport::new(input);
    diagnostic_report.gamma_rate() * diagnostic_report.epsilon_rate()
}

pub fn day03b(input: Vec<String>) -> i32 {
    let diagnostic_report = DiagnosticReport::new(input);
    diagnostic_report.oxygen_generator_rating() * diagnostic_report.co2_scrubber_rating()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03a_test() {
        assert_eq!(
            day03a(vec!(
                String::from("00100"),
                String::from("11110"),
                String::from("10110"),
                String::from("10111"),
                String::from("10101"),
                String::from("01111"),
                String::from("00111"),
                String::from("11100"),
                String::from("10000"),
                String::from("11001"),
                String::from("00010"),
                String::from("01010")
            )),
            198
        )
    }

    #[test]
    fn day03b_test() {
        assert_eq!(
            day03b(vec!(
                String::from("00100"),
                String::from("11110"),
                String::from("10110"),
                String::from("10111"),
                String::from("10101"),
                String::from("01111"),
                String::from("00111"),
                String::from("11100"),
                String::from("10000"),
                String::from("11001"),
                String::from("00010"),
                String::from("01010")
            )),
            230
        )
    }
}
