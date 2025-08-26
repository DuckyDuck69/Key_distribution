# Key Distribution System (Rust + gRPC + AWS EC2)

A distributed key–value store written in Rust, using gRPC for RPCs and Raft for replication (library-based). Targets 5–10 nodes in a single AWS AZ.

## Goals & Scope (Sprint)
- Cluster size: 5–10 nodes (single AZ)
- Workload: small values ≤ 1 KB, ~70/30 read/write
- SLA: p95 latency < 50 ms (same-AZ client), “thousands req/s”
- Durability: follower crash → auto-rejoin; no acknowledged-write loss
- Consistency: linearizable reads after write ACK (leader/read-index)
- Out of scope: multi-region, ACLs/tenancy, transactions

## Repo Structure
- `proto/kvstore.proto` — gRPC definitions
- `rust_backend/`
  - `build.rs` — proto codegen
  - `src/`
    - `lib.rs`
    - `kv/` — in-mem store and traits
    - `rpc/` — gRPC service impl
    - `cluster/` — node config/ids
- `docs/` — goals, runbooks, results

## Local Dev (single node)
```bash
cd rust_backend
cargo build
RUST_LOG=info ./target/debug/rust_backend --node-id=node-1 --grpc-port=50051
