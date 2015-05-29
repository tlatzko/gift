use std::ops::{Add, Mul};


pub fn polytope<T: Add<Output=T> + Mul<Output=T> + Copy>(params: &[T], x: T) -> T{
    assert!(params.len() > 1);
    let mut res: T = params[0];
    for i in 0..params.len() - 1{
        let p = params[i + 1];
        res = res * x + p;
    }
    return res;
}


#[test]
fn test_polytope(){
    let arr = [1f64, 1f64, 1f64, 1f64];
    let res = polytope(&arr, 1f64);
    assert!(res == 4f64);
}


