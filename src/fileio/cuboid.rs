//# The cuboid module contain all important 
//# methods for reading a cubfile
//# the main measurement file for the 
//# GLORIA FTS instrument
use super::reader::decode_struct;
use super::typed_to_bytes;

use std::mem;
use std::fs::File;
use std::path::Path;
use std::io::{SeekFrom, Seek, Read};

/// the current frame value type is short
pub type frame_value_type = u16;

/// return the cub file offset in 
/// bytes where the first frame starts
const CUB_OFFSET: u64 = 512;
/// ticks per second is required to compute
/// the time for each frame
const TICKS_PER_SEC: f64 = 1e7f64;
//################################### Structures

#[repr(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidInfo {
	version: u32,
	file_type: u32
}

#[repr(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidHeader {
  	v_width: u16,
  	h_width: u16,
  	v_offset: u16,
  	h_offset: u16,
  	length: u32,
}


#[repr(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidFrameHeader {
  	frame_number: u32,
  	timestamp_ms: u32,
  	timestamp_tick: u32,
  	status: u32,
}


#[repr(C, packed)]
#[derive(RustcDecodable)]
pub struct CuboidFrame {
  	pub header: CuboidFrameHeader,
  	pub data: Vec<frame_value_type>,
}


/// The cuboid file is the main data structure 
/// for the GLORIA experiment 
/// it contains beside the info and header
/// also an instance of the file
#[repr(C, packed)]
pub struct CuboidFile {
    file: File,
  	info: CuboidInfo,
  	header: CuboidHeader,
    frame_count: u64,
}


impl CuboidFile{
  pub fn new(path_to_cubfile: &str) -> CuboidFile{
    let path = Path::new(&path_to_cubfile);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why)  => panic!("{}", why),
    };

    // read cub info
    let info: CuboidInfo = decode_struct(&mut file).unwrap();
     // read cub header
    let header: CuboidHeader = decode_struct(&mut file).unwrap();

    // frame counter
    let byte_count = file.metadata().unwrap().len();
    let frame_count = ((byte_count as i64 - CUB_OFFSET as i64) / (mem::size_of::<frame_value_type>() as i64 * header.length as i64)) as u64;
    CuboidFile{
        file: file,
        info: info, 
        header: header,
        frame_count: frame_count
    }
    
  }

   pub fn len(&self) -> u64 {
       self.frame_count
   }

   /// access to the index't frame in the cubfile
   pub fn get_frame(&mut self, index: u64) -> CuboidFrame{
       let tsize = mem::size_of::<frame_value_type>() as i64; 
       // go the header position of the ith frame
       let pos = CUB_OFFSET + (index as i64 *
                               self.header.length as i64  *
                               tsize) as u64;

       self.file.seek(SeekFrom::Start(pos));
       let header: CuboidFrameHeader = decode_struct(&mut self.file).unwrap();

       // go to position of the first 
       let shift = tsize * self.header.v_width as i64;
       let pixel_pos = pos as i64 + shift;
       self.file.seek(SeekFrom::Start(pixel_pos as u64));

       // allocate the memory and read the frame data
       let fill_value: frame_value_type = 0;  
       let frame_size = self.header.v_width as usize * self.header.h_width as usize;
       let mut data = vec![fill_value; frame_size];
        
       unsafe {
           let mut byte_map = typed_to_bytes(&mut data[..]);
           self.file.read(&mut byte_map);

       }

       CuboidFrame {
           header: header,
           data: data
       }

            
   }
}


impl CuboidFrame {
    /// get the time of the frame in seconds
    pub fn get_timestamp(&self)-> f64{
        self.header.timestamp_ms as f64 * 1e-3f64 + self.header.timestamp_tick as f64 / TICKS_PER_SEC
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_open_cub_file(){
        let file_name = "/home/latzko/work/experimental/data/20120828_111935.cub";
        let mut cub = CuboidFile::new(&file_name);
        let frame  = cub.get_frame(10);
        println!("cub w {} frame status {} time {}", cub.header.length,
                 frame.header.status,
                 frame.get_timestamp());
    }
}
