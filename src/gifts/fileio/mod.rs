pub mod reader;


use std;
use std::mem;

unsafe fn typed_to_bytes<T>(slice: &mut [T]) -> &mut[u8] {
   
        std::slice::from_raw_parts_mut(slice.as_ptr() as *mut u8,
                                   slice.len() * mem::size_of::<T>())
        
}


#[test]
fn test_raw_write(){
    let mut v1 = vec![0u16; 4];
    unsafe {
        let v2 = typed_to_bytes(&mut v1);
        v2[7] = 1u8;
    }
    println!("{}", v1[3]);
    assert!(v1[3] == 256u16);
}

