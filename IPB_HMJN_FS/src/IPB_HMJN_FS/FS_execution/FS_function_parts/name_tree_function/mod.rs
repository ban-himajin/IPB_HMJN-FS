use crate::IPB_HMJN_FS::FS_core_types::{Entry, NameTreeEntry, SuperBlockData};
// use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData};
use crate::IPB_HMJN_FS::checksum_functions;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::tree_parts::{TreeData, ScanTreeResult};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::{Bitmap};

use std::{hash::Hash, error::Error, option::Option, result::Result, fs::File, io::{Seek, SeekFrom, Read, Write}, mem};

type EntryType = NameTreeEntry;


pub struct NameTree<Key, Value>{
    pub tree_data: TreeData<Key, Value>
}
impl<Key: Default + From<(usize, u64)> + Eq + Hash + Clone,
    Value: Default + Clone + Into<u64> + From<u64>> NameTree<Key, Value>{

    pub fn new(capacity: usize) -> Self{
        Self {
            tree_data: TreeData::<Key, Value>::new(capacity),
        }
    }

    //葉の一歩手前まで行く
    // fn scan_tree(&self, super_block: &mut SuperBlockData, search_ID: u64, bitmap: &mut Bitmap, output_file: &mut File)
    fn scan_tree(&self, super_block: &mut SuperBlockData, search_ID: u64, bitmap: &mut Bitmap, output_file: &mut File)
    -> Result<ScanTreeResult, Box<dyn Error>>
    {
        let result_data = self.tree_data.scan_tree(super_block, search_ID, super_block.name_tree_address, super_block.name_tree_depth, mem::size_of::<EntryType>() as u64, bitmap, output_file)?;
        if super_block.name_tree_address != result_data.root_address
            && super_block.name_tree_depth != result_data.tree_depth{

            super_block.name_tree_address = result_data.root_address;
            super_block.name_tree_depth = result_data.tree_depth;
        }

        Ok(result_data)
    }

    //エントリを読み込む
    //パスがかぶる可能性があるため最後までかぶっていないか？を確認するようにする
    pub fn get_entry(&self, super_block: &mut SuperBlockData, entry_name: &[u8], parent_name: &[u8], bitmap: &mut Bitmap, output_file: &mut File)
    -> Result<Option<Vec<u64>>, Box<dyn Error>>{
    // -> Result<Option<u64>, Box<dyn Error>>{
        // let search_ID = checksum_functions::CRC32(entry_name);
        // let result_data = self.scan_tree(super_block, search_ID, bitmap, output_file)?;
        // let one_cluster_entrys = super_block.get_one_cluster_bytes() / mem::size_of::<EntryType>() as u64;
        // let mut buf: Vec<u8> = vec![0u8; super_block.get_one_cluster_bytes() as usize];
        // output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address))?;
        // output_file.read_exact(buf.as_mut_slice())?;
        // let mut entry = EntryType::new(parent_name, entry_name, 0);
        // for counter in 0..one_cluster_entrys{
        //     let start = counter as usize * mem::size_of::<EntryType>();
        //     let end = start + mem::size_of::<EntryType>();
        //     let scan_entry = EntryType::from_bytes(&buf[start..end]);
        //     if scan_entry.parent == entry.parent && scan_entry.entry == entry.entry{
        //         entry = scan_entry;
        //         break;
        //     }
        // }
        // if entry.entry_ID == 0{
        //     Ok(None)
        // }
        // else{
        //     Ok(Some(entry.entry_ID))
        // }

        let mut result_vec: Vec<u64> = vec![];
        let search_ID = checksum_functions::CRC32(entry_name);
        let result_data = self.scan_tree(super_block, search_ID, bitmap, output_file)?;
        let one_cluster_entrys = super_block.get_one_cluster_bytes() / mem::size_of::<EntryType>() as u64;
        let mut buf: Vec<u8> = vec![0u8; super_block.get_one_cluster_bytes() as usize];
        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address))?;
        output_file.read_exact(buf.as_mut_slice())?;
        let mut entry = EntryType::new(parent_name, entry_name, 0);
        for counter in 0..one_cluster_entrys{
            let start = counter as usize * mem::size_of::<EntryType>();
            let end = start + mem::size_of::<EntryType>();
            let scan_entry = EntryType::from_bytes(&buf[start..end]);
            if scan_entry.parent == entry.parent && scan_entry.entry == entry.entry{
                entry = scan_entry;
                // break;
                result_vec.push(entry.entry_ID);
            }
        }
        if result_vec.len() == 0{
            Ok(None)
        }
        else{
            Ok(Some(result_vec))
        }


    }

    //エントリを書き込む
    pub fn put_entry(&self, super_block: &mut SuperBlockData, entry: EntryType, bitmap: &mut Bitmap, output_file: &mut File)
    -> Result<ScanTreeResult, Box<dyn Error>>{
        
        /*
            二分探索で現在はやっているがもしかしたらエントリの削除で
            空いた部分にアクセスできない可能性があるため修正予定
        */
        // let search_ID = entry.entry as u64;
        // let result_data = self.scan_tree(super_block, search_ID, bitmap, output_file)?;
        // let one_cluster_entrys = super_block.get_one_cluster_bytes() / mem::size_of::<EntryType>() as u64;
        // let mut buf: Vec<u8> = vec![0u8; super_block.get_one_cluster_bytes() as usize];
        // output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address))?;
        // output_file.read_exact(buf.as_mut_slice())?;
        // let mut scan_top = one_cluster_entrys;
        // let mut scan_low = 0u64;
        // let mut scan_mid = (scan_top / 2) + scan_low;
        // let mut prev_mid = None;
        // // for _ in 0..one_cluster_entrys{
        // while scan_top != scan_mid && scan_low != scan_low{
        //     let start = scan_mid as usize * mem::size_of::<EntryType>();
        //     let end = start + mem::size_of::<EntryType>();
        //     let scan_entry = EntryType::from_bytes(&buf[start..end]);
        //     if scan_entry.entry == 0 && scan_entry.parent == 0 && scan_entry.entry_ID == 0{
        //         prev_mid = Some(scan_mid);
        //         scan_top = scan_mid;
        //         scan_mid = (scan_top / 2) + scan_low;
        //     }
        //     else{
        //         scan_low = scan_mid;
        //         scan_mid = (scan_top / 2) + scan_low;
        //         if prev_mid != None{
        //             break;
        //         }
        //     }
        // }
        // if let Some(some_mid) = prev_mid{
        //     output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address + some_mid * mem::size_of::<EntryType>() as u64))?;
        //     output_file.write_all(&entry.to_le_bytes())?;
        //     println!("書き込み位置1 : {:?}", super_block.get_one_cluster_bytes() * result_data.leaf_address + some_mid * mem::size_of::<EntryType>() as u64);
        // }
        // else{
        //     output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address + scan_mid * mem::size_of::<EntryType>() as u64))?;
        //     output_file.write_all(&entry.to_le_bytes())?;
        //     println!("書き込み位置2 : {:?}", super_block.get_one_cluster_bytes() * result_data.leaf_address + scan_mid * mem::size_of::<EntryType>() as u64);
        // }

        let search_ID = entry.entry as u64;
        let one_cluster_entrys = super_block.get_one_cluster_bytes() / mem::size_of::<EntryType>() as u64;
        let result_data = self.scan_tree(super_block, search_ID, bitmap, output_file)?;
        let entry_size = mem::size_of::<EntryType>() as u64;
        let mut write_check = false;
        // let mut buf: Vec<u8> = Vec::with_capacity(super_block.get_one_cluster_bytes() as usize);
        let mut buf: Vec<u8> = vec![0u8; super_block.get_one_cluster_bytes() as usize];

        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address))?;
        output_file.read_exact(&mut buf)?;

        for i in 0..one_cluster_entrys{
            let start = (i * entry_size) as usize;
            let end = ((i + 1) * entry_size) as usize;
            let entry_buffer = EntryType::from_bytes(&buf[start..end]);
            if entry_buffer.entry == 0 && entry_buffer.entry_ID == 0 && entry_buffer.parent == 0{
                output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_data.leaf_address + i * entry_size))?;
                output_file.write_all(&entry.to_le_bytes())?;
                write_check = true;
                break;
            }
        }

        Ok(result_data)
    }

}
