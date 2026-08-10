use crate::IPB_HMJN_FS::FS_core_types::{Entry, EntryStructType, SuperBlockData, FileType, Putting};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_parts::FreeBits::fragmentation;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_parts::{FreeBits, FreeBitmap, CastAddressData, ResultFreeBitData};
use crate::API::{StreamWrite};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::{Bitmap};

use std::cell::OnceCell;
use std::mem::offset_of;
use std::{result, u64};
use std::{result::Result, option::Option, error::Error, cell::RefCell, fs::{File}, io::{Write, Read, Seek, SeekFrom}, mem};
use std::fs;

struct WriteStream<'a>{
    data: &'a [u8],
    now_pointer: RefCell<usize>,
}
impl<'a> WriteStream<'a>  {
    fn new(data: &'a [u8]) -> Self{
        Self {
            data,
            now_pointer: RefCell::new(0)
        }
    }

    fn get_line(&self, select_data_size: usize) -> &[u8]{
        let pointer = *self.now_pointer.borrow();
        let end_pointer = {
            if pointer + select_data_size > self.data.len(){
                self.data.len()
            }
            else{
                pointer + select_data_size
            }
        };
        *self.now_pointer.borrow_mut() = end_pointer;
        &self.data[pointer..end_pointer]
    }
}

fn cast_u8_vec(u64_vec: Vec<u64>) -> Vec<u8>{
    let vec_size = u64_vec.len() * mem::size_of::<u64>();
    let mut u8_vec: Vec<u8> = Vec::with_capacity(vec_size);
    for counter in u64_vec.iter(){
        let u8_arr = counter.to_le_bytes();
        for u8_value in u8_arr{
            u8_vec.push(u8_value);
        }
    };
    u8_vec
}

fn create_fragmentation_list(super_block: &SuperBlockData, bitmap: &Bitmap, data_vec: Vec<(u64, u64)>, output_file: &mut File)
-> Result<Option<(u64, u64)>, Box<dyn Error>>
{
    let one_cluster_byte = super_block.get_one_cluster_bytes();
    let get_free_size = ((data_vec.len() * mem::size_of::<u64>()) as u64 + one_cluster_byte - 1) / one_cluster_byte;
    let free_bit = bitmap.get_free_bit(get_free_size);
    // let free_bit = bitmap.get_free_bit(1);
    let free_address = if let FreeBits::defragmentation(def) = free_bit{
        def.cast_address_data()
    }
    else{
        return Ok(None);
    };
    let vec_u64_address = data_vec.iter().map(|&(x, _)| x).collect();
    let vec_u64_get_size = data_vec.iter().map(|&(_, x)| x).collect();
    let write_byte_data: Vec<u8>= cast_u8_vec(vec_u64_address)
        .into_iter().zip(cast_u8_vec(vec_u64_get_size))
        .flat_map(|(a, b)| [a, b]).collect();
    output_file.seek(SeekFrom::Start(one_cluster_byte * free_address))?;
    output_file.write_all(&write_byte_data)?;
    bitmap.write_bit_flag(free_bit);

    Ok(Some((free_address, get_free_size)))
}

pub fn write_actual_situation(super_block: &SuperBlockData, bitmap: &Bitmap, data: &[u8], output_file: &mut File)
-> Result<Option<(bool, u64, u64)>, Box<dyn Error>>
{
    let data_size = data.len();
    // let use_size = (super_block.get_one_cluster_bytes() as usize + data_size - 1) / data_size;
    let one_cluster_bytes = super_block.get_one_cluster_bytes() as usize;
    let use_size = (data_size + one_cluster_bytes - 1) / one_cluster_bytes;
    println!("data_size : {:?}", data_size);
    println!("use_size : {:?}", use_size);
    let get_free_bit = bitmap.get_free_bit(use_size as u64);
    let write_bit_data = match get_free_bit{
        FreeBits::defragmentation(defrag) => {
            let mut result: Vec<ResultFreeBitData> = Vec::new();
            result.push(defrag);
            result
        },
        FreeBits::fragmentation(ref frag) => {
            frag.to_vec()
        },
        FreeBits::error(e) => {
            eprintln!("{e}");
            return Ok(None);
        },
        _ => {
            return Ok(None);
        }
    };
    let write_address = {
        let some_data_len = write_bit_data.len();
        let mut result_vec: Vec<(u64, u64)> = Vec::with_capacity(some_data_len);
        for data in write_bit_data.iter(){
            result_vec.push((data.cast_address_data(), data.get_bit_num));
        }
        result_vec
    };

    let mut fragmentation_flag = false;
    let stream = WriteStream::new(data);
    let mut result_address = 0;
    // let mut address_size = 0;

    {//データを書き込む
        let loop_num = write_bit_data.len();
        for ((address, _), data) in write_address.iter().zip(write_bit_data.iter()){
            output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * address))?;
            let write_data = stream.get_line((super_block.get_one_cluster_bytes() * data.get_bit_num) as usize);
            output_file.write_all(write_data)?;
        }
    }
    {//アドレス取得
        //断片化していなかった場合の処理
        match get_free_bit {
            FreeBits::defragmentation(_) =>{
                result_address = write_address[0].0;
                // address_size = 0;
            },
            FreeBits::fragmentation(_) =>{
                if let Some((result_get_address, result_use_size)) = create_fragmentation_list(super_block, bitmap, write_address, output_file)?{
                    result_address = result_get_address;
                    // address_size = result_use_size;
                    fragmentation_flag = true;
                }
                else{
                    return Ok(None);
                }
            },
            FreeBits::error(e) => {
                eprintln!("{e}");
                return Ok(None);
            },
        }
    }
    bitmap.write_bit_flag(get_free_bit)?;
    Ok(Some((fragmentation_flag, result_address, use_size as u64)))
}

pub fn connect_data(super_block: &SuperBlockData, entry_data: (u64, u64), write_data: (bool, u64, u64), output_file: &mut File)
-> Result<(), Box<dyn Error>>
{
    let entry_leaf_address = entry_data.0;
    let entry_ID = entry_data.1;

    let write_fragmentation_flag = write_data.0;
    let write_result_address = write_data.1;
    let write_file_size = write_data.2;

    let mut get_entry_buf = [0u8; mem::size_of::<Entry>()];
    let one_cluster_bytes = super_block.get_one_cluster_bytes();
    let one_cluster_have_entrys = one_cluster_bytes / mem::size_of::<Entry>() as u64;
    let offsett = entry_ID % one_cluster_have_entrys;

    // let seek_size = one_cluster_have_entrys * entry_leaf_address + offsett * mem::size_of::<Entry>() as u64;
    // println!("{:?} = {:?} * {:?} + {:?} * {:?}", seek_size, one_cluster_have_entrys, entry_leaf_address, offsett, mem::size_of::<u64>() as u64);
    let seek_size = one_cluster_bytes * entry_leaf_address + offsett * mem::size_of::<Entry>() as u64;
    println!("{:?} = {:?} * {:?} + {:?} * {:?}", seek_size, one_cluster_bytes, entry_leaf_address, offsett, mem::size_of::<Entry>() as u64);

    println!("seek_size : {:?}", seek_size);
    output_file.seek(SeekFrom::Start(seek_size))?;
    output_file.read_exact(&mut get_entry_buf)?;

    let mut entry_data = Entry::from_bytes(&get_entry_buf);
    let test = entry_data.data_type;
    println!("get_entry_buf : {:?}", get_entry_buf);
    println!("write entry data : {:?}", entry_data);
    println!("testlog 10 : {:?}", test);
    println!("testlog 11 : {:?}", EntryStructType::File);

    // panic!("debug panic");

    if entry_data.data_type == EntryStructType::File{
        let mut file_type_data = FileType::from_bytes(&entry_data.putting);
        if write_fragmentation_flag{
            file_type_data.bit_flag |= 1 << 0;
        }
        file_type_data.size = write_file_size;
        file_type_data.file_address = write_result_address;

        let le_byte_data = file_type_data.to_le_bytes();
        let len = le_byte_data.len();
        let mut array = [0u8; mem::size_of::<Putting>()];
        array[..len].copy_from_slice(&le_byte_data[..len]);
        entry_data.putting = array;
        output_file.seek(SeekFrom::Start(seek_size))?;
        output_file.write(&entry_data.to_le_bytes())?;
    }

    Ok(())
}

#[cfg(feature = "debug")]
pub fn check_function(super_block: &SuperBlockData, bitmap: &Bitmap, file_path: &str, output_file: &mut File)
-> Result<Option<(bool, u64, u64)>, Box<dyn Error>>
{
    let file_byte_datas: Vec<u8> = fs::read(file_path)?;
    println!("---------------log----------------\n{:?}", file_byte_datas);
    let result = write_actual_situation(super_block, bitmap, &file_byte_datas, output_file)?;

    Ok(result)
}
