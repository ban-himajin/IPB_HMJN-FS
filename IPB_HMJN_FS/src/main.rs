use std::{result, error, fs, mem};

use crate::{API::Create, IPB_HMJN_FS::{FS_core_types::{self, DirectoryType, Entry, EntryStructType, EntryType, FileType, NameTreeEntry}, super_block_config}};
pub mod IPB_HMJN_FS;
pub mod API;
use crate::IPB_HMJN_FS::command_console;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::write_actual_situation_parts;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::read_actual_situation_parts;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut output_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(IPB_HMJN_FS::Constant::OUTPUT_FILE_NAME)?;
    // let mut super_block = IPB_HMJN_FS::FS_setup::setup_super_block_data()?;

    let mut super_block = IPB_HMJN_FS::FS_setup::debug_super_block(0, 512, 1, 1, 128, 1);
    // let mut super_block = IPB_HMJN_FS::FS_setup::debug_super_block(0, 512, 1, 8, 128, 1);

    println!("{:?}", super_block);
    IPB_HMJN_FS::FS_format::FS_format(&mut super_block, &mut output_file)?;

    
    let execution = IPB_HMJN_FS::FS_execution::executions::Execution::new(super_block, 10, &mut output_file)?;
    
    println!("free : {:?}", execution.free_ID_bitmap.borrow().bitmap_data.bitmap);
    println!("bitmap : {:?}", execution.bitmap.borrow().bitmap_data.bitmap);
    println!("test2");

    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<u64, u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);
    // 
    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<u64, u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);
    // 
    // IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type2::TreeData::<(usize, u64), u64>::scan_tree(&execution.super_block.borrow_mut(), 128, 0, 4, 512);
    // 
    // let mut data = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::TreeData::<(usize, u64), u64>::new(10);
    // let address = execution.super_block.borrow().directory_tree_address;
    // println!("test3");
    // let depth = execution.super_block.borrow().directory_tree_depth;
    // data.scan_tree(&execution.super_block.borrow_mut(), 0, address, depth, 512, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    //
    // println!("free ID : {:?}", execution.free_ID_bitmap.borrow().bitmap_data.bitmap);
    // println!("bitmap : {:?}", execution.bitmap.borrow().bitmap_data.bitmap);
    //
    // let entry = FS_core_types::Entry::new(EntryStructType::Directory, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.directory_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), 0, entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    //
    // let entry = FS_core_types::Entry::new(EntryStructType::Directory, 1, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.directory_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), 1, entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    //
    // let entry = FS_core_types::Entry::new(EntryStructType::Directory, 2, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    // execution.directory_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), 2, entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    // 
    // let new_ID = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type1::get_endpoint_address(&execution.super_block.borrow(), execution.super_block.borrow().directory_tree_address, execution.super_block.borrow().directory_tree_depth, mem::size_of::<Entry>() as u64, &mut output_file)?;
    // println!("test1 {}", new_ID.tail_address);
    // println!("test1 {}", new_ID.tail_ID);
    // 
    // let test = IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::type1::search_entry_location(&execution.super_block.borrow(), execution.super_block.borrow().directory_tree_address, execution.super_block.borrow().directory_tree_depth, 1, mem::size_of::<IPB_HMJN_FS::FS_core_types::Entry>() as u64, &mut output_file)?;
    // println!("test2 : {:?}", test);
    // println!("test4");
    // 
    // 
    // let entry  = NameTreeEntry::new("".as_bytes(), "C".as_bytes(), 1);
    // execution.name_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    // 
    // let entry  = NameTreeEntry::new("".as_bytes(), "C".as_bytes(), 2);
    // execution.name_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    // 
    // let entry  = NameTreeEntry::new("".as_bytes(), "C".as_bytes(), 3);
    // execution.name_tree.borrow().put_entry(&mut execution.super_block.borrow_mut(), entry, &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    // 
    // let data = execution.name_tree.borrow().get_entry(&mut execution.super_block.borrow_mut(), "C".as_bytes(), "".as_bytes(), &mut execution.bitmap.borrow_mut(), &mut output_file)?;
    // println!("ID : {:?}", data);

    //C
    let mut entry = FS_core_types::Entry::new(EntryStructType::Directory, 0, 0, 0, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "C");
    println!("entry : {:?}", entry);
    execution.create("C", entry, &mut output_file)?;
    
    println!("{:?}", execution.super_block.borrow());

    
    execution.bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    execution.free_ID_bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;

    // D
    entry = FS_core_types::Entry::new(EntryStructType::Directory, 1, 0, 0, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "D");
    execution.create("D", entry, &mut output_file)?;
    
    execution.bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    execution.free_ID_bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;

    // E
    let entry_path = "E";
    // entry = FS_core_types::Entry::new(EntryStructType::Directory, 2, 0, 0, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, "E");
    entry = FS_core_types::Entry::new(EntryStructType::File, 2, 0, 0, 0, 0, 0, 0, EntryType::File(FileType::new(0, 0, 0)), 0, "E");
    let data = execution.create(entry_path, entry, &mut output_file)?;
    println!("log : {:?}", data);
    // let path_name = r"C:\Users\banhi\Downloads\ロゴ_暇.png";
    let path_name = r"C:\Users\banhi\Downloads\SquirrelSetup.log";
    let result = write_actual_situation_parts::check_function(&execution.super_block.borrow(), &execution.bitmap.borrow(), path_name, &mut output_file)?;

    if let (Some(entry_data), Some(write_data)) = (data, result){
        println!("testlog");
        write_actual_situation_parts::connect_data(&execution.super_block.borrow(), entry_data, write_data, &mut output_file)?;
    }

    execution.bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    execution.free_ID_bitmap.borrow().reload_bitmap(&execution.super_block.borrow(), &mut output_file)?;
    println!("{:?}", execution.super_block.borrow());

    let read_data = read_actual_situation_parts::read_file(&mut execution.super_block.borrow_mut(), &execution.name_tree.borrow(), &execution.directory_tree.borrow(), &mut execution.bitmap.borrow_mut(), entry_path, &mut output_file)?;

    println!("--------------read_data-------------\n{:?}", read_data);

    // command_console::start_command_console(&execution);


    // println!("----bitmap log : {:?}", execution.bitmap.borrow().bitmap_data.bitmap);
    // println!("bitmap log--------");
    // for bit in execution.bitmap.borrow().bitmap_data.bitmap.borrow().iter(){
    //     print!("{:08b},", bit);
    // }
    // println!("\n--------------");
    //
    // println!("check function -------------------");
    // let path_name = r"C:\Users\banhi\Downloads\ロゴ_暇.png";
    // let result = write_actual_situation_parts::check_function(&execution.super_block.borrow(), &execution.bitmap.borrow(), path_name, &mut output_file)?;
    // println!("\nresult test : {:?}", result);
    //
    // // println!("----bitmap log : {:?}", execution.bitmap.borrow().bitmap_data.bitmap);
    // println!("bitmap log--------");
    // for bit in execution.bitmap.borrow().bitmap_data.bitmap.borrow().iter(){
    //     print!("{:08b},", bit);
    // }
    // println!("\n--------------");

    println!("{:?}", execution.super_block.borrow());
    Ok(())
}

