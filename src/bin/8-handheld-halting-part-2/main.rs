/* --- Part Two ---

After some careful analysis, you believe that exactly one instruction is corrupted.

Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)

The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.

For example, consider the same program from above:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6

If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.

However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:

nop +0  | 1
acc +1  | 2
jmp +4  | 3
acc +3  |
jmp -3  |
acc -99 |
acc +1  | 4
nop -4  | 5
acc +6  | 6

After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).

Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?
*/
const INPUT: &str = include_str!("input");

fn main() {
    let code = INPUT.lines()
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();

    for (i, instruction) in code.iter().enumerate() {
        let replacement_instruction = match instruction.opcode {
            Opcode::Nop => Instruction { opcode: Opcode::Jmp, ..*instruction },
            Opcode::Jmp => Instruction { opcode: Opcode::Nop, ..*instruction },
            Opcode::Acc => continue
        };

        let mut copy = code.clone();
        copy[i] = replacement_instruction;

        if let Ok(result) = try_run_to_end(copy) {
            println!("fixed instruction at line {}, acc = {}", i, result);
            break;
        }
    }

    let diff = start.elapsed();
    println!("time elapsed: {:?}", diff);
}

fn try_run_to_end(instructions: Vec<Instruction>) -> Result<i32, ()> {
    let mut instructions = instructions.into_iter().map(|i| (i, false)).collect::<Vec<_>>();

    let mut pc = 0;
    let mut acc = 0;

    while let Some((instruction, executed)) = instructions.get_mut(pc as usize) {
        if *executed {
            return Err(());
        }

        *executed = true;

        match instruction.opcode {
            Opcode::Jmp => {
                pc = pc + instruction.value;
                continue;
            }

            Opcode::Nop => {}

            Opcode::Acc => acc += instruction.value
        }

        pc += 1;
    }

    Ok(acc)
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