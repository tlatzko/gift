//# The cuboid module contain all important 
//# methods for reading a cubfile
//# the main measurement file for the 
//# GLORIA FTS instrument
use std;
use std::fs::File;



//################################### Structures
#[repl(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidInfo {
	version: u32,
	file_type: u32
}

#[repl(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidHeader {
  	v_width: u16,
  	h_width: u16,
  	v_offset: u16,
  	h_offset: u16,
  	length: u32,
}


#[repl(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidFrameHeader {
  	frame_number: u32,
  	timestamp_ms: u32,
  	timestamp_tick: u32,
  	status: u32,
}


#[repl(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidFrame {
  	cuboid_frame_header header: CuboidFrameHeader,
  	pixel_value©s: Vec<u16>,
}


#[repl(C, packed)]
pub struct CuboidFile {
	  file: File,
  	info: CuboidInfo,
  	header: CuboidHeader,
  	
}

impl CuboidFile{
  pub fn new(path_to_cubfile: &str){
    let path = Path::new()
  }
}