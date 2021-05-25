use crate::tree::Tree;
use std::cmp::Ordering;

pub struct RbTree<K, V> {
    root: Link<K, V>,
}

impl<K: Ord, V> Tree<K, V> for RbTree<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key)
    }

    fn insert(&mut self, key: K, val: V) {
        if let None = self.root.0 {
            let mut node = Box::new(Node::new(key, val));
            node.colour = Colour::Black;
            self.root = Link(Some(node));
        } else {
            self.root.insert(key, val);
            if let Some(node) = self.root.0.as_mut() {
                node.colour = Colour::Black;
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.root.remove(key)
    }
}

struct Node<K, V> {
    key: K,
    val: V,
    colour: Colour,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            val: value,
            colour: Colour::Red,
            left: Link(None),
            right: Link(None),
        }
    }
}

enum Colour {
    Red,
    Black,
}

struct Link<K, V>(Option<Box<Node<K, V>>>);

impl<K: Ord, V> Link<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.0.as_ref().and_then(|node| match key.cmp(&node.key) {
            Ordering::Less => node.left.get(key),
            Ordering::Greater => node.right.get(key),
            Ordering::Equal => Some(&node.val),
        })
    }

    fn insert(&mut self, key: K, val: V) {
        if self.0.is_none() {
            self.0 = Some(Box::new(Node::new(key, val)));
            return;
        }

        let node = self.0.as_mut().unwrap();
        match node.key.cmp(&node.key) {
            Ordering::Less => node.left.insert(key, val),
            Ordering::Greater => node.right.insert(key, val),
            // we have a duplicate. In case we want to store the same value it doesn't matter, but if we want
            // to store a different value under the same key we can either replace it or return some value that
            // signals we need to remove old value first.
            // here, we'll just do nothing
            Ordering::Equal => (),
        }
    }

    // fn insert_p(&mut self, parent: &mut Link<K, V>, key: K, val: V) {}

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
