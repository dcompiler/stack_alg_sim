/* Revised code originally by Aidan Goldfarb at https://github.com/AidanGoldfarb/matmultiply/blob/master/rttrace/src/lib.rs */

#[derive(Debug)]
pub struct LRUStack {
 	pub stack: Vec<String>,
}

impl LRUStack {

	pub fn new() -> LRUStack {
		LRUStack {
			stack: Vec::new(),
		}
	}

	pub fn rec_access(&mut self, val: &str) -> Option<u32> {
		let mut dist: Option<u32> = None;
		if self.stack.contains(&val.to_string()){
			let pos = self.stack.iter().position(|x| *x == val).unwrap();
			dist = Some((self.stack.len() - pos) as u32);
			self.stack.remove(pos);
			self.stack.push(val.to_string());
		}
		else{
			self.stack.push(val.to_string());
		}
		return dist;
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic() {
        let mut analyzer = LRUStack::new();
        let mut dists = Vec::new();
        for c in "abc abc".chars().filter(|c| !c.is_whitespace()) {
			dists.push( analyzer.rec_access( &c.to_string() ));
		}

        assert_eq!(dists, [None, None, None, Some(3), Some(3), Some(3)]);
    }

    #[test]
    fn sawtooth() {
        let mut analyzer = LRUStack::new();
        let mut dists = Vec::new();
        for c in "abc cba".chars().filter(|c| !c.is_whitespace()) {
			dists.push( analyzer.rec_access( &c.to_string() ));
		}

        assert_eq!(dists, [None, None, None, Some(1), Some(2), Some(3)]);
    }
}
