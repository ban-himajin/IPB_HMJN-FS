use core::error;
use std::{result, fs, cell::RefCell};

// use crate::IPB_HMJN_FS::{FS_core_types::{self, SuperBlockData}, FS_execution::FS_function_parts::bitmap_function::bitmap_parts::FreeBitmap};
use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::{Bitmap};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::free_ID_bitmap_function::{FreeIDBitmap};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::directory_tree_function::{DirectoryTree};
use crate::API::{Create, Delete, Open, Close, StreamWrite, StreamRead, SpecialOperations};

pub struct Execution{
    pub super_block: RefCell<SuperBlockData>,
    pub bitmap: RefCell<Bitmap>,
    pub free_ID_bitmap: RefCell<FreeIDBitmap>,
    pub directory_tree: RefCell<DirectoryTree<(usize, u64), u64>>
}
impl<'a> Execution{
    pub fn new(super_block: SuperBlockData, capacity: usize, output_file: &mut fs::File) -> result::Result<Self, Box<dyn error::Error>>{
        let bitmap_data = Bitmap::new(&super_block, output_file)?;
        let free_ID_bitmap_data = FreeIDBitmap::new(&super_block, output_file)?;

        Ok(
            Self{
                super_block: RefCell::new(super_block),
                bitmap: RefCell::new(bitmap_data),
                free_ID_bitmap: RefCell::new(free_ID_bitmap_data),
                directory_tree: RefCell::new(DirectoryTree::new(capacity)),
            }
        )

    }
}
impl<'a> Create<'a> for Execution{
    fn create(&self, path: &'a str) {
        
    }
}
impl<'a> Delete<'a> for Execution{
    fn delete(&self, path: &'a str) {
        
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
    fn write(&self, buf: &[u8]) {
        
    }
}
impl StreamRead for FSStream<'_>{
    fn read(&self, buf: &mut [u8]) {
        
    }
}


