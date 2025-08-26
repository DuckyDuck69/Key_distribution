use tonic::transport::Server;
use std::net::SocketAddr;


mod telemetry;
mod kv;
mod rpc;

use rust_backend::kvstore;
use kv::InMemoryStore;
use rpc::KvService;
use kvstore::kv_store_server::KvStoreServer;
