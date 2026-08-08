use core::error;
use std::{io, result, string::String, mem};
use crate::IPB_HMJN_FS;
use crate::IPB_HMJN_FS::FS_core_types::Entry;
use crate::IPB_HMJN_FS::checksum_functions::CRC32;
use crate::IPB_HMJN_FS::super_block_config;
use crate::IPB_HMJN_FS::general_function::{round_up};

use super::FS_core_types;
use super::Constant;

#[derive(Debug)]
struct GetInputData{
    partition_LBA: u64,
    one_sector_size: u64,
    one_block_sector_num: u64,
    one_cluster_block_num: u64,
    partition_cluseter_num: u64,
    back_up_nums: u64,
}
impl Default for GetInputData{
    fn default() -> Self {
        Self {
            partition_LBA: 0,
            one_sector_size: 0,
            one_block_sector_num: 0,
            one_cluster_block_num: 0,
            partition_cluseter_num: 0,
            back_up_nums: 0,
        }
    }
}
impl GetInputData{
    fn GetOneBlockBytes(&self) -> u64{
        self.one_sector_size * self.one_block_sector_num
    }

    fn GetOneClusterBytes(&self) -> u64{
        self.GetOneBlockBytes() * self.one_cluster_block_num
    }
}

#[cfg(feature = "jp")]
fn get_setup_data() -> result::Result<GetInputData, Box<dyn std::error::Error>>{
    let mut input_data = String::new();
    let mut partition_LBA: u64 = 0;
    let mut one_sector_size: u64 = 0;
    let mut one_block_sector_num: u64 = 0;
    let mut one_cluster_block_num: u64 = 0;
    let mut partition_cluster_size: u64 = 0;
    let mut back_up_nums: u64 = 0;
    {//パーティションLBAの取得
        println!("パーティションLBAを入力してください");
        io::stdin()
        .read_line(&mut input_data)?;
    }
    partition_LBA = input_data.trim().parse()?;
    input_data.clear();
    while one_sector_size / 512 == 0 && one_sector_size / 4096 == 0 {
        {//セクタサイズの取得
            println!("セクタサイズを入力してください");
            io::stdin()
                .read_line(&mut input_data)?;
        }
        one_sector_size = input_data.trim().parse()?;
        input_data.clear();
    }
    {//ブロックサイズの取得
        println!("1ブロックは何セクタですか？");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    one_block_sector_num = input_data.trim().parse()?;
    input_data.clear();
    {//クラスタサイズの取得
        println!("1クラスタは何ブロックですか？");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    one_cluster_block_num = input_data.trim().parse()?;
    input_data.clear();
    {//パーティションサイズの取得
        println!("パーティションは何クラスタですか？");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    partition_cluster_size = input_data.trim().parse()?;
    input_data.clear();
    let max_backup = partition_cluster_size / 8 / 8 + 1;
    while back_up_nums == 0 && max_backup >= back_up_nums{//バックアップ数を取得
        
        println!("バックアップはいくつですか？ (1 ~ {})", max_backup);
        io::stdin()
            .read_line(&mut input_data)?;
        back_up_nums = input_data.trim().parse()?;
    }

    Ok(GetInputData {
        partition_LBA: partition_LBA,
        one_sector_size: one_sector_size,
        one_block_sector_num: one_block_sector_num,
        one_cluster_block_num: one_cluster_block_num,
        partition_cluseter_num: partition_cluster_size,
        back_up_nums: back_up_nums,
    })
}

#[cfg(not(feature = "jp"))]
fn get_setup_data() -> result::Result<GetInputData, Box<dyn std::error::Error>>{
    let mut input_data = String::new();
    let mut partition_LBA: u64 = 0;
    let mut one_sector_size: u64 = 0;
    let mut one_block_sector_num: u64 = 0;
    let mut one_cluster_block_num: u64 = 0;
    let mut partition_cluster_size: u64 = 0;
    let mut back_up_nums: u64 = 0;
    {//Retrieving Partition LBAs
        println!("Please enter the partition LBA.");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    partition_LBA = input_data.trim().parse()?;
    input_data.clear();
    while one_sector_size / 512 == 0 && one_sector_size / 4096 == 0 {
        {//Getting the Sector Size
            println!("Please enter the sector size.");
            io::stdin()
                .read_line(&mut input_data)?;
        }
        one_sector_size = input_data.trim().parse()?;
        input_data.clear();
    }
    {//Getting the Block Size
        println!("How many sectors are in one block?");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    one_block_sector_num = input_data.trim().parse()?;
    input_data.clear();
    {//Getting the Cluster Size
        println!("How many blocks are in one cluster?");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    one_cluster_block_num = input_data.trim().parse()?;
    input_data.clear();
    {//Getting the Partition Size
        println!("How many clusters are in a partition?");
        io::stdin()
            .read_line(&mut input_data)?;
    }
    partition_cluster_size = input_data.trim().parse()?;
    input_data.clear();
    let max_backup = partition_cluster_size / 8 / 8 + 1;
    while back_up_nums == 0 && max_backup >= back_up_nums{//Get the number of backups
        println!("How many backups are there? (1 ~ {})", max_backup);
        io::stdin()
            .read_line(&mut input_data)?;
        back_up_nums = input_data.trim().parse()?;
    }

    Ok(GetInputData {
        partition_LBA: partition_LBA,
        one_sector_size: one_sector_size,
        one_block_sector_num: one_block_sector_num,
        one_cluster_block_num: one_cluster_block_num,
        partition_cluseter_num: partition_cluster_size,
    })
}

pub fn making_checksum_data(super_block: &FS_core_types::SuperBlockData) -> [u8; 13]{
    let arr = [
        super_block.fs_version.top as u8,
        super_block.fs_version.mid as u8,
        super_block.fs_version.low as u8,
        super_block.partition_LBA as u8,
        super_block.one_sector_size as u8,
        super_block.one_block_sector_num as u8,
        super_block.one_cluster_block_num as u8,
        super_block.partition_cluster_size as u8,
        super_block.bitmap_size as u8,
        super_block.free_ID_bitmap_size as u8,
        super_block.back_up_num as u8,
        super_block.read_algorithm_num as u8,
        super_block.write_algorithm_num as u8,
    ];
    arr
}

//チェックサムの種類が増えたらここに足していく
pub fn check_sum(use_data: &[u8]) -> u64{
    let result_data: u64;
    match super_block_config::WHAT_CHACKSUM {
        FS_core_types::CheckSumTypes::CRC32 => {
            result_data = CRC32(use_data);
        }
    }
    result_data
}

fn setup_super_block(setupdata: GetInputData) -> FS_core_types::SuperBlockData{
    let mut super_block = FS_core_types::SuperBlockData::default();
    let mut now_select_cluster = 1;

    super_block.partition_LBA = setupdata.partition_LBA;
    super_block.one_sector_size = setupdata.one_sector_size;
    super_block.one_block_sector_num = setupdata.one_block_sector_num;
    super_block.one_cluster_block_num = setupdata.one_cluster_block_num;
    super_block.partition_cluster_size = setupdata.partition_cluseter_num;
    super_block.back_up_num = setupdata.back_up_nums;

    super_block.bitmap_address = now_select_cluster;
    super_block.bitmap_size = round_up(round_up(setupdata.partition_cluseter_num, u8::BITS as u64), setupdata.GetOneClusterBytes());
    now_select_cluster += 1;

    super_block.back_up_list_address = now_select_cluster + super_block.bitmap_size - 1;
    now_select_cluster += super_block.bitmap_size;

    super_block.directory_tree_address = now_select_cluster;
    super_block.directory_tree_depth = 0;
    now_select_cluster += 1;

    super_block.name_tree_address = now_select_cluster;
    super_block.name_tree_depth = 0;
    now_select_cluster += 1;

    super_block.free_ID_bitmap_address = now_select_cluster;
    super_block.free_ID_bitmap_size = round_up((setupdata.partition_cluseter_num * (setupdata.GetOneClusterBytes() / mem::size_of::<Entry>() as u64)) / 2 / mem::size_of::<u64>() as u64, setupdata.GetOneClusterBytes());
    now_select_cluster += 1;

    {
        let end_data_size: u64 = (mem::size_of::<u64>() as u64) * 3;
        let beg_data_size: u64 = (mem::size_of::<IPB_HMJN_FS::FS_core_types::SuperBlockData>() as u64 - end_data_size - mem::size_of::<u64>() as u64);
        super_block.reservation_space_size = setupdata.GetOneClusterBytes() - beg_data_size - end_data_size;
    }

    super_block.read_algorithm_num = 0;

    super_block.check_sum = check_sum(&making_checksum_data(&super_block));

    super_block

}

// ホストOS上で使うことを前提としたもの
pub fn setup_super_block_data() -> result::Result<FS_core_types::SuperBlockData, Box<dyn error::Error>>{
    let setupdata = get_setup_data()?;
    let super_block = setup_super_block(setupdata);

    Ok(super_block)
}

#[cfg(feature = "debug")]
pub fn debug_super_block(
    partition_LBA: u64,
    one_sector_size: u64,
    one_block_sector_num: u64,
    one_cluster_block_num: u64,
    partition_cluseter_num: u64,
    back_up_nums: u64,
) -> FS_core_types::SuperBlockData
{
    setup_super_block(
        GetInputData{
            partition_LBA,
            one_sector_size,
            one_block_sector_num,
            one_cluster_block_num,
            partition_cluseter_num,
            back_up_nums
        }
    )
}
