# 🧬 Solana Radar: Shared Protocol Buffers

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Protobuf](https://img.shields.io/badge/protobuf-v3-blue.svg)
![gRPC](https://img.shields.io/badge/transport-tonic-green.svg)

**Solana Radar Proto** is the foundational library containing shared data structures and gRPC service definitions for the Solana Radar ecosystem.
It acts as the **Single Source of Truth**, ensuring strict type safety and contract adherence across the Validator Plugin, Ingestor Backend, and Frontend Dashboard.

---

## 🏗 Architecture Role

This crate decouples the system components. By defining schemas here, we guarantee that a change in the data structure is immediately propagated to all services at compile time.

```mermaid
graph TD
    A[Solana Validator Plugin] -->|Uses geyser.proto| B(Shared Proto Lib)
    C[Ingestor Service] -->|Uses geyser.proto| B
    D[API Server] -->|Uses radar.proto| B
    E[Frontend Angular] -->|Generated TS Clients| B