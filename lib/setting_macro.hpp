
//設定マクロ系統
#define DEBUG_MODE 1
#define EACH_VERSIONS 1

//バージョン管理マクロ
#if EACH_VERSIONS == 1
    #define DIRECTORY_VERSION_SCAN_DIR_TREE 1
    #if DIRECTORY_VERSION_SCAN_DIR_TREE == 1
        
    #endif


#endif

//デバッグ管理マクロ
#if DEBUG_MODE == 1

    #define OUTPUT_DEBUG_LOGS 1
    #if OUTPUT_DEBUG_LOGS == 1
        #define OUTPUT_SETTING_DATA_LOG 1
        #define OUTPUT_FORMAT_LOG 1
        #define OUTPUT_EXECUTION_LOG 1
        #if OUTPUT_EXECUTION_LOG == 1
            #define OUTPUT_BITMAP_DATA_LOG 1
            #define OUTPUT_TREE_DATA_LOG 1
            #define OUTPUT_DIRECTORY_TREE_LOG 1
            #define OUTPUT_FREE_ID_TREE_LOG 1


        #endif

    #endif

#endif