use core::error;
use std::{result, fs};

use crate::IPB_HMJN_FS::{FS_core_types::{self, SuperBlockData}, FS_execution::FS_function_parts::bitmap_function::bitmap_parts::FreeBitmap};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::{bitmap_function};

pub struct Execution{
    pub super_block: SuperBlockData,
    pub bitmap: FreeBitmap,
}
impl Execution{
    pub fn new(super_block: SuperBlockData, output_file: &mut fs::File) -> result::Result<Self, Box<dyn error::Error>>{
        let bitmap_data = FreeBitmap::new(&super_block, output_file)?;
        Ok(
            Self{
                super_block,
                bitmap: bitmap_data,
            }
        )
    }
}