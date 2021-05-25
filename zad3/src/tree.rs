pub trait Tree<K, V> {
    fn name(&self) -> &'static str;
    fn insert(&mut self, _: K, _: V);
    fn get(&self, _: &K) -> Option<&V>;
    fn remove(&mut self, _: &K) -> Option<V>;
}
