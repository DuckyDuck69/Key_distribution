// src/telemetry/mod.rs

//create subscriber and format layers
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tower_http::trace::{TraceLayer};
use tower_http::classify::{SharedClassifier, GrpcErrorsAsFailures};


///init(): call once at startup to setup logging
pub fn init(){

    ////Build the filter
    //if env var RUST_LOG exist, use it, if not fall back to default string
    let filter = EnvFilter::try_from_env("RUST_LOG")
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=warn,hyper=warn"));

    ////Build the formating layer
    let fmt_layer = fmt::layer()
                    .json()
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false)
                    .with_thread_ids(false)
                    .with_timer(fmt::time::UtcTime::rfc_3339());  //ISO 8601 timestamp
    
    //Assemble and install subscriber
    tracing_subscriber::registry().with(filter).with(fmt_layer).init();   //the init make this global subscriber
}

////Base gRPC-aware trace layer (uses gRPC status classifier).
pub fn grpc_layer() -> TraceLayer<SharedClassifier<GrpcErrorsAsFailures>> {
    TraceLayer::new_for_grpc()
}





