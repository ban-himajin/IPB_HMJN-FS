#include <iostream>
#include "lib/format_libs.hpp"

using namespace std;

// デバッグコード
#define DEBUG_CODE_NUMBER 1
#if DEBUG_CODE_NUMBER != 0
    #define DEBUG_VERSION 1
#endif

int main(){
    // // cout << sizeof(IPB_HMJN_FS::FS_TYPES::dir_entry) << endl;
    // IPB_HMJN_FS::PARTITION_HEADER fs_header = IPB_HMJN_FS::SET_UP::setup_header_datas();
    // IPB_HMJN_FS::FORMAT::format_fs(fs_header);
    // uint8_t name[256] = "hello";
    // IPB_HMJN_FS::EXECUTION::DIRECTORY_TREE::FS_DIRECTORY directory(fs_header);
    // fstream file(IPB_HMJN_FS::output_filename, ios::in | ios::out | ios::binary);
    // IPB_HMJN_FS::FS_TYPES::DIR_ENTRY dir_entry;
    // cout << "start dir entry test0" << endl;
    // dir_entry.data_type = IPB_HMJN_FS::FS_TYPES::DATA_TYPE::dir;
    // dir_entry.ID = 1;
    // dir_entry.parent_ID = 0;
    // dir_entry.next_sibling_ID = 0;
    // dir_entry.prev_sibling_ID = 0;
    // dir_entry.another_type_data.dir.first_child_ID = 0;
    // dir_entry.mode = 0;
    // dir_entry.last_updata_time = 0;
    // // dir_entry.name_size = 255;
    // dir_entry.name_size = 1;
    // IPB_HMJN_FS::FS_TYPES::DIR_ENTRY_INTERFACE dir_entry_data(dir_entry);
    // dir_entry_data.set_name(name);
    // IPB_HMJN_FS::EXECUTION::DIRECTORY_TREE::TREE_STRUCT dir_ID = directory.scan_dir_tree(file, fs_header.directory_tree_cluster_num, fs_header.directory_tree_depth);
    // cout << "get ID : " << dir_ID.dir_tree_cluster << endl;
    // // directory.write_directory_data(dir_entry, file);
    // // directory.write_directory_data(dir_entry, file);
    // // IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION bitmap_exe(fs_header);
    // IPB_HMJN_FS::EXECUTION::TREE_FUNCTIONS::TREE_FUNCTION bitmap_exe(fs_header);
    // IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA test1 = bitmap_exe.get_bitmap_data(30);
    // IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::RELOAD_STRUCT test2 = bitmap_exe.write_bit_flag(test1);
    // bitmap_exe.reload_bitmap(test2, file);
    // test2 = bitmap_exe.delete_bit_flag(test1);
    // bitmap_exe.reload_bitmap(test2, file);
    // IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA new_clusete_num = bitmap_exe.reload_tree(fs_header.directory_tree_cluster_num, file);
    // test2 = bitmap_exe.write_bit_flag(new_clusete_num);
    // IPB_HMJN_FS::EXECUTION::EXECUTION_STRUCT execution(fs_header);
    // execution.header_data.directory_tree_cluster_num = bitmap_exe.cast_address(new_clusete_num);
    // execution.header_data.directory_tree_depth++;
    // execution.test_write_super_block(file);
    // IPB_HMJN_FS::output_datas(fs_header);
    // IPB_HMJN_FS::EXECUTION::TREE_FUNCTIONS::TREE_FUNCTION test3(fs_header)
#if DEBUG_VERSION == 1

// 基本的なセットアップ
IPB_HMJN_FS::PARTITION_HEADER fs_header = IPB_HMJN_FS::SET_UP::setup_header_datas();
IPB_HMJN_FS::FORMAT::format_fs(fs_header);
IPB_HMJN_FS::EXECUTION::EXECUTION_STRUCT fs(fs_header);
fstream file(IPB_HMJN_FS::output_filename, ios::in | ios::out | ios::binary);
{//ツリー系統の関数チェックインデント

    IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA test3 = fs.reload_tree(fs.header_data.directory_tree_cluster_num, file);
    fs.header_data.directory_tree_cluster_num = fs.cast_address(test3);
    fs.header_data.directory_tree_depth += 1;
    fs.test_write_super_block(file);
    IPB_HMJN_FS::output_datas(fs.header_data);

    cout << "test logs1" << endl;
    IPB_HMJN_FS::EXECUTION::TREE_FUNCTIONS::result_tree_data test1 = fs.scan_tree_data(fs.header_data.directory_tree_cluster_num, fs.header_data.directory_tree_depth, file);

    cout << "address : " << test1.address << endl;
    cout << "scan order : " << test1.scan_order.size() << endl;

    fs.scan_dir_entry(file);

}

{//ディレクトリエントリ系統の操作テスト
    uint64_t ID = fs.scan_dir_entry(file);
    IPB_HMJN_FS::FS_TYPES::DIR_ENTRY entry;
    entry.data_type = IPB_HMJN_FS::FS_TYPES::DATA_TYPE::dir;
    entry.ID = ID;
    entry.parent_ID = 0;
    entry.next_sibling_ID = ID;
    entry.prev_sibling_ID = ID;
    entry.another_type_data.dir.first_child_ID = ID;
    entry.mode = 0;
    entry.last_updata_time = 0;
    IPB_HMJN_FS::FS_TYPES::DIR_ENTRY_INTERFACE dir_inter_face(entry);
    uint8_t name[256] = "hello";
    dir_inter_face.dir_entry.name_size = 5;
    dir_inter_face.set_name(name);

    cout << "input name : " << name[0] << endl;
    cout << "entry name : " << entry.name[0] << endl;
    cout << "inter face entry name : " << dir_inter_face.dir_entry.name[0] << endl;

    IPB_HMJN_FS::output_datas(fs.header_data);

    fs.addtion_dir_entry(file, ID, entry);
    fs.addtion_dir_entry(file, ID+1, entry);
    fs.addtion_dir_entry(file, ID+2, entry);

    cout << "write ID : " << ID << endl;

}



#endif

}