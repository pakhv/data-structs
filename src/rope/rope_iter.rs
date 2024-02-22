use super::rope_node::{RopeNode, RopeNodeType};
use std::rc::Rc;

pub struct RopeIter {
    pub nodes_stack: Vec<Rc<RopeNodeType>>,
}

impl RopeIter {
    fn collect_parent_right_nodes(&mut self, parent: &RopeNodeType) {
        match parent {
            RopeNodeType::Node(parent_node) => match parent_node.right.0.as_ref() {
                RopeNodeType::Node(right_node) => {
                    self.nodes_stack.push(Rc::clone(&parent_node.right.0));
                    let mut cur_node = Rc::clone(&right_node.left.0);

                    loop {
                        match cur_node.as_ref() {
                            RopeNodeType::Node(node) => {
                                self.nodes_stack.push(Rc::clone(&cur_node));
                                cur_node = Rc::clone(&node.left.0);
                            }
                            RopeNodeType::Leaf(_) => {
                                self.nodes_stack.push(Rc::clone(&cur_node));
                                break;
                            }
                            RopeNodeType::None => break,
                        }
                    }
                }
                RopeNodeType::Leaf(_) => self
                    .nodes_stack
                    .push(Rc::clone(&parent_node.right.0).into()),
                RopeNodeType::None => (),
            },
            RopeNodeType::Leaf(_) | RopeNodeType::None => (),
        }
    }
}

impl Iterator for RopeIter {
    type Item = RopeNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.nodes_stack.pop() {
                Some(rope_node) => match rope_node.as_ref() {
                    RopeNodeType::Leaf(_) => {
                        match self.nodes_stack.pop() {
                            Some(parent) => self.collect_parent_right_nodes(parent.as_ref()),
                            None => (),
                        };

                        return Some(rope_node.into());
                    }
                    RopeNodeType::Node(_) => {
                        self.collect_parent_right_nodes(rope_node.as_ref());
                    }
                    RopeNodeType::None => (),
                },
                None => return None,
            }
        }
    }
}
