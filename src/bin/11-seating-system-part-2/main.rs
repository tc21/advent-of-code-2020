use std::fmt::Display;

const INPUT: &str = include_str!("input");

fn main() {
    let seats = INPUT.lines()
        .map(|line| line.chars().map(Seat::from).collect::<Vec<_>>())
        .collect();

    let mut seats = Seats { seats };

    loop {
        let new_seats = seats.iterate();

        if new_seats == seats {
            break;
        } else {
            seats = new_seats
        }
    }

    println!("results:");
    println!("{}", seats);

    let empty_seats = seats.seats.into_iter()
        .flat_map(|x| x)
        .filter(|s| *s == Seat::Occupied)
        .count();

    println!("empty seats: {}", empty_seats);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Seats {
    pub seats: Vec<Vec<Seat>>
}

impl Seats {
    fn iterate(&self) -> Seats {
        let new_seats = self.seats.iter().enumerate().map(|(row, row_ref)| {
            row_ref.iter().enumerate().map(|(col, _)| {
                self.iterate_seat(row, col)
            }).collect()
        }).collect();

        Seats { seats: new_seats }
    }

    fn first_seat_from(&self, row: usize, col: usize, direction: (i32, i32)) -> Option<Seat> {
        let mut row = row as i32;
        let mut col = col as i32;
        let (d_row, d_col) = direction;

        loop {
            row += d_row;
            col += d_col;

            if row >= 0 && col >= 0 {
                if let Some(row) = self.seats.get(row as usize) {
                    if let Some(seat) = row.get(col as usize) {
                        if let Seat::Floor = seat {
                            continue;
                        }

                        return Some(*seat);
                    }
                }
            }

            return None
        }
    }

    fn iterate_seat(&self, row: usize, col: usize) -> Seat {
        if let Seat::Floor = self.seats[row][col] {
            return Seat::Floor;
        }

        const DIRECTIONS: &[(i32, i32)] = &[
            (-1, -1), (-1,  0), (-1,  1),
            ( 0, -1),           ( 0,  1),
            ( 1, -1), ( 1,  0), ( 1,  1)
        ];

        let occupied_count = DIRECTIONS.iter()
            .filter(|&&direction| self.first_seat_from(row, col, direction) == Some(Seat::Occupied))
            .count();

        match self.seats[row][col] {
            Seat::Occupied if occupied_count >= 5 => Seat::Empty,
            Seat::Empty if occupied_count == 0 => Seat::Occupied,
            _ => self.seats[row][col]
        }
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.seats {
            writeln!(f, "{}", row.iter().map(|&s| char::from(s)).collect::<String>())?
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            _ => panic!("invalid character for seat")
        }
    }
}

impl From<Seat> for char {
    fn from(s: Seat) -> Self {
        match s {
            Seat::Empty => 'L',
            Seat::Occupied => '#',
            Seat::Floor => '.'
        }
    }
}
