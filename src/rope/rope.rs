use std::{fmt::Display, rc::Rc};

use super::{
    rope_iter::RopeIter,
    rope_node::{Node, RopeNode, RopeNodeType},
};

#[derive(Debug)]
pub struct Rope {
    root: RopeNode,
}

impl Rope {
    pub fn new(node: RopeNode) -> Self {
        let root = match node.0.as_ref() {
            RopeNodeType::Leaf(leaf) => Rc::new(RopeNodeType::Node(Node {
                left: Rc::clone(&node.0).into(),
                right: Rc::new(RopeNodeType::None).into(),
                weight: leaf.value.len(),
            }))
            .into(),
            RopeNodeType::Node(_) | RopeNodeType::None => node,
        };

        Self { root }
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.root.get_char(index)
    }

    pub fn iter(&self) -> RopeIter {
        self.root.iter()
    }

    pub fn split(&self, index: usize) -> (RopeNode, RopeNode) {
        self.root.split(index)
    }

    pub fn substring(&mut self, start: usize, len: usize) {
        self.root = self.root.substring(start, len);
    }

    pub fn rebalance(&mut self) {
        if self.is_balanced() {
            return;
        }

        self.root = self.root.rebalance();
    }

    pub fn is_balanced(&self) -> bool {
        self.root.is_balanced()
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn insert(&mut self, index: usize, value: String) {
        self.root = self.root.insert(index, value);
        self.rebalance();
    }

    pub fn delete(&mut self, start: usize, len: usize) {
        self.root = self.root.delete(start, len);
    }
}

impl FromIterator<RopeNode> for Rope {
    fn from_iter<T: IntoIterator<Item = RopeNode>>(iter: T) -> Self {
        Rope {
            root: RopeNode::from_iter(iter),
        }
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rope::rope_node::Leaf;

    #[test]
    fn traverse_test() {
        let rope = Rope {
            root: Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("hello"),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("world"),
                    }))
                    .into(),
                    weight: 5,
                }))
                .into(),
                right: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("My name"),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("is sugondese"),
                    }))
                    .into(),
                    weight: 5,
                }))
                .into(),
                weight: 5,
            }))
            .into(),
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
            root: Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("hello "),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("world! "),
                    }))
                    .into(),
                    weight: 6,
                }))
                .into(),
                right: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("My name"),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("is sugondese"),
                    }))
                    .into(),
                    weight: 7,
                }))
                .into(),
                weight: 13,
            }))
            .into(),
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
        let node1: RopeNode = Rc::new(RopeNodeType::Node(Node {
            left: Rc::new(RopeNodeType::Leaf(Leaf {
                value: String::from("hello "),
            }))
            .into(),
            right: Rc::new(RopeNodeType::Leaf(Leaf {
                value: String::from("world! "),
            }))
            .into(),
            weight: 6,
        }))
        .into();

        let node2 = Rc::new(RopeNodeType::Node(Node {
            left: Rc::new(RopeNodeType::Leaf(Leaf {
                value: String::from("My name"),
            }))
            .into(),
            right: Rc::new(RopeNodeType::Leaf(Leaf {
                value: String::from("is sugondese"),
            }))
            .into(),
            weight: 7,
        }))
        .into();

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
            root: Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Node(Node {
                        left: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("hello "),
                        }))
                        .into(),
                        right: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("world! "),
                        }))
                        .into(),
                        weight: 6,
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Node(Node {
                        left: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("My name"),
                        }))
                        .into(),
                        right: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("is sugondese"),
                        }))
                        .into(),
                        weight: 7,
                    }))
                    .into(),
                    weight: 13,
                }))
                .into(),
                right: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Node(Node {
                        left: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("hello "),
                        }))
                        .into(),
                        right: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("world! "),
                        }))
                        .into(),
                        weight: 6,
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Node(Node {
                        left: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("My name"),
                        }))
                        .into(),
                        right: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("is sugondese"),
                        }))
                        .into(),
                        weight: 7,
                    }))
                    .into(),
                    weight: 13,
                }))
                .into(),
                weight: 1,
            }))
            .into(),
        };

        rope.substring(3, 40);

        let expected = r#"Node(Left: Node(Left: Node(Left: Leaf("lo "), Right: Leaf("world! ")), Right: Node(Left: Leaf("My name"), Right: Leaf("is sugondese"))), Right: Node(Left: Leaf("hello "), Right: Leaf("world")))"#;

        assert_eq!(expected, format!("{rope}"));
    }

    #[test]
    fn get_depth_test() {
        let node: RopeNode = Rc::new(RopeNodeType::Node(Node {
            left: Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("hello "),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("world! "),
                    }))
                    .into(),
                    weight: 6,
                }))
                .into(),
                right: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("My name"),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("is sugondese"),
                    }))
                    .into(),
                    weight: 7,
                }))
                .into(),
                weight: 13,
            }))
            .into(),
            right: Rc::new(RopeNodeType::None).into(),
            weight: 0,
        }))
        .into();

        assert_eq!(3, node.get_depth());
    }

    #[test]
    fn rebalance_test() {
        let mut rope = Rope::new(
            Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Node(Node {
                        left: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("hello "),
                        }))
                        .into(),
                        right: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("world! "),
                        }))
                        .into(),
                        weight: 6,
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Node(Node {
                        left: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("My name"),
                        }))
                        .into(),
                        right: Rc::new(RopeNodeType::Leaf(Leaf {
                            value: String::from("is sugondese"),
                        }))
                        .into(),
                        weight: 7,
                    }))
                    .into(),
                    weight: 13,
                }))
                .into(),
                right: Rc::new(RopeNodeType::None).into(),
                weight: 0,
            }))
            .into(),
        );

        rope.rebalance();

        let expected = r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: Node(Left: Leaf("My name"), Right: Leaf("is sugondese")))"#;

        assert_eq!(expected, format!("{rope}"));
    }

    #[test]
    fn split_test() {
        let rope = Rope::new(
            Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Node(Node {
                    left: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("hello "),
                    }))
                    .into(),
                    right: Rc::new(RopeNodeType::Leaf(Leaf {
                        value: String::from("world! "),
                    }))
                    .into(),
                    weight: 6,
                }))
                .into(),
                right: Rc::new(RopeNodeType::None).into(),
                weight: 13,
            }))
            .into(),
        );

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
        let root_node: RopeNode = Rc::new(RopeNodeType::Node(Node {
            left: Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Leaf(Leaf {
                    value: String::from("hello "),
                }))
                .into(),
                right: Rc::new(RopeNodeType::Leaf(Leaf {
                    value: String::from("world! "),
                }))
                .into(),
                weight: 6,
            }))
            .into(),
            right: Rc::new(RopeNodeType::None).into(),
            weight: 13,
        }))
        .into();

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
            let mut rope = Rope::new(Rc::clone(&root_node.0).into());
            rope.insert(idx, String::from("new_leaf"));
            assert_eq!(exp_result, format!("{}", rope.root));
        }
    }

    #[test]
    fn delete_test() {
        let root_node: RopeNode = Rc::new(RopeNodeType::Node(Node {
            left: Rc::new(RopeNodeType::Node(Node {
                left: Rc::new(RopeNodeType::Leaf(Leaf {
                    value: String::from("hello "),
                }))
                .into(),
                right: Rc::new(RopeNodeType::Leaf(Leaf {
                    value: String::from("world! "),
                }))
                .into(),
                weight: 6,
            }))
            .into(),
            right: Rc::new(RopeNodeType::None).into(),
            weight: 13,
        }))
        .into();

        let expected_result = vec![
            (0, 6, r#"Node(Left: None, Right: Leaf("world! "))"#),
            (
                13,
                5,
                r#"Node(Left: Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: None), Right: None)"#,
            ),
            (6, 7, r#"Node(Left: Leaf("hello "), Right: None)"#),
            (
                9,
                2,
                r#"Node(Left: Node(Left: Leaf("hello "), Right: Leaf("wor")), Right: Leaf("! "))"#,
            ),
            (
                20,
                5,
                r#"Node(Left: Node(Left: Node(Left: Leaf("hello "), Right: Leaf("world! ")), Right: None), Right: None)"#,
            ),
        ];

        for (idx, len, exp_result) in expected_result {
            println!("test");
            let result = root_node.delete(idx, len);
            assert_eq!(exp_result, format!("{}", result));
        }
    }
}
