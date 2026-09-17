use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_parts::{FreeBitmap, ResultFreeBitData, FreeBits};
use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData};

use std::{fs, result, error};

pub struct Bitmap{
    pub bitmap_data: FreeBitmap,
}
impl Bitmap{
    pub fn set_free_bitmap_data(&self, super_block: &SuperBlockData){
        self.bitmap_data.set_free_bitmap(super_block);
    }

    pub fn get_bitmap(&self, super_block: &SuperBlockData, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
        Ok(
            self.bitmap_data.get_bitmap(super_block, super_block.bitmap_address, output_file)?
        )
    }

    pub fn reload_bitmap(&self, super_block: &SuperBlockData, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
        Ok(
            self.bitmap_data.reload_bitmap(super_block, super_block.bitmap_address, output_file)?
        )
    }

    pub fn write_bit_flag(&self, bit_flag_data: FreeBits) -> result::Result<(), Box<dyn error::Error>>{
        Ok(
            self.bitmap_data.write_bit_flag(bit_flag_data)?
        )
    }

    pub fn get_free_bit(&self, select_size: u64) -> FreeBits{
        self.bitmap_data.get_free_bit(select_size)
    }

    pub fn new(super_block: &SuperBlockData, output_file: &mut fs::File) -> result::Result<Self, Box<dyn error::Error>>{
        Ok(
            Self{
                bitmap_data: FreeBitmap::new(super_block, super_block.bitmap_address, output_file)?,
            }
        )
    }
}
