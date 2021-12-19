use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(input: &str) -> Point {
        let (x, y) = input.split_once(',').unwrap();
        Point {
            x: i32::from_str_radix(x, 10).unwrap(),
            y: i32::from_str_radix(y, 10).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Line {
    e1: Point,
    e2: Point,
}

impl Line {
    pub fn new(input: String) -> Line {
        let (e1, e2) = input.split_once(" -> ").unwrap();
        Line {
            e1: Point::new(e1),
            e2: Point::new(e2),
        }
    }

    fn is_horizonta(&self) -> bool {
        self.e1.x == self.e2.x
    }

    fn is_vertical(&self) -> bool {
        self.e1.y == self.e2.y
    }

    fn points(&self) -> Vec<Point> {
        if self.is_horizonta() {
            let start = self.e1.y;
            let end = self.e2.y;
            if start < end {
                return (start..=end)
                    .into_iter()
                    .map(|y| Point { x: self.e1.x, y })
                    .collect::<Vec<Point>>();
            } else {
                return (end..=start)
                    .into_iter()
                    .map(|y| Point { x: self.e1.x, y })
                    .collect::<Vec<Point>>();
            }
        }
        if self.is_vertical() {
            let start = self.e1.x;
            let end = self.e2.x;
            if start < end {
                return (start..=end)
                    .into_iter()
                    .map(|x| Point { x, y: self.e1.y })
                    .collect::<Vec<Point>>();
            } else {
                return (end..=start)
                    .into_iter()
                    .map(|x| Point { x, y: self.e1.y })
                    .collect::<Vec<Point>>();
            }
        }
        unimplemented!("");
    }
}

#[derive(Debug)]
struct VentLines {
    vents: Vec<Line>,
}

impl VentLines {
    fn new(input: Vec<String>) -> VentLines {
        let vents = input.into_iter().map(|l| Line::new(l)).collect();
        VentLines { vents }
    }

    fn find_max_overlapping_points(&self) -> i32 {
        let mut point_overlaps: HashMap<Point, i32> = HashMap::new();
        self.vents
            .iter()
            .filter(|v| v.is_horizonta() || v.is_vertical())
            .flat_map(|v| v.points())
            .for_each(|p| {
                if point_overlaps.contains_key(&p) {
                    let new_count = *point_overlaps.get_mut(&p).unwrap() + 1;
                    point_overlaps.insert(p, new_count);
                } else {
                    point_overlaps.insert(p, 1);
                }
            });
        point_overlaps.values().filter(|c| **c >= 2).count() as i32
    }
}

pub fn day05a(input: Vec<String>) -> i32 {
    let vent_lines = VentLines::new(input);
    vent_lines.find_max_overlapping_points()
}

pub fn day05b(_input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05a() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];
        assert_eq!(
            day05a(input.into_iter().map(|l| l.to_string()).collect()),
            5
        );
    }
}
