use std::clone;
use std::collections::HashMap;
use std::collections::btree_map::Values;
use std::ffi::IntoStringError;
use std::hash::Hash;
use std::iter::TakeWhile;
use std::option::{Option};
use std::cell::{RefCell};
use std::ptr::read;

//キャッシュの仕組みを書く場所

trait NewCache{
    fn new(capacity: Capacity) -> Self;
}
trait GetCache<Key, Value>{
    fn get_cache(&mut self, select_key: Key) -> Option<Value>
    where
        Key: Eq + Hash,
        Value: Clone;
}
trait RelinkCacheNode{
    fn relink_cache_node(&mut self, index: Index);
}
trait PutCache<Key, Value>{
    fn put(&mut self, key: Key, value: Value);
}

type Index = usize;
type Capacity = usize;

#[derive(Clone)]
struct LruNode<Key, Value>{
    key: Key,
    value: Value,
    prev: Option<Index>,
    next: Option<Index>,
}
/*
Keyは
(キャッシュした階層,キャッシュ内の一番上の要素ID)のタプルか
(キャッシュ内の一番上の要素ID)にする予定
*/
pub struct LruCache<Key, Value>{
    nodes: Vec<LruNode<Key, Value>>,
    map: HashMap<Key, Index>,
    head: Option<Index>,
    tail: Option<Index>,
    capacity: Capacity,
}
impl<Key: Default, Value: Default> LruCache<Key, Value>{
    pub fn new(capacity: Capacity) -> Self{
        Self {
            nodes: Vec::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
            head: None,
            tail: None,
            capacity: capacity,
        }
    }

    //keyを使いノード番号を得る
    fn get_cache_node_num(&self, key: Key) -> Option<Index>
    where
        Key: Eq + Hash
    {
        if let Some(cache_data) = self.map.get(&key){
            Some(*cache_data)
        }else{
            None
        }
    }

    //キャッシュ順更新用関数
    pub fn relink_cache_node(&mut self, new_top_index: Index){

        if self.head == Some(new_top_index){
            return ;
        }

        let old_head = self.head;

        let (next_index, prev_index) = {
            let new_top_node = self.nodes.get_mut(new_top_index).expect("ノードに指定のインデックスが存在しませんでした");
            (new_top_node.next, new_top_node.prev)
        };

        //新しい頭ノードのデータ更新
        {
            let new_top_node = self.nodes.get_mut(new_top_index).expect("ノードに指定のインデックスが存在しませんでした");
            new_top_node.prev = None;
            new_top_node.next = old_head;
        }

        //前ノードと後ノードをつなげる
        if let Some(prev_index) = prev_index{
            let prev_node = self.nodes.get_mut(prev_index).expect("前ノードに指定のインデックスが存在しませんでした");
            prev_node.next = next_index;
        }
        if let Some(next_index) = next_index{
            let next_node = self.nodes.get_mut(next_index).expect("後ノードに指定のインデックスが存在しませんでした");
            next_node.prev = prev_index;
        }
        else{
            self.tail = prev_index;
        }

        //元頭のノード更新
        if let Some(head_index) = self.head{
            let head_node = self.nodes.get_mut(head_index).expect("頭ノードに指定のインデックスが存在しませんでした");
            head_node.prev = Some(new_top_index);
        }

        self.head = Some(new_top_index);

    }

    //putは要素を入れることを前提としているためkeyチェックはしない
    //Lruキャッシュを使うことがあるのであれば直すべき
    pub fn put(&mut self, key: Key, value: Value)
    where
        Key: Eq + Hash + Clone
    {
        //要素数が1だった時
        if self.capacity == 1{
            let mut index_value = 0;
            if let Some(index) = self.head{
                if let Some(node) = self.nodes.get_mut(index){
                    node.key = key.clone();
                    node.value = value;
                    node.prev = None;
                    node.next = None;
                }
                index_value = index;
            }
            else{
                index_value = self.nodes.len();
                self.nodes.push(
                    LruNode{
                        key: key.clone(),
                        value: value,
                        prev: None,
                        next: None
                    }
                );
            }
            self.map.clear();
            self.map.insert(key, index_value);

            self.head = Some(index_value);
            self.tail = Some(index_value);

        }
        //要素数に余裕があったとき
        else if self.nodes.len() < self.capacity{
            let nodes_len = self.nodes.len();
            let head_index = self.head;
            self.nodes.push(
                LruNode{
                    key: key.clone(),
                    value: value,
                    prev: None,
                    next: head_index,
                }
            );

            self.map.insert(key, nodes_len);

            if let Some(old_head_index) = self.head{
                if let Some(old_head_node) = self.nodes.get_mut(old_head_index){
                    old_head_node.prev = Some(nodes_len);
                }
            }

            self.head = Some(nodes_len);

            if self.tail == None {
                self.tail = Some(nodes_len);
            }

        }
        //要素数が満帆だった時
        else{
            let push_index = self.tail;
            let head_index = self.head;

            let (tail_prev_node_index, tail_node_key) = {
                    let tail_index = push_index.expect("尾インデックスが存在しませんでした");
                    let tail_node = self.nodes.get(tail_index).expect("尾ノードが存在しませんでした");
                    (tail_node.prev, tail_node.key.clone())
            };

            {//尾の前のノードを更新
                if let Some(prev_node_index) = tail_prev_node_index{
                    let prev_node = self.nodes.get_mut(prev_node_index).expect("尾の前ノードが存在しませんでした");
                    prev_node.next = None;
                }

            }
            self.tail = tail_prev_node_index;

            {
                let index = push_index.expect("プッシュインデックスが存在しません");
                let push_node = self.nodes.get_mut(index).expect("プッシュノードが存在しません");
                *push_node = LruNode{
                    key: key.clone(),
                    value: value,
                    prev: None,
                    next: head_index,
                };
                self.map.remove(&tail_node_key);
                
                self.map.insert(key, index);
            }
            {
                let index = head_index.expect("頭インデックスが存在しませんでした");
                let head_node = self.nodes.get_mut(index).expect("頭ノードが存在しませんでした");
                head_node.prev = push_index;
            }


            self.head = push_index;

        }
    }

    //指定したノードを取得する
    pub fn get_node(&mut self, key: Key) -> Option<&LruNode<Key, Value>>
    where
        Key: Eq + Hash
    {
        if let Some(cache_node_num) = self.get_cache_node_num(key){
            self.relink_cache_node(cache_node_num);
            self.nodes.get(cache_node_num)
        }
        else{
            None
        }
    }

    pub fn get_cache(&mut self, key: Key) -> Option<Value>
    where
        Key: Eq + Hash,
        Value: Clone
    {
        let node = self.get_node(key)?;
        Some(node.value.clone())
    }

}
// impl<Key, Value> NewCache for LruCache<Key, Value>{
//     fn new(capacity: Capacity) -> Self {
//         Self::new(capacity)
//     }
// }
impl<Key, Value> GetCache<Key, Value> for LruCache<Key, Value>{
    fn get_cache(&mut self, select_key: Key) -> Option<Value>
    where
        Key: Eq + Hash,
        Value: Clone
    {
        self.get_cache(select_key)
    }
}
impl<Key, Value> RelinkCacheNode for LruCache<Key, Value>{
    fn relink_cache_node(&mut self, index: Index) {
        self.relink_cache_node(index);
    }
}
impl<Key, Value> PutCache<Key, Value> for LruCache<Key, Value>{
    fn put(&mut self, key: Key, value: Value) {
        self.put(key, value);
    }
}


#[derive(Clone)]
struct LfuNode<Key, Value>{
    key :Key,
    value: Value,
    freq: usize,
    prev: Option<Index>,
    next: Option<Index>,
}
impl<Key, Value> LfuNode<Key, Value>{
    pub fn use_cache(&mut self){
        self.freq += 1;
    }
}
pub struct LfuCache<Key, Value>{
    nodes: Vec<LfuNode<Key, Value>>,
    nodemap: HashMap<Key, Index>,
    freqmap: HashMap<usize, (Index, Index)>,
    min_freq: usize,
    capacity: Capacity,
}
impl<Key: Default, Value: Default> LfuCache<Key, Value>{
    /*
    採取的にrelinkだけじゃなくunlink関数があると便利
    freq数が一定数を超えたらより探索数が多いものへ変える
     */
    pub fn new(capacity: Capacity) -> Self{
        Self{
            nodes: Vec::with_capacity(capacity),
            nodemap: HashMap::with_capacity(capacity),
            freqmap: HashMap::with_capacity(0),
            min_freq: 0,
            capacity: capacity,
        }
    }
    
    //keyを使いノード番号を得る
    fn get_cache_node_num(&self, key: Key) -> Option<Index>
    where
        Key: Eq + Hash
    {
        if let Some(cache_data) = self.nodemap.get(&key){
            Some(*cache_data)
        }else{
            None
        }
    }

    /*
    キャッシュを更新する
    freqは外部で更新し更新されたものを使いキャッシュのつながりを更新する
    ※もしかしたら更新時のnextが壊れる可能性がある
    ※最終的に定期更新を入れる予定
     */
    pub fn relink_cache_node(&mut self, reload_cache_index: Index){
        let (prev, next, freq) = {
            let node = self.nodes.get(reload_cache_index).unwrap();
            (node.prev, node.next, node.freq)
        };

        //ノードの前と後をつなぐ
        {
            if freq >= 1{
                let remove_bucket = 
                    if let Some((head_index, tail_index)) = self.freqmap.get_mut(&(freq - 1)){
                        //使用回数の要素が残り一個だった時の処理
                        if *head_index == reload_cache_index && *tail_index == reload_cache_index{
                            true
                        }
                        //リロードインデックスが頭インデックスだった時の処理
                        else{
                            if *head_index == reload_cache_index{
                                *head_index = next.unwrap();
                            }
                            //リロードインデックスが尾インデックスだった時の処理
                            else if *tail_index == reload_cache_index{
                                *tail_index = prev.unwrap();
                            }
                            false
                        }
                    }
                    else{
                        false
                    };
                if remove_bucket {
                    self.freqmap.remove(&(freq - 1));
                    if self.min_freq == freq - 1{
                        self.min_freq = freq;
                    }
                };
            }

            if let Some(prev_index) = prev{
                let prev_node = self.nodes.get_mut(prev_index).unwrap();
                prev_node.next = next;
            }

            if let Some(next_index) = next{
                let prev_node = self.nodes.get_mut(next_index).unwrap();
                prev_node.prev = prev;
            }

        }

        let mut head = None;

        //ノードを正しい位置へつなぐ
        {
            //使用回数がすでにマップ済みだった時の処理
            if let Some((head_index, _)) = self.freqmap.get_mut(&freq){
                head = Some(*head_index);
                *head_index = reload_cache_index;
            }
            //使用回数がすでにマップ済みじゃなかった時の処理
            else{
                self.freqmap.insert(freq, (reload_cache_index, reload_cache_index));
            }

            if let Some(head_index) = head{
                let head_node = self.nodes.get_mut(head_index).expect("頭ノードが存在しませんでした");
                head_node.prev = Some(reload_cache_index);
            }
        }

        {
            let node = self.nodes.get_mut(reload_cache_index).unwrap();
            node.prev = None;
            node.next = head;
        }

    }

    //putは要素を入れることを前提としているためkeyチェックはしない
    pub fn put(&mut self, key: Key, value: Value)
    where
        Key: Eq + Hash + Clone,
        // Value: Clone,
    {
        if self.nodes.len() < self.capacity{
            let nodes_len = self.nodes.len();
            let zero_freq_data = self.freqmap.get(&0);
            let mut head_index = None;

            //freq0の時頭インデックスが存在したときの処理
            if let Some((index, _)) = zero_freq_data{
                head_index = Some(*index);
                {//親ノードに新しいノードインデックスをつける
                    let head_node = self.nodes.get_mut(*index).unwrap();
                    head_node.prev = Some(nodes_len);
                }
                {//freqの頭インデックスを更新する
                    let freq_data = self.freqmap.get_mut(&0).unwrap();
                    freq_data.0 = nodes_len;
                }
            }
            //freq0の時頭インデックスが存在しなかったとき
            else{
                self.freqmap.insert(0, (nodes_len, nodes_len));
            }

            self.nodes.push(
                LfuNode{
                    key: key.clone(),
                    value: value,
                    freq: 0,
                    prev: None,
                    next: head_index,
                }
            );
            self.nodemap.insert(key, nodes_len);
            self.min_freq = 0;
        }
        else{
            //要素が満帆かつキャパシティが1の時
            if self.capacity == 1{
                let min_freq = self.min_freq;
                let delete_index = self.freqmap.get(&min_freq).unwrap().0;
                self.freqmap.clear();
                self.nodemap.clear();
                {//ノード更新
                    let node = self.nodes.get_mut(delete_index).unwrap();
                    node.key = key.clone();
                    node.value = value;
                    node.freq = 0;
                    node.next = None;
                    node.prev = None;
                }
                self.freqmap.insert(0, (0, 0));
                self.nodemap.insert(key, 0);

                self.min_freq = 0;
            }
            //要素が満帆かつキャパシティが1ではないとき
            else{
                let min_freq = self.min_freq;
                let delete_index = self.freqmap.get(&min_freq).unwrap().1;
                let prev = self.nodes.get(delete_index).unwrap().prev;
                {//freqのtailを更新
                    if let Some(new_tail) = prev{
                        let (reload_data) = self.freqmap.get_mut(&min_freq).unwrap();
                        reload_data.1 = new_tail;
                    }
                    else{
                        self.freqmap.remove(&min_freq);
                    }
                }
                {//prev_nodeの更新
                    let delete_prev = prev;
                    if let Some(delete_prev_index) = delete_prev{
                        self.nodes.get_mut(delete_prev_index).unwrap().next = None;
                    }
                }
                {//nodemapの更新
                    let delete_key = self.nodes.get(delete_index).unwrap().key.clone();
                    self.nodemap.remove(&delete_key);
                    self.nodemap.insert(key.clone(), delete_index);
                }
                {//ノードを更新する
                    let node = self.nodes.get_mut(delete_index).unwrap();
                    node.key = key.clone();
                    node.value = value;
                    node.freq = 0;
                    node.prev = None;
                    node.next = None;
                }
                {//freq0へリンク処理をする
                    self.relink_cache_node(delete_index);
                }
                self.min_freq = 0;

            }
        }
    }

    //指定したノードを取得する
    pub fn get_node(&mut self, key: Key) -> Option<&mut LfuNode<Key, Value>>
    where
        Key: Eq + Hash
    {
        let cache_node_num = self.get_cache_node_num(key)?;
        self.relink_cache_node(cache_node_num);
        self.nodes.get_mut(cache_node_num)
    }

    pub fn get_cache(&mut self, key: Key) -> Option<Value>
    where
        Key: Eq + Hash + Clone,
        Value: Clone + Default,
    {
        let mut result;
        {
            let node = self.get_node(key.clone())?;
            result = node.value.clone();
            node.use_cache();
        }
        let index = self.get_cache_node_num(key)?;
        self.relink_cache_node(index);
        Some(result)
    }

}
// impl<Key, Value> NewCache for LruCache<Key, Value>{
//     fn new(capacity: Capacity) -> Self {
//         Self::new(capacity)
//     }
// }
impl<Key, Value> GetCache<Key, Value> for LfuCache<Key, Value>{
    fn get_cache(&mut self, select_key: Key) -> Option<Value>
    where
        Key: Eq + Hash,
        Value: Clone
    {
        self.get_cache(select_key)
    }
}
impl<Key, Value> RelinkCacheNode for LfuCache<Key, Value>{
    fn relink_cache_node(&mut self, index: Index) {
        self.relink_cache_node(index);
    }
}
impl<Key, Value> PutCache<Key, Value> for LfuCache<Key, Value>{
    fn put(&mut self, key: Key, value: Value) {
        self.put(key, value);
    }
}


// struct CacheForecast<Key, Value>{
    
// }
// impl<Key, Value> Cache<Key> for CacheForecast <Key, Value>{
//     fn cache(&self, select_key: Key)
//     where
//         Key: Eq + Hash
//     {
        
//     }
// }