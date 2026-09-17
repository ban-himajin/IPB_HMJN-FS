use crate::IPB_HMJN_FS::Constant::NAME_TREE_DEPTH;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_function::Bitmap;
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::bitmap_parts::{FreeBitmapData, FreeBitmap, FreeBits};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::cash_parts::{LfuCache};
use crate::IPB_HMJN_FS::FS_execution::FS_function_parts::free_ID_bitmap_function::FreeIDBitmap;
use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData};
use crate::IPB_HMJN_FS::FS_setup::check_sum;
use crate::IPB_HMJN_FS::general_function;

use core::error;
use std::cell::OnceCell;
use std::io::{Read, Write};
use std::{fs, option::Option, result::Result, error::Error, mem, hash::Hash, clone::Clone, io::{Seek, SeekFrom}, cell::RefCell};

/*
キャッシュを効率よく使うためにすべての範囲においてキャッシュが存在している状態を作る必要がある
そのためのアルゴリズムも考える必要がある
*/

pub struct ScanTreeResult{
    // pub leaf_prev_address: u64,
    pub leaf_address: u64,
    pub root_address: u64,
    pub tree_depth: u64,
    pub leaf_num: u64,
}
// impl ScanTreeResult{
//     pub fn to_le_bytes(self) -> Vec<u8>{
//         let mut buf: Vec<u8> = Vec::new();
//         buf.extend(self.leaf_address.to_le_bytes());
//         buf.extend(self.root_address.to_le_bytes());
//         buf.extend(self.tree_depth.to_le_bytes());
//         buf.extend(self.leaf_num.to_le_bytes());
//         buf
//     }
// 
//     pub fn from_bytes(buf: &[u8]) -> Self{
//         Self{
//             leaf_address: u64::from_le_bytes(buf[0..8].try_into().unwrap()).try_into().unwrap_or_default(),
//             root_address: u64::from_le_bytes(buf[8..16].try_into().unwrap()).try_into().unwrap_or_default(),
//             tree_depth: u64::from_le_bytes(buf[16..24].try_into().unwrap()).try_into().unwrap_or_default(),
//             leaf_num: u64::from_le_bytes(buf[24..32].try_into().unwrap()).try_into().unwrap_or_default(),
//         }
//     }
// }


type Cache<Key, Value> = LfuCache<Key, Value>;
// type1
pub struct TreeData<Key, Value>{
    pub cache: Option<RefCell<Cache<Key, Value>>>,
}
// type2
// pub struct TreeData<Key, Value, Type>{
//     pub tree_data: Type,
//     pub cache: Option<Cache<Key, Value>>,
// }

// impl<Key: Default, Value: Default> TreeData<Key, Value>{
impl<Key: Default + From<(usize, u64)> + Eq + Hash + Clone,
    Value: Default + Clone + Into<u64> + From<u64>> TreeData<Key, Value>{
    /*
    フリーIDビットマップから空き位置を取り出す処理をどこかに入れる
    →フリーIDビットマップはすでにあるため型を使い取り出す方が効率的かも？
    →いっそのことファイル作成時にフリーIDを得ることにしてツリーに機能を持たせないのもありか？
     */

    pub fn new(capacity: usize) -> Self{
        if capacity > 0{
            Self{
                cache: Some(RefCell::new(Cache::new(capacity))),
            }
        }
        else{
            Self {
                cache: None,
            }
        }
    }

    //新しい親を確保する
    pub fn new_parent(super_block: &SuperBlockData, old_parent_address: u64, bitmap: &mut Bitmap, output_file: &mut fs::File) -> Result<Option<u64>, Box<dyn error::Error>>{
        let get_bit = bitmap.get_free_bit(1);
        match get_bit{
            FreeBits::defragmentation(value) => {
                let new_parent_address = value.index * 8 + value.offset as u64;
                output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * new_parent_address))?;
                output_file.write_all(&old_parent_address.to_le_bytes())?;
                bitmap.write_bit_flag(get_bit)?;
                Ok(Some(new_parent_address))
            }
            _ =>{
                Ok(None)
            }
        }
    }

    //特定の親に対し子を追加する
    pub fn new_child(super_block: &SuperBlockData, parent_address: u64, child_offset: u64, bitmap: &mut Bitmap, output_file: &mut fs::File) -> Result<Option<u64>, Box<dyn error::Error>>{
        let get_bit = bitmap.get_free_bit(1);
        match get_bit{
            FreeBits::defragmentation(value) => {
                let new_child_address = value.index * u8::BITS as u64 + value.offset as u64;
                
                output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * parent_address + child_offset * mem::size_of::<u64>() as u64))?;

                output_file.write_all(&new_child_address.to_le_bytes())?;
                bitmap.write_bit_flag(get_bit)?;
                Ok(Some(new_child_address))
            }
            _ =>{
                Ok(None)
            }
        }
    }

    // ツリーの葉一歩手前まで行く
    // 作れそうであれば葉内のエントリまで引く
    // ツリーが枯渇していた場合ツリー更新を挟む処理がないためツリー追加関数ができ次第改良
    pub fn scan_tree(&self, super_block: &SuperBlockData, search_ID: u64, tree_address: u64, tree_depth: u64, one_entry_size: u64, bitmap: &mut Bitmap, output_file: &mut fs::File)
    -> Result<ScanTreeResult, Box<dyn error::Error>>
    {
        let mut result_datas = ScanTreeResult{
            leaf_address: 0,
            root_address: tree_address,
            tree_depth: tree_depth,
            leaf_num: 0,
        };
        let mut search_data: Vec<u64> = Vec::with_capacity(tree_depth as usize);
        let mut cache_key: Vec<u64> = Vec::with_capacity(tree_depth as usize);
        let mut get_cache = None;
        let mut cache_index = 0;
        let mut get_next_address = tree_address;
        let one_cluster_have_childs = super_block.get_one_cluster_bytes() / mem::size_of::<u64>() as u64;
        let one_leaf_entrys = super_block.get_one_cluster_bytes() / one_entry_size;
        let mut tree_max_address = one_leaf_entrys * general_function::binary_pow(one_cluster_have_childs, tree_depth);
        let tree_dep = super_block.directory_tree_depth;
        // let mut tree_max_address = tree_max_address;
        {//親が不足してないか確かめる
            while tree_max_address <= search_ID {
                
                let result = TreeData::<Key, Value>::new_parent(super_block, result_datas.root_address, bitmap, output_file)?;
                if let Some(new_root_address) = result{
                    result_datas.root_address = new_root_address;
                    result_datas.tree_depth += 1;
                    tree_max_address *= one_cluster_have_childs;
                    get_next_address = new_root_address;

                }
            }
        }
        {//たどるべき子の数を調べる
            let mut search_child_num = 0;
            let mut next_child_num = search_ID;
            let mut child_pow = tree_max_address;
            for now_tree_depth in 0..=result_datas.tree_depth{
                search_child_num = next_child_num / child_pow;
                next_child_num %= child_pow;
                child_pow /= one_cluster_have_childs;

                cache_key.push(child_pow * search_child_num);
                search_data.push(search_child_num);
                if next_child_num == 0{
                    for _ in now_tree_depth + 1..result_datas.tree_depth{
                        cache_key.push(0);
                        search_data.push(0);
                    }
                    break;
                }
            }
            result_datas.leaf_num = search_child_num;
        }
        {//cashを調べseekする
            if let Some(cache) = &self.cache{
                let key_len = cache_key.len();
                for (index, cache_range) in cache_key.iter().enumerate(){
                    get_cache = cache.borrow_mut().get_cache((key_len - index, *cache_range).into());
                    if let Some(value) = get_cache.clone(){
                        cache_index = index;
                        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * value.into() + mem::size_of::<u64>() as u64 * search_data.get(index).unwrap()))?;
                        break;
                    }
                }
                if get_cache.is_none(){
                    //シーク位置が間違っている
                    //詳細なオフセット位置を足していない
                    output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_datas.root_address + result_datas.leaf_num * mem::size_of::<u64>() as u64))?;
                }
            }
            else{
                //シーク位置が間違っている
                output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * result_datas.root_address + search_data.get(0).unwrap() * mem::size_of::<u64>() as u64))?;
            }
        }
        {//ツリーを探索する
            let search_len = search_data.len();
            for index in cache_index..search_len{
                if search_len / 2 == index{
                    if let Some(cache) = &self.cache{
                        cache.borrow_mut().put((search_len - index, *cache_key.get(index).unwrap()).into(), get_next_address.into());
                    }
                }
                let parent_address = get_next_address;
                let mut buffer = [0u8; 8];
                let value = super_block.get_one_cluster_bytes();
                let seek_data = super_block.get_one_cluster_bytes() * get_next_address + search_data.get(index).unwrap() * mem::size_of::<u64>() as u64;
                
                if index != search_len{
                    output_file.seek(SeekFrom::Start(seek_data))?;
                }
                output_file.read_exact(&mut buffer)?;
                get_next_address = u64::from_le_bytes(buffer);
                if get_next_address == 0{
                    let new_child_address = TreeData::<Key, Value>::new_child(super_block, parent_address, *search_data.get(index).unwrap(), bitmap, output_file)?;
                    if let Some(child_address) = new_child_address{
                        get_next_address = child_address;
                    }
                }
            }
        }
        result_datas.leaf_address = get_next_address;
        Ok(result_datas)
    }


}


