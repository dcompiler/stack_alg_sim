use lru_stack::LRUStack;
use lru_vec::LRUVec;
use lru_trait::LRU;

pub fn nmm(a_size_row: usize, a_size_col: usize, b_size_row: usize, b_size_col: usize, lru_type: String) -> Vec<(String, Option<usize>)>{
    assert_eq!(a_size_col, b_size_row, "The number of A columns must be equal to the number of B rows for matrix multiplication.");
    
    let mut cache: Box<dyn LRU<(usize, usize, char)>> = if lru_type == "Vec" {
        Box::new(LRUVec::<(usize, usize, char)>::new())
    } else {
        Box::new(LRUStack::<(usize, usize, char)>::new())
    };

    let mut dists: Vec<(String, Option<usize>)> = Vec::new();

    for i in 0..a_size_row{
        for j in 0..b_size_col{
            for k in 0..a_size_col{
                // println!("Here");
                let a_tuple = (i, k, 'A'); 
                let cur_a = cache.rec_access(a_tuple);
                dists.push((format!("{:?}", a_tuple), cur_a));

                let b_tuple = (k, j, 'B');
                let cur_b = cache.rec_access(b_tuple);
                dists.push((format!("{:?}", b_tuple), cur_b));

                let c_tuple = (i, j, 'C');
                let cur_c = cache.rec_access(c_tuple);
                dists.push((format!("{:?}", c_tuple), cur_c));

                let c_tuple = (i, j, 'C');
                let cur_c = cache.rec_access(c_tuple);
                dists.push((format!("{:?}", c_tuple), cur_c));
            }
        }
    }

    return dists;
}

pub fn conv2d(n_size: usize, k_size: usize, c: usize, batch_sz: usize, lru_type: String) -> Vec<(String, Option<usize>)>{
    let num_passes = (c as f64 / batch_sz as f64).ceil() as usize;
    //n_size = 4;
   // k_size = 2;

    let mut cache: Box<dyn LRU<(usize, char)>> = if lru_type == "Vec" {
        Box::new(LRUVec::<(usize, char)>::new())
    } else {
        Box::new(LRUStack::<(usize, char)>::new())
    };

    let mut dists: Vec<(String, Option<usize>)> = Vec::new();


    for p in 0..num_passes {
        for i in 0..(n_size - k_size + 1) {
            for j in 0..(n_size - k_size + 1) {
                for l in (p * batch_sz)..min((p + 1) * batch_sz, c) {
                    for y in 0..k_size {
                        for x in 0..k_size {
                            let k_tuple = (y * k_size + x, 'K');
                            let cur_k = cache.rec_access(k_tuple);
                            dists.push((format!("{:?}", k_tuple), cur_k));

                            let i_tuple = (((i + y) * n_size + (j + x)) * c + l, 'I');
                            let cur_i = cache.rec_access(i_tuple);
                            dists.push((format!("{:?}", i_tuple), cur_i));
                        }
                    }
                }
                let r_tuple = (i * n_size + j, 'R');
                let cur_r = cache.rec_access(r_tuple);
                dists.push((format!("{:?}", r_tuple), cur_r));
            }
        }
    }
    return dists;
}
