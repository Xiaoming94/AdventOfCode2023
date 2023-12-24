use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(arg: char) -> Self {
        match arg {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("No instruction corresponds to: {:?}", arg),
        }
    }
}

type InstructionList = VecDeque<Instruction>;
type PathChoice = HashMap<Instruction, String>;
type GraphInstance = HashMap<String, PathChoice>;

type ProblemInstance = (InstructionList, GraphInstance);

fn build_graph(network_string: &str) -> ProblemInstance {
    fn build_instructions(instructions_str: &str) -> InstructionList {
        instructions_str
            .chars()
            .into_iter()
            .map(Instruction::from)
            .collect()
    }

    fn build_node(graph_str: &str) -> (String, PathChoice) {
        if let Some((node, left_and_right)) = graph_str.split_once(" = ") {
            let unwrapped = left_and_right.replace("(", "").replace(")", "");

            if let Some((left, right)) = unwrapped.split_once(", ") {
                let instructions = HashMap::from([
                    (Instruction::Left, left.to_owned()),
                    (Instruction::Right, right.to_owned()),
                ]);

                (node.to_owned(), instructions)
            } else {
                panic!("Malformatted Input {}", graph_str)
            }
        } else {
            panic!("Malformatted Input {}", graph_str);
        }
    }

    if let Some((instruction_str, graph_str)) = network_string.split_once("\n\n") {
        let instructions: InstructionList = build_instructions(instruction_str);
        let graph = graph_str.split('\n').map(build_node).collect();
        (instructions, graph)
    } else {
        panic!("Error, incorrect input formatting: {}", network_string);
    }
}

fn walk_graph(network_graph: GraphInstance, mut instructions: InstructionList) -> u32 {
    let mut steps = 0u32;
    let mut current_node = "AAA";

    while current_node.ne("ZZZ") {
        steps += 1;
        let current_instruction = instructions.pop_front().unwrap();
        if let Some(paths) = network_graph.get(current_node) {
            current_node = paths.get(&current_instruction).unwrap();
        }
        instructions.push_back(current_instruction);
    }
    steps
}

pub fn find_path_through_network(network_string: &str) -> u32 {
    let (instructions, network_graph): ProblemInstance = build_graph(network_string);
    walk_graph(network_graph, instructions)
}
