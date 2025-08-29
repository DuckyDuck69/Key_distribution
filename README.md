# Key Distribution System (Rust + gRPC)

A single-node key–value store written in Rust, using gRPC for RPCs. Future work will extend to persistence and Raft-based clustering.

## Goals & Scope
- Current: in-memory KV store with `Put`, `Get`, `Delete`
- Observability: structured logs with op/key/status/latency
- Error policy: invalid_argument (empty key), found/not_found for Get/Delete
- Next: sled persistence → Raft replication → AWS EC2 deployment

## Repo Structure
- `proto/kvstore.proto` — gRPC definitions
- `rust_backend/`
  - `build.rs` — proto codegen
  - `src/`
    - `lib.rs`
    - `kv/` — in-memory store
    - `rpc/` — gRPC service impl
    - `main.rs` — server entrypoint
    - `bin/client.rs` — Rust client for smoke tests

## Local Dev
```powershell
cd rust_backend
cargo build

# Run server
$env:RUST_LOG="info"
.\target\debug\rust_backend.exe --node-id=node-1 --grpc-port=50051

# Run client (new window)
cargo run --bin client
