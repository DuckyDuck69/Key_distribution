pub mod kvstore {
    tonic::include_proto!("kvstore"); 
}

pub mod kv; //in-mem store
pub mod rpc; //gRPC handlers
pub mod telemetry;