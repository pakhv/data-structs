use super::rope_node::RopeNode;
use std::rc::Rc;

pub struct RopeIter {
    pub nodes_stack: Vec<Rc<RopeNode>>,
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
