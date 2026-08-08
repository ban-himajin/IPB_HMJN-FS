use core::error;
use std::{result, fs::{File}, cell::RefCell, error::Error, io::{Seek, SeekFrom, Write, Read}};

use crate::IPB_HMJN_FS::FS_core_types::{Entry, SuperBlockData, NameTreeEntry};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::{Bitmap};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::free_ID_bitmap_function::{FreeIDBitmap};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::directory_tree_function::{DirectoryTree};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::name_tree_function::{NameTree};
use crate::IPB_HMJN_FS::general_function;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_parts::{FreeBits, CastAddressData};

use crate::API::{Create, Delete, Open, Close, StreamWrite, StreamRead, SpecialOperations};

pub struct Execution{
    pub super_block: RefCell<SuperBlockData>,
    pub bitmap: RefCell<Bitmap>,
    pub free_ID_bitmap: RefCell<FreeIDBitmap>,
    pub directory_tree: RefCell<DirectoryTree<(usize, u64), u64>>,
    pub name_tree: RefCell<NameTree<(usize, u64), u64>>,
}
impl<'a> Execution{
    pub fn new(super_block: SuperBlockData, capacity: usize, output_file: &mut File) -> result::Result<Self, Box<dyn error::Error>>{
        let bitmap_data = Bitmap::new(&super_block, output_file)?;
        let free_ID_bitmap_data = FreeIDBitmap::new(&super_block, output_file)?;

        Ok(
            Self{
                super_block: RefCell::new(super_block),
                bitmap: RefCell::new(bitmap_data),
                free_ID_bitmap: RefCell::new(free_ID_bitmap_data),
                directory_tree: RefCell::new(DirectoryTree::new(capacity)),
                name_tree: RefCell::new(NameTree::new(capacity),)
            }
        )

    }
}
impl<'a> Create<'a, Option<(u64, u64)>> for Execution{
    fn create(&self, path: &'a str, entry: Entry, output_file: &mut File)
    -> Result<Option<(u64, u64)>, Box<dyn Error>> {

        let path_analysis = general_function::str_analysis(path, "/");
        let path_nums = path_analysis.len();
        let entry_name = *path_analysis.get(path_nums - 1).unwrap();
        let parent_name = if path_nums >= 2{
            *path_analysis.get(path_nums - 2).unwrap()
        }
        else{
            ""
        };

        let free_ID = self.free_ID_bitmap.borrow().get_free_bit(1);
        let entry_ID = match free_ID.cast_address(){
            CastAddressData::defrag(data) => {
                Some(data)
            },
            _ => None,
        };

        let name_entry = NameTreeEntry::new(parent_name.as_bytes(), entry_name.as_bytes(), entry_ID.unwrap());

        self.name_tree.borrow().put_entry(&mut self.super_block.borrow_mut(), name_entry, &mut self.bitmap.borrow_mut(), output_file)?;
        let mut leaf_address = 0;
        
        if entry.name == general_function::to_fixed_array(entry_name.as_bytes()){
            let result_data = self.directory_tree.borrow().put_entry(&mut self.super_block.borrow_mut(), entry_ID.unwrap(), entry, &mut self.bitmap.borrow_mut(), output_file)?;
            leaf_address = result_data.leaf_address;
            if self.super_block.borrow().directory_tree_address != result_data.root_address
            && self.super_block.borrow().directory_tree_depth != result_data.tree_depth{
                
                self.super_block.borrow_mut().directory_tree_address = result_data.root_address;
                self.super_block.borrow_mut().directory_tree_depth = result_data.tree_depth;
                output_file.seek(SeekFrom::Start(0))?;
                self.super_block.borrow().write_super_block_data(output_file)?;
            }
            self.free_ID_bitmap.borrow().write_bit_flag(free_ID)?;
            println!("entry ID : {:?}", entry_ID);
            println!("書き込みに成功しました");
        }
        else{
            eprintln!("エントリ名の不一致 [entry name : {:?}], [path entry name : {:?}]", entry.name, entry_name.as_bytes());
        }
        if let Some(ID) = entry_ID{
            Ok(Some((leaf_address, ID)))
        }
        else{
            Ok(None)
        }
    }
}
impl<'a> Delete<'a> for Execution{
    fn delete(&self, path: &'a str, output_file: &mut File) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct FSStream<'a>{
    path: &'a str,
    pointer: RefCell<u64>,
}
impl<'a> Open<'a> for FSStream<'a>{
    fn open(path: &'a str) -> Self {
        Self {
            path,
            pointer: RefCell::new(0)
        }
    }
}
impl Close for FSStream<'_>{
    fn close(self) {
        
    }
}
impl StreamWrite for FSStream<'_>{
    fn write(&self, buf: &[u8], output_file: &mut File) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
impl StreamRead for FSStream<'_>{
    fn read(&self, buf: &mut [u8], output_file: &mut File) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}


