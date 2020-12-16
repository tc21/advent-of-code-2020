/* --- Part Two ---

Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

    Action N means to move the waypoint north by the given value.
    Action S means to move the waypoint south by the given value.
    Action E means to move the waypoint east by the given value.
    Action W means to move the waypoint west by the given value.
    Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
    Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
    Action F means to move forward to the waypoint a number of times equal to the given value.

The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

    F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
    N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
    F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
    R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
    F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.

After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
*/
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
