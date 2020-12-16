/* --- Day 12: Rain Risk ---

Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.

The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11

These instructions would be handled as follows:

    F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
    N3 would move the ship 3 units north to east 10, north 3.
    F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
    R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
    F11 would move the ship 11 units south to east 17, south 8.

At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
*/
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
