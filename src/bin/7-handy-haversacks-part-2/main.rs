/* --- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?
*/
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("input");

fn main() {
    let rules = INPUT.lines().map(parse_rule);
    let graph = DirectionalGraph::from_rules(rules);

    let mut dynamic_results = HashMap::new();

    let contained = number_of_bags_contained(
        "shiny gold",
        &|x| graph.vertices[x].children.iter().map(|(s, c)| (s as &str, *c)),
        &mut dynamic_results
    );

    dbg!(&dynamic_results);

    println!("result: {}", contained);
}

fn number_of_bags_contained<'a, F, T>(
    source: &'a str, get_children: &F, dynamic_results: &mut HashMap<String, usize>
) -> usize
where F: Fn(&str) -> T, T: Iterator<Item = (&'a str, usize)> {
    if dynamic_results.contains_key(source) {
        return dynamic_results[source]
    }

    let mut result = 0;

    for (child, count) in get_children(source) {
        result += count * (1 + number_of_bags_contained(child, get_children, dynamic_results));
    }

    dynamic_results.insert(source.to_owned(), result);

    result
}

#[derive(Debug)]
struct Rule {
    pub name: String,
    pub children: HashMap<String, usize>
}

fn parse_rule(line: &str) -> Rule {
    lazy_static! {
        static ref RULE_PARSER: Regex = Regex::new(r"(.*) bags contain (.*).").unwrap();
    }

    let captures = RULE_PARSER.captures(line).expect("bad RULE_PARSER");

    let parent_color = captures[1].to_owned();
    let child_colors = parse_child_colors(&captures[2]);

    Rule { name: parent_color, children: child_colors }
}

fn parse_child_colors(desc: &str) -> HashMap<String, usize> {
    lazy_static! {
        static ref CHILD_COLOR_PARSER: Regex = Regex::new(r"(\d+) (.*) bags?").unwrap();
    }

    if desc == "no other bags" {
        return HashMap::new()
    }

    desc.split(", ")
        .map(|child| CHILD_COLOR_PARSER.captures(child).unwrap())
        .map(|captures| (captures[2].to_owned(), captures[1].parse().unwrap()))
        .collect()
}

type Vertex = Rule;

#[derive(Debug)]
struct DirectionalGraph {
    pub vertices: HashMap<String, Vertex>
}

impl DirectionalGraph {
    pub fn from_rules<I>(rules: I) -> Self
    where I: Iterator<Item = Rule> {
        let vertices = rules
            .map(|rule| (rule.name.to_owned(), rule))
            .collect();

        Self { vertices }
    }
}
