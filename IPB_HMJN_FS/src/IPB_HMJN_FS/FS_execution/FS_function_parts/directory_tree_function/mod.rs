use std::{error, fs, io::{Read, Seek, SeekFrom, Write}, mem, result};
use crate::IPB_HMJN_FS::{FS_core_types::{Entry, SuperBlockData}, FS_execution::FS_function_parts::{bitmap_function::bitmap_parts::FreeBitmap, tree_parts::{self, reload_tree, search_entry_location, get_endpoint_ID}}};

//ディレクトリツリーの末端に位置する葉のアドレス位置を返す
fn scan_endpoint_directory_ID(super_block: &SuperBlockData, output_file: &mut fs::File) -> result::Result<u64, Box<dyn error::Error>>{
    let get_address = get_endpoint_ID(super_block, super_block.directory_tree_cluster_num, super_block.directory_tree_depth, output_file)?;
    let loop_num = super_block.get_one_block_bytes() / mem::size_of::<Entry>() as u64;
    let now_address_byte = get_address * super_block.get_one_cluster_bytes();
    let mut check_type = [0u8; 1];
    let mut top_index = 0;
    for index in 0..loop_num{
        output_file.seek(SeekFrom::Start(now_address_byte + mem::size_of::<Entry>() as u64 * index))?;
        output_file.read_exact(&mut check_type);
        if check_type[0] == 0{
            break;
        }
        else{
            top_index = index;
        }
    }
    output_file.seek(SeekFrom::Start(now_address_byte + mem::size_of::<Entry>() as u64 * top_index + mem::size_of::<u8>() as u64))?;
    let mut buffer = [0u8; 8];
    output_file.read_exact(&mut buffer);
    let mut ID = u64::from_le_bytes(buffer) + 1;

    Ok(ID)
}

//ディレクトリツリーの新しい親位置を確保する関数
fn reload_parent_directory_tree(super_block: &mut SuperBlockData, bitmap_data: &FreeBitmap, output_file: &mut fs::File) -> result::Result<bool, Box<dyn error::Error>>{
    let result_data = reload_tree(bitmap_data);
    if let Some(data) = result_data{
        output_file.seek(SeekFrom::Start(data.new_tree_adderss * super_block.get_one_cluster_bytes()))?;
        output_file.write_all(&super_block.directory_tree_cluster_num.to_le_bytes())?;
        bitmap_data.write_bit_flag(data.set_flag)?;
        super_block.directory_tree_cluster_num = data.new_tree_adderss;
    }
    else{
        return Ok(false);
    }
    Ok(true)
}

//指定したアドレスに存在するEntryを返す
fn select_entry(super_block: &SuperBlockData, select_ID: u64, output_file: &mut fs::File) -> result::Result<Option<Entry>, Box<dyn error::Error>>{
    let result = search_entry_location(super_block, super_block.directory_tree_cluster_num, super_block.directory_tree_depth, select_ID, mem::size_of::<Entry>() as u64, output_file)?;
    if let Some(some_data) = result {
        let mut buffer = [0u8; 512];
        output_file.read_exact(&mut buffer)?;
        let get_entry = Entry::from_bytes(&buffer);
        return Ok(Some(get_entry));
    }
    Ok(None)
}

//指定したツリー内の指定したoffsetに新しいツリー子へのアドレスを入れる
fn new_child_directory_tree(super_block: &SuperBlockData, now_tree_address: u64, offset: u64, bitmap_data: &FreeBitmap, output_file: &mut fs::File) -> result::Result<bool, Box<dyn error::Error>>{
    let get_data = reload_tree(bitmap_data);
    if let Some(some_data) = get_data{
        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * now_tree_address + offset * mem::size_of::<u64>() as u64))?;
        output_file.write_all(&some_data.new_tree_adderss.to_le_bytes())?;
        bitmap_data.write_bit_flag(some_data.set_flag)?;
    }
    else{
        return Ok(false)
    }

    Ok(false)
}


