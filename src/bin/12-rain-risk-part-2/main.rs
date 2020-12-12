const INPUT: &str = include_str!("input");
use Heading::*;

fn main() {
    let mut ship = Ship::new();

    for instruction in INPUT.lines().map(parse_instruction) {
        ship.act(instruction);
        // println!("ship location: ({}, {}), waypoint: ({}, {})", ship.x, ship.y, ship.waypoint_x, ship.waypoint_y);
    }

    println!("ship location: ({}, {})", ship.x, ship.y);
    println!("manhattan distance: {}", ship.x.abs() + ship.y.abs());
}

fn parse_instruction(s: &str) -> Instruction {
    let (instruction, units) = s.split_at(1);
    let units = units.parse::<i32>().unwrap();

    match instruction.chars().nth(0).unwrap() {
        'N' => Instruction::MoveWaypoint(North, units),
        'S' => Instruction::MoveWaypoint(South, units),
        'E' => Instruction::MoveWaypoint(East, units),
        'W' => Instruction::MoveWaypoint(West, units),
        'L' => Instruction::TurnWaypointRight((4 - units / 90) as usize),
        'R' => Instruction::TurnWaypointRight((units / 90) as usize),
        'F' => Instruction::MoveForward(units as usize),
        _ => panic!("invalid instruction")
    }
}

struct Ship {
    x: i32,
    y: i32,

    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    fn new() -> Self {
        Self { x: 0, y: 0, waypoint_x: 10, waypoint_y: 1 }
    }

    fn act(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::MoveWaypoint(heading, units) => self.move_waypoint(heading, units),
            Instruction::MoveForward(times) => self.move_ship(times),
            Instruction::TurnWaypointRight(times) => self.rotate_waypoint_right(times as usize)
        }
    }

    fn move_waypoint(&mut self, heading: Heading, units: i32) {
        let (dx, dy) = match heading {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };

        self.waypoint_x += units * dx;
        self.waypoint_y += units * dy;
    }

    fn move_ship(&mut self, times: usize) {
        for _ in 0..times {
            self.x += self.waypoint_x;
            self.y += self.waypoint_y;
        }
    }

    fn rotate_waypoint_right(&mut self, times: usize) {
        for _ in 0..times {
            let (x, y) = (self.waypoint_x, self.waypoint_y);

            self.waypoint_x = y;
            self.waypoint_y = -x;
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Heading {
    North, East, South, West
}

enum Instruction {
    MoveWaypoint(Heading, i32),
    MoveForward(usize),
    TurnWaypointRight(usize)
}
