use crate::IPB_HMJN_FS::{FS_core_types::{Entry, SuperBlockData}, FS_execution::FS_function_parts::bitmap_function::bitmap_parts::{CastAddressData, FreeBitmap, FreeBits}, general_function::{self, binary_pow}};
use std::{error, fs, io::{Read, Seek, SeekFrom}, mem, option::Option, result, slice::{self, GetDisjointMutError}};

#[derive(Debug)]
pub struct ResultReloadData{
    pub set_flag: FreeBits,
    pub new_tree_adderss: u64,
}

pub struct TreeData{
    pub tail_ID: u64,
    pub tail_address: u64,
}

//ツリーの末端に位置する葉のアドレス位置を返す
pub fn get_endpoint_address(super_block: &SuperBlockData, tree_address: u64, tree_depth: u64, entry_byte_size: u64, output_file: &mut fs::File) -> result::Result<TreeData, Box<dyn error::Error>>{
    let mut now_address: u64 = 0;
    let mut get_address: u64 = tree_address;
    let have_childs = super_block.get_one_cluster_bytes() / mem::size_of::<u64>() as u64;
    let mut addresses: Vec<u64> = vec![0, have_childs];
    let mut ID_pow = binary_pow(have_childs, tree_depth) + binary_pow(entry_byte_size, tree_depth);
    let mut get_ID: u64 = 0;

    for now_depth in 0..tree_depth{
        now_address = get_address;
        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * now_address))?;
        let buffer = unsafe{
            slice::from_raw_parts_mut(
                    addresses.as_mut_ptr() as *mut u8,
                    addresses.len() * mem::size_of::<u64>()
                )
            };
        output_file.read_exact(buffer)?;
        if let Some(end_address) = addresses.get(addresses.len()) {
            if *end_address != 0{
                get_address = *end_address;
                continue;
            }
        }
        let mut scan_top: u64 = addresses.len() as u64;
        let mut scan_low: u64 = 0;
        let mut scan_mid: u64 = (scan_top - scan_low) / 2 + scan_low;
        while scan_top == scan_mid || scan_low == scan_mid{
            if let Some(mid_data) = addresses.get(scan_mid as usize) {
                if *mid_data == 0{
                    scan_top = scan_mid;
                }
                else{
                    scan_low = scan_mid;
                }
            }
            scan_mid = (scan_top - scan_low) / 2 + scan_low;
        }
        if let Some(next_address) = addresses.get(scan_mid as usize){
            get_address = *next_address;
            get_ID += ID_pow * scan_mid;
            ID_pow /= have_childs / entry_byte_size;
        }
    }

    Ok(
        TreeData {
            tail_ID: get_ID,
            tail_address: get_address
        }
    )
}

//スーパーブロックに書いてある親を更新するための関数
pub fn reload_tree(bitmap_data: &FreeBitmap) -> Option<ResultReloadData>{
    let get_data = bitmap_data.get_free_bit(1);
    let cast_data = get_data.cast_address();
    let result_address: u64;
    match cast_data {
        CastAddressData::defrag(data) => {
            result_address = data;
        }
        _ => {
            return None;
        }
    }
    let result_data = ResultReloadData{
        set_flag: get_data,
        new_tree_adderss: result_address,
    };

    Some(result_data)
}

//指定のID位のの一歩手前まで行く
pub fn search_entry_location(super_block: &SuperBlockData, tree_address: u64, tree_depth: u64, ID: u64, entry_byte_size: u64, output_file: &mut fs::File) -> result::Result<Option<u64>, Box<dyn error::Error>>{
    let one_cluster_ID_num = super_block.get_one_cluster_bytes() / entry_byte_size;
    let one_cluster_child_num = super_block.get_one_block_bytes() / mem::size_of::<u64>() as u64;
    let mut one_cluster_child_pow = binary_pow(one_cluster_child_num, tree_depth);
    let mut one_cluster_ID_pow = binary_pow(one_cluster_child_num, tree_depth);
    let mut now_saerch_ID: u64 = ID % one_cluster_ID_num;
    let mut next_ID: u64 = ID / one_cluster_ID_num;
    let mut get_address: u64 = tree_address;
    let mut result_cluster_num = 0;

    let mut child_range: u64 = 0;

    for now_tree_depth in 0..tree_depth {
        output_file.seek(SeekFrom::Start(get_address * super_block.get_one_cluster_bytes() + now_saerch_ID * mem::size_of::<u64>() as u64))?;
        let mut buf = [0u8;8];
        output_file.read_exact(&mut buf)?;
        get_address = u64::from_le_bytes(buf);
        
        child_range = one_cluster_child_pow / one_cluster_ID_pow;
        one_cluster_child_pow /= one_cluster_child_num;
        one_cluster_ID_pow /= one_cluster_ID_num;

        if now_tree_depth < tree_depth{
            now_saerch_ID = next_ID / child_range;
            next_ID %= child_range;
        }
        if get_address == 0{
            return Ok(None);
        }
    }

    result_cluster_num = get_address;
    Ok(Some(result_cluster_num))
}


