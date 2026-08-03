use std::io::Write;
// use std::str::pattern::CharPredicateSearcher;
use std::{io::{Seek, SeekFrom}, fs, result, error, mem};

use super::FS_core_types;
use super::Constant;
use super::error_handlind;

fn write_super_block_data(super_block: &FS_core_types::SuperBlockData, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
    output_file.write_all(&super_block.magic_number)?;
    output_file.write_all(&super_block.back_up_or_main.to_le_bytes())?;
    output_file.write_all(&super_block.fs_version.top.to_le_bytes())?;
    output_file.write_all(&super_block.fs_version.mid.to_le_bytes())?;
    output_file.write_all(&super_block.fs_version.low.to_le_bytes())?;
    output_file.write_all(&super_block.partition_LBA.to_le_bytes())?;
    output_file.write_all(&super_block.one_sector_size.to_le_bytes())?;
    output_file.write_all(&super_block.one_block_sector_num.to_le_bytes())?;
    output_file.write_all(&super_block.one_cluster_block_num.to_le_bytes())?;
    output_file.write_all(&super_block.partition_cluster_size.to_le_bytes())?;
    output_file.write_all(&super_block.bitmap_address.to_le_bytes())?;
    output_file.write_all(&super_block.bitmap_size.to_le_bytes())?;
    output_file.write_all(&super_block.back_up_num.to_le_bytes())?;
    output_file.write_all(&super_block.back_up_list_address.to_le_bytes())?;
    output_file.write_all(&super_block.directory_tree_address.to_le_bytes())?;
    output_file.write_all(&super_block.directory_tree_depth.to_le_bytes())?;
    output_file.write_all(&super_block.name_tree_address.to_le_bytes())?;
    output_file.write_all(&super_block.name_tree_depth.to_le_bytes())?;
    output_file.write_all(&super_block.free_ID_bitmap_address.to_le_bytes())?;
    output_file.write_all(&super_block.free_ID_bitmap_size.to_le_bytes())?;
    let now_stream_point = output_file.stream_position()?;
    output_file.seek(SeekFrom::Start(now_stream_point + super_block.reservation_space_size))?;
    output_file.write_all(&super_block.read_algorithm_num.to_le_bytes())?;
    output_file.write_all(&super_block.write_algorithm_num.to_le_bytes())?;
    output_file.write_all(&super_block.check_sum.to_le_bytes())?;

    Ok(())
}

fn write_bitmap_data(super_block: &FS_core_types::SuperBlockData, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
    output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * super_block.bitmap_address))?;
    let capacity: u64 = (super_block.partition_cluster_size + (u8::BITS as u64 - 1)) / (u8::BITS as u64);
    let mut bitmap: Vec<u8> = vec![0; capacity as usize];
    let mut write_bits: u64 = 0;

    //write super block bit flag
    if let Some(bit) = bitmap.get_mut(write_bits as usize / u8::BITS as usize){
        *bit |= 1 << (write_bits % u8::BITS as u64);
    }
    write_bits += 1;

    //write bitmap bit flag
    while write_bits <= super_block.bitmap_size {
        if let Some(bit) = bitmap.get_mut(write_bits as usize / u8::BITS as usize){
            *bit |= 1 << (write_bits % u8::BITS as u64 as u64);
        }
        write_bits += 1;
    }

    //write back up list flag
    if let Some(bit) = bitmap.get_mut(write_bits as usize / u8::BITS as usize){
        *bit |= 1 << (write_bits % u8::BITS as u64 as u64);
    }
    write_bits += 1;

    //write directory tree flag
    if let Some(bit) = bitmap.get_mut(write_bits as usize / u8::BITS as usize){
        *bit |= 1 << (write_bits % u8::BITS as u64 as u64);
    }
    write_bits += 1;

    //write name tree flag
    if let Some(bit) = bitmap.get_mut(write_bits as usize / u8::BITS as usize){
        *bit |= 1 << (write_bits % u8::BITS as u64 as u64);
    }
    write_bits += 1;

    //write free ID bitmap flag
    while write_bits <= super_block.free_ID_bitmap_size {
        if let Some(bit) = bitmap.get_mut(write_bits as usize / u8::BITS as usize){
            *bit |= 1 << (write_bits % u8::BITS as u64 as u64);
        }
        write_bits += 1;
    }

    //backup data flag
    if 0 < super_block.back_up_num {
        let bitmap_len = bitmap.len() - 1;
        if let Some(bit) = bitmap.get_mut(bitmap_len){
            *bit |= 1 << super_block.partition_cluster_size % u8::BITS as u64 as u64;
        }
        for i in 0..super_block.back_up_num {
            let index = (super_block.partition_cluster_size - (super_block.partition_cluster_size) / (i+1)) / (u8::BITS as u64);
            if let Some(bit) = bitmap.get_mut(index as usize){
                *bit |= 1 << super_block.partition_cluster_size % u8::BITS as u64 as u64;
            }
            // write_bits += 1;
        }
    }

    output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * super_block.bitmap_address))?;
    output_file.write_all(&bitmap)?;

    Ok(())
}

fn write_backup_list(super_block: &mut FS_core_types::SuperBlockData, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
    //backup data flag
    if 0 < super_block.back_up_num {
        super_block.back_up_or_main = 1;
        output_file.seek(SeekFrom::Start((super_block.partition_cluster_size - 1) * super_block.get_one_cluster_bytes()))?;
        write_super_block_data(&super_block, output_file)?;
        
        for i in 1..super_block.back_up_num {
            let index = super_block.partition_cluster_size - (super_block.partition_cluster_size) / (i+1);
            output_file.seek(SeekFrom::Start(index * super_block.get_one_cluster_bytes()))?;
            write_super_block_data(&super_block, output_file)?;
        }
        super_block.back_up_or_main = 0;
    }
    Ok(())
}

pub fn FS_format(super_block: &mut FS_core_types::SuperBlockData, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
    // let mut output_file = fs::File::create(constant::OUTPUT_FILE_NAME)?;
    write_super_block_data(&super_block, output_file)?;
    write_bitmap_data(&super_block, output_file)?;
    write_backup_list(super_block, output_file)?;

    Ok(())
}