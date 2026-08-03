use std::{result, error, fs, mem};

use crate::IPB_HMJN_FS::FS_core_types::Entry;
pub mod IPB_HMJN_FS;
pub mod API;


fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut output_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(IPB_HMJN_FS::constant::OUTPUT_FILE_NAME)?;
    let mut super_block = IPB_HMJN_FS::FS_setup::setup_super_block_data()?;
    println!("{:?}", super_block);
    IPB_HMJN_FS::FS_format::FS_format(&mut super_block, &mut output_file)?;


    let execution = IPB_HMJN_FS::FS_execution::executions::Execution::new(super_block, 10, &mut output_file)?;
    println!("test1");
    let bit_flag_data = execution.bitmap.borrow().get_free_bit(11);
    execution.bitmap.borrow().write_bit_flag(bit_flag_data)?;
    
    println!("test2");
    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<u64, u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);

    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<u64, u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);

    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<(usize, u64), u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);

    let mut data = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::TreeData::<(usize, u64), u64>::new(10);
    let address = execution.super_block.borrow().directory_tree_address;
    let depth = execution.super_block.borrow().directory_tree_depth;
    data.scan_tree(&execution.super_block.borrow_mut(), 0, address, depth, 512, &mut execution.bitmap.borrow_mut(), &mut output_file)?;

    // let new_ID = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type1::get_endpoint_address(&execution.super_block.borrow(), execution.super_block.borrow().directory_tree_address, execution.super_block.borrow().directory_tree_depth, mem::size_of::<Entry>() as u64, &mut output_file)?;
    // println!("test1 {}", new_ID.tail_address);
    // println!("test1 {}", new_ID.tail_ID);

    // let test = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type1::search_entry_location(&execution.super_block.borrow(), execution.super_block.borrow().directory_tree_address, execution.super_block.borrow().directory_tree_depth, 1, mem::size_of::<IPB_HMJN_FS::FS_core_types::Entry>() as u64, &mut output_file)?;
    // println!("test2 : {:?}", test);
    Ok(())
}

