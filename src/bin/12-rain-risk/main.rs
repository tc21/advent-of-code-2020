const INPUT: &str = include_str!("input");
use Heading::*;

fn main() {
    let mut ship = Ship::new();

    for instruction in INPUT.lines().map(parse_instruction) {
        ship.act(instruction);
        // println!("ship location: ({}, {}), heading: {:?}", ship.x, ship.y, ship.heading);
    }

    println!("ship location: ({}, {})", ship.x, ship.y);
    println!("manhattan distance: {}", ship.x.abs() + ship.y.abs());
}

fn parse_instruction(s: &str) -> Instruction {
    let (instruction, units) = s.split_at(1);
    let units = units.parse::<i32>().unwrap();

    match instruction.chars().nth(0).unwrap() {
        'N' => Instruction::Move(North, units),
        'S' => Instruction::Move(South, units),
        'E' => Instruction::Move(East, units),
        'W' => Instruction::Move(West, units),
        'L' => Instruction::TurnRight(360 - units),
        'R' => Instruction::TurnRight(units),
        'F' => Instruction::MoveForward(units),
        _ => panic!("invalid instruction")
    }
}

struct Ship {
    x: i32,
    y: i32,

    heading: Heading
}

impl Ship {
    fn new() -> Self {
        Self { x: 0, y: 0, heading: East }
    }

    fn act(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(heading, units) => self.move_to(heading, units),
            Instruction::MoveForward(units) => self.move_to(self.heading, units),
            Instruction::TurnRight(degrees) => self.heading = self.heading.turn(degrees as usize)
        }
    }

    fn move_to(&mut self, heading: Heading, units: i32) {
        let (dx, dy) = match heading {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };

        self.x += units * dx;
        self.y += units * dy;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Heading {
    North, East, South, West
}

impl Heading {
    fn turn(&self, degrees: usize) -> Heading {
        const HEADINGS: &[Heading] = &[North, East, South, West];

        if degrees % 90 != 0 {
            panic!("invalid turn angle; must be multiple of 90")
        }

        let offset = HEADINGS.iter().position(|h| h == self).unwrap();

        HEADINGS[(offset + degrees / 90) % 4]
    }
}

enum Instruction {
    Move(Heading, i32),
    MoveForward(i32),
    TurnRight(i32)
}
