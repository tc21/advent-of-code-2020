/* --- Part Two ---

Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
*/
const INPUT: &str = include_str!("input");

fn main() {
    let mut graph = vec![vec![' '; 8]; 128];

    for input in INPUT.split_ascii_whitespace() {
        let (row, col) = get_seat_info(input);
        graph[row][col] = 'X';
    }

    let output = graph.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>();

    println!(" row  col       id(col=0)");
    println!("      01234567");
    for (row, seats) in output.into_iter().enumerate() {
        println!("{:>4}  {}  {}", row, seats, seat_id(row, 0));
    }
}

// returns row, column
fn get_seat_info(desc: &str) -> (usize, usize) {
    (
        parse_binary(&desc[..7], 'B', 'F'),
        parse_binary(&desc[7..], 'R', 'L')
    )
}

fn parse_binary(s: &str, one: char, zero: char) -> usize {
    let mut result = 0;

    for c in s.chars() {
        result <<= 1;
        if c == one {
            result += 1;
        } else if c != zero {
            eprintln!("expected {} or {}, got {}", one, zero, c);
            panic!();
        }
    }

    result
}

fn seat_id(row: usize, col: usize) -> usize {
    row * 8 + col
}
