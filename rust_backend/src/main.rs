use std::collections::{HashMap};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct KVEntry {
    owner_name: String,
    value: String,
    version: u64,
    timestamp: u64,
    origin_node: String,
}

impl KVEntry {
    
}

#[derive(Debug, Clone)]
struct NodeState{
    node_id: String,
    store: Arc<Mutex<HashMap<String, KVEntry>>>,  //thread-safe share
    peer_nodes: Vec<String>,
}

impl NodeState {
    fn put( &self, key: String, entry: KVEntry) {
        //lock() to pause any thread that is using our hashMap
        //unwrap() to access the hashMap
        self.store.lock().unwrap().insert(key, entry);
    }
    fn get(&self, key: &str) -> Option<KVEntry>{
        let storeHash = self.store.lock().unwrap();
        match storeHash.get(key){
            Some(entry) => Some(entry.clone()),
            None => None,
        }
    }
    fn print_store(&self){
        let map = self.store.lock().unwrap();
        for (key, entry) in map.iter(){
            println!("key is {:?}", key);
            println!("entry is {:?}", entry);
        } 
    }
}

fn main(){
    let KVEntry_1 = KVEntry {
        owner_name : "Duc".to_string(),
        value: "hello".to_string(),
        version: 1,
        timestamp: 12345678,
        origin_node: "node-1".to_string(),
    };
    let KVEntry_2 = KVEntry {
        owner_name : "My".to_string(),
        value: "bonsoir".to_string(),
        version: 2,
        timestamp: 12345679,
        origin_node: "node-2".to_string(),
    };
    let KVEntry_3 = KVEntry {
        owner_name : "Bao".to_string(),
        value: "mary-chan".to_string(),
        version: 1,
        timestamp: 12345670,
        origin_node: "node-3".to_string(),
    };

    let node_state_1 = NodeState {
        node_id: "node_1".to_string(),
        //create empty map, wrap it in Mutex to legally mutate data, 
        //and wrap in Arc (smart pointer for multi ownership) for multi thread
        store: Arc::new(Mutex::new(HashMap::new())), 
        peer_nodes: vec!["node_2".to_string()],
    };
    let node_state_2 = NodeState {
        node_id: "node_2".to_string(),
        store: Arc::new(Mutex::new(HashMap::new())), 
        peer_nodes: vec!["node_1".to_string(), "node_3".to_string()],
    };
    let node_state_3 = NodeState {
        node_id: "node_3".to_string(),
        store: Arc::new(Mutex::new(HashMap::new())), 
        peer_nodes: vec!["node_2".to_string()],
    };

    node_state_1.put("k1".to_string(), KVEntry_1);
    node_state_2.put("k2".to_string(), KVEntry_2);
    node_state_3.put("k3".to_string(), KVEntry_3);
    node_state_1.print_store();
    node_state_2.print_store();
    node_state_3.print_store();

}