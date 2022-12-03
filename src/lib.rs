#![allow(unused)]

use std::collections::VecDeque;

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

struct LevelIterator<'a> {
    node: Option<&'a Box<Node>>,
    dequeue: VecDeque<&'a Box<Node>>,
}

impl<'a> LevelIterator<'a> {
    fn new(node: Option<&'a Box<Node>>) -> Self {
        Self {
            node,
            dequeue: VecDeque::new(),
        }
    }
}

impl<'a> Iterator for LevelIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.node, &mut self.dequeue) {
            (None, dq) if dq.is_empty() => None,
            (None, dq) => {
                self.node = dq.pop_front();
                self.next()
            }
            (Some(node), dq) => {
                if let Some(ref left) = node.left {
                    dq.push_back(left)
                }

                if let Some(ref right) = node.right {
                    dq.push_back(right)
                }

                self.node = None;
                Some(node.value)
            }
        }
    }
}

struct InOrderIterator<'a> {
    node: Option<&'a Box<Node>>,
    queue: Vec<&'a Box<Node>>,
}

impl<'a> InOrderIterator<'a> {
    fn new(node: Option<&'a Box<Node>>) -> Self {
        Self {
            node,
            queue: Vec::new(),
        }
    }
}

impl<'a> Iterator for InOrderIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.node, &mut self.queue) {
            (None, q) if q.is_empty() => None,
            (None, q) => {
                // we know q is not empty
                let node = q.pop().unwrap();
                self.node = node.right.as_ref();
                Some(node.value)
            }
            (Some(node), q) => {
                q.push(node);
                self.node = node.left.as_ref();
                self.next()
            }
        }
    }
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn level_iter(&self) -> LevelIterator {
        LevelIterator::new(self.root.as_ref())
    }

    fn in_ord_iter(&self) -> InOrderIterator {
        InOrderIterator::new(self.root.as_ref())
    }

    fn values(&self) -> Vec<i32> {
        if self.root.is_none() {
            return vec![];
        }

        let mut results = vec![];
        let mut dq: VecDeque<&Box<Node>> = VecDeque::new();
        let root = self.root.as_ref().unwrap();
        results.push(root.value);
        dq.push_back(root);

        while !dq.is_empty() {
            // so as to only iterate on the same depth
            for _ in 0..dq.len() {
                if let Some(node) = dq.pop_front() {
                    if let Some(ref left) = node.left {
                        results.push(left.value);
                        dq.push_back(left)
                    }

                    if let Some(ref right) = node.right {
                        results.push(right.value);
                        dq.push_back(right)
                    }
                }
            }
        }

        results
    }

    fn insert(&mut self, value: i32) {
        // if let Some(ref mut node) = self.root {
        //     Self::insert_recursive(node, value);
        // } else {
        //     self.root = Node::new(value).into();
        // }

        self.insert_iterative(value);
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match node.right {
                None => node.right = Node::new(value).into(),
                Some(ref mut child_node) => Self::insert_recursive(child_node, value),
            }
        } else if value < node.value {
            match node.left {
                None => node.left = Node::new(value).into(),
                Some(ref mut child_node) => Self::insert_recursive(child_node, value),
            }
        }
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
        }

        // creating a vec with capacity 1, which will hold the parent at all times
        let mut queue: Vec<&mut Box<Node>> = Vec::with_capacity(1);
        let root = self.root.as_mut().unwrap();
        queue.push(root);

        while let Some(node) = queue.pop() {
            if value > node.value {
                match node.right {
                    None => node.right = Node::new(value).into(),
                    Some(ref mut child_node) => queue.push(child_node),
                }
            } else if value < node.value {
                match node.left {
                    None => node.left = Node::new(value).into(),
                    Some(ref mut child_node) => Self::insert_recursive(child_node, value),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_insertion_works() {
        let mut tree = Tree::new();
        tree.insert(42);
        tree.insert(2);
        tree.insert(12);
        tree.insert(62);
        tree.insert(20);
        tree.insert(40);
        //          42
        //         /  \
        //        2    62
        //         \
        //          12
        //            \
        //             20
        //               \
        //                40
        assert!(tree.root.is_some());
    }

    #[test]
    fn tree_traversal_works() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(32);
        tree.insert(64);
        tree.insert(2);
        tree.insert(128);
        tree.insert(4);
        //       8
        //      / \
        //     2   32
        //      \    \
        //       4    64
        //              \
        //              128
        assert_eq!(tree.values(), vec![8, 2, 32, 4, 64, 128]);
    }

    #[test]
    fn tree_level_iter_works() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(32);
        tree.insert(64);
        tree.insert(2);
        tree.insert(128);
        tree.insert(4);
        assert_eq!(
            tree.level_iter().collect::<Vec<_>>(),
            vec![8, 2, 32, 4, 64, 128]
        );
    }

    #[test]
    fn tree_in_ord_iter_works() {
        let mut tree = Tree::new();
        tree.insert(42);
        tree.insert(2);
        tree.insert(12);
        tree.insert(62);
        tree.insert(20);
        tree.insert(40);
        assert_eq!(
            tree.in_ord_iter().collect::<Vec<_>>(),
            vec![2, 12, 20, 40, 42, 62]
        );
    }
}
