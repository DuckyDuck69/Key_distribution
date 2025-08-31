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
    PutRequest, PutReply, GetRequest, GetReply, DeleteRequest, DeleteReply
};

use std::time::Instant;
use tracing::{info};

pub struct KvService {
    pub node_id: String,
    pub store: InMemoryStore,
}

impl KvService{
    pub fn new(node_id: String, store: InMemoryStore) -> Self{
        Self {node_id, store}
    }
}

#[tonic::async_trait]
impl kvstore::kv_store_server::KvStore for KvService{

    async fn put(&self, req: Request<PutRequest>) -> Result<tonic::Response<PutReply>, tonic::Status>{
        let t0 = Instant::now(); //start timer

        let msg = req.into_inner();
        let key = msg.key; //extract the key
        let val = msg.value;   //get the value from the key

        //return error when the key is empty
        if key.is_empty(){
            info!(op="put", key="", status="invalid_argument");
            return Err(Status::invalid_argument("key is empty"));
        }
        //call the store 
        let replaced = self.store.put(&key, val);

        //compute latency and log the info span status
        let latency = t0.elapsed().as_millis();

        //return the new status as a response
        match replaced{
            Ok(true) => {
                info!(op="put", key=%key, status="inserted key", latency_ms=%latency);
                Ok(Response::new(PutReply { put: true }))
            }
            Ok(false) =>{
                info!(op="put", key=%key, status="failed to insert key", latency_ms=%latency);
                Ok(Response::new(PutReply { put: false }))
            }
            Err(e) => {
                info!(op="put", key=%key, status="db_error", error=%e.to_string(), latency_ms=%latency);
                Err(Status::internal("database error"))
            }
        }
    }

    async fn get(&self, req: Request<GetRequest>) -> Result<tonic::Response<GetReply>, tonic::Status> {
        let t0 = Instant::now();


        let key = req.into_inner().key; //extract the key 

        if key.is_empty() {
            info!(op="get", key="", status="invalid_argument");
            return Err(Status::invalid_argument("key is empty"));
        }

        let value_option = self.store.get(&key);  

        let latency = t0.elapsed().as_millis();

        //handle sled-backed get 
        match value_option {
            Ok(Some(v)) => {
                info!(op ="get", key = %key, status = "found", latency_ms = %latency);
                Ok(Response::new(GetReply {found: true, val: v}))
            },
            Ok(None) =>{
                info!(op ="get", key = %key, status = "not_found", latency_ms = %latency);
                Ok(Response::new(GetReply {found: false, val: vec![]}))
            } ,
            Err(e) => {
                info!(op ="get", key = %key, status = "db error", latency_ms = %latency);
                Err(Status::internal("database error"))
            }
        }
    }

    async fn delete(&self, req: Request<DeleteRequest>) ->  Result<tonic::Response<DeleteReply>, tonic::Status>{
        let t0 = Instant::now();

        let key = req.into_inner().key;
        let deleted = self.store.delete(&key);

        let latency = t0.elapsed().as_millis();

        match deleted {
            Ok(_) =>{
                info!(op = "delete", key = %key, status = "ok", latency_ms = %latency);
                Ok(Response::new(DeleteReply { deleted: true }))
            }
            Err(e) =>{
                info!(op ="delete", key = %key, status = "db error", latency_ms = %latency);
                Err(Status::internal("database error"))
            }
        }
    }
}