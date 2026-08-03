use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::{TreeData, ScanTreeResult};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::Bitmap;
use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData, Entry};

use std::io::{Read, Seek, Write};
use std::{hash::Hash, option::Option, result::Result, error::Error, cell::Cell, default::Default, fs::File, mem};

pub struct DirectoryTree<Key, Value>{
    pub tree_data: TreeData<Key, Value>
}
impl<Key: Default + From<(usize, u64)> + Eq + Hash + Clone,
    Value: Default + Clone + Into<u64> + From<u64>> DirectoryTree<Key, Value>{
    pub fn new(capacity: usize) -> Self{
        Self {
            tree_data: TreeData::<Key, Value>::new(capacity),
        }
    }

    // pub fn new_parent(&self, super_block: &mut SuperBlockData, bitmap: &mut Bitmap, output_file: &mut File) -> Result<bool, Box<dyn Error>>{
    //     let new_parent_data= TreeData::<Key, Value>::new_parent(super_block, super_block.directory_tree_address, bitmap, output_file)?;
    //     if let Some(new_paret) = new_parent_data{
    //         super_block.directory_tree_address = new_paret;
    //         super_block.directory_tree_depth += 1;
    //         Ok(true)
    //     }
    //     else {
    //         Ok(false)
    //     }
    // }
    // pub fn new_child(super_block: &SuperBlockData, parent_address: u64, child_offset: u64, bitmap: &mut Bitmap, output_file: &mut fs::File) -> Result<Option<u64>, Box<dyn error::Error>>{
    //     TreeData::<Key, Value>::new_child(super_block, super_block.directory_tree_address, bitmap, output_file)?;
    // }

    //葉の一歩手前まで行く
    fn scan_tree(&self, super_block: &mut SuperBlockData, search_ID: u64, bitmap: &mut Bitmap, output_file: &mut File)
    -> Result<u64, Box<dyn Error>>
    {
        let result_data = self.tree_data.scan_tree(super_block, search_ID, super_block.directory_tree_address, super_block.directory_tree_depth, mem::size_of::<Entry>() as u64, bitmap, output_file)?;
        if super_block.directory_tree_address != result_data.root_address
            && super_block.directory_tree_depth != result_data.tree_depth{
            super_block.directory_tree_address = result_data.root_address;
            super_block.directory_tree_depth = result_data.tree_depth;
        }

        Ok(result_data.leaf_address)
    }

    //エントリを読み込む
    pub fn get_entry(&self, super_block: &mut SuperBlockData, search_ID: u64, bitmap: &mut Bitmap, output_file: &mut File)
    -> Result<Entry, Box<dyn Error>>{
        let leaf_address = self.scan_tree(super_block, search_ID, bitmap, output_file)?;
        let one_cluster_entrys = super_block.get_one_cluster_bytes() / mem::size_of::<Entry>() as u64;
        let offset = search_ID % one_cluster_entrys;
        let mut buf = [0u8;512];
        output_file.seek(std::io::SeekFrom::Start(super_block.get_one_cluster_bytes() * leaf_address + mem::size_of::<Entry>() as u64 * offset))?;
        output_file.read_exact(&mut buf)?;
        let read_entry = Entry::from_bytes(&buf);
        Ok(read_entry)
    }

    //エントリを書き込む
    pub fn put_entry(&self, super_block: &mut SuperBlockData, search_ID: u64, entry_data: Entry, bitmap: &mut Bitmap, output_file: &mut File)
    -> Result<(), Box<dyn Error>>{
        let leaf_address = self.scan_tree(super_block, search_ID, bitmap, output_file)?;
        let one_cluster_entrys = super_block.get_one_cluster_bytes() / mem::size_of::<Entry>() as u64;
        let offset = search_ID % one_cluster_entrys;
        output_file.seek(std::io::SeekFrom::Start(super_block.get_one_cluster_bytes() * leaf_address + mem::size_of::<Entry>() as u64 * offset))?;
        output_file.write_all(&entry_data.to_le_bytes())?;
        Ok(())
    }

}

