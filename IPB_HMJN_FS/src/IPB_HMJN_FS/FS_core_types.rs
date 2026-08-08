use crate::IPB_HMJN_FS::Constant::{self, NAME_TREE_ADDRESS, NAME_TREE_DEPTH};
use crate::IPB_HMJN_FS::checksum_functions;
use crate::IPB_HMJN_FS::general_function;

use std::convert::TryFrom;
use std::io::{Seek,SeekFrom, Write, Read};
use std::fs;
use std::result;
use std::error;
use std::time::{SystemTime, UNIX_EPOCH};

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
            top: Constant::VERSION_TOP,
            mid: Constant::VERSION_MID,
            low: Constant::VERSION_LOW,
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
            magic_number: Constant::MAGIC_NUMBER,
            back_up_or_main: Constant::BACK_UP_OR_MAIN,
            fs_version: Version::default(),
            partition_LBA: Constant::PARTIITON_LBA,
            one_sector_size: Constant::ONE_SECTOR_SIZE,
            one_block_sector_num: Constant::ONE_BLOCK_SECTOR_NUM,
            one_cluster_block_num: Constant::ONE_CLUSTER_BLOCK_NUM,
            partition_cluster_size: Constant::PARTITION_CLUSTER_SIZE,
            bitmap_address: Constant::BITMAP_ADDRESS,
            bitmap_size: Constant::BITMAP_SIZE,
            back_up_num: Constant::BACK_UP_NUM,
            back_up_list_address: Constant::BACK_UP_LIST_ADDRESS,
            directory_tree_address: Constant::DIRECTORY_TREE_ADDRESS,
            directory_tree_depth: Constant::DIRECTORY_TREE_DEPTH,
            name_tree_address: NAME_TREE_ADDRESS,
            name_tree_depth: NAME_TREE_DEPTH,
            free_ID_bitmap_address: Constant::FREE_ID_BITMAP_ADDRESS,
            free_ID_bitmap_size: Constant::FREE_ID_BITMAP_SIZE,
            reservation_space_size: Constant::RESERVATION_SPACE_SIZE,
            read_algorithm_num: Constant::READ_ALGORITHM_NUM,
            write_algorithm_num: Constant::WRITE_ALGORITHM_NUM,
            check_sum: Constant::CHECK_SUM,
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

    pub fn get_partition_LBA_byte(&self) -> u64{
        self.partition_LBA * self.one_sector_size
    }

    pub fn write_super_block_data(&self, output_file: &mut fs::File) -> result::Result<(), Box<dyn error::Error>>{
        output_file.write_all(&self.magic_number)?;
        output_file.write_all(&self.back_up_or_main.to_le_bytes())?;
        output_file.write_all(&self.fs_version.top.to_le_bytes())?;
        output_file.write_all(&self.fs_version.mid.to_le_bytes())?;
        output_file.write_all(&self.fs_version.low.to_le_bytes())?;
        output_file.write_all(&self.partition_LBA.to_le_bytes())?;
        output_file.write_all(&self.one_sector_size.to_le_bytes())?;
        output_file.write_all(&self.one_block_sector_num.to_le_bytes())?;
        output_file.write_all(&self.one_cluster_block_num.to_le_bytes())?;
        output_file.write_all(&self.partition_cluster_size.to_le_bytes())?;
        output_file.write_all(&self.bitmap_address.to_le_bytes())?;
        output_file.write_all(&self.bitmap_size.to_le_bytes())?;
        output_file.write_all(&self.back_up_num.to_le_bytes())?;
        output_file.write_all(&self.back_up_list_address.to_le_bytes())?;
        output_file.write_all(&self.directory_tree_address.to_le_bytes())?;
        output_file.write_all(&self.directory_tree_depth.to_le_bytes())?;
        output_file.write_all(&self.name_tree_address.to_le_bytes())?;
        output_file.write_all(&self.name_tree_depth.to_le_bytes())?;
        output_file.write_all(&self.free_ID_bitmap_address.to_le_bytes())?;
        output_file.write_all(&self.free_ID_bitmap_size.to_le_bytes())?;
        let now_stream_point = output_file.stream_position()?;
        output_file.seek(SeekFrom::Start(now_stream_point + self.reservation_space_size))?;
        output_file.write_all(&self.read_algorithm_num.to_le_bytes())?;
        output_file.write_all(&self.write_algorithm_num.to_le_bytes())?;
        output_file.write_all(&self.check_sum.to_le_bytes())?;

    Ok(())
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
    pub fn new(first_child_ID: u64) -> Self{
        Self {
            first_child_ID: first_child_ID
        }
    }

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
    pub fn new(flag: u64, file_size: u64, file_address: u64) -> Self{
        Self{
            bit_flag: flag,
            size: file_size,
            file_address: file_address,
        }
    }

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
    pub fn new(target_ID: u64) -> Self{
        Self { target_ID }
    }

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
    pub fn new(flag: u64, block_address: u64, sector_address: u64) -> Self{
        Self { bit_flag: flag, block_address, sector_address }
    }

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
    // pub data_type: EntryStructType,
    // pub ID: u64,
    // pub parent_ID: u64,
    // pub next_sibling_ID: u64,
    // pub prev_sibling_ID: u64,

    // // entry_type: EntryType,
    // // pub putting: [u8; 256-1-8-8-25],
    // pub putting: [u8; 512-256-1*2-8*6],

    // pub mode: u64,
    // pub last_updatated_time: u64,
    // pub name_size: u8,
    // // name: [u8; 255],
    // pub name: [u8; 256],
    pub data_type: EntryStructType,
    pub ID: u64,
    pub parent_ID: u64,
    pub parent_address: u64,
    pub next_sibling_ID: u64,
    pub next_sibling_address: u64,
    pub prev_sibling_ID: u64,
    pub prev_sibling_address: u64,

    // entry_type: EntryType,
    // pub putting: [u8; 256-1-8-8-25],
    pub putting: [u8; 512-256-1*2-8*9],

    pub mode: u64,
    pub last_updatated_time: u64,
    pub name_size: u8,
    // name: [u8; 255],
    pub name: [u8; 256],
}
impl Entry{
    pub fn new(data_type: EntryStructType, ID: u64, parent_ID: u64, parent_address: u64, next_sibling_ID: u64, next_sibling_adderss: u64, prev_sibling_ID: u64, prev_sibling_address: u64, entry_type: EntryType, mode: u64, name: &str) -> Self{
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self{
            data_type: data_type,
            ID: ID,
            parent_ID: parent_ID,
            parent_address: parent_address,
            next_sibling_ID: next_sibling_ID,
            next_sibling_address: next_sibling_adderss,
            prev_sibling_ID: prev_sibling_ID,
            prev_sibling_address: prev_sibling_address,
            putting: general_function::to_fixed_array(&entry_type.to_le_bytes().as_slice()),
            mode: mode,
            last_updatated_time: timestamp,
            name_size: name.len() as u8,
            name: general_function::to_fixed_array(name.as_bytes()),

        }
    }

    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            data_type: EntryStructType::try_from(buf[0]).unwrap_or_default(),
            ID: u64::from_le_bytes(buf[1..9].try_into().unwrap()).try_into().unwrap_or_default(),
            parent_ID: u64::from_le_bytes(buf[9..17].try_into().unwrap()).try_into().unwrap_or_default(),
            parent_address: u64::from_le_bytes(buf[17..25].try_into().unwrap()).try_into().unwrap_or_default(),
            next_sibling_ID: u64::from_le_bytes(buf[25..33].try_into().unwrap()).try_into().unwrap_or_default(),
            next_sibling_address: u64::from_le_bytes(buf[33..41].try_into().unwrap()).try_into().unwrap_or_default(),
            prev_sibling_ID: u64::from_le_bytes(buf[41..49].try_into().unwrap()).try_into().unwrap_or_default(),
            prev_sibling_address: u64::from_le_bytes(buf[49..57].try_into().unwrap()).try_into().unwrap_or_default(),
            putting: buf[57..239].try_into().unwrap(),
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
        buf.extend(self.parent_address.to_le_bytes());
        buf.extend(self.next_sibling_ID.to_le_bytes());
        buf.extend(self.next_sibling_address.to_le_bytes());
        buf.extend(self.prev_sibling_ID.to_le_bytes());
        buf.extend(self.prev_sibling_address.to_le_bytes());
        buf.extend(self.putting);
        buf.extend(self.mode.to_le_bytes());
        buf.extend(self.last_updatated_time.to_le_bytes());
        buf.extend([self.name_size]);
        buf.extend(self.name);
        buf
    }


}

#[derive(Debug)]
pub struct NameTreeEntry{
    pub parent: u32,
    pub entry: u32,
    pub entry_ID: u64,
}
impl NameTreeEntry{

    pub fn new(parent_name: &[u8], entry_name: &[u8], entry_ID: u64) -> Self{
        Self{
            parent: checksum_functions::CRC32(parent_name) as u32,
            entry: checksum_functions::CRC32(entry_name) as u32,
            entry_ID: entry_ID,
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Self{
        Self{
            parent: u32::from_le_bytes(buf[0..4].try_into().unwrap()).try_into().unwrap_or_default(),
            entry: u32::from_le_bytes(buf[4..8].try_into().unwrap()).try_into().unwrap_or_default(),
            entry_ID: u64::from_le_bytes(buf[8..16].try_into().unwrap()).try_into().unwrap_or_default(),
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8>{
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(self.parent.to_le_bytes());
        buf.extend(self.entry.to_le_bytes());
        buf.extend(self.entry_ID.to_le_bytes());
        buf
    }

}
