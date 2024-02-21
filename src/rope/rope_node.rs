use std::{fmt::Display, rc::Rc};

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

    pub fn concat(s1: Rc<RopeNode>, s2: Rc<RopeNode>) -> Rc<RopeNode> {
        let weight = Rope::new(Rc::clone(&s1))
            .iter()
            .map(|n| {
                n.map_leaf()
                    .expect("error while iterating leafs. None leaf node found")
                    .value
                    .len()
            })
            .sum();

        Rc::new(RopeNode::Node(Node {
            left: s1,
            right: s2,
            weight,
        }))
    }

    pub fn is_not_none(&self) -> bool {
        match self {
            RopeNode::None => false,
            _ => true,
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
