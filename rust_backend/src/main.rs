use tonic::transport::Server;   //like Flask(app) or express() in Nodejs
use std::net::SocketAddr;    //IP port


mod telemetry;  //logging/metrics setup
mod kv;   //in-process store 
mod rpc;   

use rust_backend::kvstore;  
use kv::InMemoryStore;
use rpc::KvService;
use kvstore::kv_store_server::KvStoreServer;  //import server wrapper Tonic generated

use http::Request;
use tracing::{info_span, Level};
use tower_http::trace::DefaultOnResponse;
use tower_http::LatencyUnit;

#[tokio::main]   //needed to turn main() to async function 

//Result<OK, ERR>, Box<dyn ERR> is a trait object for any error type -> useful for main 
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    telemetry::init();

    // TODO: parse args or set defaults:
    let node_id = "node-1".to_string();
    let port = 50051;  //typical port for testing

    //convert the string into SocketAddr, if fails return error
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse()?;
    
    let svc = KvService::new(node_id, InMemoryStore::new());

    Server::builder()  //start the Tonic server 
        .layer(
            telemetry::grpc_layer() //attach a tower Layer (middleware) to the server 
                    .make_span_with(|req: &Request<_>| {   //create a tracing span 
                info_span!(
                    "grpc_request",
                    method = %req.method(),
                    path   = %req.uri().path(),
                )
            })
            .on_response(   //run when the response finishes
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            )
        )  
        .add_service(KvStoreServer::new(svc))  //register the KvService as the handler
        .serve(addr)   //bind the TCP listener + start accepting gRPC request 
        .await?;   //run until the task complete 

    Ok(())
}