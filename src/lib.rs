
extern crate rustc_serialize;
extern crate byteorder;

pub mod fileio;
pub mod math;

use std::sync::mpsc::sync_channel;
use std::marker::Send;
use std::thread;
use fileio::cuboid::{CuboidFile, CuboidFrame};

fn cub_reader<T: Send>(tx: T) {

}


// no_mangle lets us find the name in the symbol table
// extern makes the function externally visible
#[no_mangle]
pub extern fn square(x: i32) -> i32 {
    x * x
}

// no_mangle lets us find the name in the symbol table
// extern makes the function externally visible
#[no_mangle]
pub extern fn run_pipe() {
    let (tx1, rx1) = sync_channel(2);

    thread::spawn(move || {
        let file_name = "/home/latzko/work/experimental/data/20120828_111935.cub";
        let mut cub = CuboidFile::new(&file_name);
        for i in 0..cub.len(){
            let frame = cub.get_frame(i);
            tx1.send(frame.data).unwrap();
        }
        drop(tx1);
    });

    let (tx2, rx2) = sync_channel(2);
    thread::spawn(move || {
        let params = [1.2912459e-013f64, -4.8528599e-9f64, 7.6709233e-5f64, 4.4685754e-1f64, 1.4696225e3f64];
        loop {
            let data = match rx1.recv() {
                Err(why) => {
                    println!("{}", why);
                    break;
                },
                Ok(data) => data
            };
            let mut nlc = Vec::with_capacity(data.len());
            for x in data{
                let val = math::polytope(&params, x as f64);
                nlc.push(val);
            }
            tx2.send(nlc).unwrap();
        }
        drop(tx2);
    });
    
    loop {
        let d = match rx2.recv() {
            Err(why) => {
                println!("{}", why);
                
                break;
                },
            Ok(data) => data
        };
    }
                
    
}


#[test]
fn test_pipe(){
    run_pipe();
}
