use tokio::fs;
use tonic::transport::Server;  
//like Flask(app) or express() in Nodejs
use std::net::SocketAddr;
use std::path::Path;    //IP port


mod telemetry;  //logging/metrics setup
mod kv;   //in-process store 
mod rpc;   

use rust_backend::kvstore;  
use kv::InMemoryStore;
use rpc::KvService;
use kvstore::kv_store_server::KvStoreServer;  //import server wrapper Tonic generated

use http::Request;
use tracing::{info, info_span, Level};
use tower_http::trace::DefaultOnResponse;
use tower_http::LatencyUnit;

#[tokio::main]   //needed to turn main() to async function 

//Result<OK, ERR>, Box<dyn ERR> is a trait object for any error type -> useful for main 
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    ////CLI parse for --data-dir 
    let args: Vec<String> = std::env::args().collect();  //collect CLI tokens into a string vec
    let mut data_dir = String::from("./data"); //set default directory
    let mut i = 0;

    //scan for --data-dir flag and read its value 
    //if the flag appears multiple time, the last time wins because the last one overwrite any other
    while i + 1 < args.len(){
        if args[i] == "--data-dir"{
            data_dir = args[i+1].clone(); 
            i+= 2;  //skip because we already read it 
        }else{
            i+=1;
        }
    }
    
    //ensure the directory exists
    if !Path::new(&data_dir).exists(){  //turn string data_dir to a path 
        fs::create_dir_all(&data_dir)
            .await.map_err(|e| anyhow::anyhow!("failed to create data dir {} {}", data_dir, e))?;
    }

    //log data dir using
    info!("Using data dir: {}", data_dir);

    let db = sled::open(&data_dir)
        .map_err(|e|anyhow::anyhow!("failed to open sled at {}: {}", data_dir, e))?;

    //pass the db to the store 
    let store = InMemoryStore::new(db);


    telemetry::init();

    // TODO: parse args or set defaults:
    let node_id = "node-1".to_string();
    let svc = KvService::new(node_id, store);
    let port = 50051;  //typical port for testing

    //convert the string into SocketAddr, if fails return error
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;

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