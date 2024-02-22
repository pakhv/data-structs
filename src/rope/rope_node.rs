use std::{cmp, fmt::Display, rc::Rc};

use crate::helpers::fibonacci_seq::get_fibonacci_number;

use super::rope_iter::RopeIter;

#[derive(Debug)]
pub enum RopeNode {
    Node(Node),
    Leaf(Leaf),
    None,
}

#[derive(Debug)]
pub struct Node {
    pub left: RopeNodeWrapper,
    pub right: RopeNodeWrapper,
    pub weight: usize,
}

#[derive(Debug)]
pub struct Leaf {
    pub value: String,
}

impl RopeNode {}

impl Display for RopeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RopeNode::Node(node) => {
                write!(f, "Node(Left: {}, Right: {})", node.left.0, node.right.0)
            }
            RopeNode::Leaf(leaf) => write!(f, "Leaf(\"{}\")", leaf.value),
            RopeNode::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub struct RopeNodeWrapper(pub Rc<RopeNode>);

impl From<Rc<RopeNode>> for RopeNodeWrapper {
    fn from(value: Rc<RopeNode>) -> Self {
        RopeNodeWrapper(value)
    }
}

impl Display for RopeNodeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl RopeNodeWrapper {
    pub fn concat(self, s2: RopeNodeWrapper) -> RopeNodeWrapper {
        let weight = self
            .iter()
            .map(|n| {
                n.map_leaf()
                    .expect("error while iterating leafs. None leaf node found")
                    .value
                    .len()
            })
            .sum();

        RopeNodeWrapper(Rc::new(RopeNode::Node(Node {
            left: self,
            right: RopeNodeWrapper::from(s2),
            weight,
        })))
    }

    pub fn iter(&self) -> RopeIter {
        let mut nodes_stack: Vec<Rc<RopeNode>> = vec![];
        let mut cur_node = Rc::clone(&self.0);

        loop {
            if let RopeNode::None = cur_node.as_ref() {
                break;
            }

            nodes_stack.push(Rc::clone(&cur_node));

            cur_node = match cur_node.as_ref() {
                RopeNode::Node(node) => Rc::clone(&node.left.0).into(),
                RopeNode::Leaf(_) | RopeNode::None => Rc::new(RopeNode::None),
            }
        }

        RopeIter { nodes_stack }
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.get_char_rec(index, &self.0.as_ref())
    }

    pub fn split(&self, index: usize) -> (RopeNodeWrapper, RopeNodeWrapper) {
        match index {
            0 => (Rc::new(RopeNode::None).into(), Rc::clone(&self.0).into()),
            i if i >= self.len() => (Rc::clone(&self.0).into(), Rc::new(RopeNode::None).into()),
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
                                    left_subtree.push(
                                        Rc::new(RopeNode::Leaf(Leaf {
                                            value: str_part[0..index - cur_idx].to_string(),
                                        }))
                                        .into(),
                                    );
                                    right_subtree.push(
                                        Rc::new(RopeNode::Leaf(Leaf {
                                            value: str_part[index - cur_idx..].to_string(),
                                        }))
                                        .into(),
                                    )
                                }
                                _ if cur_idx < index => {
                                    left_subtree.push(Rc::clone(&node.0).into());
                                }
                                _ if cur_idx >= index => {
                                    right_subtree.push(Rc::clone(&node.0).into());
                                }
                                _ => (),
                            }

                            cur_idx += str_part.len();
                        }
                        None => break,
                    };
                }

                (
                    RopeNodeWrapper::from_iter(left_subtree),
                    RopeNodeWrapper::from_iter(right_subtree),
                )
            }
        }
    }

    pub fn substring(&self, start: usize, len: usize) -> RopeNodeWrapper {
        let mut leafs = vec![];
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
                            leafs.push(Rc::clone(&node.0).into());
                            len_left -= str_part.len();
                        }
                        i if i > start_idx => {
                            let new_value: String =
                                str_part.chars().skip(start_idx).take(len_left).collect();

                            start_idx = 0;
                            len_left -= new_value.len();

                            if new_value.len() > 0 {
                                leafs.push(
                                    Rc::new(RopeNode::Leaf(Leaf { value: new_value })).into(),
                                );
                            }
                        }
                        _ => start_idx -= str_part.len(),
                    }
                }
                None => break,
            };
        }

        RopeNodeWrapper::from_iter(leafs)
    }

    pub fn len(&self) -> usize {
        self.iter().map(|n| n.map_leaf().unwrap().value.len()).sum()
    }

    pub fn get_depth(&self) -> usize {
        match self.0.as_ref() {
            RopeNode::Node(n) => {
                let left = &n.left.get_depth();
                let right = &n.right.get_depth();

                cmp::max(left, right) + 1
            }
            RopeNode::Leaf(_) | RopeNode::None => 0,
        }
    }

    pub fn insert(&self, index: usize, value: String) -> RopeNodeWrapper {
        let new_leaf = RopeNodeWrapper(Rc::new(RopeNode::Leaf(Leaf { value })));

        let new_node = match index {
            0 => new_leaf.concat(Rc::clone(&self.0).into()),
            i if i >= self.len() => RopeNodeWrapper(Rc::clone(&self.0)).concat(new_leaf),
            _ => {
                let (left, right) = self.split(index);

                left.concat(new_leaf).concat(right)
            }
        };

        new_node
    }

    pub fn rebalance(&self) -> RopeNodeWrapper {
        RopeNodeWrapper::from_iter(self.iter())
    }

    pub fn is_balanced(&self) -> bool {
        let depth = self.get_depth();
        let min_length = get_fibonacci_number(depth + 2);

        self.iter().count() >= min_length
    }

    pub fn map_leaf(&self) -> Option<&Leaf> {
        match self.0.as_ref() {
            RopeNode::Leaf(l) => Some(l),
            RopeNode::Node(_) | RopeNode::None => None,
        }
    }

    pub fn is_not_none(&self) -> bool {
        match self.0.as_ref() {
            RopeNode::None => false,
            _ => true,
        }
    }

    fn get_char_rec(&self, index: usize, node: &RopeNode) -> Option<char> {
        match node {
            RopeNode::Node(node) => {
                if index >= node.weight {
                    return self.get_char_rec(index - node.weight, &node.right.0);
                }

                self.get_char_rec(index, &node.left.0)
            }
            RopeNode::Leaf(leaf) => leaf.value.chars().nth(index),
            RopeNode::None => None,
        }
    }
}

impl FromIterator<RopeNodeWrapper> for RopeNodeWrapper {
    fn from_iter<T: IntoIterator<Item = RopeNodeWrapper>>(iter: T) -> Self {
        let mut nodes_with_weights = vec![];

        for node in iter {
            match node.0.as_ref() {
                RopeNode::Leaf(leaf) => {
                    nodes_with_weights.push((Rc::clone(&node.0), leaf.value.len(), 0 as usize))
                }
                RopeNode::Node(_) | RopeNode::None => (),
            }
        }

        loop {
            match nodes_with_weights.len() {
                0 => return RopeNodeWrapper(Rc::new(RopeNode::None)),
                1 => return RopeNodeWrapper(nodes_with_weights.pop().unwrap().0),
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
                                        left: Rc::clone(&left.0).into(),
                                        right: Rc::clone(&right.0).into(),
                                        weight: left.1 + left.2,
                                    })),
                                    left.1 + left.2,
                                    right.1 + right.2,
                                ),
                            }
                        })
                        .filter(|n| RopeNodeWrapper::from(Rc::clone(&n.0)).is_not_none())
                        .collect();
                }
            };
        }
    }
}
