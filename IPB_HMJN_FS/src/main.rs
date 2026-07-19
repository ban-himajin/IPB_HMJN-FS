use std::{result, error, fs};
mod IPB_HMJN_FS;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut output_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(IPB_HMJN_FS::constant::OUTPUT_FILE_NAME)?;
    let mut super_block = IPB_HMJN_FS::FS_setup::setup_super_block_data()?;
    println!("{:?}", super_block);
    IPB_HMJN_FS::FS_format::FS_format(&mut super_block, &mut output_file)?;

    let execution = IPB_HMJN_FS::FS_execution::executions::Execution::new(super_block, &mut output_file)?;

    let bit_flag_data = execution.bitmap.get_free_bit(11);
    execution.bitmap.write_bit_flag(bit_flag_data)?;

    Ok(())
}
