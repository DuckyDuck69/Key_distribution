


use sled::Db;


#[derive(Clone)]
pub struct InMemoryStore{
    db: Db,
}

impl InMemoryStore{
    pub fn new(db: Db) -> Self{
        Self { db }
    }
    
    ////Read path: many readers can hold the lock concurrently
    pub fn get(&self, key: &str) ->  Result<Option<Vec<u8>>, sled::Error> {
        self.db.get(key.as_bytes()) //sled stores as raw u8 bytes
            .map(|opt| opt.map(|v|v.to_vec()))  //conver IVect that sled returns to Vec
    }  //end of lock

    ////Write path: exclusive writer
    pub fn put(&self, key: &str, val: Vec<u8>) -> Result<bool, sled::Error>{
        self.db.insert(key.as_bytes(), val)
                .map(|opt| opt.is_some()) //return true if new value, false if exists/error
    }

    pub fn delete(&self, key:&str) -> Result<bool, sled::Error> {
        self.db.remove(key.as_bytes())
                .map(|opt| opt.is_some()) 
    }

}