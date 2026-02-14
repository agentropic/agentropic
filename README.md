
# Agentropic

[![Crates.io](https://img.shields.io/crates/v/agentropic.svg)](https://crates.io/crates/agentropic)
[![Documentation](https://docs.rs/agentropic/badge.svg)](https://docs.rs/agentropic)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**Agent-Oriented Programming in Rust — Batteries Included**

Agentropic is a comprehensive framework for building autonomous, intelligent multi-agent systems in Rust. This is the main facade crate that re-exports all components of the Agentropic ecosystem, providing everything you need to build production-ready agentic applications.

> ⚠️ **Status: Active Development** — APIs may change before the 1.0 release.

---

## What is Agentropic?

Agentropic enables you to build software systems composed of **autonomous agents** that:

- **Think** — Use BDI (Belief-Desire-Intention) cognitive architecture for reasoning and planning
- **Communicate** — Exchange messages using FIPA-inspired Agent Communication Language (ACL)
- **Coordinate** — Work together using proven multi-agent patterns (hierarchy, swarm, market, coalition, and more)
- **Execute** — Run efficiently on an async Tokio-based runtime with scheduling and fault tolerance

---

## Project Status

| Crate | Status | Description | Repository |
|-------|--------|-------------|------------|
| **agentropic-core** | ✅ Complete | Agent primitives, traits, lifecycle, identity | [Link](https://github.com/agentropic/agentropic-core) |
| **agentropic-messaging** | ✅ Complete | FIPA messaging, routing, mailboxes, protocols | [Link](https://github.com/agentropic/agentropic-messaging) |
| **agentropic-cognition** | ✅ Complete | BDI architecture, reasoning, decision-making | [Link](https://github.com/agentropic/agentropic-cognition) |
| **agentropic-patterns** | ✅ Complete | Hierarchy, swarm, market, coalition, holarchy, blackboard | [Link](https://github.com/agentropic/agentropic-patterns) |
| **agentropic-runtime** | ✅ Complete | Async runtime, scheduler, supervisor, tracing | [Link](https://github.com/agentropic/agentropic-runtime) |
| **agentropic** (facade) | ✅ Complete | Unified re-export of all crates | Updating |
| **agentropic-website** | ✅ Live | Project website | [agentropic.org](https://www.agentropic.org/) |

**Core framework: Complete and compiling.** Now stabilizing toward 1.0.

---

## Quick Start

### Using the Facade (Recommended)
```toml
[dependencies]
agentropic = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```
```rust
use agentropic::prelude::*;

// Everything available from one crate — core, messaging,
// cognition, patterns, and runtime.
```

### Using Individual Crates
```toml
[dependencies]
agentropic-core = "0.1.0"
agentropic-messaging = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```
```rust
use agentropic_core::prelude::*;
use agentropic_messaging::prelude::*;

struct MyAgent {
    id: AgentId,
}

#[async_trait]
impl Agent for MyAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("Agent initialized");
        Ok(())
    }

    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("Agent executing");
        Ok(())
    }

    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("Agent shutting down");
        Ok(())
    }
}
```

---

## Features

### Core (`agentropic-core`)
- Agent trait with lifecycle hooks (initialize, execute, shutdown)
- UUID-based agent identity system
- Agent context and state management
- Error types and result handling

### Messaging (`agentropic-messaging`)
- FIPA-inspired performatives (Inform, Request, Query, Propose, etc.)
- Type-safe message builder with validation
- Async mailboxes with bounded capacity
- Message routing and subscription
- Request-reply protocol

### Cognition (`agentropic-cognition`)
- BDI (Belief-Desire-Intention) cognitive architecture
- Belief base with revision and querying
- Goal reasoning and plan libraries
- Utility-based decision making

### Patterns (`agentropic-patterns`)
- **Hierarchy** — Tree-structured delegation and reporting
- **Swarm** — Emergent behavior with local communication
- **Market** — Auction-based task allocation
- **Coalition** — Dynamic team formation by capability
- **Holarchy** — Recursive nested hierarchies
- **Federation** — Peer-to-peer agent networks
- **Blackboard** — Shared knowledge space for collaboration
- **Team** — Fixed-role cooperative groups

### Runtime (`agentropic-runtime`)
- Async execution engine built on Tokio
- Round-robin scheduler with fair task cycling
- Supervisor trees with configurable backoff
- Agent tracing and instrumentation

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│              agentropic (facade)                    │
│         Unified re-export of all crates             │
└─────────────────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Patterns   │  │   Runtime    │  │  Cognition   │
│ Hierarchy,   │  │  Scheduler,  │  │ BDI, Plans,  │
│ Swarm, Market│  │  Supervisor  │  │  Decisions   │
└──────────────┘  └──────────────┘  └──────────────┘
        │                │                │
        └────────────────┼────────────────┘
                         ↓
        ┌────────────────┼────────────────┐
        ↓                                 ↓
┌──────────────┐                   ┌──────────────┐
│  Messaging   │                   │     Core     │
│ ACL, Routing │                   │ Agent Traits │
│  Mailboxes   │                   │  Lifecycle   │
└──────────────┘                   └──────────────┘
```

---

## Why Agentropic?

### Rust-First Design

Unlike Python-based agent frameworks, Agentropic leverages Rust's:
- **Performance** — Native speed, zero-cost abstractions
- **Safety** — Memory-safe, thread-safe by design
- **Concurrency** — Fearless async/await with Tokio
- **Tooling** — Cargo, clippy, and the Rust ecosystem

### Academic Foundations, Practical Implementation

Based on decades of multi-agent systems research:
- BDI Architecture (Rao & Georgeff)
- FIPA Agent Communication Language
- Contract Net Protocol
- Market-Based Control
- Swarm Intelligence (Kennedy & Eberhart)

---

## Use Cases

### Blockchain & DeFi
- Agent-oriented dApp development
- Autonomous trading and MEV agents
- Cross-chain coordination

### Financial Systems
- Algorithmic trading with risk management
- Portfolio optimization
- Market making and liquidity provision

### Robotics & IoT
- Swarm robotics coordination
- Distributed sensor networks
- Autonomous vehicle fleets

### Enterprise
- Workflow automation
- Supply chain coordination
- Multi-party business processes

---

## Roadmap to 1.0

### Phase 1: Foundation ✅
- Core agent primitives and traits
- Message passing and communication
- BDI cognitive architecture
- Planning and reasoning
- Organizational patterns
- Async runtime with scheduling

### Phase 2: Stabilization (Current)
- API review and consistency pass
- Comprehensive test coverage
- Performance benchmarking
- Documentation and examples

### Phase 3: Production Hardening
- Deployment tooling and CLI
- Monitoring and observability
- Advanced fault tolerance
- Security auditing

### Version 1.0
- Stable API with semver guarantees
- Production-hardened runtime
- Complete documentation

---

## Performance Targets

- **Agent spawn latency**: < 1ms
- **Message passing**: < 10μs latency
- **Throughput**: 100,000+ messages/second
- **Memory**: ~50KB per agent baseline

*Benchmarks will be published as part of the stabilization phase.*

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Ways to Contribute
- Report bugs and open issues
- Suggest features and improvements
- Improve documentation
- Submit code to any crate
- Add tests and benchmarks
- Star the repo and spread the word

### Development Setup
```bash
# Clone the workspace
git clone https://github.com/agentropic/agentropic.git
cd agentropic

# Clone dependency crates
git clone https://github.com/agentropic/agentropic-core.git
git clone https://github.com/agentropic/agentropic-messaging.git
git clone https://github.com/agentropic/agentropic-cognition.git
git clone https://github.com/agentropic/agentropic-patterns.git
git clone https://github.com/agentropic/agentropic-runtime.git

# Build and test everything
cargo build
cargo test
```

---

## Documentation

- **Website**: [agentropic.org](https://www.agentropic.org/)
- **Getting Started**: [Guide](https://www.agentropic.org/docs/getting-started)
- **API Docs**: [docs.rs/agentropic](https://docs.rs/agentropic)
- **GitHub**: [@agentropic](https://github.com/agentropic)

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## Acknowledgments

Agentropic is inspired by decades of research in multi-agent systems:

- **FIPA Standards** — Agent communication protocols
- **BDI Architecture** — Rao & Georgeff
- **Swarm Intelligence** — Kennedy & Eberhart
- **Market-Based Control** — Clearwater
- **Contract Net Protocol** — Smith
- **Erlang/OTP** — Fault tolerance patterns

---

<div align="center">

**Build intelligent, autonomous agents in Rust with Agentropic**

[Website](https://www.agentropic.org/) · [GitHub](https://github.com/agentropic) · [Docs](https://docs.rs/agentropic)

**Made with 🦀 by the Agentropic team**

</div>