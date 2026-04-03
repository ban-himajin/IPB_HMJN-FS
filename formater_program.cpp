#include<iostream>
#include<string>
#include<fstream>
#include<filesystem>
#include<vector>
#include<cstdint>
#include<cmath>
#include<bitset>
#include<set>

//--------------------fs writing and copying function and datas start---------------------
//この領域はFSの初期化やチェックのみに使用する領域

#define VM_FILE_NAME "vm_disk.bin"

#define BIT 1
#define BYTE (BIT*8)
#define KBYTE (BYTE * 1024)
#define MBYTE (KBYTE * 1024)
#define GBYTE (MBYTE * 1024)

#define FS_VERSION_TOP 0
#define FS_VERSION_MID 0
#define FS_VERSION_LOWER 0
#define FS_SECTOR_SIZE 512
#define FS_BACK_UP_NUM 2

#define cast_const_char(x) reinterpret_cast<const char*>(x)
#define cast_char(x) reinterpret_cast<char*>(x)

#define SUPER_BLOCK_CLUSTER_SIZE 1
#define USE_CLUSTER_SIZE 1
#define ADDRESS_BYTE_SIZE 8

using namespace std;
namespace fs = std::filesystem;

#pragma pack(push, 1)
struct IPB_HMJN_FS_struct{
    uint8_t magic_number[8] = {'I', 'P', 'B', '_', 'H', 'M', 'J', 'N'};
    uint8_t back_up_or_main = 0;
    struct{
        uint8_t top = FS_VERSION_TOP;
        uint8_t mid = FS_VERSION_MID;
        uint8_t lower = FS_VERSION_LOWER;
    }fs_version;
    uint64_t partition_LBA = 0;
    uint64_t partition_start_cluster_num = 0;
    uint64_t one_sector_size = FS_SECTOR_SIZE;
    uint64_t one_block_sector_num = 0;
    uint64_t one_cluster_block_num = 0;
    uint64_t partition_cluster_size = 0;
    uint64_t bitmap_num = 0;
    uint64_t bitmap_size = 0;
    uint64_t tree_cluster_num = 0;
    uint64_t tree_depth = 0;
    uint8_t back_up_num = FS_BACK_UP_NUM;
    uint64_t back_up_list_tree_cluster_num = 0;
    uint64_t directory_tree_cluster_num = 0;
    uint64_t directory_tree_depth = 0;
    uint64_t free_ID_tree_cluster_num = 0;
    uint64_t free_ID_tree_depth = 0;
    uint64_t reservation_space_size = 0;//reservation space size num
    uint64_t read_algorithm_num = 0;
    uint64_t write_algorithm_num = 0;
};
#pragma pack(pop)

int output_fs_struct_log(const IPB_HMJN_FS_struct& FS_meta_data){
    cout << "---------------FS_struct_datas---------------" << endl;
    cout << "magic_number: " << string_view(cast_const_char(FS_meta_data.magic_number), 8) << endl;
    cout << "back_up_or_main: " << static_cast<uint64_t>(FS_meta_data.back_up_or_main) << endl;
    cout << "fs_version.top: " << static_cast<uint64_t>(FS_meta_data.fs_version.top) << "."
         << static_cast<uint64_t>(FS_meta_data.fs_version.mid) << "."
         << static_cast<uint64_t>(FS_meta_data.fs_version.lower) << endl;;
    cout << "partition_LBA: " << static_cast<uint64_t>(FS_meta_data.partition_LBA) << endl;
    cout << "partition_start_cluster_num : " << static_cast<uint64_t>(FS_meta_data.partition_start_cluster_num) << endl;
    cout << "one_sector_size: " << static_cast<uint64_t>(FS_meta_data.one_sector_size) << endl;
    cout << "one_block_sector_num: " << static_cast<uint64_t>(FS_meta_data.one_block_sector_num) << endl;
    cout << "one_cluster_block_num: " << static_cast<uint64_t>(FS_meta_data.one_cluster_block_num) << endl;
    cout << "partition_cluster_size: " << static_cast<uint64_t>(FS_meta_data.partition_cluster_size) << endl;
    cout << "bitmap_num: " << static_cast<uint64_t>(FS_meta_data.bitmap_num) << endl;
    cout << "bitmap_size: " << static_cast<uint64_t>(FS_meta_data.bitmap_size) << endl;
    cout << "tree_cluster_num: " << static_cast<uint64_t>(FS_meta_data.tree_cluster_num) << endl;
    cout << "tree_depth: " << static_cast<uint64_t>(FS_meta_data.tree_depth) << endl;
    cout << "back_up_num: " << static_cast<uint64_t>(FS_meta_data.back_up_num) << endl;
    cout << "back_up_list_tree_cluster_num: " << static_cast<uint64_t>(FS_meta_data.back_up_list_tree_cluster_num) << endl;
    cout << "directory_tree_cluster_num: " << static_cast<uint64_t>(FS_meta_data.directory_tree_cluster_num) << endl;
    cout << "directory_tree_depth: " << static_cast<uint64_t>(FS_meta_data.directory_tree_depth) << endl;
    cout << "free_ID_tree_cluster_num: " << static_cast<uint64_t>(FS_meta_data.free_ID_tree_cluster_num) << endl;
    cout << "free_ID_tree_depth: " << static_cast<uint64_t>(FS_meta_data.free_ID_tree_depth) << endl;
    cout << "reservation_space_size: " << static_cast<uint64_t>(FS_meta_data.reservation_space_size) << endl;
    cout << "read_algorithm_num: " << static_cast<uint64_t>(FS_meta_data.read_algorithm_num) << endl;
    cout << "write_algorithm_num: " << static_cast<uint64_t>(FS_meta_data.write_algorithm_num) << endl;
    cout << "---------------------------------------------" << endl;

    return 0;
}

template<typename T>
int output_fs_bitmap_log(const vector<T>& bitmaps){
    cout << "-------------bitmap_datas-----------" << endl;
    uint32_t count = 0;
    cout << "bit map size : " << bitmaps.size() << endl;
    for(T bits : bitmaps){
        cout << "(" << count << ")" << bitset<BYTE * sizeof(T)>(bits) << ",";
        count++;
    }
    cout << "\n------------------------------------" << endl;
    return 0;
}

int fs_data_format(IPB_HMJN_FS_struct& FS_meta_datas){
    uint32_t input_variable;
    cout << "format the vm disk file" << endl;

    cout << "How many sectors make up one block?" << endl;
    cin >> input_variable;
    FS_meta_datas.one_block_sector_num = input_variable;

    cout << "How many block make up one cluster?" << endl;
    cin >> input_variable;
    FS_meta_datas.one_cluster_block_num = input_variable;

    cout << "How many cluster make up disk?" << endl;
    cin >> input_variable;
    FS_meta_datas.partition_cluster_size = input_variable;
    uint64_t one_cluster_bytes = (FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num);
    FS_meta_datas.bitmap_size = USE_CLUSTER_SIZE + ceil(FS_meta_datas.partition_cluster_size / BYTE) / one_cluster_bytes;
    FS_meta_datas.reservation_space_size = one_cluster_bytes - (sizeof(IPB_HMJN_FS_struct) - sizeof(uint64_t));
    FS_meta_datas.bitmap_num = 1;
    FS_meta_datas.tree_cluster_num = FS_meta_datas.bitmap_size + 1;
    FS_meta_datas.directory_tree_cluster_num = FS_meta_datas.tree_cluster_num + 1;
    FS_meta_datas.back_up_list_tree_cluster_num = FS_meta_datas.directory_tree_cluster_num + 1;
    FS_meta_datas.free_ID_tree_cluster_num = FS_meta_datas.back_up_list_tree_cluster_num + 1;
    FS_meta_datas.partition_start_cluster_num = SUPER_BLOCK_CLUSTER_SIZE + FS_meta_datas.free_ID_tree_cluster_num + USE_CLUSTER_SIZE;

    output_fs_struct_log(FS_meta_datas);
    return 0;
}

template<typename T>
int write_one_bitmap_bit(vector<T>&bitmaps, uint64_t bit_num){
    uint64_t write_bit_index = bit_num/(BYTE * sizeof(T));
    uint8_t write_bit = bit_num%(BYTE * sizeof(T));
    if(bitmaps.size() <= write_bit_index){
        cerr << "designation cluster number is out of range access:" << write_bit_index << endl;
        return 1;
    }
    try{
        bitmaps.at(write_bit_index) |= 1<<write_bit;
    }catch(const out_of_range& e){
        cerr << "exception occurrence! designation cluster number is out of range access:" << e.what() << endl;
        return 1;
    }
    return 0;
}

template<typename T>
int flash_bitmap_datas(fstream& file,const IPB_HMJN_FS_struct& FS_meta_datas, vector<T>&bitmaps){
    uint64_t one_cluste_bytes = FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num;
    uint64_t bitmap_bytes = one_cluste_bytes * FS_meta_datas.bitmap_num;
    file.seekp(bitmap_bytes, ios::beg);
    file.write(cast_const_char(bitmaps.data()), bitmaps.size());
    return 0;
}

template<typename T>
int create_bitmap_data(fstream& file,const IPB_HMJN_FS_struct& FS_meta_datas, vector<T>&bitmaps){
    if(write_one_bitmap_bit(bitmaps, 0) != 0)return 1;
    for(uint64_t count = 0;count < FS_meta_datas.bitmap_size;count++){
        if(write_one_bitmap_bit(bitmaps, FS_meta_datas.bitmap_num + count) != 0)return 1;
    }
    if(write_one_bitmap_bit(bitmaps, FS_meta_datas.tree_cluster_num) != 0)return 1;
    if(write_one_bitmap_bit(bitmaps, FS_meta_datas.directory_tree_cluster_num) != 0)return 1;
    if(write_one_bitmap_bit(bitmaps, FS_meta_datas.free_ID_tree_cluster_num) != 0)return 1;
    if(write_one_bitmap_bit(bitmaps, FS_meta_datas.back_up_list_tree_cluster_num) != 0)return 1;
    flash_bitmap_datas(file,FS_meta_datas,bitmaps);
    return 0;
}

int write_super_block_data(fstream& file, const IPB_HMJN_FS_struct& FS_meta_datas, uint64_t seek_num, ios_base::seekdir seek_mode){
    file.seekp(seek_num,seek_mode);
    file.write(cast_const_char(FS_meta_datas.magic_number), sizeof(FS_meta_datas.magic_number));
    file.write(cast_const_char(&FS_meta_datas.back_up_or_main), sizeof(FS_meta_datas.back_up_or_main));
    file.write(cast_const_char(&FS_meta_datas.fs_version), sizeof(FS_meta_datas.fs_version));
    file.write(cast_const_char(&FS_meta_datas.partition_LBA), sizeof(FS_meta_datas.partition_LBA));
    file.write(cast_const_char(&FS_meta_datas.partition_start_cluster_num), sizeof(FS_meta_datas.partition_start_cluster_num));
    file.write(cast_const_char(&FS_meta_datas.one_sector_size), sizeof(FS_meta_datas.one_sector_size));
    file.write(cast_const_char(&FS_meta_datas.one_block_sector_num), sizeof(FS_meta_datas.one_block_sector_num));
    file.write(cast_const_char(&FS_meta_datas.one_cluster_block_num), sizeof(FS_meta_datas.one_cluster_block_num));
    file.write(cast_const_char(&FS_meta_datas.partition_cluster_size), sizeof(FS_meta_datas.partition_cluster_size));
    file.write(cast_const_char(&FS_meta_datas.bitmap_num), sizeof(FS_meta_datas.bitmap_num));
    file.write(cast_const_char(&FS_meta_datas.bitmap_size), sizeof(FS_meta_datas.bitmap_size));
    file.write(cast_const_char(&FS_meta_datas.tree_cluster_num), sizeof(FS_meta_datas.tree_cluster_num));
    file.write(cast_const_char(&FS_meta_datas.tree_depth), sizeof(FS_meta_datas.tree_depth));
    file.write(cast_const_char(&FS_meta_datas.back_up_num), sizeof(FS_meta_datas.back_up_num));
    file.write(cast_const_char(&FS_meta_datas.back_up_list_tree_cluster_num), sizeof(FS_meta_datas.back_up_list_tree_cluster_num));
    file.write(cast_const_char(&FS_meta_datas.directory_tree_cluster_num), sizeof(FS_meta_datas.directory_tree_cluster_num));
    file.write(cast_const_char(&FS_meta_datas.directory_tree_depth), sizeof(FS_meta_datas.directory_tree_depth));
    file.write(cast_const_char(&FS_meta_datas.free_ID_tree_cluster_num), sizeof(FS_meta_datas.free_ID_tree_cluster_num));
    file.write(cast_const_char(&FS_meta_datas.free_ID_tree_depth), sizeof(FS_meta_datas.free_ID_tree_depth));
    file.seekp(FS_meta_datas.reservation_space_size,ios::cur);
    file.write(cast_const_char(&FS_meta_datas.read_algorithm_num), sizeof(FS_meta_datas.read_algorithm_num));
    file.write(cast_const_char(&FS_meta_datas.write_algorithm_num), sizeof(FS_meta_datas.write_algorithm_num));
    return 0;
}

template<typename T>
int write_back_up_super_block(fstream& file, IPB_HMJN_FS_struct FS_meta_datas, uint64_t seek_num, ios_base::seekdir seek_mode, vector<T>& bitmaps){
    uint64_t one_cluster_bytes = FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num;
    FS_meta_datas.back_up_or_main = 1;
    if(seek_mode == ios::end)seek_num = FS_meta_datas.partition_cluster_size * one_cluster_bytes - seek_num;
    else if(seek_mode == ios::cur)seek_num = file.tellp();
    write_super_block_data(file, FS_meta_datas, seek_num, ios::beg);
    write_one_bitmap_bit(bitmaps, seek_num / one_cluster_bytes);
    return 0;
}

template<typename T>
int create_back_up_list_tree_data(fstream& file,const IPB_HMJN_FS_struct& FS_meta_datas, vector<T>& bitmaps){
    if(FS_meta_datas.back_up_num == 0)return 0;
    uint64_t one_cluster_bytes = FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num;
    vector<uint64_t>back_up_data((one_cluster_bytes/ADDRESS_BYTE_SIZE), 0);
    if(back_up_data.size() == 0)return 1;

    write_back_up_super_block(file, FS_meta_datas, one_cluster_bytes, ios::end, bitmaps);
    back_up_data.at(0) = FS_meta_datas.partition_cluster_size - 1;

    if(FS_meta_datas.back_up_num > 1){
        for(uint64_t count = 2;count <= FS_meta_datas.back_up_num; count++){
            uint64_t back_up_byte_num = FS_meta_datas.partition_cluster_size;
            back_up_byte_num -= FS_meta_datas.partition_cluster_size / count;
            if(FS_meta_datas.partition_cluster_size / count == FS_meta_datas.partition_cluster_size / (count - 1))break;

            uint64_t seek_num = back_up_byte_num * one_cluster_bytes;
            if(seek_num != 0)write_back_up_super_block(file, FS_meta_datas, seek_num, ios::end, bitmaps);
            back_up_data.at(count - 1) = back_up_byte_num;
        }
    }
    for(uint64_t lists : back_up_data){
        if(lists == 0)break;
        cout << "back_up_list : " << lists << endl;
    }
    file.seekp(one_cluster_bytes * FS_meta_datas.back_up_list_tree_cluster_num,ios::beg);
    file.write(cast_const_char(back_up_data.data()), back_up_data.size());
    return 0;
}

int write_FS_data(fstream& file, IPB_HMJN_FS_struct& FS_meta_datas){
    uint64_t one_cluster_bytes = FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num;
    vector<uint8_t> bitmaps((FS_meta_datas.partition_cluster_size + 7) / BYTE, 0);
    write_super_block_data(file, FS_meta_datas, 0, ios::beg);
    create_back_up_list_tree_data(file, FS_meta_datas, bitmaps);
    if(create_bitmap_data(file, FS_meta_datas, bitmaps) == 1)return 1;
    file.flush();
    output_fs_bitmap_log(bitmaps);

    return 0;
}

int get_super_block_datas(fstream& file, IPB_HMJN_FS_struct& FS_meta_datas, uint64_t seek_num, ios_base::seekdir seek_mode){
    file.seekp(seek_num, seek_mode);
    file.read(cast_char(&FS_meta_datas.magic_number), sizeof(FS_meta_datas.magic_number));
    file.read(cast_char(&FS_meta_datas.back_up_or_main), sizeof(FS_meta_datas.back_up_or_main));
    file.read(cast_char(&FS_meta_datas.fs_version), sizeof(FS_meta_datas.fs_version));
    file.read(cast_char(&FS_meta_datas.partition_LBA), sizeof(FS_meta_datas.partition_LBA));
    file.read(cast_char(&FS_meta_datas.partition_start_cluster_num), sizeof(FS_meta_datas.partition_start_cluster_num));
    file.read(cast_char(&FS_meta_datas.one_sector_size), sizeof(FS_meta_datas.one_sector_size));
    file.read(cast_char(&FS_meta_datas.one_block_sector_num), sizeof(FS_meta_datas.one_block_sector_num));
    file.read(cast_char(&FS_meta_datas.one_cluster_block_num), sizeof(FS_meta_datas.one_cluster_block_num));
    file.read(cast_char(&FS_meta_datas.partition_cluster_size), sizeof(FS_meta_datas.partition_cluster_size));
    file.read(cast_char(&FS_meta_datas.bitmap_num), sizeof(FS_meta_datas.bitmap_num));
    file.read(cast_char(&FS_meta_datas.bitmap_size), sizeof(FS_meta_datas.bitmap_size));
    file.read(cast_char(&FS_meta_datas.tree_cluster_num), sizeof(FS_meta_datas.tree_cluster_num));
    file.read(cast_char(&FS_meta_datas.tree_depth), sizeof(FS_meta_datas.tree_depth));
    file.read(cast_char(&FS_meta_datas.back_up_num), sizeof(FS_meta_datas.back_up_num));
    file.read(cast_char(&FS_meta_datas.back_up_list_tree_cluster_num), sizeof(FS_meta_datas.back_up_list_tree_cluster_num));
    file.read(cast_char(&FS_meta_datas.directory_tree_cluster_num), sizeof(FS_meta_datas.directory_tree_cluster_num));
    file.read(cast_char(&FS_meta_datas.directory_tree_depth), sizeof(FS_meta_datas.directory_tree_depth));
    file.read(cast_char(&FS_meta_datas.free_ID_tree_cluster_num), sizeof(FS_meta_datas.free_ID_tree_cluster_num));
    file.read(cast_char(&FS_meta_datas.free_ID_tree_depth), sizeof(FS_meta_datas.free_ID_tree_depth));
    uint64_t one_cluster_bytes = FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num;
    FS_meta_datas.reservation_space_size = one_cluster_bytes - (sizeof(IPB_HMJN_FS_struct) - sizeof(uint64_t));
    file.seekp(FS_meta_datas.reservation_space_size,ios::cur);
    file.read(cast_char(&FS_meta_datas.read_algorithm_num), sizeof(FS_meta_datas.read_algorithm_num));
    file.read(cast_char(&FS_meta_datas.write_algorithm_num), sizeof(FS_meta_datas.write_algorithm_num));
    return 0;
}

template<typename T>
int get_bitmap(fstream& file, const IPB_HMJN_FS_struct& FS_meta_datas, vector<T>& bitmaps){
    uint64_t one_cluster_bytes = FS_meta_datas.one_sector_size * FS_meta_datas.one_block_sector_num * FS_meta_datas.one_cluster_block_num;
    file.seekp(FS_meta_datas.bitmap_num * one_cluster_bytes, ios::beg);
    file.read(cast_char(bitmaps.data()),bitmaps.size());
    output_fs_bitmap_log(bitmaps);
    return 0;
}

template<typename T>
int read_block_datas(fstream& file, IPB_HMJN_FS_struct& FS_meta_datas, vector<T>&bitmaps){
    get_super_block_datas(file, FS_meta_datas, 0, ios::beg);
    bitmaps.resize((FS_meta_datas.partition_cluster_size + 7) / BYTE, 0);
    
    return 0;
}

template<typename T>
int data_check_process(const fs::path& vm_disk_path, IPB_HMJN_FS_struct& FS_meta_datas, vector<T>& bitmaps){
    fstream file(vm_disk_path, ios::out | ios::in | ios::binary);
    uint32_t input_variable;
    if(!file){
        cerr << "Could not open file" << endl;
        return 0;
    }
    uint64_t file_size = fs::file_size(vm_disk_path);
    if(file_size < 512){
        cout << "super block not found. create super block? 0 = no / 1 = yes" << endl;
        cin >> input_variable;
        if(input_variable == 0){
            cout << "Processing cannot be performed if there is no superblock." << endl;
            return 1;
        }
        if(fs_data_format(FS_meta_datas)!=0)return 1;
        cout << "successfully created super block" << endl;
        cout << "write to VM disk" << endl;
        if(write_FS_data(file, FS_meta_datas)!=0)return 1;
    }
    read_block_datas(file, FS_meta_datas, bitmaps);

    cout << "found super block! Do you want super block check? 0 = n / 1 = yes" << endl;
    cin >> input_variable;
    if(input_variable == 1){
        output_fs_struct_log(FS_meta_datas);
        get_bitmap(file,FS_meta_datas,bitmaps);
    }
    return 0;
}

//--------------------fs writing and copying function end---------------------

//--------------------fs process functions and datas start-------------------
//この領域はFSの操作にかかわるものを置く領域

#pragma pack(push, 1)
struct free_bitmap_nums{
    uint64_t bitmap_index = 0;
    uint8_t bits = 0;
    bool true_or_false = 0;
};
#pragma pack(pop)

struct fs_system_function_struct{

};

struct common_interface_struct{
    int (*create_function) = 0;
    int (*read_function) = 0;
    int (*write_function) = 0;
    int (*seve_function) = 0;
    int (*fulsh_function) = 0;
    int (*delete_function) = 0;
    int (*scan_function) = 0;
    int (*chenge_function) = 0;
    void (*fs_system_function_struct) = 0;
};


//空きbitはあったが要求サイズじゃなかったときに使用

struct free_bit_data_struct{
    uint64_t index;
    uint64_t zero_count;

    bool operator<(const free_bit_data_struct& other) const{
        return index < other.index;
    }
};

struct bitmap_common_structs{
    uint64_t bitmap_index;
    vector<uint64_t> size;
    vector<uint64_t>offset;
};

struct bitmap_result_data{
    vector<bitmap_common_structs> bitmap_datas;
    uint64_t result_num = 0;
};

template<typename T>
bitmap_result_data get_free_bitmap(vector<T>&bitmaps ,const uint64_t request_cluster_size, set<free_bit_data_struct>& free_bit_data){
    bitmap_result_data get_bitmap_datas;
    free_bit_data_struct free_bit_datas;
    T all_ones = ~static_cast<T>(0);
    uint64_t index = 0;
    uint64_t bit_space = 0;
    uint64_t bit_count = 0;
    uint64_t start_index_num = 0;
    uint64_t max_continuous_zero = 0;
    uint64_t continuous_zero = 0;
    uint64_t bit_offset = 0;
    uint64_t one_count = 0;
    uint64_t zero_count = 0;

    //bit連続性が確保されているかの確認ループ
    if(free_bit_data.size() > 0){
        index = free_bit_data.rbegin()->index;
    }
    for(;index < bitmaps.size();index++){
        if(index > bitmaps.size()){
            cout << "not get free bitmap!!" << endl;
            get_bitmap_datas.result_num = 1;
            return get_bitmap_datas;
        }
        try{
            bit_space = static_cast<T>(~bitmaps.at(index));
        }
        catch(const out_of_range& e){
            cout << "out of range exception not get free bitmap!! : " << e.what() << endl;
            get_bitmap_datas.result_num = 2;
            return get_bitmap_datas;
        }
        if(all_ones != ~bit_space){
            ////ここから先は__builtin_ctzll(組み込み関数)をつかったbitmapの走行をするような実装をする
            zero_count = __builtin_popcountll(~bit_space);
            one_count = __builtin_popcountll(bit_space);
            
            free_bit_datas.index = index;
            free_bit_datas.zero_count = zero_count;
            
            for(uint64_t count = 0; count < (sizeof(T) * BYTE);count += bit_count){
                ////今いる地点がbit 0,1 可を判断して処理を変える必要がある
                if(bit_space & 1){
                    bit_count = __builtin_ctzll(~bit_space);
                    if(bit_count > (sizeof(T) * BYTE)){
                        bit_count = bit_count - (bit_count - (sizeof(T) * BYTE));
                    }
                    bit_space >>= bit_count;
                    continuous_zero += bit_count;
                    if(continuous_zero > max_continuous_zero){
                        max_continuous_zero += continuous_zero;
                        if(max_continuous_zero >= request_cluster_size){
                            bitmap_common_structs common_data;
                            common_data.bitmap_index = start_index_num;
                            common_data.size.push_back(request_cluster_size);
                            common_data.offset.push_back(bit_offset);
                            get_bitmap_datas.bitmap_datas.push_back(common_data);
                            get_bitmap_datas.result_num = 0;
                            return get_bitmap_datas;
                        }
                    }
                }
                else{
                    continuous_zero = 0;
                    bit_count = __builtin_ctzll(bit_space);
                    bit_space >>= bit_count;

                    max_continuous_zero = 0;
                    start_index_num = index;
                    
                    bit_offset = count + bit_count;
                }
            }
            if(continuous_zero == 0){
                free_bit_data.insert(free_bit_datas);
            }
        }else{
            continuous_zero = 0;
            max_continuous_zero = 0;
        }
    }
    
    //断片化することを前提で領域を確保できるかチェック
    zero_count = 0;
    uint64_t loop_num = free_bit_data.size();
    for(index = 0; index < loop_num; index++){
        auto free_bit_struct = next(free_bit_data.begin(), 0);
        uint64_t bit_zero_size = free_bit_struct->zero_count;
        uint64_t bit_index = free_bit_struct->index;
        vector<bitmap_common_structs> &bit_datas = get_bitmap_datas.bitmap_datas;
        bitmap_common_structs bitmap_common_datas;
        try{
            bit_space = static_cast<T>(~bitmaps.at(bit_index));
        }
        catch(const out_of_range& e){
            cerr << "bitmaps fragmentition out of range!! : " << e.what() << endl;
            get_bitmap_datas.result_num = 3;
            return get_bitmap_datas;
        }
        for(uint64_t count = 0; count < (sizeof(T) * BYTE);count += bit_count){
            if(bit_space & 1){
                bit_count = __builtin_ctzll(~bit_space);
                bit_space >>= bit_count;
                zero_count += bit_count;
                bitmap_common_datas.bitmap_index = bit_index;
                bitmap_common_datas.offset.push_back(count);
                if(zero_count < request_cluster_size){
                    bitmap_common_datas.size.push_back(bit_count);
                }
                else{
                    free_bit_data_struct insert_free_bit_data;
                    insert_free_bit_data.index = bit_index;
                    bit_zero_size = bit_count - (zero_count - request_cluster_size);
                    insert_free_bit_data.zero_count = bit_zero_size;
                    bitmap_common_datas.size.push_back(bit_zero_size);
                    free_bit_data.erase(free_bit_struct);
                    free_bit_data.insert(insert_free_bit_data);
                    bit_datas.push_back(bitmap_common_datas);
                    return get_bitmap_datas;
                }
            }else{
                bit_count = __builtin_ctzll(bit_space);
                bit_space >>= bit_count;
            }
        }
        bit_datas.push_back(bitmap_common_datas);
        free_bit_data.erase(free_bit_struct);
    }
    get_bitmap_datas.result_num = 4;
    return get_bitmap_datas;
}


int create_directory(){
    
    return 0;
}


//--------------------fs process functions and datas end-------------------

//--------------------fs command functions start-------------------
//この領域はFSを使用するためのものを置く領域

int commands(){

    return 0;
}

int mkdir(){

    return 0;
}


template<typename T>
int FS_operation_main(const fs::path& disk_path, IPB_HMJN_FS_struct& FS_meta_data, vector<T>&bitmaps){
    set<free_bit_data_struct> free_bit_data;
    bitmap_result_data get_bitmap_data = get_free_bitmap(bitmaps, 20, free_bit_data);
    if(get_bitmap_data.result_num != 0)return 1;
    cout << "\n------------get bitmap data-----------------" << endl;
    if(get_bitmap_data.bitmap_datas.size() > 1){
        cout << "fragmentition" << endl;
    }else{
        cout << "not fragmentition" << endl;
    }
    for(bitmap_common_structs common_struct : get_bitmap_data.bitmap_datas){
        cout << "\nindex : " << common_struct.bitmap_index << endl;
        cout << "designation : " << bitset<BYTE * sizeof(T)>(bitmaps.at(common_struct.bitmap_index)) << endl;
        for(uint64_t index = 0; index < common_struct.size.size();index++){
            cout << "(" << index << ")get size : " << common_struct.size.at(index) << endl;
            cout << "(" << index << ")offset : " << common_struct.offset.at(index) << endl;
        }
    }

    return 0;
}

//--------------------fs command functions end-------------------

int main(){
    fs::path vm_disk_path = VM_FILE_NAME;
    if(fs::exists(vm_disk_path)){
        cout << "found file" << endl;
    }
    else{
        cout << "not found file" << endl;
        cout << "create file? 0 = no / 1 = yes" << endl;
        uint32_t input_variable;
        cin >> input_variable;
        if(input_variable == 0)return 0;
        cout << "created file" << endl;
        ofstream file(vm_disk_path, ios::out | ios::trunc);
        if(!file){
            cout << "failure create file" << endl;
            return 1;
        }
    }
    IPB_HMJN_FS_struct FS_meta_datas;
    vector<uint8_t> bitmaps(0, 0);
    if(data_check_process(vm_disk_path, FS_meta_datas, bitmaps) == 1)return 2;
    uint64_t fs_main_result = FS_operation_main(vm_disk_path, FS_meta_datas, bitmaps);
    switch(fs_main_result){
        case 0:
            return 0;

        case 1:
            cout << "get bitmapdata" << endl;
            return 1;

        default:
            cout << "undefined behavior" << endl;
            return static_cast<int>(~0);
    }
    return 0;
}

