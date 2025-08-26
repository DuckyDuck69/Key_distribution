Cluster size: 5–10 nodes (single AZ).

Workload: small values ≤1 KB, 70/30 read/write.

SLA: p95 latency < 50 ms (same-AZ client), “thousands req/s”.

Durability: follower crash → rejoins and serves after restart; no acknowledged-write loss.

Consistency: linearizable reads after write ACK (reads via leader/read-index).

Out of scope: multi-region, ACID txns, ACLs.