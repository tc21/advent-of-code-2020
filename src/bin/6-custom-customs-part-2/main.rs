/* --- Part Two ---

As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
*/
use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    let groups_questions_covered = INPUT.split("\n\n")
        .map(|group| questions_completely_covered(&mut group.split_ascii_whitespace()));

    let sum_of_those_counts = groups_questions_covered.sum::<usize>();

    println!("sum: {}", sum_of_those_counts);
}

fn questions_completely_covered<I, S>(group_members: &mut I) -> usize
where I: Iterator<Item = S>, S: AsRef<str> {
    let first_member_questions = group_members.next().unwrap()
        .as_ref().chars().collect::<HashSet<_>>();

    group_members.fold(first_member_questions, |questions, next| {
        let member_questions = next.as_ref().chars().collect::<HashSet<_>>();

        // I admit this map->collect reduction is not really efficient,
        // but you don't optimize for performance unless it's a problem
        questions
            .intersection(&member_questions)
            .map(|c| *c)
            .collect::<HashSet<_>>()
    }).len()
}
