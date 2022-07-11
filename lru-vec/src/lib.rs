/* Revised code originally by Aidan Goldfarb at https://github.com/AidanGoldfarb/matmultiply/blob/master/rttrace/src/lib.rs */

#[derive(Debug)]
pub struct LRUStack {
 	pub stack: Vec<String>,
}

pub fn init() -> LRUStack {
	let stack = Vec::new();
	LRUStack {
		stack,
	}
}

pub fn rec_access(val: &str, data: &mut LRUStack) -> Option<u32> {
	let dist: Option<u32> = None;
    if data.stack.contains(&val.to_string()){
        let pos = data.stack.iter().position(|x| *x == val).unwrap();
        dist = Some((data.stack.len() - pos) as u32);
        data.stack.remove(pos);
        data.stack.push(val.to_string());
        *data.dmd += (dist as f32).sqrt();
    }
    else{
        data.stack.push(val.to_string());
    }
    return dist;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic() {
        // a b c a b c
        let mut data = init();
        trace("a", &mut data);
        trace("b", &mut data);
        trace("c", &mut data);
        trace("a", &mut data);
        trace("b", &mut data);
        trace("c", &mut data);

        let gt = 3.0*(3.0_f32.sqrt());

        println!("cyclic: {}", *data.dmd);

        assert!((*data.dmd - gt).abs() < E);
    }

    #[test]
    fn sawtooth() {
        // a b c c b a
        let mut data = init();
        trace("a", &mut data);
        trace("b", &mut data);
        trace("c", &mut data);
        trace("c", &mut data);
        trace("b", &mut data);
        trace("a", &mut data);

        let gt = 1.0 + 2.0_f32.sqrt() + 3.0_f32.sqrt();

        println!("sawtooth: {}", *data.dmd);

        assert!((*data.dmd - gt).abs() < E);
    }
}
