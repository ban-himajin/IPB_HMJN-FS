use std::{error, fs, io::{Read, Seek, SeekFrom, Write}, mem, result, cell::RefCell};
use crate::IPB_HMJN_FS::{FS_core_types::{Entry, SuperBlockData}, FS_execution::FS_function_parts::{bitmap_function::bitmap_parts::FreeBitmap, tree_parts::type1::{self, reload_tree, search_entry_location, get_endpoint_address, TreeData}, free_ID_tree_function::type1::{FreeIDTree}}};

struct DirectoryTree{
    tail_ID: RefCell<u64>,
    tail_cluster: RefCell<u64>,
}
impl DirectoryTree{
    //ディレクトリツリーの末端にある使われていないIDを取得する
    pub fn new(super_block: &SuperBlockData, output_file: &mut fs::File) -> result::Result<Self, Box<dyn error::Error>>{
        let tree_data = get_endpoint_address(super_block, super_block.directory_tree_address, super_block.directory_tree_depth, mem::size_of::<Entry>() as u64, output_file)?;
        let loop_num = super_block.get_one_block_bytes() / mem::size_of::<Entry>() as u64;
        let now_address_byte = tree_data.tail_address * super_block.get_one_cluster_bytes();
        let mut get_ID = tree_data.tail_ID;
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

        Ok(
            Self {
                tail_ID: RefCell::new(ID),
                tail_cluster: RefCell::new(tree_data.tail_address),
            }
        )
    }

    //ディレクトリツリーの新しい親位置を確保する関数
    pub fn reload_parent_directory_tree(&self, super_block: &mut SuperBlockData, bitmap_data: &FreeBitmap, output_file: &mut fs::File) -> result::Result<bool, Box<dyn error::Error>>{
        let result_data = reload_tree(bitmap_data);
        if let Some(data) = result_data{
            output_file.seek(SeekFrom::Start(data.new_tree_adderss * super_block.get_one_cluster_bytes()))?;
            output_file.write_all(&super_block.directory_tree_address.to_le_bytes())?;
            bitmap_data.write_bit_flag(data.set_flag)?;
            super_block.directory_tree_address = data.new_tree_adderss;
        }
        else{
            return Ok(false);
        }
        Ok(true)
    }

    //指定したアドレスに存在するEntryを返す
    pub fn get_entry(&self, super_block: &SuperBlockData, select_ID: u64, output_file: &mut fs::File) -> result::Result<Option<Entry>, Box<dyn error::Error>>{
        let result = search_entry_location(super_block, super_block.directory_tree_address, super_block.directory_tree_depth, select_ID, mem::size_of::<Entry>() as u64, output_file)?;
        if let Some(some_data) = result {
            let mut buffer = [0u8; 512];
            output_file.seek(SeekFrom::Start(some_data * super_block.get_one_cluster_bytes()))?;
            output_file.read_exact(&mut buffer)?;
            let get_entry = Entry::from_bytes(&buffer);
            return Ok(Some(get_entry));
        }
        Ok(None)
    }

    //指定したツリー内の指定したoffsetに新しいツリー子へのアドレスを入れる
    pub fn new_child_directory_tree(&self, super_block: &SuperBlockData, now_tree_address: u64, offset: u64, bitmap_data: &FreeBitmap, output_file: &mut fs::File) -> result::Result<bool, Box<dyn error::Error>>{
        let get_data = reload_tree(bitmap_data);
        if let Some(some_data) = get_data{
            output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * now_tree_address + offset * mem::size_of::<u64>() as u64))?;
            output_file.write_all(&some_data.new_tree_adderss.to_le_bytes())?;
            bitmap_data.write_bit_flag(some_data.set_flag)?;
            return Ok(true)
        }

        Ok(false)
    }

    //entryを入れる
    pub fn input_entry(&self,mut select_entry: Entry, free_ID_tree: &FreeIDTree, super_block: &SuperBlockData, output_file: &mut fs::File) -> result::Result<bool, Box<dyn error::Error>>{
        if *free_ID_tree.tail_ID.borrow() != 0{
            if let Some(get_free_ID) = free_ID_tree.get_free_ID(super_block, output_file)?{
                if let Some(result) = search_entry_location(super_block, super_block.directory_tree_address, super_block.directory_tree_depth, get_free_ID, mem::size_of::<Entry>() as u64, output_file)?{
                    output_file.seek(SeekFrom::Start(result * super_block.get_one_cluster_bytes() + ( (get_free_ID % (super_block.get_one_cluster_bytes() / mem::size_of::<Entry>() as u64) ) *  mem::size_of::<Entry>() as u64) ) )?;
                    select_entry.ID = get_free_ID;
                    output_file.write_all(&select_entry.to_le_bytes())?;
                    let t_or_f = free_ID_tree.delete_free_ID(super_block, output_file)?;
                    return Ok(t_or_f)
                }
            }
        }
        else{
            //free IDがあったときの処理
        }

        Ok(false)
    }

    pub fn delete_entry(&self, super_block: &SuperBlockData, delete_ID: u64, free_ID_tree: &FreeIDTree, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
        let result = search_entry_location(super_block, super_block.directory_tree_address, super_block.directory_tree_depth, delete_ID, mem::size_of::<Entry>() as u64, output_file)?;
        if let Some(some_data) = result{
            let mut buffer = [0u8; 512];
            let have_entry_num = super_block.get_one_cluster_bytes() / mem::size_of::<Entry>() as u64;
            output_file.seek(SeekFrom::Start(some_data * super_block.get_one_cluster_bytes() + mem::size_of::<Entry>() as u64 * (delete_ID % have_entry_num) ) )?;
            output_file.read_exact(&mut buffer)?;
            let delete_entry_data = Entry::from_bytes(&buffer);
            free_ID_tree.input_free_ID(delete_entry_data.ID, super_block, output_file)?;
            let prev_sibling_ID = delete_entry_data.prev_sibling_ID;
            let next_sibling_ID = delete_entry_data.next_sibling_ID;
            if prev_sibling_ID != delete_entry_data.ID{
                //前のエントリが存在した場合の処理
                let result = search_entry_location(super_block, super_block.directory_tree_address, super_block.directory_tree_depth, prev_sibling_ID, mem::size_of::<Entry>() as u64, output_file)?;
                if let Some(some_data) = result{
                    let mut buffer = [0u8; 512];
                    let have_entry_num = super_block.get_one_cluster_bytes() / mem::size_of::<Entry>() as u64;
                    output_file.seek(SeekFrom::Start(some_data * super_block.get_one_cluster_bytes() + mem::size_of::<Entry>() as u64 * (delete_ID % have_entry_num) ) )?;
                    output_file.read_exact(&mut buffer)?;
                    let mut entry_data = Entry::from_bytes(&buffer);
                    if next_sibling_ID != delete_entry_data.ID{
                        entry_data.next_sibling_ID = next_sibling_ID;
                    }
                    else{
                        entry_data.next_sibling_ID = entry_data.ID;
                    }
                    
                    output_file.seek(SeekFrom::Start(some_data * super_block.get_one_cluster_bytes() + mem::size_of::<Entry>() as u64 * (delete_ID % have_entry_num) ) )?;
                    output_file.write_all(&entry_data.to_le_bytes())?;
                }
            }
            
            if next_sibling_ID != delete_entry_data.ID{
                //次のエントリが存在した場合の処理
                let result = search_entry_location(super_block, super_block.directory_tree_address, super_block.directory_tree_depth, next_sibling_ID, mem::size_of::<Entry>() as u64, output_file)?;
                if let Some(some_data) = result{
                    let mut buffer = [0u8; 512];
                    let have_entry_num = super_block.get_one_cluster_bytes() / mem::size_of::<Entry>() as u64;
                    output_file.seek(SeekFrom::Start(some_data * super_block.get_one_cluster_bytes() + mem::size_of::<Entry>() as u64 * (delete_ID % have_entry_num) ) )?;
                    output_file.read_exact(&mut buffer)?;
                    let mut entry_data = Entry::from_bytes(&buffer);
                    
                    entry_data.prev_sibling_ID = prev_sibling_ID;
                    
                    output_file.seek(SeekFrom::Start(some_data * super_block.get_one_cluster_bytes() + mem::size_of::<Entry>() as u64 * (delete_ID % have_entry_num) ) )?;
                    output_file.write_all(&entry_data.to_le_bytes())?;
                }
                
            }
        }

        Ok(())
    }

}

