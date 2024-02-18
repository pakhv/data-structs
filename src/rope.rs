use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
pub struct Rope {
    root: Rc<RopeNode>,
}

#[derive(Debug)]
pub enum RopeNode {
    Node(Node),
    Leaf(Leaf),
    None,
}

#[derive(Debug)]
pub struct Node {
    pub left: Rc<RopeNode>,
    pub right: Rc<RopeNode>,
    pub weight: usize,
}

#[derive(Debug)]
pub struct Leaf {
    pub value: String,
}

pub struct RopeIter {
    nodes_stack: Vec<Rc<RopeNode>>,
}

impl Rope {
    pub fn new(node: Rc<RopeNode>) -> Self {
        let root = match node.as_ref() {
            RopeNode::Leaf(leaf) => Rc::new(RopeNode::Node(Node {
                left: Rc::clone(&node),
                right: Rc::new(RopeNode::None),
                weight: leaf.value.len(),
            })),
            RopeNode::Node(_) | RopeNode::None => Rc::clone(&node),
        };

        Self { root }
    }

    pub fn concat(s1: Rc<RopeNode>, s2: Rc<RopeNode>) -> Result<Self, String> {
        match (s1.as_ref(), s2.as_ref()) {
            (RopeNode::Node(_), RopeNode::Node(_)) => {
                let weight = Rope::new(Rc::clone(&s1))
                    .iter()
                    .map(|n| {
                        n.map_leaf()
                            .expect("error while iterating leafs. None leaf node found")
                            .value
                            .len()
                    })
                    .sum();

                Ok(Rope::new(Rc::new(RopeNode::Node(Node {
                    left: s1,
                    right: s2,
                    weight,
                }))))
            }
            _ => Err(String::from(
                "Both \"s1\" and \"s2\" must be Nodes to concat them",
            )),
        }
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.get_char_rec(index, &self.root)
    }

    pub fn iter(&self) -> RopeIter {
        let mut nodes_stack: Vec<Rc<RopeNode>> = vec![];
        let mut cur_node = Rc::clone(&self.root);

        loop {
            if let RopeNode::None = cur_node.as_ref() {
                break;
            }

            nodes_stack.push(Rc::clone(&cur_node));

            cur_node = match cur_node.as_ref() {
                RopeNode::Node(node) => Rc::clone(&node.left),
                RopeNode::Leaf(_) | RopeNode::None => Rc::new(RopeNode::None),
            }
        }

        RopeIter { nodes_stack }
    }

    fn get_char_rec(&self, index: usize, node: &RopeNode) -> Option<char> {
        match node {
            RopeNode::Node(node) => {
                if index >= node.weight {
                    return self.get_char_rec(index - node.weight, &node.right);
                }

                self.get_char_rec(index, &node.left)
            }
            RopeNode::Leaf(leaf) => leaf.value.chars().nth(index),
            RopeNode::None => None,
        }
    }
}

impl RopeIter {
    fn collect_parent_right_nodes(&mut self, parent: &RopeNode) {
        match parent {
            RopeNode::Node(parent_node) => match parent_node.right.as_ref() {
                RopeNode::Node(right_node) => {
                    self.nodes_stack.push(Rc::clone(&parent_node.right));
                    let mut cur_node = Rc::clone(&right_node.left);

                    loop {
                        match cur_node.as_ref() {
                            RopeNode::Node(node) => {
                                self.nodes_stack.push(Rc::clone(&cur_node));
                                cur_node = Rc::clone(&node.left);
                            }
                            RopeNode::Leaf(_) => {
                                self.nodes_stack.push(Rc::clone(&cur_node));
                                break;
                            }
                            RopeNode::None => break,
                        }
                    }
                }
                RopeNode::Leaf(_) => self.nodes_stack.push(Rc::clone(&parent_node.right)),
                RopeNode::None => (),
            },
            RopeNode::Leaf(_) | RopeNode::None => (),
        }
    }
}

impl Iterator for RopeIter {
    type Item = Rc<RopeNode>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.nodes_stack.pop() {
            Some(rope_node) => match rope_node.as_ref() {
                RopeNode::Leaf(_) => {
                    match self.nodes_stack.pop() {
                        Some(parent) => self.collect_parent_right_nodes(parent.as_ref()),
                        None => (),
                    };

                    Some(rope_node)
                }
                // how did this happen lol?
                RopeNode::Node(_) | RopeNode::None => None,
            },
            None => None,
        }
    }
}

impl Display for RopeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RopeNode::Node(node) => write!(f, "Node(Left: {}, Right: {})", node.left, node.right),
            RopeNode::Leaf(leaf) => write!(f, "Leaf({})", leaf.value),
            RopeNode::None => write!(f, "None"),
        }
    }
}

impl RopeNode {
    pub fn map_leaf(&self) -> Option<&Leaf> {
        match self {
            RopeNode::Leaf(l) => Some(l),
            RopeNode::Node(_) | RopeNode::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverse_test() {
        let rope = Rope {
            root: Rc::new(RopeNode::Node(Node {
                left: Rc::new(RopeNode::Node(Node {
                    left: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("hello"),
                    })),
                    right: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("world"),
                    })),
                    weight: 5,
                })),
                right: Rc::new(RopeNode::Node(Node {
                    left: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("My name"),
                    })),
                    right: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("is sugondese"),
                    })),
                    weight: 5,
                })),
                weight: 5,
            })),
        };

        let mut iter = rope.iter();

        let expected_values = vec!["hello", "world", "My name", "is sugondese"];

        for expected in expected_values {
            assert_eq!(&iter.next().unwrap().map_leaf().unwrap().value, expected);
        }

        assert!(&iter.next().is_none());
    }

    #[test]
    fn get_char_test() {
        let rope = Rope {
            root: Rc::new(RopeNode::Node(Node {
                left: Rc::new(RopeNode::Node(Node {
                    left: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("hello "),
                    })),
                    right: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("world! "),
                    })),
                    weight: 6,
                })),
                right: Rc::new(RopeNode::Node(Node {
                    left: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("My name"),
                    })),
                    right: Rc::new(RopeNode::Leaf(Leaf {
                        value: String::from("is sugondese"),
                    })),
                    weight: 7,
                })),
                weight: 13,
            })),
        };

        assert_eq!(rope.get_char(4).unwrap(), 'o');
        assert_eq!(rope.get_char(6).unwrap(), 'w');
        assert_eq!(rope.get_char(13).unwrap(), 'M');
        assert_eq!(rope.get_char(16).unwrap(), 'n');
        assert_eq!(rope.get_char(16).unwrap(), 'n');
        assert_eq!(rope.get_char(23).unwrap(), 's');
        assert_eq!(rope.get_char(31).unwrap(), 'e');
        assert!(rope.get_char(32).is_none());
    }

    #[test]
    fn concat_test() {
        let node1 = Rc::new(RopeNode::Node(Node {
            left: Rc::new(RopeNode::Leaf(Leaf {
                value: String::from("hello "),
            })),
            right: Rc::new(RopeNode::Leaf(Leaf {
                value: String::from("world! "),
            })),
            weight: 6,
        }));
        let node2 = Rc::new(RopeNode::Node(Node {
            left: Rc::new(RopeNode::Leaf(Leaf {
                value: String::from("My name"),
            })),
            right: Rc::new(RopeNode::Leaf(Leaf {
                value: String::from("is sugondese"),
            })),
            weight: 7,
        }));

        let rope = Rope::concat(node1, node2);
        assert!(rope.is_ok());

        let rope = rope.unwrap();

        let mut iter = rope.iter();

        let expected_values = vec!["hello ", "world! ", "My name", "is sugondese"];

        for expected in expected_values {
            assert_eq!(&iter.next().unwrap().map_leaf().unwrap().value, expected);
        }

        assert!(&iter.next().is_none());
    }
}
