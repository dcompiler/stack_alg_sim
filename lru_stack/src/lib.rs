#![feature(linked_list_remove)]   // Need nightly build of the Rust compiler

use lru_trait::LRU;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct LRUStack<T> {
 	pub stack: LinkedList<T>,
}

impl<T: PartialEq + Clone> LRU<T> for LRUStack<T> {
    fn rec_access(&mut self, val: T) -> Option<u32> {
        self.rec_access_impl(val)
    }
}

impl<T: PartialEq> LRUStack<T> {

    pub fn new() -> LRUStack<T> {
        LRUStack {
            stack: LinkedList::new(),
        }
    }

    pub fn rec_access_impl(&mut self, val: T) -> Option<u32> {

        let pos = self.stack.iter().position( |x| *x == val );


        if let Some(rd) = pos {
            self.stack.remove(rd);
        }

        self.stack.push_front(val);
        
        return pos.map(|x| (x+1) as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic() {
        let mut analyzer = LRUStack::<String>::new();
        let mut dists = Vec::new();
		// let st = "abc abc";
        for c in "abc abc".chars().filter(|c| !c.is_whitespace()) {
			dists.push( analyzer.rec_access( c.to_string() ));
		}

        assert_eq!(dists, [None, None, None, Some(3), Some(3), Some(3)]);
    }

	#[test]
    fn cyclic_slice() {
        let mut analyzer = LRUStack::<&str>::new();
        let mut dists = Vec::new();
		let st = "abcabc";
		for i in 0..st.len() {
			dists.push(analyzer.rec_access(&st[i..i+1]));
		}

        assert_eq!(dists, [None, None, None, Some(3), Some(3), Some(3)]);
    }


    #[test]
    fn sawtooth() {
        let mut analyzer = LRUStack::<String>::new();
        let mut dists = Vec::new();
        for c in "abc cba".chars().filter(|c| !c.is_whitespace()) {
			dists.push( analyzer.rec_access( c.to_string() ));
		}

        assert_eq!(dists, [None, None, None, Some(1), Some(2), Some(3)]);
    }
}
