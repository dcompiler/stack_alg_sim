use std::collections::HashMap;
use std::fmt;

//use csv::WriterBuilder;

pub struct Hist {
    hist: HashMap<usize,usize>,
    // attrs: HashMap<String,String>
}

impl Hist {
    pub fn add_dist(&mut self, d: usize ) {
	self.hist.entry(d).and_modify(|counter| *counter += 1).or_insert(1);
    }

    pub fn to_vec(&self) -> Vec<(usize,usize)> {
	let mut hvec: Vec<(usize,usize)> = self.hist.iter().map(|(x,y)| (*x, *y)).collect();
	hvec.sort_by(|a,b| a.0.cmp(&b.0));
	return hvec;
    }
}

impl fmt::Display for Hist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let hvec = self.to_vec();    
	let tot = hvec.iter().fold(0, |acc, x| acc+x.1);
  
	writeln!(f, "Reuse distance histogram: \n\t{} distance value(s), min {}, max {}\n\t{} accesses", 
		 hvec.len(), hvec[0].0, hvec[hvec.len()-1].0, tot)?;
	writeln!(f, "value, count")?;
	hvec.into_iter().fold( Ok(()), |_, (d,cnt)| writeln!(f, "{}, {}", d, cnt) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let mut h = Hist{hist: HashMap::new()};
	h.add_dist(1);
	h.add_dist(1);
	h.add_dist(100);

	let v = h.to_vec();
	assert_eq!(v[0], (1,2));
	assert_eq!(v[1], (100,1));

	assert_eq!(format!("{}", h),
"Reuse distance histogram: 
	2 distance value(s), min 1, max 100
	3 accesses
value, count
1, 2
100, 1
");
	
	// use cargo test -- --show-output to see the result
	println!("{}", h);
    }
}
