/* --- Part Two ---

The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
*/
const INPUT: &str = include_str!("input");

// this is the result from part 1, hard-coded.
const TARGET: i64 = 1398413738;

fn main() {
    let input = INPUT.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let slice = find_contiguous_set(&input);

    println!("found a slice of len {}, starting from {}", slice.len(), slice[0]);

    let min = *slice.iter().min().unwrap();
    let max = *slice.iter().max().unwrap();

    println!("min(slice) = {}, max(slice) = {}, sum = {}", min, max, min + max);
}

fn find_contiguous_set(numbers: &Vec<i64>) -> &[i64] {
    const MIN_SLICE_SIZE: usize = 2;

    let mut left_index = 0;
    let mut right_index = MIN_SLICE_SIZE - 1;
    let mut rolling_sum = numbers.iter().take(MIN_SLICE_SIZE).sum::<i64>();

    loop {
        if right_index >= numbers.len() {
            panic!("could not find contiguous set");
        }

        if rolling_sum == TARGET {
            return &numbers[left_index..(1 + right_index)];
        } else if rolling_sum > TARGET && right_index - left_index > (MIN_SLICE_SIZE - 1) {
            rolling_sum -= numbers[left_index];
            left_index += 1;
        } else {
            right_index += 1;
            rolling_sum += numbers[right_index];
        }
    }
}
