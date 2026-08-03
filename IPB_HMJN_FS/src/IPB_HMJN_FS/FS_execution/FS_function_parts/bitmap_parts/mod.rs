use std::{cell::RefCell, collections::VecDeque, error, fs, io::{Read, Seek, SeekFrom, Write}, mem, result};
use crate::IPB_HMJN_FS::{FS_core_types, FS_execution::FS_function_parts::bitmap_parts::FreeBits::defragmentation};
//bitmapから空きを探し空いている位置をデータとして持つための構造体
#[derive(Debug, Clone, Copy)]
pub struct ResultFreeBitData{
    pub get_bit_num: u64,
    pub index: u64,
    pub offset: u8,
}
impl Default for ResultFreeBitData{
    fn default() -> Self {
        ResultFreeBitData {
            get_bit_num: 0,
            index: 0,
            offset: 0 ,
        }
    }
}
impl ResultFreeBitData{
    fn cast_address_data(&self) -> u64{
        self.index * u8::BITS as u64 + self.offset as u64
    }
}

//キャストアドレスで得られるアドレスの戻り値
#[derive(Debug)]
pub enum CastAddressData{
    defrag(u64),
    frag(Vec<u64>),
    error(u8),
}

#[derive(Debug, Clone)]
pub enum FreeBits{
    defragmentation(ResultFreeBitData),
    fragmentation(Vec<ResultFreeBitData>),
    error(u8),
}
impl FreeBits{
    pub fn cast_address(&self) -> CastAddressData{
        let cast_data: CastAddressData;
        match self {
            Self::defragmentation(data) =>{
                cast_data = CastAddressData::defrag(data.cast_address_data())
            }

            Self::fragmentation(data) => {
                let mut vec_data: Vec<u64> = vec![0; data.len()];
                for index in 0..data.len(){
                    if let Some(s) = data.get(index){
                        vec_data.get_mut(index).map(|x| *x = s.cast_address_data());
                    }
                }
                cast_data = CastAddressData::frag(vec_data)
            }
            
            Self::error(error) => {
                cast_data = CastAddressData::error(*error);
            }
        }
        cast_data
    }
}

//bitmapの空きを高速に探すために使うvecに入れる構造体
#[derive(Debug, Clone, Copy)]
pub struct FreeBitmapData{
    bitmap_index: u64,
    free_bits: u32,
}
impl Default for FreeBitmapData{
    fn default() -> Self {
        FreeBitmapData {
            bitmap_index:0,
            free_bits: 0,
        }
    }
}

#[derive(Debug)]
pub struct FreeBitmap{
    pub bitmap: RefCell<Vec<u8>>,
    pub free_bitmap_datas: RefCell<Vec<FreeBitmapData>>
}
impl FreeBitmap{
    //空いているbitmapの配列を作る
    pub fn set_free_bitmap(&self, super_block: &FS_core_types::SuperBlockData){
        for i in 0..self.bitmap.borrow().len(){
            let mut free_bitmap_datas: FreeBitmapData = FreeBitmapData::default();
            if let Some(free) = self.free_bitmap_datas.borrow_mut().get_mut(i){
                if let Some(bit) = self.bitmap.borrow().get(i){
                    free_bitmap_datas.bitmap_index = i as u64;
                    if  i == self.bitmap.borrow().len() - 1{
                        free_bitmap_datas.free_bits = (u8::BITS as u64 - (super_block.partition_cluster_size % u8::BITS  as u64) - bit.count_zeros() as u64) as u32 - bit.count_ones();
                    }
                    else{
                        free_bitmap_datas.free_bits = bit.count_zeros();
                    }

                    *free = free_bitmap_datas;
                }
            }
        }
    }

    //実際のディスク内のbitmapを取得をする
    pub fn get_bitmap(&self, super_block: &FS_core_types::SuperBlockData, select_address: u64, output_file: &mut fs::File) -> result::Result<(), Box<dyn  error::Error>>{
        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * select_address))?;
        output_file.read_exact(&mut self.bitmap.borrow_mut()[..])?;
        Ok(())
    }

    //実際のディスク内のbitmapを更新をする
    //現在はすべてのbitmapを同時更新をしている
    //修正予定★
    //最終的に特定のbit範囲だけを書き換えをする形にしたい
    pub fn reload_bitmap(&self, super_block: &FS_core_types::SuperBlockData, select_address: u64, output_file: &mut fs::File) -> result::Result<(), Box<dyn  error::Error>>{
        output_file.seek(SeekFrom::Start(super_block.get_one_cluster_bytes() * select_address))?;
        output_file.write_all(&self.bitmap.borrow())?;
        Ok(())
    }

    //bitmapに特定のフラグを立てる
    fn write(&self, data: ResultFreeBitData){
        let loop_num = (data.get_bit_num + data.offset as u64 + (u8::BITS as u64 - 1)) / u8::BITS as u64;
        let mut write_bits: u64 = data.get_bit_num;
        for index in 0..loop_num {
            if let (Some(bit), Some(freedata)) = (self.bitmap.borrow_mut().get_mut((data.index + index) as usize), self.free_bitmap_datas.borrow_mut().get_mut((data.index + index) as usize)){
                let mut shift_bit = 0;
                shift_bit = bit.trailing_ones();
                let zero_bits = u8::BITS - shift_bit;
                if zero_bits as u64 > write_bits {
                    let write_flag = (1 << write_bits) - 1;
                    *bit |= (write_flag as u8) << shift_bit;
                    break;
                }
                else{
                    let write_flag = (1 << zero_bits) - 1;
                    *bit |= (write_flag as u8) << shift_bit;
                }
                freedata.free_bits = bit.count_zeros();
                write_bits -= zero_bits as u64;
            }
        }
    }

    //write関数を使った処理
    //基本的にbitmapへの書き込みはwrite_bit_flagを使う
    pub fn write_bit_flag(&self, bit_flag_data: FreeBits) -> result::Result<(), Box<dyn  error::Error>>{
        match bit_flag_data {
            FreeBits::defragmentation(data) => {
                self.write(data);
            }
            FreeBits::fragmentation(datas) => {
                for data in datas{
                    self.write(data);
                }
            }
            FreeBits::error(err) => {
                println!("error code : {}", err);
            }
        }

        Ok(())
    }

    //指定した数分の連続するbitmapのindexとoffsetを返す
    //指定した数分の連続するbitがない場合は断片化の上で情報を渡す
    // // 修正予定★
    // // 修正内容→連続するbitを探しながら断片化前提のbitを探すことで効率を上げる書き換えを想定している
    pub fn get_free_bit(&self, select_size: u64) -> FreeBits{
        let mut result_data = ResultFreeBitData::default();
        
        let mut free_bit_fragmentation: Vec<ResultFreeBitData>  = Vec::new();
        let mut fragmentation_get_bits: u64 = 0;
    
        let mut get_zeros: u64 = 0;
        let mut start_index: u64 = 0;
        let mut start_offset: u8 = 0;

        for index in 0..self.bitmap.borrow().len() {
            if let (Some(mut bit_data), Some(free_bit_data)) = (self.bitmap.borrow().get(index).copied().map(|x| !x), self.free_bitmap_datas.borrow().get(index)){
                //空きbit数が探索中のbit幅と同じであればすべて空いているためbit探索短縮
                let mut shift_bit: u32 = 0;
                if free_bit_data.free_bits == u8::BITS {
                    get_zeros += u8::BITS as u64;
                }
                else{
                    let mut count: u32;
                    while shift_bit < u8::BITS {
                        if bit_data & 1 != 0 {
                            count = bit_data.trailing_ones();
                            get_zeros += count as u64;
                            bit_data >>= count;
                            shift_bit += count;
                            if shift_bit < u8::BITS {
                                if fragmentation_get_bits < select_size{
                                    let required_bit_difference = select_size - fragmentation_get_bits;
                                    if required_bit_difference < get_zeros{
                                        result_data.get_bit_num = required_bit_difference;
                                    }
                                    else{
                                        result_data.get_bit_num = get_zeros;
                                    }
                                    result_data.index = start_index;
                                    result_data.offset = start_offset;
                                    
                                    free_bit_fragmentation.push(result_data.clone());
                                    fragmentation_get_bits += get_zeros;
                                }

                                get_zeros = 0;
                                count = bit_data.trailing_zeros();
                                bit_data >>= count;
                                shift_bit += count;
                                start_index = index as u64;
                                start_offset = shift_bit as u8;
                            }
                        }
                        else{
                            count = bit_data.trailing_zeros();
                            bit_data >>= count;
                            shift_bit += count;
                            start_index = index as u64;
                            start_offset = shift_bit as u8;
                        }
                    }
                }
                
                //空きが見つかった場合
                if get_zeros >= select_size {
                    result_data.get_bit_num = select_size;
                    result_data.index = start_index;
                    result_data.offset = start_offset;

                    return FreeBits::defragmentation(result_data);
                }
            }
        }


        FreeBits::fragmentation(free_bit_fragmentation)
    
    }

    pub fn new(super_block: &FS_core_types::SuperBlockData, select_address: u64, output_file: &mut fs::File) -> result::Result<Self, Box<dyn error::Error>>{
        let capacity = (super_block.partition_cluster_size as usize + (mem::size_of::<u8>() * 8) - 1) / (mem::size_of::<u8>() * 8);
        let bitmap: RefCell<Vec<u8>> = RefCell::new(vec![0; capacity]);
        let free_bitmap_datas: RefCell<Vec<FreeBitmapData>> = RefCell::new(vec![FreeBitmapData::default(); capacity]);

        let bitmap_datas = FreeBitmap{
            bitmap,
            free_bitmap_datas,
        };

        bitmap_datas.get_bitmap(super_block, select_address, output_file)?;
        bitmap_datas.set_free_bitmap(super_block);

        Ok(bitmap_datas)
    }
}
