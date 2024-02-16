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
    pub weight: usize,
    pub value: String,
}

pub struct RopeIter {
    nodes_stack: Vec<Rc<RopeNode>>,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RopeNode::None),
        }
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.nodes_stack.pop() {
            Some(rope_node) => match rope_node.as_ref() {
                RopeNode::Leaf(leaf) => {
                    match self.nodes_stack.pop() {
                        Some(parent) => self.collect_parent_right_nodes(parent.as_ref()),
                        None => (),
                    };

                    return Some(leaf.value.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverse_test() {
        let rope = Rope {
            root: Rc::new(RopeNode::Node(Node {
                left: Rc::new(RopeNode::Node(Node {
                    left: Rc::new(RopeNode::Leaf(Leaf {
                        weight: 5,
                        value: String::from("hello"),
                    })),
                    right: Rc::new(RopeNode::Leaf(Leaf {
                        weight: 5,
                        value: String::from("world"),
                    })),
                    weight: 5,
                })),
                right: Rc::new(RopeNode::Node(Node {
                    left: Rc::new(RopeNode::Leaf(Leaf {
                        weight: 5,
                        value: String::from("My name"),
                    })),
                    right: Rc::new(RopeNode::Leaf(Leaf {
                        weight: 5,
                        value: String::from("is sugondese"),
                    })),
                    weight: 5,
                })),
                weight: 5,
            })),
        };

        let mut iter = rope.iter();
        println!("{}", rope.root);

        assert_eq!(&iter.next().unwrap(), "hello");
        assert_eq!(&iter.next().unwrap(), "world");
        assert_eq!(&iter.next().unwrap(), "My name");
        assert_eq!(&iter.next().unwrap(), "is sugondese");
        assert!(&iter.next().is_none());
    }
}