use std::{fs::File, result::Result, error::Error};

use crate::IPB_HMJN_FS::FS_core_types::{Entry};

pub trait Create<'a, ResultType>{

    // fn create(&self, path: &'a str, entry: Entry, output_file: &mut File) -> Result<Option<(u64, u64)>, Box<dyn Error>>;
    fn create(&self, path: &'a str, entry: Entry, output_file: &mut File) -> Result<ResultType, Box<dyn Error>>;

}
pub trait Delete<'a>{

    fn delete(&self, path: &'a str, output_file: &mut File) -> Result<(), Box<dyn Error>>;

}

pub trait Open<'a>{

    fn open(path: &'a str) -> Self;

}
pub trait Close{

    fn close(self);

}
pub trait StreamWrite{

    fn write(&self, data: &[u8], output_file: &mut File) -> Result<(), Box<dyn Error>>;

}
pub trait StreamRead{

    fn read(&self, buf: &mut [u8], output_file: &mut File) -> Result<(), Box<dyn Error>>;

}

pub trait SpecialOperations{

    fn execution(&self);

}