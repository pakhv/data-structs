use crate::rope::rope_node::Leaf;
use std::rc::Rc;

use super::{
    rope_iter::RopeIter,
    rope_node::{Node, RopeNode},
};

#[derive(Debug)]
pub struct Rope {
    root: Rc<RopeNode>,
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

    pub fn concat(s1: Rc<RopeNode>, s2: Rc<RopeNode>) -> Self {
        Rope::new(RopeNode::concat(s1, s2))
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.get_char_rec(index, &self.root)
    }

    pub fn iter(&self) -> RopeIter {
        Rope::iter_node(Rc::clone(&self.root))
    }

    pub fn iter_node(node: Rc<RopeNode>) -> RopeIter {
        let mut nodes_stack: Vec<Rc<RopeNode>> = vec![];
        let mut cur_node = node;

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

    pub fn substring(&mut self, start: usize, len: usize) {
        let mut leafs: Vec<Rc<RopeNode>> = vec![];
        let mut start_idx = start;
        let mut len_left = len;

        let mut iter = self.iter();

        loop {
            if len_left <= 0 {
                break;
            }

            match iter.next() {
                Some(node) => {
                    let str_part = &node.map_leaf().expect("leaf expected").value;

                    if str_part.len() == 0 {
                        continue;
                    }

                    let str_part_max_idx = str_part.len() - 1;

                    match str_part_max_idx {
                        _ if start_idx == 0 && len_left >= str_part.len() - 1 => {
                            leafs.push(Rc::clone(&node));
                            len_left -= str_part.len();
                        }
                        i if i > start_idx => {
                            let new_value: String =
                                str_part.chars().skip(start_idx).take(len_left).collect();

                            start_idx = 0;
                            len_left -= new_value.len();

                            if new_value.len() > 0 {
                                leafs.push(Rc::new(RopeNode::Leaf(Leaf { value: new_value })));
                            }
                        }
                        _ => start_idx -= str_part.len(),
                    }
                }
                None => break,
            };
        }

        self.root = Rope::from_iter(leafs).root;
    }

    pub fn rebalance(&mut self) {
        self.root = Rope::from_iter(self.iter()).root;
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

impl FromIterator<Rc<RopeNode>> for Rope {
    fn from_iter<T: IntoIterator<Item = Rc<RopeNode>>>(iter: T) -> Self {
        let mut nodes_with_weights = vec![];

        for node in iter {
            match node.as_ref() {
                RopeNode::Leaf(leaf) => {
                    nodes_with_weights.push((Rc::clone(&node), leaf.value.len(), 0 as usize))
                }
                RopeNode::Node(_) | RopeNode::None => (),
            }
        }

        loop {
            match nodes_with_weights.len() {
                0 => {
                    return Rope {
                        root: Rc::new(RopeNode::None),
                    }
                }
                1 => {
                    return Rope {
                        root: Rc::clone(&nodes_with_weights.first().unwrap().0),
                    }
                }
                _ => {
                    let nodes_num = (nodes_with_weights.len() as f32 / 2.0).ceil() as usize;

                    nodes_with_weights = (0..nodes_num)
                        .map(|i| {
                            let left = RopeNode::get_by_index(&nodes_with_weights, 2 * i);
                            let right = RopeNode::get_by_index(&nodes_with_weights, 2 * i + 1);

                            (
                                Rc::new(RopeNode::Node(Node {
                                    left: Rc::clone(&left.0),
                                    right: Rc::clone(&right.0),
                                    weight: left.1,
                                })),
                                left.1 + left.2,
                                right.1 + right.2,
                            )
                        })
                        .collect();
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

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

        let mut iter = rope.iter();

        let expected_values = vec!["hello ", "world! ", "My name", "is sugondese"];

        for expected in expected_values {
            assert_eq!(&iter.next().unwrap().map_leaf().unwrap().value, expected);
        }

        assert!(&iter.next().is_none());
    }

    #[test]
    fn substring_test() {
        let mut rope = Rope {
            root: Rc::new(RopeNode::Node(Node {
                left: Rc::new(RopeNode::Node(Node {
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
                right: Rc::new(RopeNode::Node(Node {
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
                weight: 1,
            })),
        };

        rope.substring(0, 600);
        println!("{:?}", rope.root);
        panic!();
    }
}
