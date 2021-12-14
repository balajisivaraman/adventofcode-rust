enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Default)]
struct Coordinate {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
    improved_command_processor: bool,
}

impl Coordinate {
    pub fn run_command(&mut self, command: Command) {
        use Command::*;
        match command {
            Forward(by) => self.forward(by),
            Up(by) => self.up(by),
            Down(by) => self.down(by),
        }
    }

    fn with_improved_command_processor(&mut self, fixed_command_processor: bool) {
        self.improved_command_processor = fixed_command_processor;
    }

    fn forward(&mut self, by: i32) {
        if self.improved_command_processor {
            self.horizontal_position += by;
            self.depth += self.aim * by;
        } else {
            self.horizontal_position += by;
        }
    }

    fn up(&mut self, by: i32) {
        if self.improved_command_processor {
            self.aim -= by;
        } else {
            self.depth -= by;
        }
    }

    fn down(&mut self, by: i32) {
        if self.improved_command_processor {
            self.aim += by;
        } else {
            self.depth += by;
        }
    }
}

fn parse_command(instr: String) -> Command {
    use Command::*;
    let split = instr.split(" ").collect::<Vec<&str>>();
    let command = split[0];
    let amount = split[1].parse::<i32>().unwrap();
    match command {
        "forward" => Forward(amount),
        "up" => Up(amount),
        _ => Down(amount),
    }
}

pub fn day02a(input: Vec<String>) -> i32 {
    let commands: Vec<Command> = input.into_iter().map(parse_command).collect();
    let mut coordinate = Coordinate::default();
    commands.into_iter().for_each(|c| coordinate.run_command(c));
    coordinate.horizontal_position * coordinate.depth
}

pub fn day02b(input: Vec<String>) -> i32 {
    let commands: Vec<Command> = input.into_iter().map(parse_command).collect();
    let mut coordinate = Coordinate::default();
    coordinate.with_improved_command_processor(true);
    commands.into_iter().for_each(|c| coordinate.run_command(c));
    coordinate.horizontal_position * coordinate.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02a_test() {
        assert_eq!(
            day02a(vec!(
                String::from("forward 5"),
                String::from("down 5"),
                String::from("forward 8"),
                String::from("up 3"),
                String::from("down 8"),
                String::from("forward 2")
            )),
            150
        );
    }

    #[test]
    fn day02b_test() {
        assert_eq!(
            day02b(vec!(
                String::from("forward 5"),
                String::from("down 5"),
                String::from("forward 8"),
                String::from("up 3"),
                String::from("down 8"),
                String::from("forward 2")
            )),
            900
        );
    }
}
