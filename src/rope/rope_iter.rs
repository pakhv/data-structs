use super::rope_node::{RopeNode, RopeNodeWrapper};
use std::rc::Rc;

pub struct RopeIter {
    pub nodes_stack: Vec<Rc<RopeNode>>,
}

impl RopeIter {
    fn collect_parent_right_nodes(&mut self, parent: &RopeNode) {
        match parent {
            RopeNode::Node(parent_node) => match parent_node.right.0.as_ref() {
                RopeNode::Node(right_node) => {
                    self.nodes_stack.push(Rc::clone(&parent_node.right.0));
                    let mut cur_node = Rc::clone(&right_node.left.0);

                    loop {
                        match cur_node.as_ref() {
                            RopeNode::Node(node) => {
                                self.nodes_stack.push(Rc::clone(&cur_node));
                                cur_node = Rc::clone(&node.left.0);
                            }
                            RopeNode::Leaf(_) => {
                                self.nodes_stack.push(Rc::clone(&cur_node));
                                break;
                            }
                            RopeNode::None => break,
                        }
                    }
                }
                RopeNode::Leaf(_) => self
                    .nodes_stack
                    .push(Rc::clone(&parent_node.right.0).into()),
                RopeNode::None => (),
            },
            RopeNode::Leaf(_) | RopeNode::None => (),
        }
    }
}

impl Iterator for RopeIter {
    type Item = RopeNodeWrapper;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.nodes_stack.pop() {
                Some(rope_node) => match rope_node.as_ref() {
                    RopeNode::Leaf(_) => {
                        match self.nodes_stack.pop() {
                            Some(parent) => self.collect_parent_right_nodes(parent.as_ref()),
                            None => (),
                        };

                        return Some(rope_node.into());
                    }
                    RopeNode::Node(_) => {
                        self.collect_parent_right_nodes(rope_node.as_ref());
                    }
                    RopeNode::None => (),
                },
                None => return None,
            }
        }
    }
}
