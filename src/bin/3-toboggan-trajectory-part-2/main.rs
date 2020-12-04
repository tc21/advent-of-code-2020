/* --- Part Two ---

Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.

In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/
const INPUT: &str = include_str!("input");

// (right, down)
const SLOPES: &[(usize, usize)] = &[
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
];

fn main() {
    let map = Map::from(INPUT.lines());

    let multiply_result = SLOPES.iter()
        .map(|slope| {
            let trees = number_of_trees_encountered(&map, slope);
            println!("slope {:?}: hit {} trees", slope, trees);
            trees
        }).product::<usize>();

    println!("result: {}", multiply_result);
}

fn number_of_trees_encountered(map: &Map, slope: &(usize, usize)) -> usize {
    let (slope_right, slope_down) = slope;
    let (mut row, mut col) = (0, 0);
    let mut trees = 0;

    loop {
        row += slope_down;
        col += slope_right;

        if row >= map.height() {
            break
        }

        if map.is_tree(row, col) {
            trees += 1;
        }
    }

    trees
}

struct Map {
    // true for tree, false for empty space
    lines: Vec<Vec<bool>>,
    width: usize
}

impl Map {
    pub fn from<I, S>(input: I) -> Self
    where I: Iterator<Item = S>, S : AsRef<str> {
        let lines = input.map(|line| {
            let chars = line.as_ref().chars();
            let trees = chars.map(|c| c == '#');
            trees.collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let width = lines[0].len();

        Self { lines, width }
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn is_tree(&self, row: usize, col: usize) -> bool {
        self.lines[row][col % self.width]
    }
}
