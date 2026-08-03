use crate::IPB_HMJN_FS::Constant::{self, NAME_TREE_ADDRESS, NAME_TREE_DEPTH};
use std::convert::TryFrom;

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
    pub bitmap_address: u64,
    pub bitmap_size: u64,
    pub back_up_num: u64,
    pub back_up_list_address: u64,
    pub directory_tree_address: u64,
    pub directory_tree_depth: u64,
    pub name_tree_address: u64,
    pub name_tree_depth: u64,
    pub free_ID_bitmap_address: u64,
    pub free_ID_bitmap_size: u64,
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
            bitmap_address: constant::BITMAP_ADDRESS,
            bitmap_size: constant::BITMAP_SIZE,
            back_up_num: constant::BACK_UP_NUM,
            back_up_list_address: constant::BACK_UP_LIST_ADDRESS,
            directory_tree_address: constant::DIRECTORY_TREE_ADDRESS,
            directory_tree_depth: constant::DIRECTORY_TREE_DEPTH,
            name_tree_address: NAME_TREE_ADDRESS,
            name_tree_depth: NAME_TREE_DEPTH,
            free_ID_bitmap_address: constant::FREE_ID_BITMAP_ADDRESS,
            free_ID_bitmap_size: constant::FREE_ID_BITMAP_SIZE,
            reservation_space_size: constant::RESERVATION_SPACE_SIZE,
            read_algorithm_num: constant::READ_ALGORITHM_NUM,
            write_algorithm_num: constant::WRITE_ALGORITHM_NUM,
            check_sum: constant::CHECK_SUM,
        }
    }
}
impl SuperBlockData{
    pub fn get_one_block_bytes(&self) -> u64{
        self.one_sector_size * self.one_block_sector_num
    }

    pub fn get_one_cluster_bytes(&self) -> u64{
        self.get_one_block_bytes() * self.one_cluster_block_num
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
    directory = 0,
    file = 1,
    symbolic = 2,
    division = 3,
}
impl TryFrom<u8> for EntryStructType{
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::directory),
            2 => Ok(Self::file),
            3 => Ok(Self::symbolic),
            4 => Ok(Self::division),
            _ => Err(())
        }
    }
}
impl Default for EntryStructType{
    fn default() -> Self {
        EntryStructType::directory
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C , packed(1))]
struct EntryPutting{
    putting: [u8; 256-1-8-8-25],
    // putting: [u8; 214],
    // putting: [u8; 211],
}
impl EntryPutting{
    // pub fn from_bytes(buf: &[u8]) -> Self{
    //     Self{
    //         putting: buf,
    //     }
    // }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::new();
        buf.extend(self.putting);
        buf
    }

}

#[derive(Debug, Clone, Copy)]
pub struct DirectoryType{
    pub first_child_ID: u64,
}
impl DirectoryType{
    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            first_child_ID: u64::from_le_bytes(buf[0..8].try_into().unwrap()),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::new();
        buf.extend(self.first_child_ID.to_le_bytes());
        buf
    }

}

#[derive(Debug, Clone, Copy)]
pub struct FileType{
    pub bit_flag: u64,
    pub size: u64,
    pub file_address: u64,
}
impl FileType{
    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            bit_flag: u64::from_le_bytes(buf[0..8].try_into().unwrap()),
            size: u64::from_le_bytes(buf[9..16].try_into().unwrap()),
            file_address: u64::from_le_bytes(buf[17..25].try_into().unwrap()),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::new();
        buf.extend(self.bit_flag.to_le_bytes());
        buf.extend(self.size.to_le_bytes());
        buf.extend(self.file_address.to_le_bytes());
        buf
    }

}

#[derive(Debug, Clone, Copy)]
pub struct SymbolicType{
    pub target_ID: u64,
}
impl SymbolicType{
    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            target_ID: u64::from_le_bytes(buf[0..8].try_into().unwrap()),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::new();
        buf.extend(self.target_ID.to_le_bytes());
        buf
    }
}

//ここは改善予定
#[derive(Debug, Clone, Copy)]
pub struct DivisionType{
    pub bit_flag: u64,
    pub block_address: u64,
    pub sector_address: u64,
}
impl DivisionType{
    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            bit_flag: u64::from_le_bytes(buf[0..8].try_into().unwrap()),
            block_address: u64::from_le_bytes(buf[9..16].try_into().unwrap()),
            sector_address: u64::from_le_bytes(buf[17..25].try_into().unwrap()),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::new();
        buf.extend(self.bit_flag.to_le_bytes());
        buf.extend(self.block_address.to_le_bytes());
        buf.extend(self.sector_address.to_le_bytes());
        buf
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EntryType{
    Pad(EntryPutting),
    Directory(DirectoryType),
    File(FileType),
    SymbolicType(SymbolicType),
    Division(DivisionType),
}
impl EntryType{
    pub fn from_bytes(buf: &[u8], entry_type: EntryStructType) -> Self{
        match entry_type {
            EntryStructType::directory => {
                Self::Directory(DirectoryType::from_bytes(buf))
            }
            EntryStructType::file => {
                Self::File(FileType::from_bytes(buf))
            }
            EntryStructType::symbolic => {
                Self::SymbolicType(SymbolicType::from_bytes(buf))
            }
            EntryStructType::division => {
                Self::Division(DivisionType::from_bytes(buf))
            }
    
    
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        match self{
            EntryType::Pad(data) => {
                return data.to_le_bytes();
            }
            EntryType::Directory(data) => {
                return data.to_le_bytes();
            }
            EntryType::File(data) => {
                return data.to_le_bytes();
            }
            EntryType::SymbolicType(data) => {
                return data.to_le_bytes();
            }
            EntryType::Division(data) => {
                return data.to_le_bytes();
            }
        
        
        
        }
    }

}

#[derive(Debug)]
#[repr(C , packed(1))]
pub struct Entry{
    pub data_type: EntryStructType,
    pub ID: u64,
    pub parent_ID: u64,
    pub next_sibling_ID: u64,
    pub prev_sibling_ID: u64,

    // entry_type: EntryType,
    // pub putting: [u8; 256-1-8-8-25],
    pub putting: [u8; 512-256-1*2-8*6],

    pub mode: u64,
    pub last_updatated_time: u64,
    pub name_size: u8,
    // name: [u8; 255],
    pub name: [u8; 256],
}
impl Entry{
    // pub fn new() -> Self{

    // }

    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            data_type: EntryStructType::try_from(buf[0]).unwrap_or_default(),
            ID: u64::from_le_bytes(buf[1..9].try_into().unwrap()).try_into().unwrap_or_default(),
            parent_ID: u64::from_le_bytes(buf[9..17].try_into().unwrap()).try_into().unwrap_or_default(),
            next_sibling_ID: u64::from_le_bytes(buf[17..25].try_into().unwrap()).try_into().unwrap_or_default(),
            prev_sibling_ID: u64::from_le_bytes(buf[25..33].try_into().unwrap()).try_into().unwrap_or_default(),
            putting: buf[33..239].try_into().unwrap(),
            mode: u64::from_le_bytes(buf[239..247].try_into().unwrap()).try_into().unwrap_or_default(),
            last_updatated_time: u64::from_le_bytes(buf[247..255].try_into().unwrap()).try_into().unwrap_or_default(),
            name_size: buf[256].try_into().unwrap(),
            name: buf[257..512].try_into().unwrap(),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf:Vec<u8> = Vec::new();
        buf.extend([self.data_type as u8]);
        buf.extend(self.ID.to_le_bytes());
        buf.extend(self.parent_ID.to_le_bytes());
        buf.extend(self.next_sibling_ID.to_le_bytes());
        buf.extend(self.prev_sibling_ID.to_le_bytes());
        buf.extend(self.putting);
        buf.extend(self.mode.to_le_bytes());
        buf.extend(self.last_updatated_time.to_le_bytes());
        buf.extend([self.name_size]);
        buf.extend(self.name);
        buf
    }


}


