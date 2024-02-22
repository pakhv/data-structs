use std::{cmp, fmt::Display, rc::Rc};

use super::rope::Rope;

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

impl RopeNode {
    pub fn map_leaf(&self) -> Option<&Leaf> {
        match self {
            RopeNode::Leaf(l) => Some(l),
            RopeNode::Node(_) | RopeNode::None => None,
        }
    }

    pub fn is_not_none(&self) -> bool {
        match self {
            RopeNode::None => false,
            _ => true,
        }
    }

    pub fn get_depth(&self) -> usize {
        match self {
            RopeNode::Node(n) => {
                let left = &n.left.get_depth();
                let right = &n.right.get_depth();

                cmp::max(left, right) + 1
            }
            RopeNode::Leaf(_) | RopeNode::None => 0,
        }
    }
}

impl Display for RopeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RopeNode::Node(node) => write!(f, "Node(Left: {}, Right: {})", node.left, node.right),
            RopeNode::Leaf(leaf) => write!(f, "Leaf(\"{}\")", leaf.value),
            RopeNode::None => write!(f, "None"),
        }
    }
}

pub trait RopeConcat<T> {
    fn concat(self, s2: T) -> T;
}

impl RopeConcat<Rc<RopeNode>> for Rc<RopeNode> {
    fn concat(self, s2: Rc<RopeNode>) -> Rc<RopeNode> {
        let weight = Rope::new(Rc::clone(&self))
            .iter()
            .map(|n| {
                n.map_leaf()
                    .expect("error while iterating leafs. None leaf node found")
                    .value
                    .len()
            })
            .sum();

        Rc::new(RopeNode::Node(Node {
            left: self,
            right: s2,
            weight,
        }))
    }
}
