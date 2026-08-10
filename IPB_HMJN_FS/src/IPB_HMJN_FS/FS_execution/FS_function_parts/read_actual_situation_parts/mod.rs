use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData, Entry, FileType, EntryStructType};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::Bitmap;
use crate::IPB_HMJN_FS::general_function;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::name_tree_function::NameTree;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::directory_tree_function::DirectoryTree;

use std::io::{Read, Seek, SeekFrom};
use std::{result::Result, error::Error};
use std::hash::Hash;
use std::fs::File;

fn read_actual_situation(super_block: &SuperBlockData, read_address: u64, read_size: u64, output_file: &mut File)
-> Result<Vec<u8>, Box<dyn Error>>
{
    let mut vec_buffer: Vec<u8> = vec![0u8; (super_block.get_one_cluster_bytes() * read_size) as usize];
    output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * read_address))?;
    output_file.read(&mut vec_buffer)?;
    Ok(vec_buffer)
}

pub fn read_file<Key, Value>(super_block: &mut SuperBlockData, name_tree: &NameTree<Key, Value>, dir_tree: &DirectoryTree<Key, Value>, bitmap: &mut Bitmap, path: &str, output_file: &mut File)
-> Result<Option<Vec<u8>>, Box<dyn Error>>
where
    Key: Default + From<(usize, u64)> + Eq + Hash + Clone,
    Value: Default + Clone + Into<u64> + From<u64>,
{
    let path_analysis = general_function::str_analysis(path, "/");
    let path_nums = path_analysis.len();
    let entry_name = *path_analysis.get(path_nums - 1).unwrap();
    let parent_name = if path_nums >= 2{
        *path_analysis.get(path_nums - 2).unwrap()
    }
    else{
        ""
    };

    if let Some(entry_ID) = name_tree.get_entry(super_block, entry_name.as_bytes(), parent_name.as_bytes(), bitmap, output_file)?{
        // 一旦パスの衝突がないものとして構築をする
        let ID = entry_ID[0];
        let data = dir_tree.get_entry(super_block, ID, bitmap, output_file)?;
        if data.data_type == EntryStructType::File{
            let file_data = FileType::from_bytes(&data.putting);
            
            let mut read_file_data: Vec<u8> = Vec::with_capacity((file_data.size * super_block.get_one_cluster_bytes()) as usize);

            //断片化していた時の処理
            if (file_data.bit_flag & (1 << 0)) > 0{
                let mut buffer: Vec<u8> = vec![0u8; super_block.get_one_cluster_bytes() as usize];
                let mut fragmentation_read_vec: Vec<(u64, u64)> = Vec::with_capacity((super_block.get_one_cluster_bytes() / (8 * 2)) as usize);
                output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * file_data.file_address))?;
                output_file.read_exact(&mut buffer);
                let loop_len = buffer.len() / (8 * 2);
                
                //断片化したアドレスと取得できるサイズ数を取得する
                for i in 0..loop_len{
                    let offset = i*16;
                    let stert_0 = offset;
                    let end_0 = stert_0 + 8;
                    let stert_1 = end_0;
                    let end_1 = stert_1 + 8;

                    fragmentation_read_vec.push((
                        u64::from_le_bytes(buffer[stert_0..end_0].try_into().unwrap()),
                        u64::from_le_bytes(buffer[stert_1..end_1].try_into().unwrap()),
                    ));

                }

                for (address, size) in fragmentation_read_vec{
                    read_file_data.extend(read_actual_situation(&super_block, address, size, output_file)?);
                }

            }
            else{
                read_file_data.extend(read_actual_situation(&super_block, file_data.file_address, file_data.size, output_file)?);
            }
            return Ok(Some(read_file_data));

        }
    }
    
    Ok(None)
}

