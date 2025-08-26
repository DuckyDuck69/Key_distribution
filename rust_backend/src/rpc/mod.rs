// -----------------------------------------------------------------------------
// Notes for Me (future):
//
// Purpose:
// - This file implements the gRPC service (KvStore) defined in proto/kvstore.proto.
// - tonic_build generates the trait (KvStore), server wrapper (KvStoreServer),
//   and message types (PutRequest, GetReply, etc.) into target/.../out/kvstore.rs.
//
// Why functions return Result<Response<...>, Status>:
// - Result<T, E> = Rust’s success/error enum.
//   * Ok(T)  → success, with tonic::Response<YourReply>
//   * Err(E) → failure, with tonic::Status (gRPC error)
//
// What are Request<T> and Response<T>?
// - Provided by the tonic crate.
// - Request<T>: incoming gRPC request wrapper (proto message + metadata).
//   Use req.into_inner() to get the actual proto message.
// - Response<T>: outgoing wrapper around your reply + optional metadata.
//   Create with Response::new(reply).
//
// TL;DR: You implement KvStore trait methods, unwrap the Request, call the store,
//        and return either Ok(Response::new(...)) or Err(Status::...).
// -----------------------------------------------------------------------------


use tonic::{Request, Response, Status};

use crate::kv::InMemoryStore;   

use crate::kvstore;
//Generated types
use crate::kvstore::{
    kv_store_server:: {KvStore, KvStoreServer},
    PutRequest, PutReply, GetRequest, GetReply, DeleteRequest, DeleteReply
};

pub struct KvService {
    pub node_id: String,
    pub store: InMemoryStore,
}

#[tonic::async_trait]
impl kvstore::kv_store_server::KvStore for KvService{

    async fn put(&self, req: Request<PutRequest>) -> Result<tonic::Response<PutReply>, tonic::Status>{
        let msg = req.into_inner();
        let key = msg.key; //extract the key
        let val = msg.value;   //get the value from the key

        //return error when the key is empty
        if key.is_empty(){
            return Err(Status::invalid_argument("key is empty"));
        }
        //call the store 
        let _replaced = self.store.put(&key, val);

        //return the new status as a response
        Ok(Response::new(PutReply {put: true}))
    }

    async fn get(&self, req: Request<GetRequest>) -> Result<tonic::Response<GetReply>, tonic::Status> {
        let key = req.into_inner().key; //extract the key 
        let value_option = self.store.get(&key);  
        //check to see if a key exists, if not return a false status
        match value_option {
            Some(v) => Ok(Response::new(GetReply {found: true, val: v})),
            None => Ok(Response::new(GetReply {found: false, val: vec![]})),
        }
    }

    async fn delete(&self, req: Request<DeleteRequest>) ->  Result<tonic::Response<DeleteReply>, tonic::Status>{
        let key = req.into_inner().key;
        let deleted = self.store.delete(&key);
        Ok(Response::new(DeleteReply {deleted: true}))
    }
}