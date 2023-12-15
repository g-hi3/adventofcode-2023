use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>
}

impl Network {
    pub fn new(s: &str) -> Option<Network> {
        let mut lines = s.lines();
        let instructions = lines.next()?;
        let instructions = Instruction::extract(instructions);
        let lines = lines.skip(1);
        let nodes = lines
            .filter_map(Node::new)
            .map(|node| (node.name.clone(), node))
            .collect::<HashMap<String, Node>>();

        Some(Network { instructions, nodes })
    }

    pub fn run_instructions(&self, first_node: &str) -> Option<Path<'_>> {
        let mut nodes = Vec::<&Node>::new();
        let first_node = self.nodes.get(first_node)?;
        let mut node = first_node;
        let mut instructions = self.instructions.iter();
        let mut instruction = instructions.next()?;
        while !node.name.ends_with("Z") {
            nodes.push(node);
            node = self.navigate(&node, &instruction)?;
            instruction = match instructions.next() {
                None => {
                    instructions = self.instructions.iter();
                    instructions.next()?
                }
                Some(next_instruction) => next_instruction
            };
        }

        nodes.push(node);
        Some(Path { nodes })
    }

    fn navigate(&self, from: &Node, instruction: &Instruction) -> Option<&Node> {
        let to = match instruction {
            Instruction::Left => &from.left,
            Instruction::Right => &from.right
        };

        self.nodes.get(to)
    }

    pub fn starting_nodes(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter_map(|(key, value)|
                if key.ends_with("A") { Some(value) }
                else { None })
            .collect::<Vec<&Node>>()
    }
}

#[derive(Debug, PartialEq)]
pub struct Path<'a> {
    nodes: Vec<&'a Node>
}

impl Path<'_> {
    pub fn len(&self) -> usize {
        self.nodes.len() - 1
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn new(s: &str) -> Option<Node> {
        let mut def_parts = s.split('=');
        let name = def_parts
            .next()?
            .trim()
            .to_string();
        let instructions = def_parts
            .next()?
            .trim();

        if instructions.len() < 2 {
            return None;
        }

        let mut instruction_parts = instructions[1..instructions.len()-1]
            .split(',');
        let left = instruction_parts
            .next()?
            .trim()
            .to_string();
        let right = instruction_parts.next()?
            .trim()
            .to_string();

        Some(Self { name, left, right })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right
}

impl Instruction {
    fn extract(s: &str) -> Vec<Self> {
        s.chars().filter_map(Self::new).collect::<Vec<Instruction>>()
    }

    fn new(c: char) -> Option<Self> {
        match c {
            'L' => Some(Instruction::Left),
            'R' => Some(Instruction::Right),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_new() {
        let network = Network::new("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(network, Some(
            Network {
                instructions: vec![
                    Instruction::Right,
                    Instruction::Left
                ],
                nodes: HashMap::from_iter([
                                              ("AAA".to_string(), Node { name: String::from("AAA"), left: String::from("BBB"), right: String::from("CCC") }),
                                               ("BBB".to_string(), Node { name: String::from("BBB"), left: String::from("DDD"), right: String::from("EEE") }),
                                                ("CCC".to_string(), Node { name: String::from("CCC"), left: String::from("ZZZ"), right: String::from("GGG") }),
                                                 ("DDD".to_string(), Node { name: String::from("DDD"), left: String::from("DDD"), right: String::from("DDD") }),
                                                  ("EEE".to_string(), Node { name: String::from("EEE"), left: String::from("EEE"), right: String::from("EEE") }),
                                                   ("GGG".to_string(), Node { name: String::from("GGG"), left: String::from("GGG"), right: String::from("GGG") }),
                                                    ("ZZZ".to_string(), Node { name: String::from("ZZZ"), left: String::from("ZZZ"), right: String::from("ZZZ") }),
                ])
            }
        ));

        let network = Network::new("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(network, Some(
            Network {
                instructions: vec![
                    Instruction::Left,
                    Instruction::Left,
                    Instruction::Right
                ],
                nodes: HashMap::from_iter([
                    ("AAA".to_string(), Node { name: String::from("AAA"), left: String::from("BBB"), right: String::from("BBB") }),
                     ("BBB".to_string(), Node { name: String::from("BBB"), left: String::from("AAA"), right: String::from("ZZZ") }),
                      ("ZZZ".to_string(), Node { name: String::from("ZZZ"), left: String::from("ZZZ"), right: String::from("ZZZ") })
                ])
            }
        ));
    }

    #[test]
    fn test_network_navigate() {
        let network = Network {
            instructions: vec![],
            nodes: HashMap::from_iter([
                ("AAA".to_string(), Node { name: String::from("AAA"), left: String::from("BBB"), right: String::from("BBB") }),
                 ("BBB".to_string(), Node { name: String::from("BBB"), left: String::from("AAA"), right: String::from("ZZZ") }),
                  ("ZZZ".to_string(), Node { name: String::from("ZZZ"), left: String::from("ZZZ"), right: String::from("ZZZ") })
            ])
        };

        let from = network.nodes.get("AAA").unwrap();
        let to = network.nodes.get("BBB").unwrap();
        assert_eq!(network.navigate(from, &Instruction::Left), Some(to));

        let from = network.nodes.get("BBB").unwrap();
        let to = network.nodes.get("ZZZ").unwrap();
        assert_eq!(network.navigate(from, &Instruction::Right), Some(to));

        let from = network.nodes.get("ZZZ").unwrap();
        let to = network.nodes.get("ZZZ").unwrap();
        assert_eq!(network.navigate(from, &Instruction::Right), Some(to));

        let from = network.nodes.get("ZZZ").unwrap();
        let to = network.nodes.get("ZZZ").unwrap();
        assert_eq!(network.navigate(from, &Instruction::Left), Some(to));
    }

    #[test]
    fn test_network_run_instructions() {
        let network = Network::new("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)").unwrap();
        let path = network.run_instructions("AAA");
        assert_eq!(path, Some(Path {
            nodes: vec![
                network.nodes.get("AAA").unwrap(),
                network.nodes.get("CCC").unwrap(),
                network.nodes.get("ZZZ").unwrap()
            ]
        }));
        assert_eq!(path.unwrap().len(), 2);

        let network = Network::new("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)").unwrap();
        let path = network.run_instructions("AAA").unwrap();
        assert_eq!(path.len(), 6);

        let network = Network::new("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)").unwrap();
        let path = network.run_instructions("AAA").unwrap();
        assert_eq!(path.len(), 1);
        let path = network.run_instructions("BBB").unwrap();
        assert_eq!(path.len(), 1);
    }

    #[test]
    fn test_node_new() {
        let node = Node::new("AAA = (BBB, CCC)");
        assert_eq!(node, Some(Node {
            name: String::from("AAA"),
            left: String::from("BBB"),
            right: String::from("CCC")
        }));

        let node = Node::new("ZZZ = (ZZZ, ZZZ)");
        assert_eq!(node, Some(Node {
            name: String::from("ZZZ"),
            left: String::from("ZZZ"),
            right: String::from("ZZZ")
        }));
    }

    #[test]
    fn test_instruction_new() {
        let instruction = Instruction::new('R');
        assert_eq!(instruction, Some(Instruction::Right));

        let instruction = Instruction::new('L');
        assert_eq!(instruction, Some(Instruction::Left));

        let instruction = Instruction::new('X');
        assert_eq!(instruction, None);
    }

    #[test]
    fn test_instruction_extract() {
        let instructions = Instruction::extract("RL");
        assert_eq!(instructions, vec![
            Instruction::Right,
            Instruction::Left
        ]);

        let instructions = Instruction::extract("LLR");
        assert_eq!(instructions, vec![
            Instruction::Left,
            Instruction::Left,
            Instruction::Right
        ]);
    }
}