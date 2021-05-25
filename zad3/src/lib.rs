mod bst;
mod tree;

pub use bst::Bst;
pub use rb_tree::RBMap as RbTree;
pub use tree::Tree;

impl<K: Ord + Clone, V: Clone> Tree<K, V> for RbTree<K, V> {
    fn name(&self) -> &'static str {
        "Red-Black Tree"
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn insert(&mut self, key: K, val: V) {
        self.insert(key, val);
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}
