use super::constant;

//supere block types
#[derive(Debug, Clone, Copy)]
#[repr(C , packed(1))]
pub struct Version{
    pub top: u8,
    pub mid: u8,
    pub low: u8,
}
impl Default for Version{
    fn default() -> Self{
        Self {
            top: constant::VERSION_TOP,
            mid: constant::VERSION_MID,
            low: constant::VERSION_LOW,
        }
    }
}

#[derive(Debug)]
#[repr(C , packed(1))]
pub struct SuperBlockData{
    pub magic_number: [u8;8],
    pub back_up_or_main: u8,
    pub fs_version: Version,
    pub partition_LBA: u64,
    pub one_sector_size: u64,
    pub one_block_sector_num: u64,
    pub one_cluster_block_num: u64,
    pub partition_cluster_size: u64,
    pub bitmap_cluster_num: u64,
    pub bitmap_size: u64,
    pub back_up_num: u64,
    pub back_up_list_cluster_num: u64,
    pub directory_tree_cluster_num: u64,
    pub directory_tree_depth: u64,
    pub free_ID_tree_cluster_num: u64,
    pub free_ID_tree_depth: u64,
    pub reservation_space_size: u64,
    pub read_algorithm_num: u64,
    pub write_algorithm_num: u64,
    pub check_sum: u64,
}
impl Default for SuperBlockData{
    fn default() -> Self {
        Self{
            magic_number: constant::MAGIC_NUMBER,
            back_up_or_main: constant::BACK_UP_OR_MAIN,
            fs_version: Version::default(),
            partition_LBA: constant::PARTIITON_LBA,
            one_sector_size: constant::ONE_SECTOR_SIZE,
            one_block_sector_num: constant::ONE_BLOCK_SECTOR_NUM,
            one_cluster_block_num: constant::ONE_CLUSTER_BLOCK_NUM,
            partition_cluster_size: constant::PARTITION_CLUSTER_SIZE,
            bitmap_cluster_num: constant::BITMAP_CLUSTER_NUM,
            bitmap_size: constant::BITMAP_SIZE,
            back_up_num: constant::BACK_UP_NUM,
            back_up_list_cluster_num: constant::BACK_UP_LIST_CLUSTER_NUM,
            directory_tree_cluster_num: constant::DIRECTORY_TREE_CLUSTER_NUM,
            directory_tree_depth: constant::DIRECTORY_TREE_DEPTH,
            free_ID_tree_cluster_num: constant::FREE_ID_TREE_CLUSTER_NUM,
            free_ID_tree_depth: constant::FREE_ID_TREE_DEPTH,
            reservation_space_size: constant::RESERVATION_SPACE_SIZE,
            read_algorithm_num: constant::READ_ALGORITHM_NUM,
            write_algorithm_num: constant::WRITE_ALGORITHM_NUM,
            check_sum: constant::CHECK_SUM,
        }
    }
}
impl SuperBlockData{
    pub fn GetOneBlockBytes(&self) -> u64{
        self.one_sector_size * self.one_block_sector_num
    }

    pub fn GetOneClusterBytes(&self) -> u64{
        self.GetOneBlockBytes() * self.one_cluster_block_num
    }

}

#[repr(u64)]
pub enum CheckSumTypes {
    CRC32 = 1,
}

//entry type
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EntryStructType{
    directory = 1,
    file = 2,
    symbolic = 3,
    division = 4,
}

#[derive(Debug, Clone, Copy)]
struct EntryPutting{
    putting: [u8; 256-1-8-8-25],
}

#[derive(Debug, Clone, Copy)]
struct DirectoryType{
    first_child_ID: u64,
}

#[derive(Debug, Clone, Copy)]
struct FileType{
    bit_flag: u64,
    size: u64,
    file_address: u64,
}

#[derive(Debug, Clone, Copy)]
struct SymbolicType{
    target_ID: u64,
}

//ここは改善予定
#[derive(Debug, Clone, Copy)]
struct DivisionType{
    bit_flag: u64,
    block_address: u64,
    sector_address: u64,
}

#[derive(Debug, Clone, Copy)]
enum EntryType{
    Pad(EntryPutting),
    Directory(DirectoryType),
    File(FileType),
    SymbolicType(SymbolicType),
    Division(DivisionType),
}

#[derive(Debug)]
#[repr(C , packed(1))]
pub struct Entry{
    data_type: EntryStructType,
    ID: u64,
    parent_ID: u64,
    next_sibling_ID: u64,
    prev_sibling_ID: u64,
    entry_type: EntryType,
    mode: u64,
    last_updatated_time: u64,
    name_size: u8,
    name: [u8; 255],
}





