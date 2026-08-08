use std::{result, error, fs, mem};

use crate::{API::Create, IPB_HMJN_FS::{FS_core_types::{self, DirectoryType, Entry, EntryStructType, EntryType, NameTreeEntry}, super_block_config}};
pub mod IPB_HMJN_FS;
pub mod API;


fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut output_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(IPB_HMJN_FS::Constant::OUTPUT_FILE_NAME)?;
    // let mut super_block = IPB_HMJN_FS::FS_setup::setup_super_block_data()?;

    let mut super_block = IPB_HMJN_FS::FS_setup::debug_super_block(0, 512, 1, 1, 128, 1);

    println!("{:?}", super_block);
    IPB_HMJN_FS::FS_format::FS_format(&mut super_block, &mut output_file)?;

    
    let execution = IPB_HMJN_FS::FS_execution::executions::Execution::new(super_block, 0, &mut output_file)?;
    
    println!("free : {:?}", execution.free_ID_bitmap.borrow().bitmap_data.bitmap);
    println!("bitmap : {:?}", execution.bitmap.borrow().bitmap_data.bitmap);
    println!("test2");
    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<u64, u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);
    
    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<u64, u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);
    
    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<(usize, u64), u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);
    
    // let mut data = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::TreeData::<(usize, u64), u64>::new(10);
    // let address = execution.super_block.borrow().directory_tree_address;
    // println!("test3");
    // let depth = execution.super_block.borrow().directory_tree_depth;
    // data.scan_tree(&execution.super_block.borrow_mut(), 0, address, depth, 512, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    //
    // println!("free ID : {:?}", execution.free_ID_bitmap.borrow().bitmap_data.bitmap);
    // println!("bitmap : {:?}", execution.bitmap.borrow().bitmap_data.bitmap);
    //
    // let entry = FS_core_types::Entry::new(EntryStructType::directory, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.directory_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), 0, entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    //
    // let entry = FS_core_types::Entry::new(EntryStructType::directory, 1, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.directory_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), 1, entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    //
    // let entry = FS_core_types::Entry::new(EntryStructType::directory, 2, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.directory_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), 2, entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;


    // let new_ID = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type1::get_endpoint_address(&execution.super_block.borrow(), execution.super_block.borrow().directory_tree_address, execution.super_block.borrow().directory_tree_depth, mem::size_of::<Entry>() as u64, &mut output_file)?;
    // println!("test1 {}", new_ID.tail_address);
    // println!("test1 {}", new_ID.tail_ID);
    
    // let test = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type1::search_entry_location(&execution.super_block.borrow(), execution.super_block.borrow().directory_tree_address, execution.super_block.borrow().directory_tree_depth, 1, mem::size_of::<IPB_HMJN_FS::FS_core_types::Entry>() as u64, &mut output_file)?;
    // println!("test2 : {:?}", test);
    // println!("test4");



    let entry  = NameTreeEntry::new("".as_bytes(), "C".as_bytes(), 1);
    execution.name_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;

    let entry  = NameTreeEntry::new("".as_bytes(), "C".as_bytes(), 2);
    execution.name_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;

    let entry  = NameTreeEntry::new("".as_bytes(), "C".as_bytes(), 3);
    execution.name_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    
    let data = execution.name_tree.borrow().get_entry(&mut execution.super_block.borrow_mut(), "C".as_bytes(), "a".as_bytes(), &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    println!("ID : {:?}", data);

    // //C
    // let mut entry = FS_core_types::Entry::new(EntryStructType::directory, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // println!("entry : {:?}", entry);
    // execution.create("C", entry, &mut output_file)?;
    
    // println!("{:?}", execution.super_block.borrow());

    
    // execution.bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    // execution.free_ID_bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;

    // // B
    // // let entry = FS_core_types::Entry::new(EntryStructType::directory, 1, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // entry = FS_core_types::Entry::new(EntryStructType::directory, 1, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.create("C", entry, &mut output_file)?;
    
    // execution.bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    // execution.free_ID_bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;

    // // D
    // // let entry = FS_core_types::Entry::new(EntryStructType::directory, 2, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // entry = FS_core_types::Entry::new(EntryStructType::directory, 2, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.create("C", entry, &mut output_file)?;

    // execution.bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    // execution.free_ID_bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    println!("{:?}", execution.super_block.borrow());
    Ok(())
}

