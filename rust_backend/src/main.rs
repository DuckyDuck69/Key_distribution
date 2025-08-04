use std::collections::{HashMap, VecDeque};
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

//must be pascal case in enum and struct
#[derive(Debug, Clone)]
enum FunctionCode {   //enum: a type that can be one or several variant
    Put {key: String, entry: KVEntry},
    Get {key: String},
    Print,
}

#[derive(Debug, Clone)]
struct NodeState{
    node_id: String,
    store: Arc<Mutex<HashMap<String, KVEntry>>>,  //thread-safe share
    peer_nodes: Vec<String>,
    command_queue: Arc<Mutex<VecDeque<FunctionCode>>>,
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
    fn handle_command(&self, cmd: FunctionCode){
        match cmd {
            FunctionCode::Get { key } => {
                match self.get(&key){
                    Some(entry) => println!("Found: {:?}", key),
                    None => println!("Can't find the key!"),
                }
            },
            FunctionCode::Put { key, entry } => {
                self.put(key, entry);
                println!("Node now contains: \n");
                self.print_store();
            },
            FunctionCode::Print =>{
                self.print_store();
            }   
        };
    }
    fn enque_command(&self, cmd:FunctionCode){
        self.command_queue.lock().unwrap().push_back(cmd);
    }
    fn deque_command(&self) -> Option<FunctionCode>{  //return Option<> because Rust is type safe and handle None/Null 
        self.command_queue.lock().unwrap().pop_front()
    }
    fn execute_queue_command(&self){
        //why we can still pull a cmd, handle it
        // the command below is equal to "while queue: cmd = queue.pop(0)" in Python
        while let Some(cmd) = self.deque_command(){
            self.handle_command(cmd);
        }
    }
}

fn main(){
    let knentry_1 = KVEntry {
        owner_name : "Duc".to_string(),
        value: "hello".to_string(),
        version: 1,
        timestamp: 12345678,
        origin_node: "node-1".to_string(),
    };
    let kventry_2 = KVEntry {
        owner_name : "My".to_string(),
        value: "bonsoir".to_string(),
        version: 2,
        timestamp: 12345679,
        origin_node: "node-2".to_string(),
    };
    let kventry_3 = KVEntry {
        owner_name : "Mary-chan".to_string(),
        value: "Haiiiii".to_string(),
        version: 1,
        timestamp: 12345670,
        origin_node: "node-3".to_string(),
    };

    let node_1 = NodeState{
        node_id: "node_1".to_string(),
        store: Arc::new(Mutex::new(HashMap::new())),  
        peer_nodes: Vec::new(),
        command_queue: Arc::new(Mutex::new(VecDeque::new())),
    };
    let command = vec![
        FunctionCode::Put { key: "k1".to_string(), entry: knentry_1 },
        FunctionCode::Get { key: "k1".to_string() },
        FunctionCode::Print,
    ];

    for cmd in command{
        node_1.enque_command(cmd);
    }
    node_1.execute_queue_command();

}