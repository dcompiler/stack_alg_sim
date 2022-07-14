#[derive(Debug)]
pub struct LRUStack {
 	pub stack: Vec<Option<Box<String>>>,
}

impl LRUStack {

	pub fn new() -> LRUStack {
		LRUStack {
			stack: Vec::new(),
		}
	}

	pub fn rec_access(&mut self, val: &str) -> Option<u32> {
            if self.stack.len() == 0 {
                self.stack.push(Some(Box::new(val.to_string())));
                return None;
            }
	    if **self.stack[0].as_ref().unwrap() == val.to_string() {
			return Some(1);
	    }
	    let mut last = self.stack[0].take(); 
	    for pos in 1..self.stack.len() {
		let temp = self.stack[pos].take();
		self.stack[pos] = last;
		last = temp; 
		if **last.as_ref().unwrap() == val.to_string() {
		    self.stack[0] = last;
                    return Some(pos as u32 + 1);
		}
	    }
            // a cold miss
            self.stack.push( last ); // add to the end of the vector
	    self.stack[0] = Some(Box::new(val.to_string()));
	    return None;
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
