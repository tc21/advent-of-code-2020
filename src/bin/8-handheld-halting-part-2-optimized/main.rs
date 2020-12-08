/* --- Optimization Note ---
I haven't removed the benchmark code, so anyone can run this themselves.
Tl;dr 4x total runtime improvement; 90x search time improvement for example input (n = 623)

We can optimize Day 8 Part 2 as a Dijkstra's algorithm problem. We can represent
the ops we have to change as costs for edge traversal.

In this case, since we know there is only one path with a cost of 1, and no paths
with a cost of 0, we can actually perform a depth-first search constraining max_cost to 1.

I believe the performance of Dijkstra's is |E| + |V|log|V|, which works out to
2N + N*logN in our scenario, while our DFS with max_cost is
|{all possible remaining costs}| * (|E| + |V|), which works out to 2 * (2N + N).
demonstrating that indeed this is only possible under the constraint of max_cost=1.

Compare to the naive algorithm's runtime of N^2.
*/
const INPUT: &str = include_str!("input");

fn main() {
    let code = INPUT.lines()
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();

    // last node indicating end
    let mut graph = Graph::new(code.len() + 1);

    for (index, instruction) in code.iter().enumerate() {
        match instruction.opcode {
            Opcode::Nop => {
                graph.create_edge(index, index + 1, 0);
                graph.create_edge(index, ((index as i32) + instruction.value) as usize, 1);
            }
            Opcode::Acc => {
                graph.create_edge(index, index + 1, 0);
            }
            Opcode::Jmp => {
                graph.create_edge(index, index + 1, 1);
                graph.create_edge(index, ((index as i32) + instruction.value) as usize, 0);
            }
        }
    }

    let search_start = std::time::Instant::now();

    let pathway = graph.reverse_search(graph.nodes.len() - 1, 0, 1).unwrap();

    println!("time elapsed (search): {:?}", search_start.elapsed());

    let accumulator_result = pathway.into_iter()
        .map(|edge| code[edge.target])
        .filter(|instruction| if let Opcode::Acc = instruction.opcode { true } else { false })
        .map(|instruction| instruction.value)
        .sum::<i32>();


    dbg!(accumulator_result);

    println!("time elapsed (total): {:?}", start.elapsed());
}

struct Graph {
    pub nodes: Vec<Node>
}

#[derive(Debug, Clone)]
struct Edge {
    pub target: usize,
    pub cost: usize
}

impl Edge {
    pub fn new(target: usize, cost: usize) -> Self {
        Self { target, cost }
    }
}

#[derive(Debug, Clone)]
struct Node {
    pub parents: Vec<Edge>,
    pub children: Vec<Edge>
}

impl Node {
    pub fn new() -> Self {
        Self { parents: vec![], children: vec![] }
    }
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Self { nodes: vec![Node::new(); size] }
    }

    pub fn create_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.nodes[from].children.push(Edge::new(to, cost));
        self.nodes[to].parents.push(Edge::new(from, cost));
    }

    // depth-first search since we know there's only one solution
    pub fn reverse_search(&self, from: usize, to: usize, max_cost: usize) -> Option<Vec<&Edge>> {
        let goal = to;
        let start = from;

        if start == goal {
            return Some(vec![])
        }

        for edge in &self.nodes[start].parents {
            if edge.cost <= max_cost {
                if let Some(mut path) = self.reverse_search(edge.target, goal, max_cost - edge.cost) {
                    path.push(edge);
                    return Some(path)
                }
            }
        }

        None
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    pub opcode: Opcode,
    pub value: i32
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Nop, Acc, Jmp,
}

impl Instruction {
    pub fn parse(instruction: &str) -> Self {
        let mut parts = instruction.split_ascii_whitespace();

        let instruction = parts.next().expect("could not get instruction");
        let argument = parts.next().expect("could not get argument")
            .parse::<i32>().expect("failed to parse int");

        let opcode = match instruction {
            "nop" => Opcode::Nop,
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            _ => panic!("invalid instruction")
        };

        Instruction { opcode, value: argument }
    }
}
