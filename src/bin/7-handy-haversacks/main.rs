/* --- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
*/
use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("input");

fn main() {
    let rules = INPUT.lines().map(parse_rule);
    let graph = DirectionalGraph::from_rules(rules);

    let found = explore_destinations(
        "shiny gold",
        |x| graph.vertices[x].parents.iter().map(|s| s as &str)
    );

    dbg!(&found);

    println!("found {} colors", found.len());
}

// breadth-first search of all destinations reachable from source
fn explore_destinations<'a, F, T>(source: &'a str, get_targets: F) -> Vec<&str>
where F: Fn(&str) -> T, T: Iterator<Item = &'a str>{
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut found = HashSet::new();

    queue.push_back(source);

    loop {
        let next = match queue.pop_front() {
            Some(x) => x,
            None => break
        };

        explored.insert(next);

        for destination in get_targets(next) {
            found.insert(destination);

            if !explored.contains(&destination) {
                queue.push_back(destination)
            }
        }
    }

    found.into_iter().collect()
}

struct Rule {
    pub name: String,
    pub children: Vec<String>
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

fn parse_child_colors(desc: &str) -> Vec<String> {
    lazy_static! {
        static ref CHILD_COLOR_PARSER: Regex = Regex::new(r"\d+ (.*) bags?").unwrap();
    }

    if desc == "no other bags" {
        return vec![]
    }

    desc.split(", ")
        .map(|child| CHILD_COLOR_PARSER.captures(child).unwrap())
        .map(|captures| captures[1].to_owned())
        .collect()
}

#[derive(Debug)]
struct Vertex {
    pub name: String,
    pub children: HashSet<String>,
    pub parents: HashSet<String>
}

impl Vertex {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned(), children: HashSet::new(), parents: HashSet::new() }
    }
}

#[derive(Debug)]
struct DirectionalGraph {
    pub vertices: HashMap<String, Vertex>
}

impl DirectionalGraph {
    pub fn from_rules<I>(rules: I) -> Self
    where I: Iterator<Item = Rule> {
        let mut graph = Self { vertices: HashMap::new() };

        for rule in rules {
            graph.add_vertex(rule);
        }

        graph
    }

    fn add_vertex(&mut self, rule: Rule) {
        let Rule {name, children} = rule;

        let vertex = self.get_or_create_vertex(&name);

        if vertex.children.len() != 0 {
            eprintln!("vertex already added: {}", vertex.name);
            panic!()
        }

        vertex.children = children.clone().into_iter().collect::<HashSet<_>>();

        for child in children {
            self.get_or_create_vertex(&child).parents.insert(name.to_owned());
        }
    }

    fn get_or_create_vertex(&mut self, name: &str) -> &mut Vertex {
        if !self.vertices.contains_key(name) {
            self.vertices.insert(name.to_owned(), Vertex::new(name));
        }

        self.vertices.get_mut(name).unwrap()
    }
}
