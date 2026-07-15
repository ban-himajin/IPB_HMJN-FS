use std::{result, error};
mod IPB_HMJN_FS;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut super_block = IPB_HMJN_FS::FS_setup::SetupSuperBlockData()?;
    println!("{:?}", super_block);
    IPB_HMJN_FS::FS_format::FS_format(&mut super_block)?;
    
    
    Ok(())
}
