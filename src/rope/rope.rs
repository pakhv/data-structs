use crate::helpers::fibonacci_seq::get_fibonacci_number;
use crate::rope::rope_node::Leaf;
use std::{fmt::Display, rc::Rc};

use super::{
    rope_iter::RopeIter,
    rope_node::{Node, RopeConcat, RopeNode},
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
            RopeNode::Node(_) | RopeNode::None => node,
        };

        Self { root }
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

    pub fn split(&self, index: usize) -> (Rc<RopeNode>, Rc<RopeNode>) {
        match index {
            0 => (Rc::new(RopeNode::None), Rc::clone(&self.root)),
            i if i >= self.len() => (Rc::clone(&self.root), Rc::new(RopeNode::None)),
            _ => {
                let mut cur_idx = 0;
                let mut iter = self.iter();

                let mut left_subtree = vec![];
                let mut right_subtree = vec![];

                loop {
                    match iter.next() {
                        Some(node) => {
                            let str_part = &node.map_leaf().expect("leaf expected").value;

                            let str_part_max_idx = cur_idx + str_part.len();

                            match str_part_max_idx {
                                i if cur_idx < index && i > index => {
                                    left_subtree.push(Rc::new(RopeNode::Leaf(Leaf {
                                        value: str_part[0..index - cur_idx].to_string(),
                                    })));
                                    right_subtree.push(Rc::new(RopeNode::Leaf(Leaf {
                                        value: str_part[index - cur_idx..].to_string(),
                                    })))
                                }
                                _ if cur_idx < index => {
                                    left_subtree.push(Rc::clone(&node));
                                }
                                _ if cur_idx >= index => {
                                    right_subtree.push(Rc::clone(&node));
                                }
                                _ => (),
                            }

                            cur_idx += str_part.len();
                        }
                        None => break,
                    };
                }

                (
                    Rope::from_iter(left_subtree).root,
                    Rope::from_iter(right_subtree).root,
                )
            }
        }
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
        if self.is_balanced() {
            return;
        }

        self.root = Rope::from_iter(self.iter()).root;
    }

    pub fn is_balanced(&self) -> bool {
        let depth = self.root.get_depth();
        let min_length = get_fibonacci_number(depth + 2);

        self.iter().count() >= min_length
    }

    pub fn len(&self) -> usize {
        self.iter().map(|n| n.map_leaf().unwrap().value.len()).sum()
    }

    pub fn insert(&mut self, index: usize, value: String) {
        let new_leaf = Rc::new(RopeNode::Leaf(Leaf { value }));

        let new_root = match index {
            0 => new_leaf.concat(Rc::clone(&self.root)),
            i if i >= self.len() => Rc::clone(&self.root).concat(new_leaf),
            _ => {
                let (left, right) = self.split(index);

                left.concat(new_leaf).concat(right)
            }
        };

        self.root = new_root;
        self.rebalance();
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
                    };
                }
                _ => {
                    let nodes_num = (nodes_with_weights.len() as f32 / 2.0).ceil() as usize;

                    nodes_with_weights = (0..nodes_num)
                        .map(|i| {
                            let left = nodes_with_weights.get(2 * i);
                            let right = nodes_with_weights.get(2 * i + 1);

                            match (left, right) {
                                (None, None) => (Rc::new(RopeNode::None), 0, 0),
                                (None, Some(n)) | (Some(n), None) => (Rc::clone(&n.0), n.1, n.2),
                                (Some(left), Some(right)) => (
                                    Rc::new(RopeNode::Node(Node {
                                        left: Rc::clone(&left.0),
                                        right: Rc::clone(&right.0),
                                        weight: left.1 + left.2,
                                    })),
                                    left.1 + left.2,
                                    right.1 + right.2,
                                ),
                            }
                        })
                        .filter(|n| (&n.0).is_not_none())
                        .collect();
                }
            };
        }
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)
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

        let rope = Rope::new(node1.concat(node2));

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

        rope.substring(3, 40);

        let expected = r#"Node(Left: Node(Left: Node(Left: Leaf("lo "), Right: Leaf("world! ")), Right: Node(Left: Leaf("My name"), Right: Leaf("is sugondese"))), Right: Node(Left: Leaf("hello "), Right: Leaf("world")))"#;

        assert_eq!(expected, format!("{rope}"));
    }

    #[test]
    fn get_depth_test() {
        let node = Rc::new(RopeNode::Node(Node {
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
            right: Rc::new(RopeNode::None),
            weight: 0,
        }));

        assert_eq!(3, node.get_depth());
    }

    #[test]
    fn rebalance_test() {
        let mut rope = Rope::new(Rc::new(RopeNode::Node(Node {
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
            right: Rc::new(RopeNode::None),
            weight: 0,
        })));

        rope.rebalance();

        let expected = r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: Node(Left: Leaf("My name"), Right: Leaf("is sugondese")))"#;

        assert_eq!(expected, format!("{rope}"));
    }

    #[test]
    fn split_test() {
        let rope = Rope::new(Rc::new(RopeNode::Node(Node {
            left: Rc::new(RopeNode::Node(Node {
                left: Rc::new(RopeNode::Leaf(Leaf {
                    value: String::from("hello "),
                })),
                right: Rc::new(RopeNode::Leaf(Leaf {
                    value: String::from("world! "),
                })),
                weight: 6,
            })),
            right: Rc::new(RopeNode::None),
            weight: 13,
        })));

        let expected_result = vec![
            (
                0,
                "None",
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: None)"#,
            ),
            (
                13,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: None)"#,
                "None",
            ),
            (6, r#"Leaf("hello ")"#, r#"Leaf("world! ")"#),
            (
                9,
                r#"Node(Left: Leaf("hello "), Right: Leaf("wor"))"#,
                r#"Leaf("ld! ")"#,
            ),
            (
                20,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: None)"#,
                "None",
            ),
        ];

        for (idx, exp_left, exp_right) in expected_result {
            let (left, right) = rope.split(idx);
            assert_eq!(exp_left, format!("{left}"));
            assert_eq!(exp_right, format!("{right}"));
        }
    }

    #[test]
    fn insert_test() {
        let root_node = Rc::new(RopeNode::Node(Node {
            left: Rc::new(RopeNode::Node(Node {
                left: Rc::new(RopeNode::Leaf(Leaf {
                    value: String::from("hello "),
                })),
                right: Rc::new(RopeNode::Leaf(Leaf {
                    value: String::from("world! "),
                })),
                weight: 6,
            })),
            right: Rc::new(RopeNode::None),
            weight: 13,
        }));

        let expected_result = vec![
            (
                0,
                r#"Node(Left: Node(Left: Leaf("new_leaf"), Right: Leaf("hello ")), Right: Leaf("world! "))"#,
            ),
            (
                13,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: Leaf("new_leaf"))"#,
            ),
            (
                6,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("new_leaf")), Right: Leaf("world! "))"#,
            ),
            (
                9,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("wor")), Right: Node(Left: Leaf("new_leaf"), Right: Leaf("ld! ")))"#,
            ),
            (
                20,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: Leaf("new_leaf"))"#,
            ),
        ];

        for (idx, exp_result) in expected_result {
            let mut rope = Rope::new(Rc::clone(&root_node));
            rope.insert(idx, String::from("new_leaf"));
            assert_eq!(exp_result, format!("{}", rope.root));
        }
    }
}
