/* --- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?
*/
const INPUT: &str = include_str!("input");
const TARGET: i32 = 2020;

fn main() -> Result<(), &'static str> {
    let mut numbers = INPUT
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    numbers.sort();

    for pinned_index in 1..(numbers.len() - 1){
        let pinned = numbers[pinned_index];

        let mut front_index = 0;
        let mut back_index = numbers.len() - 1;

        loop {
            if front_index >= back_index {
                break;
            }

            let front = numbers[front_index];
            let back = numbers[back_index];

            let current_result = pinned + front + back;

            if current_result == TARGET {
                println!("found 2020 = {} + {} + {}", pinned, front, back);
                println!("{} * {} * {} = {}", pinned, front, back, pinned * front * back);
                return Ok(());
            } else if current_result < TARGET {
                front_index += 1;

                if front_index == pinned_index {
                    front_index += 1;
                }
            } else {
                back_index -= 1;

                if back_index == pinned_index {
                    back_index -= 1;
                }
            }
        }
    }

    Err("could not find result")
}
