use std::cmp::Ordering;

use crate::tree::Tree;

#[derive(Debug, Clone)]
pub struct Bst<K: Ord, V> {
    root: Link<K, V>,
}

impl<K: Ord, V> Bst<K, V> {
    pub fn new() -> Self {
        Self { root: Link(None) }
    }
}

impl<K: Ord, V> Tree<K, V> for Bst<K, V> {
    fn name(&self) -> &'static str {
        "BST"
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }

    fn insert(&mut self, key: K, val: V) {
        self.root.insert(key, val)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.root.remove(key)
    }
}

#[derive(Debug, Clone)]
struct Node<K: Ord, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            left: Link(None),
            right: Link(None),
        }
    }
}

#[derive(Debug, Clone)]
struct Link<K: Ord, V>(Option<Box<Node<K, V>>>);

impl<K: Ord, V> Link<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.0.as_ref().and_then(|node| match key.cmp(&node.key) {
            Ordering::Less => node.left.get(key),
            Ordering::Greater => node.right.get(key),
            Ordering::Equal => Some(&node.val),
        })
    }

    fn insert(&mut self, key: K, val: V) {
        match self.0.as_mut() {
            None => {
                self.0.replace(Box::new(Node::new(key, val)));
            }
            Some(node) => match key.cmp(&node.key) {
                Ordering::Less => node.left.insert(key, val),
                Ordering::Greater => node.right.insert(key, val),
                // we have a duplicate. In case we want to store the same value it doesn't matter, but if we want
                // to store a different value under the same key we can either replace it or return some value that
                // signals we need to remove old value first.
                // here, we'll just do nothing
                Ordering::Equal => {
                    println!("collision");
                }
            },
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        match self.0.as_mut() {
            None => None,
            Some(node) => match key.cmp(&node.key) {
                Ordering::Less => node.left.remove(key),
                Ordering::Greater => node.right.remove(key),
                Ordering::Equal => match (node.left.0.as_ref(), node.right.0.as_ref()) {
                    (None, None) => self.0.take().map(|node| node.val),
                    (Some(c), None) => {
                        let successor = node.left.0.take();
                        self.0.replace(successor.unwrap()).map(|node| node.val)
                    }
                    (None, Some(c)) => {
                        let successor = node.right.0.take();
                        self.0.replace(successor.unwrap()).map(|node| node.val)
                    }
                    (Some(l), Some(r)) => {
                        // obtain the minimum element
                        let succ = node.right.get_min();

                        // replace the current root with the successor
                        let root = self.0.take();
                        self.0 = succ.0;
                        root.map(|node| node.val)
                    }
                },
            },
        }
    }

    fn get_min(&mut self) -> Link<K, V> {
        let mut to_return = Link(None);

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().left.0.is_some() {
                current = &mut current.0.as_mut().unwrap().left;
            }

            let mut node = current.0.take().unwrap();
            let right = node.right.0.take();
            to_return = Link(Some(node));
            current.0 = right;
        }
        to_return
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_root() {
        let mut bst = Bst::new();
        bst.insert(69, 69);
        assert_eq!(bst.root.0.unwrap().val, 69);
    }

    #[test]
    fn get_test() {
        let mut bst = Bst::new();

        bst.insert(8, 8);
        bst.insert(3, 3);
        bst.insert(9, 9);
        bst.insert(6, 6);

        println!("{:#?}", bst);

        assert_eq!(bst.get(&6), Some(&6));
    }
}
