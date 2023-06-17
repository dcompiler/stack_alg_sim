pub trait LRU<T> {
    fn rec_access(&mut self, val: T) -> Option<usize>;
}
