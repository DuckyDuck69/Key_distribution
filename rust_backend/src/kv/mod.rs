

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type SharedStore = Arc<RwLock<HashMap<String, Vec<u8>>>>;

pub struct InMemoryStore{
    pub inner: SharedStore,
}

impl InMemoryStore{
    pub fn new() -> Self{
        Self { inner: Arc::new(RwLock::new(HashMap::new()))}
    }
    
    ////Read path: many readers can hold the lock concurrently
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let map = self.inner.read().unwrap(); //get read lock
        map.get(key).cloned()   //clone bytes to return ownershup
    }  //end of lock

    ////Write path: exclusive writer
    pub fn put(&self, key: &str, val: Vec<u8>) -> bool{
        let mut map = self.inner.write().unwrap();  //get write lock
        map.insert(key.to_string(), val).is_some() //is_some return true if replaced succeed
    }

    pub fn delete(&self, key:&str) -> bool {
        let mut map = self.inner.write().unwrap();
        map.remove(key).is_some()  //is_some return true if a key exist
    }

}